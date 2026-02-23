"""NATLaB_BUILDR26 – command-line interface.

Usage
-----
    natlab build  MyApp --type desktop --platform linux
    natlab repair ./path/to/project
    natlab templates
    natlab icons   --name MyApp --output ./icons
    natlab server  --port 5026
    natlab version
"""
from __future__ import annotations

import os
import sys
from pathlib import Path
from typing import Optional

import click

from natlab import __version__

try:
    from rich.console import Console
    from rich.table import Table
    from rich.panel import Panel
    _console = Console()
    _err_console = Console(stderr=True)
    HAS_RICH = True
except ImportError:  # pragma: no cover
    HAS_RICH = False


def _print(msg: str, style: str = "") -> None:
    if HAS_RICH:
        _console.print(msg, style=style)
    else:
        click.echo(msg)


def _print_err(msg: str) -> None:
    if HAS_RICH:
        _err_console.print(f"[red]ERROR:[/red] {msg}")
    else:
        click.echo(f"ERROR: {msg}", err=True)


# ---------------------------------------------------------------------------
# Main group
# ---------------------------------------------------------------------------

@click.group()
@click.version_option(__version__, prog_name="natlab")
def main() -> None:
    """NATLaB_BUILDR26 – high-precision cross-platform app builder."""


# ---------------------------------------------------------------------------
# build
# ---------------------------------------------------------------------------

@main.command()
@click.argument("app_name")
@click.option("--type", "app_type", default="cli",
              type=click.Choice(["cli", "desktop", "web", "service"], case_sensitive=False),
              show_default=True, help="Application type.")
@click.option("--output", "-o", default=None, type=click.Path(), help="Output directory.")
@click.option("--platform", "-p", default="native",
              help="Target platform: native|linux|windows|darwin|all")
@click.option("--no-boot", is_flag=True, default=False, help="Skip boot manager injection.")
@click.option("--no-icons", is_flag=True, default=False, help="Skip icon generation.")
@click.option("--ai", is_flag=True, default=False, help="Enable AI-assisted enhancements.")
@click.option("--ai-key", default=None, envvar="NATLAB_AI_KEY",
              help="AI API key (or set NATLAB_AI_KEY).")
def build(
    app_name: str,
    app_type: str,
    output: Optional[str],
    platform: str,
    no_boot: bool,
    no_icons: bool,
    ai: bool,
    ai_key: Optional[str],
) -> None:
    """Build a new application called APP_NAME."""
    from natlab.builder.engine import BuildEngine

    output_dir = Path(output) if output else None
    engine = BuildEngine(ai_api_key=ai_key)

    _print(
        f"\n[bold cyan]NATLaB_BUILDR26 v{__version__}[/bold cyan] – building "
        f"[bold]{app_name}[/bold] ({app_type})" if HAS_RICH
        else f"\nNATLaB_BUILDR26 v{__version__} – building {app_name} ({app_type})"
    )

    result = engine.build(
        app_name=app_name,
        app_type=app_type,
        output_dir=output_dir,
        platform=platform,
        with_boot_manager=not no_boot,
        with_icons=not no_icons,
        with_ai=ai,
    )

    for msg in result.messages:
        _print(msg)

    if result.success:
        _print(
            f"\n[bold green]✔ Build succeeded[/bold green] in "
            f"{result.duration_seconds:.2f}s" if HAS_RICH
            else f"\n✔ Build succeeded in {result.duration_seconds:.2f}s"
        )
        if result.artefact:
            _print(f"  Artefact : {result.artefact}")
    else:
        _print_err("Build failed. See messages above.")
        sys.exit(1)


# ---------------------------------------------------------------------------
# repair
# ---------------------------------------------------------------------------

@main.command()
@click.argument("project_dir", type=click.Path(exists=True, file_okay=False))
@click.option("--ai", is_flag=True, default=False, help="Enable AI-assisted repair.")
@click.option("--ai-key", default=None, envvar="NATLAB_AI_KEY", help="AI API key.")
def repair(project_dir: str, ai: bool, ai_key: Optional[str]) -> None:
    """Repair an existing NATLaB project located at PROJECT_DIR."""
    from natlab.builder.engine import BuildEngine

    engine = BuildEngine(ai_api_key=ai_key)
    _print(f"\nRepairing project: {project_dir}")

    result = engine.repair(Path(project_dir), with_ai=ai)

    for msg in result.messages:
        _print(msg)

    if result.success:
        _print("\n✔ Repair succeeded." if not HAS_RICH else "\n[bold green]✔ Repair succeeded.[/bold green]")
    else:
        _print_err("Repair failed. See messages above.")
        sys.exit(1)


# ---------------------------------------------------------------------------
# templates
# ---------------------------------------------------------------------------

@main.command("templates")
def list_templates() -> None:
    """List all available application templates."""
    from natlab.builder.templates import TemplateRegistry

    reg = TemplateRegistry()
    templates = reg.list_templates()

    if HAS_RICH:
        table = Table(title="Available Templates", show_header=True, header_style="bold cyan")
        table.add_column("Name", style="bold")
        table.add_column("Description")
        for t in templates:
            table.add_row(t["name"], t["description"])
        _console.print(table)
    else:
        for t in templates:
            click.echo(f"  {t['name']:12s}  {t['description']}")


# ---------------------------------------------------------------------------
# icons
# ---------------------------------------------------------------------------

@main.command()
@click.option("--name", "-n", required=True, help="Application name for icon label.")
@click.option("--output", "-o", default="./icons", show_default=True,
              type=click.Path(), help="Output directory.")
@click.option("--color", default="#2563EB", show_default=True, help="Primary hex colour.")
@click.option("--text-color", default="#FFFFFF", show_default=True, help="Label hex colour.")
def icons(name: str, output: str, color: str, text_color: str) -> None:
    """Generate a full icon set for an application."""
    from natlab.icon.builder import IconBuilder

    out_dir = Path(output)
    builder = IconBuilder(app_name=name, primary_color=color, text_color=text_color)
    _print(f"\nGenerating icons for '{name}' → {out_dir}")
    created = builder.build_all(out_dir)
    _print(f"  Created {len(created)} icon file(s).")
    for p in created[:10]:
        _print(f"  • {p}")
    if len(created) > 10:
        _print(f"  … and {len(created) - 10} more.")


# ---------------------------------------------------------------------------
# server
# ---------------------------------------------------------------------------

@main.command()
@click.option("--host", default="0.0.0.0", show_default=True)
@click.option("--port", default=5026, show_default=True, type=int)
@click.option("--token", default=None, envvar="NATLAB_SERVER_TOKEN",
              help="Bearer token for API authentication.")
@click.option("--debug", is_flag=True, default=False)
def server(host: str, port: int, token: Optional[str], debug: bool) -> None:
    """Start the NATLaB remote build server."""
    from natlab.network.server import BuildServer

    srv = BuildServer(host=host, port=port, api_token=token)
    _print(
        f"\n[bold cyan]NATLaB_BUILDR26 Build Server[/bold cyan] listening on "
        f"[bold]{host}:{port}[/bold]" if HAS_RICH
        else f"\nNATLaB_BUILDR26 Build Server listening on {host}:{port}"
    )
    srv.run(debug=debug)


# ---------------------------------------------------------------------------
# version
# ---------------------------------------------------------------------------

@main.command()
def version() -> None:
    """Show NATLaB_BUILDR26 version information."""
    import platform as _platform

    if HAS_RICH:
        panel = Panel(
            f"[bold cyan]NATLaB_BUILDR26[/bold cyan]  v{__version__}\n"
            f"Python  {_platform.python_version()}\n"
            f"OS      {_platform.system()} {_platform.release()}",
            title="Version Info",
            expand=False,
        )
        _console.print(panel)
    else:
        click.echo(f"NATLaB_BUILDR26 v{__version__}")
        click.echo(f"Python  {_platform.python_version()}")
        click.echo(f"OS      {_platform.system()} {_platform.release()}")
