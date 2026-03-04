"""Tests for the NATLaB icon builder."""
from __future__ import annotations

import json
from pathlib import Path

import pytest

from natlab.icon.builder import IconBuilder


class TestIconBuilderInitials:
    def test_single_word(self):
        ib = IconBuilder(app_name="MyApp")
        assert ib.initials == "MY"

    def test_multi_word(self):
        ib = IconBuilder(app_name="My Cool App")
        assert ib.initials == "MA"

    def test_empty_name(self):
        ib = IconBuilder(app_name="")
        assert ib.initials == "?"


class TestHexToRgb:
    def test_6digit(self):
        ib = IconBuilder()
        assert ib._hex_to_rgb("#FF0000") == (255, 0, 0)
        assert ib._hex_to_rgb("00FF00") == (0, 255, 0)

    def test_3digit(self):
        ib = IconBuilder()
        assert ib._hex_to_rgb("#FFF") == (255, 255, 255)

    def test_invalid_hex_raises(self):
        ib = IconBuilder()
        with pytest.raises(ValueError, match="Invalid hex color"):
            ib._hex_to_rgb("#ZZZZZZ")

    def test_wrong_length_raises(self):
        ib = IconBuilder()
        with pytest.raises(ValueError, match="Invalid hex color"):
            ib._hex_to_rgb("#FF00")

    def test_empty_raises(self):
        ib = IconBuilder()
        with pytest.raises(ValueError, match="Invalid hex color"):
            ib._hex_to_rgb("")


class TestIconBuilderSVGFallback:
    """Tests that work without Pillow installed."""

    def test_svg_fallback_content(self):
        ib = IconBuilder(app_name="TestApp", primary_color="#123456", text_color="#ABCDEF")
        svg = ib._render_svg(64)
        assert "#123456" in svg
        assert "#ABCDEF" in svg
        assert "TE" in svg  # initials (single word → first 2 chars)

    def test_svg_fallback_size_in_attributes(self):
        ib = IconBuilder(app_name="App")
        svg = ib._render_svg(128)
        assert 'width="128"' in svg
        assert 'height="128"' in svg


class TestIconBuilderBuildAll:
    def test_build_all_creates_files(self, tmp_path: Path):
        ib = IconBuilder(app_name="TestApp")
        files = ib.build_all(tmp_path)
        assert len(files) > 0
        for f in files:
            assert f.exists()

    def test_build_all_creates_manifest(self, tmp_path: Path):
        ib = IconBuilder(app_name="TestApp")
        ib.build_all(tmp_path)
        manifest_path = tmp_path / "icons_manifest.json"
        assert manifest_path.exists()
        data = json.loads(manifest_path.read_text())
        assert data["app_name"] == "TestApp"
        assert len(data["icons"]) > 0

    def test_build_png_creates_file(self, tmp_path: Path):
        ib = IconBuilder(app_name="PngTest")
        path = ib.build_png(64, tmp_path)
        assert path.exists()
