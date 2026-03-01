"""Tests for the NATLaB builder engine and templates."""
from __future__ import annotations

import json
import zipfile
from pathlib import Path

import pytest

from natlab.builder.engine import BuildEngine, BuildResult
from natlab.builder.templates import TemplateRegistry, CLITemplate, DesktopTemplate
from natlab.builder.packager import Packager, PackResult


# ---------------------------------------------------------------------------
# Fixtures
# ---------------------------------------------------------------------------

@pytest.fixture()
def tmp_output(tmp_path: Path) -> Path:
    return tmp_path / "output"


# ---------------------------------------------------------------------------
# TemplateRegistry
# ---------------------------------------------------------------------------

class TestTemplateRegistry:
    def test_default_templates_present(self):
        reg = TemplateRegistry()
        names = {t["name"] for t in reg.list_templates()}
        assert {"cli", "desktop", "web", "service"} == names

    def test_get_known_template(self):
        reg = TemplateRegistry()
        tmpl = reg.get("cli")
        assert isinstance(tmpl, CLITemplate)

    def test_get_unknown_raises(self):
        reg = TemplateRegistry()
        with pytest.raises(KeyError, match="unknown_type"):
            reg.get("unknown_type")

    def test_custom_template_registration(self):
        class MyTemplate(CLITemplate):
            name = "custom"
            description = "My custom template"

        reg = TemplateRegistry()
        reg.register(MyTemplate())
        assert reg.get("custom").description == "My custom template"


# ---------------------------------------------------------------------------
# CLI template scaffolding
# ---------------------------------------------------------------------------

class TestCLITemplate:
    def test_scaffold_creates_expected_files(self, tmp_output: Path):
        t = CLITemplate()
        t.render(output_dir=tmp_output, app_name="MyApp")

        assert (tmp_output / "src" / "myapp" / "__init__.py").exists()
        assert (tmp_output / "src" / "myapp" / "main.py").exists()
        assert (tmp_output / "pyproject.toml").exists()
        assert (tmp_output / "README.md").exists()

    def test_scaffold_app_name_in_readme(self, tmp_output: Path):
        t = CLITemplate()
        t.render(output_dir=tmp_output, app_name="AwesomeApp")
        readme = (tmp_output / "README.md").read_text()
        assert "AwesomeApp" in readme

    def test_scaffold_spaces_in_name(self, tmp_output: Path):
        t = CLITemplate()
        t.render(output_dir=tmp_output, app_name="My Cool App")
        # Directory name should use underscores
        assert (tmp_output / "src" / "my_cool_app").is_dir()


# ---------------------------------------------------------------------------
# Desktop / Web templates
# ---------------------------------------------------------------------------

class TestDesktopTemplate:
    def test_scaffold_creates_app_file(self, tmp_output: Path):
        t = DesktopTemplate()
        t.render(output_dir=tmp_output, app_name="DesktopDemo")
        assert (tmp_output / "src" / "desktopdemo" / "app.py").exists()


# ---------------------------------------------------------------------------
# Packager
# ---------------------------------------------------------------------------

class TestPackager:
    def test_pack_creates_zip(self, tmp_path: Path):
        src = tmp_path / "src"
        src.mkdir()
        (src / "main.py").write_text("print('hello')")

        packager = Packager()
        archive = packager.pack(src, "TestApp", "linux", output_dir=tmp_path)

        assert archive.exists()
        assert archive.suffix == ".zip"
        assert zipfile.is_zipfile(archive)

    def test_pack_excludes_natlab_meta(self, tmp_path: Path):
        src = tmp_path / "src"
        src.mkdir()
        (src / "main.py").write_text("print('hello')")
        meta = src / ".natlab"
        meta.mkdir()
        (meta / "manifest.json").write_text("{}")

        packager = Packager()
        archive = packager.pack(src, "TestApp", "linux", output_dir=tmp_path)

        with zipfile.ZipFile(archive) as zf:
            names = zf.namelist()
        assert not any(".natlab" in n for n in names)

    def test_pack_full_returns_pack_result(self, tmp_path: Path):
        src = tmp_path / "src"
        src.mkdir()
        (src / "main.py").write_text("x = 1\n" * 200)  # some compressible data

        packager = Packager()
        result = packager.pack_full(src, "TestApp", "linux", output_dir=tmp_path)

        assert isinstance(result, PackResult)
        assert result.archive_path.exists()
        assert result.checksum_path.exists()
        assert result.original_bytes > 0
        assert result.compressed_bytes > 0

    def test_pack_full_sha256_checksum_valid(self, tmp_path: Path):
        import hashlib
        src = tmp_path / "src"
        src.mkdir()
        (src / "main.py").write_text("print('checksum test')")

        packager = Packager()
        result = packager.pack_full(src, "TestApp", "linux", output_dir=tmp_path)

        # Verify the checksum file contents match the archive
        checksum_line = result.checksum_path.read_text().strip()
        expected_hash = checksum_line.split("  ")[0]
        h = hashlib.sha256()
        with result.archive_path.open("rb") as f:
            for chunk in iter(lambda: f.read(65536), b""):
                h.update(chunk)
        assert h.hexdigest() == expected_hash

    def test_pack_full_compression_ratio(self, tmp_path: Path):
        src = tmp_path / "src"
        src.mkdir()
        # Highly compressible content
        (src / "big.py").write_text("# padding\n" * 1000)

        packager = Packager()
        result = packager.pack_full(src, "TestApp", "linux", output_dir=tmp_path)

        # We should achieve meaningful compression on repetitive text
        assert result.ratio > 0


# ---------------------------------------------------------------------------
# BuildEngine
# ---------------------------------------------------------------------------

class TestBuildEngine:
    def test_build_cli_success(self, tmp_output: Path):
        engine = BuildEngine()
        result = engine.build(
            app_name="CliTest",
            app_type="cli",
            output_dir=tmp_output,
            with_boot_manager=True,
            with_icons=False,
        )
        assert result.success
        assert result.output_dir == tmp_output
        assert result.artefact is not None
        assert result.artefact.exists()

    def test_build_writes_manifest(self, tmp_output: Path):
        engine = BuildEngine()
        result = engine.build(
            app_name="ManifestTest",
            app_type="cli",
            output_dir=tmp_output,
            with_boot_manager=False,
            with_icons=False,
        )
        assert result.success
        manifest = json.loads((tmp_output / ".natlab" / "manifest.json").read_text())
        assert manifest["app_name"] == "ManifestTest"
        assert manifest["app_type"] == "cli"

    def test_build_manifest_includes_step_timings(self, tmp_output: Path):
        engine = BuildEngine()
        result = engine.build(
            app_name="TimingTest",
            app_type="cli",
            output_dir=tmp_output,
            with_boot_manager=False,
            with_icons=False,
        )
        assert result.success
        manifest = json.loads((tmp_output / ".natlab" / "manifest.json").read_text())
        assert "step_timings_s" in manifest
        assert "scaffold" in manifest["step_timings_s"]

    def test_build_result_has_step_timings(self, tmp_output: Path):
        engine = BuildEngine()
        result = engine.build(
            app_name="StepTimingsTest",
            app_type="cli",
            output_dir=tmp_output,
            with_boot_manager=True,
            with_icons=False,
        )
        assert result.success
        assert "scaffold" in result.step_timings
        assert "boot_manager" in result.step_timings
        assert "package" in result.step_timings

    def test_build_unknown_type_fails(self, tmp_output: Path):
        engine = BuildEngine()
        result = engine.build(
            app_name="BadType",
            app_type="nonexistent",
            output_dir=tmp_output,
            with_boot_manager=False,
            with_icons=False,
        )
        assert not result.success

    def test_repair_no_manifest_fails(self, tmp_path: Path):
        engine = BuildEngine()
        result = engine.repair(tmp_path)
        assert not result.success
        assert any("manifest" in m.lower() for m in result.messages)

    def test_repair_with_manifest(self, tmp_path: Path):
        engine = BuildEngine()
        # First build to create a valid project
        build_result = engine.build(
            app_name="RepairMe",
            app_type="cli",
            output_dir=tmp_path / "RepairMe",
            with_boot_manager=False,
            with_icons=False,
        )
        assert build_result.success
        # Now repair
        repair_result = engine.repair(tmp_path / "RepairMe")
        assert repair_result.success

    def test_build_with_ai_no_key_raises(self, tmp_output: Path):
        engine = BuildEngine(ai_api_key=None)
        # Ensure env var is not set
        import os
        os.environ.pop("NATLAB_AI_KEY", None)
        result = engine.build(
            app_name="AiTest",
            app_type="cli",
            output_dir=tmp_output,
            with_boot_manager=False,
            with_icons=False,
            with_ai=True,
        )
        # Should fail gracefully (no key)
        assert not result.success
        assert any("key" in m.lower() or "api" in m.lower() for m in result.messages)

    def test_build_result_to_dict(self, tmp_output: Path):
        engine = BuildEngine()
        result = engine.build(
            app_name="DictTest",
            app_type="cli",
            output_dir=tmp_output,
            with_boot_manager=False,
            with_icons=False,
        )
        d = result.to_dict()
        assert d["success"] is True
        assert "duration_seconds" in d
        assert "messages" in d
        assert "step_timings" in d

    def test_build_checksum_written(self, tmp_output: Path):
        engine = BuildEngine()
        result = engine.build(
            app_name="ChecksumTest",
            app_type="cli",
            output_dir=tmp_output,
            with_boot_manager=False,
            with_icons=False,
        )
        assert result.success
        # SHA-256 checksum file should exist alongside the archive
        checksum_files = list(tmp_output.parent.glob("*.sha256"))
        assert len(checksum_files) == 1
