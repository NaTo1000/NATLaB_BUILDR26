// Live Docs Generator — Tauri commands
// Auto-generates, updates, and exports project documentation.

use serde::{Deserialize, Serialize};
use tauri::State;
use crate::AppState;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DocFormat {
    Markdown,
    Html,
    Pdf,
    Docx,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocGenRequest {
    pub project_path: String,
    pub format: DocFormat,
    pub include_private: bool,
    pub include_examples: bool,
    pub output_path: String,
    pub template: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocGenResult {
    pub job_id: String,
    pub output_path: String,
    pub pages_generated: u32,
    pub symbols_documented: u32,
    pub duration_ms: u64,
    pub warnings: Vec<String>,
    pub success: bool,
}

/// Generate documentation for a project.
#[tauri::command]
pub async fn docs_generate(
    request: DocGenRequest,
    _state: State<'_, AppState>,
) -> Result<DocGenResult, String> {
    if request.project_path.is_empty() {
        return Err("Project path cannot be empty".into());
    }
    if request.output_path.is_empty() {
        return Err("Output path cannot be empty".into());
    }

    let start = std::time::Instant::now();

    // Stub: In production parses source ASTs, extracts doc comments,
    // applies templates, and renders to the target format.
    Ok(DocGenResult {
        job_id: uuid::Uuid::new_v4().to_string(),
        output_path: request.output_path,
        pages_generated: 0,
        symbols_documented: 0,
        duration_ms: start.elapsed().as_millis() as u64,
        warnings: vec![],
        success: true,
    })
}

/// Watch a project directory and regenerate docs on change.
#[tauri::command]
pub async fn docs_watch_start(
    project_path: String,
    output_path: String,
    _state: State<'_, AppState>,
) -> Result<String, String> {
    if project_path.is_empty() {
        return Err("Project path cannot be empty".into());
    }
    // Returns a watcher ID that can be used to stop the watcher.
    Ok(uuid::Uuid::new_v4().to_string())
}

/// Stop a running docs watcher.
#[tauri::command]
pub async fn docs_watch_stop(
    watcher_id: String,
    _state: State<'_, AppState>,
) -> Result<(), String> {
    if watcher_id.is_empty() {
        return Err("Watcher ID cannot be empty".into());
    }
    Ok(())
}
