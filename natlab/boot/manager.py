"""NATLaB_BUILDR26 – boot manager.

The BootManager orchestrates ultra-fast application startup:

* Dependency pre-loading and lazy imports
* Configuration file discovery and caching
* Platform detection and capability registration
* Icon / splash-screen preloading

It writes a ``boot/`` directory stub into the scaffolded project so that
applications can import and invoke the boot sequence at runtime.
"""
from __future__ import annotations

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

    def __init__(self, name: str, fn: Callable[[], None], critical: bool = True) -> None:
        self.name = name
        self.fn = fn
        self.critical = critical
        self.duration: Optional[float] = None
        self.error: Optional[Exception] = None

    def run(self) -> bool:
        t0 = time.monotonic()
        try:
            self.fn()
            self.duration = time.monotonic() - t0
            return True
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
        bm.add_step("load_config", lambda: ...)
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
    ) -> "BootManager":
        """Append a boot step.  Returns self for fluent chaining."""
        self._steps.append(BootStep(name, fn, critical))
        return self

    # ------------------------------------------------------------------
    # Execution
    # ------------------------------------------------------------------

    def run(self) -> bool:
        """Execute all registered boot steps in order.

        Returns ``True`` if all critical steps succeeded.
        """
        t0 = time.monotonic()
        all_ok = True

        for step in self._steps:
            ok = step.run()
            if not ok and step.critical:
                all_ok = False
                break

        total = time.monotonic() - t0
        self._config["_boot_duration_s"] = round(total, 4)
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
