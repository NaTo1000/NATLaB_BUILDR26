"""Tests for the NATLaB AI assistant."""
from __future__ import annotations

import pytest

from natlab.ai.assistant import AIAssistant, _mask


class TestMask:
    def test_short_key(self):
        assert _mask("abc") == "***"

    def test_long_key(self):
        result = _mask("sk-1234567890abcdef")
        assert result.startswith("sk-1")
        assert result.endswith("cdef")
        assert "***" in result


class TestAIAssistant:
    def test_no_key_raises(self, monkeypatch):
        monkeypatch.delenv("NATLAB_AI_KEY", raising=False)
        with pytest.raises(ValueError, match="API key"):
            AIAssistant(api_key=None)

    def test_key_accepted(self):
        # Should not raise
        a = AIAssistant(api_key="test-key-12345678")
        assert a is not None

    def test_query_network_error(self):
        a = AIAssistant(api_key="dummy-key-12345678", api_url="http://127.0.0.1:19999/v1/chat")
        with pytest.raises(RuntimeError, match="Connection error|refused"):
            a.query("hello")

    def test_enhance_project_returns_list(self, tmp_path):
        # Write a Python file
        (tmp_path / "app.py").write_text("x = 1\n")
        a = AIAssistant(api_key="dummy-key-12345678", api_url="http://127.0.0.1:19999/v1")
        # Will fail at network level – result should still be a list
        results = a.enhance_project(tmp_path)
        assert isinstance(results, list)
        assert len(results) == 1
        assert "error" in results[0]

    def test_enhance_project_skips_natlab_dir(self, tmp_path):
        meta = tmp_path / ".natlab"
        meta.mkdir()
        (meta / "manifest.json").write_text("{}")
        (tmp_path / "app.py").write_text("x=1\n")

        a = AIAssistant(api_key="dummy-key-12345678", api_url="http://127.0.0.1:19999/v1")
        results = a.enhance_project(tmp_path)
        # Should only find app.py, not the manifest
        assert all(".natlab" not in r["file"] for r in results)
