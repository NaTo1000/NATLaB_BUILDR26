"""NATLaB_BUILDR26 – AI-assisted code enhancement.

The AIAssistant wraps an external LLM API (configurable) to provide:

* Code quality improvement suggestions for scaffolded files
* Automated repair of common anti-patterns
* Documentation generation
* Dependency optimisation hints

The API key is validated on construction and stored securely in memory only
(never written to disk).
"""
from __future__ import annotations

import hashlib
import json
import os
from pathlib import Path
from typing import Any, Dict, Iterator, List, Optional
import urllib.request
import urllib.error


# ---------------------------------------------------------------------------
# Key validation
# ---------------------------------------------------------------------------

def _mask(key: str) -> str:
    """Return a redacted representation of an API key for logging."""
    if len(key) <= 8:
        return "***"
    return key[:4] + "***" + key[-4:]


# ---------------------------------------------------------------------------
# AI Assistant
# ---------------------------------------------------------------------------

class AIAssistant:
    """NATLaB AI assistant – API-key-activated code enhancement engine.

    Parameters
    ----------
    api_key:
        API key for the configured AI provider.  May also be supplied via the
        ``NATLAB_AI_KEY`` environment variable.
    api_url:
        Base URL for the AI provider endpoint.  Defaults to the OpenAI
        chat-completion API.  Override with ``NATLAB_AI_URL``.
    model:
        Model identifier.  Defaults to ``gpt-4o-mini``.
    """

    DEFAULT_API_URL = "https://api.openai.com/v1/chat/completions"
    DEFAULT_MODEL = "gpt-4o-mini"

    def __init__(
        self,
        api_key: Optional[str] = None,
        api_url: Optional[str] = None,
        model: Optional[str] = None,
    ) -> None:
        key = api_key or os.environ.get("NATLAB_AI_KEY", "")
        if not key:
            raise ValueError(
                "No AI API key provided. "
                "Pass api_key= or set the NATLAB_AI_KEY environment variable."
            )
        self._api_key = key
        self._api_url = api_url or os.environ.get("NATLAB_AI_URL", self.DEFAULT_API_URL)
        self._model = model or os.environ.get("NATLAB_AI_MODEL", self.DEFAULT_MODEL)
        self._key_hash = hashlib.sha256(key.encode()).hexdigest()[:16]

    # ------------------------------------------------------------------
    # Public API
    # ------------------------------------------------------------------

    def enhance_project(self, project_dir: Path) -> List[Dict[str, Any]]:
        """Iterate over Python source files and suggest AI enhancements.

        Returns a list of result dicts (one per file) with keys:
        ``file``, ``suggestions``, ``applied``.
        """
        project_dir = Path(project_dir)
        results: List[Dict[str, Any]] = []

        for py_file in sorted(project_dir.rglob("*.py")):
            if ".natlab" in py_file.parts:
                continue
            result = self._enhance_file(py_file)
            results.append(result)

        return results

    def query(self, prompt: str) -> str:
        """Send an arbitrary prompt to the AI and return the response text.

        Raises :class:`RuntimeError` on API errors.
        """
        return self._call_api(prompt)

    def generate_docs(self, source_code: str) -> str:
        """Generate docstrings and inline documentation for *source_code*."""
        prompt = (
            "Add clear, concise docstrings and inline comments to the "
            "following Python code. Return only the updated code:\n\n"
            f"```python\n{source_code}\n```"
        )
        return self._call_api(prompt)

    def suggest_improvements(self, source_code: str) -> str:
        """Return improvement suggestions for *source_code* as plain text."""
        prompt = (
            "Review the following Python code and list concrete, actionable "
            "improvement suggestions (security, performance, readability):\n\n"
            f"```python\n{source_code}\n```"
        )
        return self._call_api(prompt)

    # ------------------------------------------------------------------
    # Private helpers
    # ------------------------------------------------------------------

    def _enhance_file(self, path: Path) -> Dict[str, Any]:
        source = path.read_text(encoding="utf-8")
        try:
            suggestions = self.suggest_improvements(source)
            return {"file": str(path), "suggestions": suggestions, "applied": False}
        except Exception as exc:  # noqa: BLE001
            return {"file": str(path), "suggestions": None, "error": str(exc)}

    def _call_api(self, prompt: str) -> str:
        """Make an HTTP POST to the AI API endpoint."""
        payload = json.dumps({
            "model": self._model,
            "messages": [{"role": "user", "content": prompt}],
            "max_tokens": 1024,
        }).encode("utf-8")

        req = urllib.request.Request(
            self._api_url,
            data=payload,
            headers={
                "Content-Type": "application/json",
                "Authorization": f"Bearer {self._api_key}",
            },
            method="POST",
        )

        try:
            with urllib.request.urlopen(req, timeout=30) as resp:  # noqa: S310
                body = json.loads(resp.read().decode("utf-8"))
                return body["choices"][0]["message"]["content"]
        except urllib.error.HTTPError as exc:
            raise RuntimeError(
                f"AI API request failed with HTTP {exc.code}: {exc.reason}"
            ) from exc
        except urllib.error.URLError as exc:
            raise RuntimeError(f"AI API connection error: {exc.reason}") from exc
        except (KeyError, IndexError, json.JSONDecodeError) as exc:
            raise RuntimeError(f"Unexpected AI API response format: {exc}") from exc

    def __repr__(self) -> str:  # pragma: no cover
        return (
            f"<AIAssistant model={self._model} "
            f"key_hash={self._key_hash} url={self._api_url}>"
        )
