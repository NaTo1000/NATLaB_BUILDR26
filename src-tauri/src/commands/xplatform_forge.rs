// XPlatform Forge — Tauri commands
// Compiles and packages applications for Windows, macOS, Linux, Android, iOS, and WASM.

use serde::{Deserialize, Serialize};
use tauri::State;
use crate::AppState;
use crate::core::{CompilerTarget, CompileResult, PolyglotCompiler, TargetPlatform, SupportedLanguage};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgeRequest {
    pub project_path: String,
    pub platforms: Vec<String>,
    pub optimisation: u8,
    pub sign: bool,
    pub notarise: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgeResult {
    pub request_id: String,
    pub platform_results: Vec<PlatformBuildResult>,
    pub total_duration_ms: u64,
    pub overall_success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformBuildResult {
    pub platform: String,
    pub success: bool,
    pub artifact_path: Option<String>,
    pub duration_ms: u64,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

fn parse_platform(s: &str) -> Result<TargetPlatform, String> {
    match s.to_lowercase().as_str() {
        "windows" => Ok(TargetPlatform::Windows),
        "macos" | "mac" => Ok(TargetPlatform::MacOs),
        "linux" => Ok(TargetPlatform::Linux),
        "android" => Ok(TargetPlatform::Android),
        "ios" => Ok(TargetPlatform::Ios),
        "wasm" | "web" => Ok(TargetPlatform::Wasm),
        _ => Err(format!("Unknown platform: '{}'. Supported: windows, macos, linux, android, ios, wasm", s)),
    }
}

/// Start a cross-platform forge build.
#[tauri::command]
pub async fn forge_build(
    request: ForgeRequest,
    _state: State<'_, AppState>,
) -> Result<ForgeResult, String> {
    if request.project_path.is_empty() {
        return Err("Project path cannot be empty".into());
    }
    if request.platforms.is_empty() {
        return Err("At least one target platform is required".into());
    }

    let start = std::time::Instant::now();
    let request_id = uuid::Uuid::new_v4().to_string();
    let mut platform_results: Vec<PlatformBuildResult> = Vec::new();

    for platform_str in &request.platforms {
        let platform = parse_platform(platform_str)?;
        let output_path = std::path::PathBuf::from(&request.project_path)
            .join("dist")
            .join(platform_str)
            .to_string_lossy()
            .to_string();
        let target = CompilerTarget {
            id: format!("{}-{}", request_id, platform_str),
            language: SupportedLanguage::Rust,
            source_path: request.project_path.clone(),
            output_path,
            optimisation_level: request.optimisation,
            target_platforms: vec![platform],
        };

        match PolyglotCompiler::compile(target).await {
            Ok(mut result) => {
                if request.sign {
                    result.warnings.push("Signing requested but not yet implemented".into());
                }
                if request.notarise {
                    result.warnings.push("Notarisation requested but not yet implemented".into());
                }
                platform_results.push(PlatformBuildResult {
                    platform: platform_str.clone(),
                    success: result.success,
                    artifact_path: result.output_path,
                    duration_ms: result.duration_ms,
                    errors: result.errors,
                    warnings: result.warnings,
                });
            }
            Err(e) => platform_results.push(PlatformBuildResult {
                platform: platform_str.clone(),
                success: false,
                artifact_path: None,
                duration_ms: 0,
                errors: vec![e.to_string()],
                warnings: vec![],
            }),
        }
    }

    let overall_success = platform_results.iter().all(|r| r.success);

    Ok(ForgeResult {
        request_id,
        total_duration_ms: start.elapsed().as_millis() as u64,
        overall_success,
        platform_results,
    })
}

/// List supported target platforms.
#[tauri::command]
pub async fn forge_list_platforms(_state: State<'_, AppState>) -> Result<Vec<String>, String> {
    Ok(vec![
        "windows".into(),
        "macos".into(),
        "linux".into(),
        "android".into(),
        "ios".into(),
        "wasm".into(),
    ])
}
