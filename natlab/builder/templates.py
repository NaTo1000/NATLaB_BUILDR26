"""NATLaB_BUILDR26 – application templates.

Templates define the skeleton file tree that the build engine scaffolds for
each supported application type.  New templates can be registered at runtime
via :class:`TemplateRegistry`.
"""
from __future__ import annotations

import os
import textwrap
from pathlib import Path
from typing import Any, Dict


# ---------------------------------------------------------------------------
# Base template
# ---------------------------------------------------------------------------

class AppTemplate:
    """Abstract base class for application templates."""

    name: str = "base"
    description: str = "Generic NATLaB application template"

    def render(
        self,
        output_dir: Path,
        app_name: str,
        platform: str = "native",
        extra: Optional[Dict[str, Any]] = None,
    ) -> None:
        """Scaffold the application skeleton into *output_dir*."""
        output_dir.mkdir(parents=True, exist_ok=True)
        self._write_files(output_dir, app_name, platform, extra or {})

    def _write_files(
        self,
        output_dir: Path,
        app_name: str,
        platform: str,
        extra: Dict[str, Any],
    ) -> None:
        raise NotImplementedError

    # ------------------------------------------------------------------
    # Helpers
    # ------------------------------------------------------------------

    @staticmethod
    def _write(path: Path, content: str) -> None:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(textwrap.dedent(content), encoding="utf-8")


# ---------------------------------------------------------------------------
# Concrete templates
# ---------------------------------------------------------------------------

class CLITemplate(AppTemplate):
    """Scaffold for a cross-platform command-line application."""

    name = "cli"
    description = "Cross-platform command-line application"

    def _write_files(self, output_dir, app_name, platform, extra):
        safe = app_name.lower().replace(" ", "_")

        self._write(
            output_dir / "src" / f"{safe}" / "__init__.py",
            f'"""{ app_name } – CLI application built with NATLaB_BUILDR26."""\n'
            f'__version__ = "1.0.0"\n',
        )
        self._write(
            output_dir / "src" / f"{safe}" / "main.py",
            f"""\
            \"\"\"Entry point for {app_name}.\"\"\"
            import sys


            def main(argv=None):
                print("Welcome to {app_name}!")
                return 0


            if __name__ == "__main__":
                sys.exit(main())
            """,
        )
        self._write(
            output_dir / "pyproject.toml",
            f"""\
            [build-system]
            requires = ["setuptools>=68", "wheel"]
            build-backend = "setuptools.build_meta"

            [project]
            name = "{safe}"
            version = "1.0.0"
            description = "Built with NATLaB_BUILDR26"
            requires-python = ">=3.9"

            [project.scripts]
            {safe} = "{safe}.main:main"
            """,
        )
        self._write(
            output_dir / "README.md",
            f"# {app_name}\n\nBuilt with [NATLaB_BUILDR26](https://github.com/NaTo1000/NATLaB_BUILDR26).\n",
        )


class DesktopTemplate(AppTemplate):
    """Scaffold for a cross-platform desktop (GUI) application."""

    name = "desktop"
    description = "Cross-platform desktop GUI application"

    def _write_files(self, output_dir, app_name, platform, extra):
        safe = app_name.lower().replace(" ", "_")

        self._write(
            output_dir / "src" / f"{safe}" / "__init__.py",
            f'"""{ app_name } – Desktop application built with NATLaB_BUILDR26."""\n'
            f'__version__ = "1.0.0"\n',
        )
        self._write(
            output_dir / "src" / f"{safe}" / "app.py",
            f"""\
            \"\"\"Desktop app entry point for {app_name}.\"\"\"
            import sys
            try:
                import tkinter as tk

                def run():
                    root = tk.Tk()
                    root.title("{app_name}")
                    root.geometry("800x600")
                    label = tk.Label(root, text="Welcome to {app_name}!", font=("Arial", 20))
                    label.pack(expand=True)
                    root.mainloop()

            except ImportError:
                def run():
                    print("GUI runtime not available. Running in headless mode.")

            if __name__ == "__main__":
                run()
            """,
        )
        self._write(
            output_dir / "src" / f"{safe}" / "main.py",
            f"""\
            \"\"\"Entry point for {app_name}.\"\"\"
            from {safe}.app import run
            import sys


            def main(argv=None):
                run()
                return 0


            if __name__ == "__main__":
                sys.exit(main())
            """,
        )
        self._write(
            output_dir / "pyproject.toml",
            f"""\
            [build-system]
            requires = ["setuptools>=68", "wheel"]
            build-backend = "setuptools.build_meta"

            [project]
            name = "{safe}"
            version = "1.0.0"
            description = "Built with NATLaB_BUILDR26"
            requires-python = ">=3.9"

            [project.scripts]
            {safe} = "{safe}.main:main"
            """,
        )
        self._write(
            output_dir / "README.md",
            f"# {app_name}\n\nDesktop application built with [NATLaB_BUILDR26](https://github.com/NaTo1000/NATLaB_BUILDR26).\n",
        )


class WebTemplate(AppTemplate):
    """Scaffold for a cross-platform web / service application."""

    name = "web"
    description = "Cross-platform web application (Flask)"

    def _write_files(self, output_dir, app_name, platform, extra):
        safe = app_name.lower().replace(" ", "_")

        self._write(
            output_dir / "src" / f"{safe}" / "__init__.py",
            f'"""{ app_name } – Web application built with NATLaB_BUILDR26."""\n'
            f'__version__ = "1.0.0"\n',
        )
        self._write(
            output_dir / "src" / f"{safe}" / "app.py",
            f"""\
            \"\"\"Flask web application for {app_name}.\"\"\"
            from flask import Flask

            app = Flask(__name__)


            @app.route("/")
            def index():
                return "<h1>Welcome to {app_name}</h1><p>Built with NATLaB_BUILDR26.</p>"


            def create_app():
                return app
            """,
        )
        self._write(
            output_dir / "src" / f"{safe}" / "main.py",
            f"""\
            \"\"\"Entry point for {app_name}.\"\"\"
            from {safe}.app import create_app
            import sys


            def main(argv=None):
                application = create_app()
                application.run(host="0.0.0.0", port=8080, debug=False)
                return 0


            if __name__ == "__main__":
                sys.exit(main())
            """,
        )
        self._write(
            output_dir / "requirements.txt",
            "flask>=3.0\ngunicorn>=21.0\n",
        )
        self._write(
            output_dir / "pyproject.toml",
            f"""\
            [build-system]
            requires = ["setuptools>=68", "wheel"]
            build-backend = "setuptools.build_meta"

            [project]
            name = "{safe}"
            version = "1.0.0"
            description = "Built with NATLaB_BUILDR26"
            requires-python = ">=3.9"
            dependencies = ["flask>=3.0"]

            [project.scripts]
            {safe} = "{safe}.main:main"
            """,
        )
        self._write(
            output_dir / "README.md",
            f"# {app_name}\n\nWeb application built with [NATLaB_BUILDR26](https://github.com/NaTo1000/NATLaB_BUILDR26).\n",
        )


class ServiceTemplate(AppTemplate):
    """Scaffold for a background service / daemon."""

    name = "service"
    description = "Background service / daemon"

    def _write_files(self, output_dir, app_name, platform, extra):
        safe = app_name.lower().replace(" ", "_")

        self._write(
            output_dir / "src" / f"{safe}" / "__init__.py",
            f'"""{ app_name } – Service built with NATLaB_BUILDR26."""\n'
            f'__version__ = "1.0.0"\n',
        )
        self._write(
            output_dir / "src" / f"{safe}" / "service.py",
            f"""\
            \"\"\"Service loop for {app_name}.\"\"\"
            import time
            import signal
            import sys

            _running = True


            def _shutdown(signum, frame):
                global _running
                _running = False


            def run():
                signal.signal(signal.SIGTERM, _shutdown)
                signal.signal(signal.SIGINT, _shutdown)
                print("{app_name} service started.")
                while _running:
                    time.sleep(1)
                print("{app_name} service stopped.")
            """,
        )
        self._write(
            output_dir / "src" / f"{safe}" / "main.py",
            f"""\
            \"\"\"Entry point for {app_name} service.\"\"\"
            from {safe}.service import run
            import sys


            def main(argv=None):
                run()
                return 0


            if __name__ == "__main__":
                sys.exit(main())
            """,
        )
        self._write(
            output_dir / "pyproject.toml",
            f"""\
            [build-system]
            requires = ["setuptools>=68", "wheel"]
            build-backend = "setuptools.build_meta"

            [project]
            name = "{safe}"
            version = "1.0.0"
            description = "Built with NATLaB_BUILDR26"
            requires-python = ">=3.9"

            [project.scripts]
            {safe} = "{safe}.main:main"
            """,
        )
        self._write(
            output_dir / "README.md",
            f"# {app_name}\n\nService built with [NATLaB_BUILDR26](https://github.com/NaTo1000/NATLaB_BUILDR26).\n",
        )


# ---------------------------------------------------------------------------
# Registry
# ---------------------------------------------------------------------------

# Keep the Optional import that the base class references
from typing import Optional  # noqa: E402


class TemplateRegistry:
    """Registry of available application templates."""

    def __init__(self) -> None:
        self._templates: Dict[str, AppTemplate] = {}
        for cls in (CLITemplate, DesktopTemplate, WebTemplate, ServiceTemplate):
            t = cls()
            self._templates[t.name] = t

    def register(self, template: AppTemplate) -> None:
        """Register a custom template."""
        self._templates[template.name] = template

    def get(self, name: str) -> AppTemplate:
        """Return a template by name or raise :class:`KeyError`."""
        if name not in self._templates:
            raise KeyError(
                f"Unknown template '{name}'. "
                f"Available: {', '.join(sorted(self._templates))}"
            )
        return self._templates[name]

    def list_templates(self) -> list[Dict[str, str]]:
        return [
            {"name": t.name, "description": t.description}
            for t in self._templates.values()
        ]
