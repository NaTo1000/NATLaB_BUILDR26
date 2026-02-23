"""NATLaB_BUILDR26 – remote build server.

Exposes the BuildEngine over HTTP/JSON so that builds can be executed on a
remote machine and distributed across a network of build agents.

Requires the optional ``flask`` dependency::

    pip install natlab-buildr26[server]
"""
from __future__ import annotations

import json
import os
from pathlib import Path
from typing import Any, Dict, Optional


# ---------------------------------------------------------------------------
# Server
# ---------------------------------------------------------------------------

class BuildServer:
    """HTTP build server that wraps the NATLaB BuildEngine.

    Parameters
    ----------
    host:
        Interface to listen on (default ``0.0.0.0``).
    port:
        TCP port (default ``5026``).
    output_root:
        Root directory for all remote builds (default ``./remote_builds``).
    """

    DEFAULT_PORT = 5026

    def __init__(
        self,
        host: str = "0.0.0.0",
        port: int = DEFAULT_PORT,
        output_root: Optional[Path] = None,
        api_token: Optional[str] = None,
    ) -> None:
        self.host = host
        self.port = port
        self.output_root = Path(output_root or "remote_builds")
        self._api_token = api_token or os.environ.get("NATLAB_SERVER_TOKEN")

    def create_app(self) -> Any:
        """Create and return the Flask WSGI application."""
        try:
            from flask import Flask, request, jsonify, abort
        except ImportError as exc:
            raise ImportError(
                "Flask is required for the build server. "
                "Install it with: pip install natlab-buildr26[server]"
            ) from exc

        from natlab.builder.engine import BuildEngine

        app = Flask("natlab_server")
        engine = BuildEngine()

        def _check_token() -> None:
            if self._api_token:
                auth = request.headers.get("Authorization", "")
                if not auth.startswith("Bearer ") or auth[7:] != self._api_token:
                    abort(401)

        @app.route("/health", methods=["GET"])
        def health():
            return jsonify({"status": "ok", "version": "26.0.0"})

        @app.route("/build", methods=["POST"])
        def build():
            _check_token()
            data: Dict[str, Any] = request.get_json(force=True) or {}
            app_name = data.get("app_name", "RemoteApp")
            app_type = data.get("app_type", "cli")
            platform = data.get("platform", "native")
            output_dir = self.output_root / app_name

            result = engine.build(
                app_name=app_name,
                app_type=app_type,
                output_dir=output_dir,
                platform=platform,
            )
            return jsonify(result.to_dict()), 200 if result.success else 500

        @app.route("/templates", methods=["GET"])
        def templates():
            from natlab.builder.templates import TemplateRegistry
            reg = TemplateRegistry()
            return jsonify(reg.list_templates())

        return app

    def run(self, *, debug: bool = False) -> None:
        """Start the Flask development server (blocking)."""
        flask_app = self.create_app()
        flask_app.run(host=self.host, port=self.port, debug=debug)
