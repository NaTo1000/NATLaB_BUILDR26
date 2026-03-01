"""NATLaB_BUILDR26 – remote build client.

Sends build requests to a :class:`~natlab.network.server.BuildServer` instance
running on a remote machine.

Elite-level client features
-----------------------------
* **Gzip Accept-Encoding** – requests compressed responses, reducing inbound
  bandwidth on large build result payloads.
* **HTTP keep-alive connection pooling** – reuses the underlying TCP
  connection across multiple requests to the same host, eliminating
  repeated TLS/TCP handshakes.
* **Exponential-backoff retry** – transparently retries on transient
  network errors (connection refused, 502, 503, 504) with jitter.
* **Bandwidth stats** – :attr:`bytes_received` and :attr:`bytes_sent`
  track cumulative transfer volume for the lifetime of the client.
"""
from __future__ import annotations

import gzip
import http.client
import json
import random
import time
import urllib.parse
from typing import Any, Dict, Optional


_MAX_RETRIES = 3
_RETRY_BASE_DELAY = 0.5
_RETRY_MAX_DELAY = 8.0
_RETRYABLE_HTTP = {502, 503, 504}


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
        parsed = urllib.parse.urlparse(base_url.rstrip("/"))
        self._scheme = parsed.scheme
        self._host = parsed.netloc
        self._prefix = parsed.path
        self._token = api_token
        self._timeout = timeout
        self._conn: Optional[http.client.HTTPConnection] = None
        # Bandwidth counters (bytes)
        self.bytes_sent: int = 0
        self.bytes_received: int = 0

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

    def close(self) -> None:
        """Close the persistent connection."""
        if self._conn is not None:
            self._conn.close()
            self._conn = None

    def __enter__(self) -> "BuildClient":
        return self

    def __exit__(self, *_: Any) -> None:
        self.close()

    # ------------------------------------------------------------------
    # Private helpers
    # ------------------------------------------------------------------

    def _connection(self) -> http.client.HTTPConnection:
        if self._conn is None:
            if self._scheme == "https":
                import http.client as hc
                self._conn = hc.HTTPSConnection(self._host, timeout=self._timeout)
            else:
                self._conn = http.client.HTTPConnection(self._host, timeout=self._timeout)
        return self._conn

    def _base_headers(self) -> Dict[str, str]:
        h: Dict[str, str] = {
            "Content-Type": "application/json",
            "Accept": "application/json",
            "Accept-Encoding": "gzip",
            "Connection": "keep-alive",
        }
        if self._token:
            h["Authorization"] = f"Bearer {self._token}"
        return h

    def _get(self, path: str) -> Any:
        return self._do_request("GET", path, body=None)

    def _post(self, path: str, data: Dict[str, Any]) -> Any:
        return self._do_request("POST", path, body=json.dumps(data).encode("utf-8"))

    def _do_request(self, method: str, path: str, body: Optional[bytes]) -> Any:
        full_path = self._prefix + path
        headers = self._base_headers()
        if body:
            headers["Content-Length"] = str(len(body))

        last_exc: Optional[Exception] = None

        for attempt in range(_MAX_RETRIES):
            try:
                conn = self._connection()
                conn.request(method, full_path, body=body, headers=headers)
                self.bytes_sent += len(body) if body else 0

                resp = conn.getresponse()
                raw = resp.read()
                self.bytes_received += len(raw)

                if resp.getheader("Content-Encoding") == "gzip":
                    raw = gzip.decompress(raw)

                if resp.status in _RETRYABLE_HTTP and attempt < _MAX_RETRIES - 1:
                    self._conn = None  # force reconnect
                    delay = min(_RETRY_BASE_DELAY * (2 ** attempt) + random.uniform(0, 1), _RETRY_MAX_DELAY)
                    time.sleep(delay)
                    continue

                if not (200 <= resp.status < 300):
                    raise RuntimeError(f"Server returned HTTP {resp.status}: {resp.reason}")

                return json.loads(raw.decode("utf-8"))

            except (ConnectionRefusedError, ConnectionResetError, OSError) as exc:
                # Force fresh connection on next attempt
                self._conn = None
                last_exc = exc
                if attempt < _MAX_RETRIES - 1:
                    delay = min(_RETRY_BASE_DELAY * (2 ** attempt) + random.uniform(0, 0.5), _RETRY_MAX_DELAY)
                    time.sleep(delay)
                    continue
                raise RuntimeError(f"Connection error: {exc}") from exc

        raise RuntimeError(f"Request failed after {_MAX_RETRIES} attempts") from last_exc
