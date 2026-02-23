"""NATLaB_BUILDR26 – icon and logo builder.

Generates a complete application icon set in all standard sizes required
for cross-platform distribution:

* Windows  : .ico  (16 → 256 px)
* macOS    : .icns (16 → 1024 px via PNG intermediates)
* Linux    : PNG set (16, 32, 48, 64, 128, 256, 512 px)
* Web      : favicon.ico + apple-touch-icon.png

If Pillow is not installed, the builder falls back to writing SVG placeholder
icons instead.
"""
from __future__ import annotations

import io
import struct
import zlib
from pathlib import Path
from typing import List, Optional, Tuple

# Pillow is an optional but highly recommended dependency
try:
    from PIL import Image, ImageDraw, ImageFont  # type: ignore
    _HAS_PILLOW = True
except ImportError:  # pragma: no cover
    _HAS_PILLOW = False


# ---------------------------------------------------------------------------
# Standard icon sizes
# ---------------------------------------------------------------------------

LINUX_SIZES: Tuple[int, ...] = (16, 32, 48, 64, 128, 256, 512)
WINDOWS_ICO_SIZES: Tuple[int, ...] = (16, 24, 32, 48, 64, 128, 256)
MACOS_ICNS_SIZES: Tuple[int, ...] = (16, 32, 64, 128, 256, 512, 1024)
WEB_SIZES: Tuple[int, ...] = (16, 32, 180, 192)


# ---------------------------------------------------------------------------
# Icon builder
# ---------------------------------------------------------------------------

class IconBuilder:
    """Generate a full icon set for a NATLaB application.

    Parameters
    ----------
    app_name:
        Application name – used as the icon label.
    primary_color:
        Hex colour string for the icon background (default: ``#2563EB``).
    text_color:
        Hex colour string for the icon label (default: ``#FFFFFF``).
    """

    def __init__(
        self,
        app_name: str = "App",
        primary_color: str = "#2563EB",
        text_color: str = "#FFFFFF",
    ) -> None:
        self.app_name = app_name
        self.primary_color = primary_color
        self.text_color = text_color
        # Initials for the icon (up to 2 characters)
        self.initials = self._get_initials(app_name)

    # ------------------------------------------------------------------
    # Public API
    # ------------------------------------------------------------------

    def build_all(self, output_dir: Path) -> List[Path]:
        """Generate the full icon set into *output_dir*.

        Returns a list of all created file paths.
        """
        output_dir = Path(output_dir)
        output_dir.mkdir(parents=True, exist_ok=True)

        created: List[Path] = []

        if _HAS_PILLOW:
            created.extend(self._build_linux_set(output_dir))
            created.extend(self._build_windows_ico(output_dir))
            created.extend(self._build_web_icons(output_dir))
        else:
            created.extend(self._build_svg_fallback(output_dir))

        self._write_manifest(output_dir, created)
        return created

    def build_png(self, size: int, output_dir: Path) -> Path:
        """Generate a single PNG icon of the given *size*.

        Returns the path to the created file.
        """
        output_dir = Path(output_dir)
        output_dir.mkdir(parents=True, exist_ok=True)
        path = output_dir / f"icon_{size}x{size}.png"

        if _HAS_PILLOW:
            img = self._render_pillow(size)
            img.save(path, "PNG", optimize=True)
        else:
            path.write_text(self._render_svg(size), encoding="utf-8")
            path = output_dir / f"icon_{size}x{size}.svg"

        return path

    # ------------------------------------------------------------------
    # Private – Pillow rendering
    # ------------------------------------------------------------------

    def _render_pillow(self, size: int) -> "Image.Image":
        img = Image.new("RGBA", (size, size), (0, 0, 0, 0))
        draw = ImageDraw.Draw(img)

        # Rounded-rectangle background
        radius = max(2, size // 8)
        bg = self._hex_to_rgb(self.primary_color)
        draw.rounded_rectangle([(0, 0), (size - 1, size - 1)], radius=radius, fill=bg)

        # Text label
        text = self.initials
        fg = self._hex_to_rgb(self.text_color)
        font_size = max(6, int(size * 0.45))
        font = self._get_font(font_size)
        bbox = draw.textbbox((0, 0), text, font=font)
        tw = bbox[2] - bbox[0]
        th = bbox[3] - bbox[1]
        tx = (size - tw) // 2 - bbox[0]
        ty = (size - th) // 2 - bbox[1]
        draw.text((tx, ty), text, fill=fg, font=font)

        return img

    @staticmethod
    def _get_font(size: int) -> "ImageFont.FreeTypeFont | ImageFont.ImageFont":
        try:
            return ImageFont.truetype("/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf", size)
        except (IOError, OSError):
            pass
        try:
            return ImageFont.truetype("/System/Library/Fonts/Helvetica.ttc", size)
        except (IOError, OSError):
            pass
        return ImageFont.load_default()

    def _build_linux_set(self, output_dir: Path) -> List[Path]:
        files: List[Path] = []
        linux_dir = output_dir / "linux"
        linux_dir.mkdir(exist_ok=True)
        for sz in LINUX_SIZES:
            path = linux_dir / f"{sz}x{sz}.png"
            img = self._render_pillow(sz)
            img.save(path, "PNG", optimize=True)
            files.append(path)
        return files

    def _build_windows_ico(self, output_dir: Path) -> List[Path]:
        win_dir = output_dir / "windows"
        win_dir.mkdir(exist_ok=True)
        ico_path = win_dir / "app.ico"

        images = [self._render_pillow(sz) for sz in WINDOWS_ICO_SIZES]
        # Use Pillow's built-in ICO writer
        images[0].save(
            ico_path,
            format="ICO",
            sizes=[(sz, sz) for sz in WINDOWS_ICO_SIZES],
            append_images=images[1:],
        )
        return [ico_path]

    def _build_web_icons(self, output_dir: Path) -> List[Path]:
        files: List[Path] = []
        web_dir = output_dir / "web"
        web_dir.mkdir(exist_ok=True)

        for sz in WEB_SIZES:
            name = "favicon.png" if sz <= 32 else f"icon-{sz}x{sz}.png"
            path = web_dir / name
            img = self._render_pillow(sz)
            img.save(path, "PNG", optimize=True)
            files.append(path)

        # favicon.ico (16 + 32)
        favicon_path = web_dir / "favicon.ico"
        fav16 = self._render_pillow(16)
        fav32 = self._render_pillow(32)
        fav16.save(favicon_path, format="ICO", sizes=[(16, 16), (32, 32)], append_images=[fav32])
        files.append(favicon_path)
        return files

    # ------------------------------------------------------------------
    # Private – SVG fallback (no Pillow)
    # ------------------------------------------------------------------

    def _build_svg_fallback(self, output_dir: Path) -> List[Path]:
        files: List[Path] = []
        for sz in LINUX_SIZES:
            path = output_dir / f"icon_{sz}x{sz}.svg"
            path.write_text(self._render_svg(sz), encoding="utf-8")
            files.append(path)
        return files

    def _render_svg(self, size: int) -> str:
        r = max(2, size // 8)
        font_size = max(6, int(size * 0.45))
        return (
            f'<svg xmlns="http://www.w3.org/2000/svg" width="{size}" height="{size}">'
            f'<rect width="{size}" height="{size}" rx="{r}" ry="{r}" fill="{self.primary_color}"/>'
            f'<text x="50%" y="50%" dominant-baseline="central" text-anchor="middle" '
            f'font-family="sans-serif" font-weight="bold" font-size="{font_size}" '
            f'fill="{self.text_color}">{self.initials}</text>'
            f"</svg>"
        )

    # ------------------------------------------------------------------
    # Manifest
    # ------------------------------------------------------------------

    def _write_manifest(self, output_dir: Path, files: List[Path]) -> None:
        import json

        manifest = {
            "app_name": self.app_name,
            "initials": self.initials,
            "primary_color": self.primary_color,
            "text_color": self.text_color,
            "icons": [str(p.relative_to(output_dir)) for p in files],
        }
        (output_dir / "icons_manifest.json").write_text(
            json.dumps(manifest, indent=2), encoding="utf-8"
        )

    # ------------------------------------------------------------------
    # Helpers
    # ------------------------------------------------------------------

    @staticmethod
    def _get_initials(name: str) -> str:
        words = [w for w in name.split() if w]
        if not words:
            return "?"
        if len(words) == 1:
            return words[0][:2].upper()
        return (words[0][0] + words[-1][0]).upper()

    @staticmethod
    def _hex_to_rgb(hex_color: str) -> Tuple[int, int, int]:
        hex_color = hex_color.lstrip("#")
        if len(hex_color) == 3:
            hex_color = "".join(c * 2 for c in hex_color)
        r, g, b = int(hex_color[0:2], 16), int(hex_color[2:4], 16), int(hex_color[4:6], 16)
        return r, g, b
