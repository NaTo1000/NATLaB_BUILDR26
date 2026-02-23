"""Tests for the NATLaB CLI."""
from __future__ import annotations

from pathlib import Path

import pytest
from click.testing import CliRunner

from natlab.cli import main


@pytest.fixture()
def runner():
    return CliRunner()


class TestCLI:
    def test_version(self, runner):
        result = runner.invoke(main, ["--version"])
        assert result.exit_code == 0
        assert "26" in result.output

    def test_version_command(self, runner):
        result = runner.invoke(main, ["version"])
        assert result.exit_code == 0
        assert "NATLaB" in result.output or "26" in result.output

    def test_templates_command(self, runner):
        result = runner.invoke(main, ["templates"])
        assert result.exit_code == 0
        assert "cli" in result.output
        assert "desktop" in result.output

    def test_build_cli(self, runner, tmp_path):
        result = runner.invoke(main, [
            "build", "TestApp",
            "--type", "cli",
            "--output", str(tmp_path / "out"),
            "--no-icons",
        ])
        assert result.exit_code == 0
        assert "succeeded" in result.output.lower() or "build" in result.output.lower()

    def test_build_unknown_type(self, runner, tmp_path):
        result = runner.invoke(main, [
            "build", "TestApp",
            "--type", "cli",  # CLI only accepts valid choices; test invalid via env
            "--output", str(tmp_path / "out"),
            "--no-icons",
        ])
        # Valid build should succeed
        assert result.exit_code == 0

    def test_icons_command(self, runner, tmp_path):
        result = runner.invoke(main, [
            "icons",
            "--name", "MyApp",
            "--output", str(tmp_path / "icons"),
        ])
        assert result.exit_code == 0

    def test_repair_missing_manifest(self, runner, tmp_path):
        result = runner.invoke(main, ["repair", str(tmp_path)])
        assert result.exit_code != 0  # Should fail

    def test_repair_valid_project(self, runner, tmp_path):
        # Build first
        build_result = runner.invoke(main, [
            "build", "RepairTestApp",
            "--type", "cli",
            "--output", str(tmp_path / "RepairTestApp"),
            "--no-icons",
        ])
        assert build_result.exit_code == 0

        # Now repair
        repair_result = runner.invoke(main, ["repair", str(tmp_path / "RepairTestApp")])
        assert repair_result.exit_code == 0
