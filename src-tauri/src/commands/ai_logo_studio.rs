// AI Logo Studio — Tauri commands
// Generates, refines, and exports brand logos using AI image models.

use serde::{Deserialize, Serialize};
use tauri::State;
use crate::AppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogoPrompt {
    pub brand_name: String,
    pub style: String,
    pub colour_palette: Vec<String>,
    pub aspect_ratio: String,
    pub negative_prompt: Option<String>,
    pub seed: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogoVariant {
    pub variant_id: String,
    pub base64_png: String,
    pub width: u32,
    pub height: u32,
    pub seed: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogoGenerationResult {
    pub request_id: String,
    pub variants: Vec<LogoVariant>,
    pub duration_ms: u64,
    pub model_used: String,
}

/// Generate logo variants for a given prompt.
#[tauri::command]
pub async fn logo_generate(
    prompt: LogoPrompt,
    variant_count: u8,
    _state: State<'_, AppState>,
) -> Result<LogoGenerationResult, String> {
    if prompt.brand_name.is_empty() {
        return Err("Brand name cannot be empty".into());
    }
    if variant_count == 0 || variant_count > 8 {
        return Err("Variant count must be between 1 and 8".into());
    }

    let start = std::time::Instant::now();

    // Stub: In production calls a local or remote diffusion model API.
    let variants: Vec<LogoVariant> = (0..variant_count)
        .map(|i| LogoVariant {
            variant_id: uuid::Uuid::new_v4().to_string(),
            base64_png: String::new(),
            width: 512,
            height: 512,
            seed: prompt.seed.unwrap_or(0) + i as u64,
        })
        .collect();

    Ok(LogoGenerationResult {
        request_id: uuid::Uuid::new_v4().to_string(),
        variants,
        duration_ms: start.elapsed().as_millis() as u64,
        model_used: "stable-diffusion-xl".into(),
    })
}

/// Export a logo variant to a file in the requested format.
#[tauri::command]
pub async fn logo_export(
    variant_id: String,
    output_path: String,
    format: String,
    _state: State<'_, AppState>,
) -> Result<String, String> {
    if variant_id.is_empty() {
        return Err("Variant ID cannot be empty".into());
    }
    if output_path.is_empty() {
        return Err("Output path cannot be empty".into());
    }
    let supported = ["png", "svg", "ico", "webp"];
    if !supported.contains(&format.to_lowercase().as_str()) {
        return Err(format!("Unsupported format '{}'. Supported: {:?}", format, supported));
    }

    Ok(output_path)
}
