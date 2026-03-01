"""NATLaB_BUILDR26 – core build engine.

The BuildEngine is responsible for orchestrating the full application build
lifecycle: selecting a template, scaffolding files, injecting the boot
manager, applying AI-assisted enhancements, generating icons and producing
a distributable artefact.

Elite-level engine features
----------------------------
* **Parallel independent steps** – boot-manager injection and icon generation
  run concurrently on separate threads, cutting wall-clock build time.
* **Per-step timing** recorded and stored in the build manifest for
  performance diagnostics.
* **Compression stats** from the packager surfaced in build messages.
"""
from __future__ import annotations

import concurrent.futures
import json
import os
import sys
import time
from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple

from natlab.builder.templates import TemplateRegistry
from natlab.builder.packager import Packager


# ---------------------------------------------------------------------------
# Build result
# ---------------------------------------------------------------------------

class BuildResult:
    """Immutable value-object that describes the outcome of a build."""

    def __init__(
        self,
        success: bool,
        output_dir: Path,
        duration_seconds: float,
        messages: list[str],
        artefact: Optional[Path] = None,
        step_timings: Optional[Dict[str, float]] = None,
    ) -> None:
        self.success = success
        self.output_dir = output_dir
        self.duration_seconds = duration_seconds
        self.messages = list(messages)
        self.artefact = artefact
        self.step_timings: Dict[str, float] = step_timings or {}

    def to_dict(self) -> Dict[str, Any]:
        return {
            "success": self.success,
            "output_dir": str(self.output_dir),
            "duration_seconds": round(self.duration_seconds, 3),
            "messages": self.messages,
            "artefact": str(self.artefact) if self.artefact else None,
            "step_timings": {k: round(v, 4) for k, v in self.step_timings.items()},
        }

    def __repr__(self) -> str:  # pragma: no cover
        status = "OK" if self.success else "FAILED"
        return f"<BuildResult {status} output={self.output_dir}>"


# ---------------------------------------------------------------------------
# Build engine
# ---------------------------------------------------------------------------

class BuildEngine:
    """NATLaB_BUILDR26 high-precision build engine.

    Usage::

        engine = BuildEngine()
        result = engine.build(
            app_name="MyApp",
            app_type="desktop",
            output_dir=Path("./dist/MyApp"),
        )
    """

    VERSION = "26.0.0"

    def __init__(
        self,
        registry: Optional[TemplateRegistry] = None,
        packager: Optional[Packager] = None,
        ai_api_key: Optional[str] = None,
    ) -> None:
        self._registry = registry or TemplateRegistry()
        self._packager = packager or Packager()
        self._ai_api_key = ai_api_key or os.environ.get("NATLAB_AI_KEY")

    # ------------------------------------------------------------------
    # Public API
    # ------------------------------------------------------------------

    def build(
        self,
        app_name: str,
        app_type: str = "cli",
        output_dir: Optional[Path] = None,
        *,
        platform: str = "native",
        with_boot_manager: bool = True,
        with_icons: bool = True,
        with_ai: bool = False,
        extra_config: Optional[Dict[str, Any]] = None,
    ) -> BuildResult:
        """Build a complete, distributable application.

        Parameters
        ----------
        app_name:
            Human-readable name for the new application.
        app_type:
            One of ``cli``, ``desktop``, ``web``, ``service``.
        output_dir:
            Destination directory (created if it does not exist).
        platform:
            Target platform string, e.g. ``native``, ``linux``, ``windows``,
            ``darwin``, ``all``.
        with_boot_manager:
            Inject a NATLaB boot-manager stub for instant startup.
        with_icons:
            Generate and allocate application icon set.
        with_ai:
            Apply AI-assisted code enhancements (requires API key).
        extra_config:
            Additional key/value pairs forwarded to the template renderer.
        """
        t0 = time.monotonic()
        messages: list[str] = []
        step_timings: Dict[str, float] = {}

        if output_dir is None:
            output_dir = Path.cwd() / "dist" / app_name

        output_dir = Path(output_dir)
        messages.append(f"NATLaB_BUILDR26 v{self.VERSION} – starting build")
        messages.append(f"  app_name : {app_name}")
        messages.append(f"  app_type : {app_type}")
        messages.append(f"  platform : {platform}")
        messages.append(f"  output   : {output_dir}")

        try:
            # 1. Scaffold from template
            template = self._registry.get(app_type)
            messages.append(f"[1/5] Scaffolding template '{app_type}'")
            t1 = time.monotonic()
            template.render(
                output_dir=output_dir,
                app_name=app_name,
                platform=platform,
                extra=extra_config or {},
            )
            step_timings["scaffold"] = time.monotonic() - t1

            # 2 + 3. Boot manager and icon generation run in parallel (independent)
            parallel_errors: List[Tuple[str, Exception]] = []

            def _boot() -> None:
                if with_boot_manager:
                    messages.append("[2/5] Injecting boot manager")
                    tb = time.monotonic()
                    self._inject_boot_manager(output_dir, app_name)
                    step_timings["boot_manager"] = time.monotonic() - tb
                else:
                    messages.append("[2/5] Boot manager skipped")

            def _icons() -> None:
                if with_icons:
                    messages.append("[3/5] Generating icon set")
                    ti = time.monotonic()
                    self._generate_icons(output_dir, app_name)
                    step_timings["icons"] = time.monotonic() - ti
                else:
                    messages.append("[3/5] Icon generation skipped")

            with concurrent.futures.ThreadPoolExecutor(max_workers=2) as pool:
                futures = {pool.submit(_boot): "boot", pool.submit(_icons): "icons"}
                for fut, label in futures.items():
                    try:
                        fut.result()
                    except Exception as exc:  # noqa: BLE001
                        parallel_errors.append((label, exc))

            if parallel_errors:
                # Log all failures, then raise the first one
                for label, exc in parallel_errors:
                    messages.append(f"  Parallel step '{label}' failed: {exc}")
                raise parallel_errors[0][1]

            # 4. AI enhancements (sequential – depends on scaffolded files)
            if with_ai:
                messages.append("[4/5] Applying AI-assisted enhancements")
                t4 = time.monotonic()
                self._apply_ai_enhancements(output_dir, app_name)
                step_timings["ai"] = time.monotonic() - t4
            else:
                messages.append("[4/5] AI enhancements skipped (no key or disabled)")

            # 5. Package with smart compression
            messages.append("[5/5] Packaging distributable artefact")
            t5 = time.monotonic()
            pack_result = self._packager.pack_full(output_dir, app_name, platform)
            step_timings["package"] = time.monotonic() - t5
            artefact = pack_result.archive_path
            messages.append(
                f"  Compression: {pack_result.ratio * 100:.1f}% "
                f"({pack_result.savings_kb:.1f} KB saved)"
            )
            messages.append(f"  Checksum (SHA-256): {pack_result.checksum_path.name}")

            duration = time.monotonic() - t0
            messages.append(f"Build completed in {duration:.2f}s → {artefact}")

            self._write_build_manifest(
                output_dir, app_name, app_type, platform, messages, step_timings
            )

            return BuildResult(
                success=True,
                output_dir=output_dir,
                duration_seconds=duration,
                messages=messages,
                artefact=artefact,
                step_timings=step_timings,
            )

        except Exception as exc:  # noqa: BLE001
            duration = time.monotonic() - t0
            messages.append(f"Build FAILED: {exc}")
            return BuildResult(
                success=False,
                output_dir=output_dir,
                duration_seconds=duration,
                messages=messages,
                step_timings=step_timings,
            )

    def repair(self, project_dir: Path, *, with_ai: bool = False) -> BuildResult:
        """Attempt to repair an existing NATLaB project.

        The repair process checks the project manifest, re-installs missing
        files from the template and re-runs the packaging step.
        """
        t0 = time.monotonic()
        messages: list[str] = []
        project_dir = Path(project_dir)

        manifest_path = project_dir / ".natlab" / "manifest.json"
        if not manifest_path.exists():
            return BuildResult(
                success=False,
                output_dir=project_dir,
                duration_seconds=time.monotonic() - t0,
                messages=["No NATLaB manifest found – cannot repair."],
            )

        with manifest_path.open() as fh:
            manifest = json.load(fh)

        app_name = manifest.get("app_name", "UnknownApp")
        app_type = manifest.get("app_type", "cli")
        platform = manifest.get("platform", "native")
        messages.append(f"Repairing '{app_name}' ({app_type} / {platform})")

        return self.build(
            app_name=app_name,
            app_type=app_type,
            output_dir=project_dir,
            platform=platform,
            with_ai=with_ai,
        )

    # ------------------------------------------------------------------
    # Private helpers
    # ------------------------------------------------------------------

    def _inject_boot_manager(self, output_dir: Path, app_name: str) -> None:
        from natlab.boot.manager import BootManager
        bm = BootManager(app_name=app_name)
        bm.write_stub(output_dir / "boot")

    def _generate_icons(self, output_dir: Path, app_name: str) -> None:
        from natlab.icon.builder import IconBuilder
        ib = IconBuilder(app_name=app_name)
        ib.build_all(output_dir / "icons")

    def _apply_ai_enhancements(self, output_dir: Path, app_name: str) -> None:
        if not self._ai_api_key:
            raise RuntimeError(
                "AI enhancements requested but no API key provided. "
                "Set NATLAB_AI_KEY or pass ai_api_key= to BuildEngine."
            )
        from natlab.ai.assistant import AIAssistant
        assistant = AIAssistant(api_key=self._ai_api_key)
        assistant.enhance_project(output_dir)

    def _write_build_manifest(
        self,
        output_dir: Path,
        app_name: str,
        app_type: str,
        platform: str,
        messages: list[str],
        step_timings: Optional[Dict[str, float]] = None,
    ) -> None:
        meta_dir = output_dir / ".natlab"
        meta_dir.mkdir(parents=True, exist_ok=True)
        manifest = {
            "natlab_version": self.VERSION,
            "app_name": app_name,
            "app_type": app_type,
            "platform": platform,
            "build_time": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
            "python_version": sys.version,
            "step_timings_s": {k: round(v, 4) for k, v in (step_timings or {}).items()},
            "log": messages,
        }
        with (meta_dir / "manifest.json").open("w") as fh:
            json.dump(manifest, fh, indent=2)
