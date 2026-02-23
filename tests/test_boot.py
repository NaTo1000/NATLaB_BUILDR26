"""Tests for the NATLaB boot manager."""
from __future__ import annotations

import json
from pathlib import Path

import pytest

from natlab.boot.manager import BootManager, BootStep


class TestBootStep:
    def test_successful_step(self):
        called = []
        step = BootStep("test", lambda: called.append(1))
        ok = step.run()
        assert ok
        assert len(called) == 1
        assert step.duration >= 0
        assert step.error is None

    def test_failing_step(self):
        def bad():
            raise ValueError("boom")

        step = BootStep("bad", bad, critical=True)
        ok = step.run()
        assert not ok
        assert step.error is not None
        assert isinstance(step.error, ValueError)


class TestBootManager:
    def test_add_and_run_steps(self):
        log = []
        bm = BootManager("TestApp")
        bm.add_step("step1", lambda: log.append("a"))
        bm.add_step("step2", lambda: log.append("b"))
        ok = bm.run()
        assert ok
        assert log == ["a", "b"]

    def test_critical_step_stops_on_failure(self):
        log = []
        bm = BootManager("TestApp")
        bm.add_step("ok", lambda: log.append("ok"), critical=True)
        bm.add_step("fail", lambda: (_ for _ in ()).throw(RuntimeError("x")), critical=True)
        bm.add_step("after", lambda: log.append("after"), critical=False)
        ok = bm.run()
        assert not ok
        assert "after" not in log

    def test_non_critical_step_continues_on_failure(self):
        log = []
        bm = BootManager("TestApp")
        bm.add_step("fail", lambda: (_ for _ in ()).throw(RuntimeError("x")), critical=False)
        bm.add_step("ok", lambda: log.append("ok"), critical=True)
        ok = bm.run()
        assert ok
        assert "ok" in log

    def test_config_get_set(self):
        bm = BootManager("App")
        bm.set("key", "value")
        assert bm.get("key") == "value"
        assert bm.get("missing", "default") == "default"

    def test_load_config_from_file(self, tmp_path: Path):
        cfg = tmp_path / "config.json"
        cfg.write_text(json.dumps({"debug": True, "port": 8080}))
        bm = BootManager("App")
        bm.load_config(cfg)
        assert bm.get("debug") is True
        assert bm.get("port") == 8080

    def test_load_config_missing_file(self, tmp_path: Path):
        bm = BootManager("App")
        # Should not raise
        bm.load_config(tmp_path / "nonexistent.json")

    def test_platform_info(self):
        bm = BootManager("App")
        info = bm.platform_info
        assert "system" in info
        assert "python" in info

    def test_write_stub(self, tmp_path: Path):
        bm = BootManager("MyApp")
        bm.write_stub(tmp_path / "boot")
        assert (tmp_path / "boot" / "manager.py").exists()
        assert (tmp_path / "boot" / "__init__.py").exists()
        content = (tmp_path / "boot" / "manager.py").read_text()
        assert "BootManager" in content

    def test_fluent_chaining(self):
        bm = BootManager("App")
        result = bm.add_step("a", lambda: None).add_step("b", lambda: None)
        assert result is bm
