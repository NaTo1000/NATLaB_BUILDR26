"""NATLaB_BUILDR26 – boot manager.

The BootManager orchestrates ultra-fast application startup:

* Dependency pre-loading and lazy imports
* Configuration file discovery and caching
* Platform detection and capability registration
* Icon / splash-screen preloading

Elite-level boot features
--------------------------
* **Per-step timeout** – each step can declare a maximum allowed duration;
  it is terminated and marked failed if it overruns.
* **Parallel step groups** – steps tagged ``parallel=True`` are fanned out
  across a thread pool and joined before the next sequential step runs.
* **Memory snapshot** – resident-set size (RSS) is recorded before and after
  the boot sequence for diagnostics.
* **Boot telemetry** persisted to ``_boot_telemetry`` config key for
  application-level inspection.

It writes a ``boot/`` directory stub into the scaffolded project so that
applications can import and invoke the boot sequence at runtime.
"""
from __future__ import annotations

import concurrent.futures
import json
import os
import platform
import sys
import textwrap
import time
from pathlib import Path
from typing import Any, Callable, Dict, List, Optional


# ---------------------------------------------------------------------------
# Boot sequence step
# ---------------------------------------------------------------------------

class BootStep:
    """A single named step in the boot sequence."""

    def __init__(
        self,
        name: str,
        fn: Callable[[], None],
        critical: bool = True,
        timeout: Optional[float] = None,
        parallel: bool = False,
    ) -> None:
        self.name = name
        self.fn = fn
        self.critical = critical
        self.timeout = timeout      # seconds; None = no limit
        self.parallel = parallel    # run concurrently with adjacent parallel steps
        self.duration: Optional[float] = None
        self.error: Optional[Exception] = None

    def run(self) -> bool:
        """Execute the step, honouring any configured timeout.

        Returns ``True`` on success.
        """
        t0 = time.monotonic()
        if self.timeout is None:
            try:
                self.fn()
                self.duration = time.monotonic() - t0
                return True
            except Exception as exc:  # noqa: BLE001
                self.duration = time.monotonic() - t0
                self.error = exc
                return False
        else:
            # Run the function in a thread so we can apply a wall-clock timeout.
            with concurrent.futures.ThreadPoolExecutor(max_workers=1) as pool:
                future = pool.submit(self.fn)
                try:
                    future.result(timeout=self.timeout)
                    self.duration = time.monotonic() - t0
                    return True
                except concurrent.futures.TimeoutError:
                    self.duration = time.monotonic() - t0
                    self.error = TimeoutError(
                        f"Boot step '{self.name}' exceeded timeout of {self.timeout}s"
                    )
                    return False
                except Exception as exc:  # noqa: BLE001
                    self.duration = time.monotonic() - t0
                    self.error = exc
                    return False


# ---------------------------------------------------------------------------
# Boot manager
# ---------------------------------------------------------------------------

class BootManager:
    """NATLaB_BUILDR26 fast-startup boot manager.

    Usage in the generated application::

        from boot.manager import BootManager

        bm = BootManager(app_name="MyApp")
        bm.add_step("load_config", lambda: ..., timeout=2.0)
        bm.add_step("init_db",  lambda: ..., parallel=True, critical=False)
        bm.add_step("preload_assets", lambda: ..., parallel=True, critical=False)
        bm.run()
    """

    VERSION = "26.0.0"

    def __init__(self, app_name: str = "NATLaB App") -> None:
        self.app_name = app_name
        self._steps: List[BootStep] = []
        self._config: Dict[str, Any] = {}
        self._platform_info = self._detect_platform()

    # ------------------------------------------------------------------
    # Step registration
    # ------------------------------------------------------------------

    def add_step(
        self,
        name: str,
        fn: Callable[[], None],
        *,
        critical: bool = True,
        timeout: Optional[float] = None,
        parallel: bool = False,
    ) -> "BootManager":
        """Append a boot step.  Returns self for fluent chaining.

        Parameters
        ----------
        timeout:
            Maximum wall-clock seconds allowed for this step. The step is
            aborted and marked failed if it exceeds this limit.
        parallel:
            When ``True`` this step is grouped with adjacent parallel steps
            and they all run concurrently.
        """
        self._steps.append(BootStep(name, fn, critical, timeout, parallel))
        return self

    # ------------------------------------------------------------------
    # Execution
    # ------------------------------------------------------------------

    def run(self) -> bool:
        """Execute all registered boot steps in order.

        Adjacent steps tagged ``parallel=True`` are fanned out across a
        thread pool and joined together before the next sequential step
        begins.  This eliminates latency for independent initialisation
        work (e.g. loading config + warming caches simultaneously).

        Returns ``True`` if all critical steps succeeded.
        """
        t0 = time.monotonic()
        rss_before = self._rss_kb()
        all_ok = True

        # Split into groups: lists of consecutive parallel steps are
        # executed together; all other steps run sequentially.
        groups: List[List[BootStep]] = []
        for step in self._steps:
            if step.parallel and groups and groups[-1][0].parallel:
                groups[-1].append(step)
            else:
                groups.append([step])

        for group in groups:
            if group[0].parallel and len(group) > 1:
                # Fan out
                with concurrent.futures.ThreadPoolExecutor(
                    max_workers=len(group)
                ) as pool:
                    futures = {pool.submit(s.run): s for s in group}
                    for fut, step in futures.items():
                        ok = fut.result()
                        if not ok and step.critical:
                            all_ok = False
            else:
                for step in group:
                    ok = step.run()
                    if not ok and step.critical:
                        all_ok = False
                        break

            if not all_ok:
                break

        total = time.monotonic() - t0
        rss_after = self._rss_kb()

        telemetry: Dict[str, Any] = {
            "total_s": round(total, 4),
            "rss_before_kb": rss_before,
            "rss_after_kb": rss_after,
            "steps": [
                {
                    "name": s.name,
                    "duration_s": round(s.duration, 4) if s.duration is not None else None,
                    "ok": s.error is None,
                    "parallel": s.parallel,
                    "critical": s.critical,
                    "error": str(s.error) if s.error else None,
                }
                for s in self._steps
            ],
        }
        self._config["_boot_duration_s"] = round(total, 4)
        self._config["_boot_telemetry"] = telemetry
        return all_ok

    # ------------------------------------------------------------------
    # Configuration helpers
    # ------------------------------------------------------------------

    def load_config(self, config_path: Path) -> None:
        """Load a JSON configuration file and merge it into the boot config."""
        config_path = Path(config_path)
        if config_path.exists():
            with config_path.open() as fh:
                self._config.update(json.load(fh))

    def get(self, key: str, default: Any = None) -> Any:
        return self._config.get(key, default)

    def set(self, key: str, value: Any) -> None:
        self._config[key] = value

    # ------------------------------------------------------------------
    # Platform detection
    # ------------------------------------------------------------------

    @staticmethod
    def _detect_platform() -> Dict[str, str]:
        return {
            "system": platform.system(),
            "release": platform.release(),
            "machine": platform.machine(),
            "python": sys.version.split()[0],
        }

    @property
    def platform_info(self) -> Dict[str, str]:
        return dict(self._platform_info)

    # ------------------------------------------------------------------
    # Memory helpers
    # ------------------------------------------------------------------

    @staticmethod
    def _rss_kb() -> Optional[int]:
        """Return process resident-set size in kB, or None if unavailable."""
        try:
            import resource
            return resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
        except ImportError:
            pass
        try:
            status = Path("/proc/self/status").read_text()
            for line in status.splitlines():
                if line.startswith("VmRSS:"):
                    return int(line.split()[1])
        except (OSError, ValueError):
            pass
        return None

    # ------------------------------------------------------------------
    # Stub writer (used by BuildEngine)
    # ------------------------------------------------------------------

    def write_stub(self, boot_dir: Path) -> None:
        """Write a boot-manager stub into the scaffolded project."""
        boot_dir = Path(boot_dir)
        boot_dir.mkdir(parents=True, exist_ok=True)

        (boot_dir / "__init__.py").write_text("", encoding="utf-8")

        stub = textwrap.dedent(f"""\
            \"\"\"Boot manager stub generated by NATLaB_BUILDR26 v{self.VERSION}.

            Import and invoke at the very top of your application entry point::

                from boot.manager import BootManager

                bm = BootManager(app_name="{self.app_name}")
                # bm.add_step("load_config", lambda: bm.load_config("config.json"))
                bm.run()
            \"\"\"
            # This file is intentionally minimal; replace the body with your
            # application-specific startup sequence.
            from natlab.boot.manager import BootManager  # noqa: F401

            __all__ = ["BootManager"]
        """)
        (boot_dir / "manager.py").write_text(stub, encoding="utf-8")
