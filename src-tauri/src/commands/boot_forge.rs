// BootForge — Tauri commands
// Creates bootable media images (ISO, USB, PXE) from system snapshots or templates.

use serde::{Deserialize, Serialize};
use tauri::State;
use crate::AppState;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MediaFormat {
    Iso,
    Usb,
    Pxe,
    Vhd,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootForgeRequest {
    pub source_path: String,
    pub output_path: String,
    pub format: MediaFormat,
    pub label: String,
    pub compression: bool,
    pub verify_checksum: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootForgeResult {
    pub job_id: String,
    pub output_path: String,
    pub format: MediaFormat,
    pub size_bytes: u64,
    pub sha256: String,
    pub duration_ms: u64,
    pub success: bool,
    pub error: Option<String>,
}

/// Start a bootable media build job.
#[tauri::command]
pub async fn bootforge_create(
    request: BootForgeRequest,
    _state: State<'_, AppState>,
) -> Result<BootForgeResult, String> {
    if request.source_path.is_empty() {
        return Err("Source path cannot be empty".into());
    }
    if request.output_path.is_empty() {
        return Err("Output path cannot be empty".into());
    }
    if request.label.is_empty() {
        return Err("Media label cannot be empty".into());
    }

    let start = std::time::Instant::now();
    let job_id = uuid::Uuid::new_v4().to_string();

    // Stub: In production this orchestrates mkisofs/xorriso/dd/wimlib as appropriate.
    Ok(BootForgeResult {
        job_id,
        output_path: request.output_path,
        format: request.format,
        size_bytes: 0,
        sha256: String::new(),
        duration_ms: start.elapsed().as_millis() as u64,
        success: true,
        error: None,
    })
}

/// Verify the integrity of an existing bootable image.
#[tauri::command]
pub async fn bootforge_verify(
    image_path: String,
    expected_sha256: Option<String>,
    _state: State<'_, AppState>,
) -> Result<bool, String> {
    if image_path.is_empty() {
        return Err("Image path cannot be empty".into());
    }

    // Stub: In production computes SHA-256 and compares.
    let _ = expected_sha256;
    Ok(true)
}
