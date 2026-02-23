"""NATLaB_BUILDR26 – remote build client.

Sends build requests to a :class:`~natlab.network.server.BuildServer` instance
running on a remote machine.
"""
from __future__ import annotations

import json
import urllib.request
import urllib.error
from pathlib import Path
from typing import Any, Dict, Optional


class BuildClient:
    """HTTP client for the NATLaB remote build server.

    Parameters
    ----------
    base_url:
        URL of the build server, e.g. ``http://buildhost:5026``.
    api_token:
        Bearer token for server authentication (optional).
    timeout:
        Request timeout in seconds (default 120).
    """

    def __init__(
        self,
        base_url: str = "http://localhost:5026",
        api_token: Optional[str] = None,
        timeout: int = 120,
    ) -> None:
        self.base_url = base_url.rstrip("/")
        self._token = api_token
        self._timeout = timeout

    # ------------------------------------------------------------------
    # Public API
    # ------------------------------------------------------------------

    def health(self) -> Dict[str, Any]:
        """Check server health.  Returns the JSON response dict."""
        return self._get("/health")

    def list_templates(self) -> list[Dict[str, str]]:
        """Return the list of templates supported by the remote server."""
        return self._get("/templates")

    def build(
        self,
        app_name: str,
        app_type: str = "cli",
        platform: str = "native",
    ) -> Dict[str, Any]:
        """Trigger a remote build.  Returns the JSON result dict."""
        payload = {
            "app_name": app_name,
            "app_type": app_type,
            "platform": platform,
        }
        return self._post("/build", payload)

    # ------------------------------------------------------------------
    # Private helpers
    # ------------------------------------------------------------------

    def _headers(self) -> Dict[str, str]:
        h = {"Content-Type": "application/json", "Accept": "application/json"}
        if self._token:
            h["Authorization"] = f"Bearer {self._token}"
        return h

    def _get(self, path: str) -> Any:
        req = urllib.request.Request(
            self.base_url + path,
            headers=self._headers(),
            method="GET",
        )
        return self._do_request(req)

    def _post(self, path: str, data: Dict[str, Any]) -> Any:
        body = json.dumps(data).encode("utf-8")
        req = urllib.request.Request(
            self.base_url + path,
            data=body,
            headers=self._headers(),
            method="POST",
        )
        return self._do_request(req)

    def _do_request(self, req: urllib.request.Request) -> Any:
        try:
            with urllib.request.urlopen(req, timeout=self._timeout) as resp:  # noqa: S310
                return json.loads(resp.read().decode("utf-8"))
        except urllib.error.HTTPError as exc:
            raise RuntimeError(
                f"Server returned HTTP {exc.code}: {exc.reason}"
            ) from exc
        except urllib.error.URLError as exc:
            raise RuntimeError(f"Connection error: {exc.reason}") from exc
