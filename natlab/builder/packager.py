"""NATLaB_BUILDR26 – distributable application packager.

Packages a built application directory into a cross-platform distributable
archive (.zip) ready for end-user distribution.
"""
from __future__ import annotations

import platform
import shutil
import time
import zipfile
from pathlib import Path
from typing import Optional


class Packager:
    """Creates distributable artefacts from a built project directory."""

    def pack(
        self,
        source_dir: Path,
        app_name: str,
        target_platform: str = "native",
        *,
        output_dir: Optional[Path] = None,
    ) -> Path:
        """Compress *source_dir* into a distributable zip archive.

        Returns the path to the created archive.
        """
        source_dir = Path(source_dir)
        if output_dir is None:
            output_dir = source_dir.parent

        output_dir = Path(output_dir)
        output_dir.mkdir(parents=True, exist_ok=True)

        # Resolve target platform label
        if target_platform in ("native", ""):
            target_platform = self._current_platform()

        safe_name = app_name.lower().replace(" ", "_")
        timestamp = time.strftime("%Y%m%d%H%M%S", time.gmtime())
        archive_name = f"{safe_name}_{target_platform}_{timestamp}.zip"
        archive_path = output_dir / archive_name

        with zipfile.ZipFile(archive_path, "w", zipfile.ZIP_DEFLATED) as zf:
            for file_path in sorted(source_dir.rglob("*")):
                if file_path.is_file():
                    # Skip the .natlab metadata dir from the archive
                    if ".natlab" in file_path.parts:
                        continue
                    arcname = file_path.relative_to(source_dir.parent)
                    zf.write(file_path, arcname)

        return archive_path

    @staticmethod
    def _current_platform() -> str:
        sys_map = {"Windows": "windows", "Darwin": "darwin", "Linux": "linux"}
        return sys_map.get(platform.system(), "unknown")
