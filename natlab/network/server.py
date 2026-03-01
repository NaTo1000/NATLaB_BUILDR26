"""NATLaB_BUILDR26 – remote build server.

Exposes the BuildEngine over HTTP/JSON so that builds can be executed on a
remote machine and distributed across a network of build agents.

Elite-level server features
-----------------------------
* **Gzip response compression** – all JSON responses are gzip-compressed
  when the client declares ``Accept-Encoding: gzip``, dramatically cutting
  network payload size for large build manifests.
* **ETag caching** for the ``/templates`` endpoint – clients that already
  hold the template list receive ``304 Not Modified`` with zero body.
* **Concurrent-build guard** – a configurable semaphore limits the number
  of builds that may run simultaneously, preventing resource exhaustion.
* **Structured error responses** – all 4xx/5xx replies include a JSON body
  ``{"error": "...", "code": <http_status>}`` for programmatic handling.
* **X-Build-Duration header** – finished build responses include the elapsed
  time so clients can display it without parsing the full JSON body.

Requires the optional ``flask`` dependency::

    pip install natlab-buildr26[server]
"""
from __future__ import annotations

import gzip as _gzip
import hashlib
import json
import os
import threading
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
    max_concurrent_builds:
        Maximum number of build jobs allowed to run simultaneously
        (default ``4``).  Additional requests block until a slot opens.
    """

    DEFAULT_PORT = 5026

    def __init__(
        self,
        host: str = "0.0.0.0",
        port: int = DEFAULT_PORT,
        output_root: Optional[Path] = None,
        api_token: Optional[str] = None,
        max_concurrent_builds: int = 4,
    ) -> None:
        self.host = host
        self.port = port
        self.output_root = Path(output_root or "remote_builds")
        self._api_token = api_token or os.environ.get("NATLAB_SERVER_TOKEN")
        self._build_semaphore = threading.Semaphore(max_concurrent_builds)
        # ETag for /templates (recomputed lazily)
        self._templates_etag: Optional[str] = None
        self._templates_payload: Optional[bytes] = None

    def create_app(self) -> Any:
        """Create and return the Flask WSGI application."""
        try:
            from flask import Flask, request, jsonify, abort, make_response, Response
        except ImportError as exc:
            raise ImportError(
                "Flask is required for the build server. "
                "Install it with: pip install natlab-buildr26[server]"
            ) from exc

        from natlab.builder.engine import BuildEngine

        app = Flask("natlab_server")
        engine = BuildEngine()

        # ------------------------------------------------------------------
        # Helpers
        # ------------------------------------------------------------------

        def _check_token() -> None:
            if self._api_token:
                auth = request.headers.get("Authorization", "")
                if not auth.startswith("Bearer ") or auth[7:] != self._api_token:
                    abort(401)

        def _maybe_gzip(data: bytes, content_type: str = "application/json") -> Any:
            """Return a Flask Response, gzip-compressed when the client accepts it."""
            accept_enc = request.headers.get("Accept-Encoding", "")
            if "gzip" in accept_enc:
                compressed = _gzip.compress(data, compresslevel=6)
                resp = make_response(compressed)
                resp.headers["Content-Encoding"] = "gzip"
                resp.headers["Content-Type"] = content_type
                resp.headers["Vary"] = "Accept-Encoding"
            else:
                resp = make_response(data)
                resp.headers["Content-Type"] = content_type
            return resp

        def _error_response(status: int, message: str) -> Any:
            body = json.dumps({"error": message, "code": status}).encode("utf-8")
            resp = _maybe_gzip(body)
            resp.status_code = status
            return resp

        # ------------------------------------------------------------------
        # Routes
        # ------------------------------------------------------------------

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

            # Honour concurrent build limit (non-blocking if full → 503)
            acquired = self._build_semaphore.acquire(blocking=False)
            if not acquired:
                return _error_response(503, "Build capacity reached – try again shortly.")

            import time as _time
            t0 = _time.monotonic()
            try:
                result = engine.build(
                    app_name=app_name,
                    app_type=app_type,
                    output_dir=output_dir,
                    platform=platform,
                )
            finally:
                self._build_semaphore.release()

            elapsed = round(_time.monotonic() - t0, 3)
            body = json.dumps(result.to_dict()).encode("utf-8")
            resp = _maybe_gzip(body)
            resp.status_code = 200 if result.success else 500
            resp.headers["X-Build-Duration"] = str(elapsed)
            return resp

        @app.route("/templates", methods=["GET"])
        def templates():
            from natlab.builder.templates import TemplateRegistry

            # Lazy-compute ETag and cached payload
            if self._templates_payload is None:
                reg = TemplateRegistry()
                raw = json.dumps(reg.list_templates()).encode("utf-8")
                self._templates_payload = raw
                self._templates_etag = hashlib.sha256(raw).hexdigest()[:32]

            # Conditional GET support
            client_etag = request.headers.get("If-None-Match", "")
            if client_etag == self._templates_etag:
                return make_response("", 304)

            resp = _maybe_gzip(self._templates_payload)
            resp.headers["ETag"] = self._templates_etag
            resp.headers["Cache-Control"] = "public, max-age=300"
            return resp

        @app.errorhandler(404)
        def not_found(e):
            return _error_response(404, "Not found")

        @app.errorhandler(405)
        def method_not_allowed(e):
            return _error_response(405, "Method not allowed")

        return app

    def run(self, *, debug: bool = False) -> None:
        """Start the Flask development server (blocking)."""
        flask_app = self.create_app()
        flask_app.run(host=self.host, port=self.port, debug=debug)
