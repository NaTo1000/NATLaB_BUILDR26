// Core Engine — Tauri commands
// Module registry and polyglot compiler commands exposed to the frontend.

use tauri::State;
use crate::AppState;
use crate::core::{ModuleDescriptor, CompilerTarget, CompileResult, PolyglotCompiler};

/// List all registered modules.
#[tauri::command]
pub async fn core_list_modules(
    state: State<'_, AppState>,
) -> Result<Vec<ModuleDescriptor>, String> {
    Ok(state.registry.list().await)
}

/// Load (activate) a module by ID.
#[tauri::command]
pub async fn core_load_module(
    module_id: String,
    state: State<'_, AppState>,
) -> Result<ModuleDescriptor, String> {
    if module_id.is_empty() {
        return Err("Module ID cannot be empty".into());
    }
    state.registry.load(&module_id).await.map_err(|e| e.to_string())
}

/// Get a specific module descriptor.
#[tauri::command]
pub async fn core_get_module(
    module_id: String,
    state: State<'_, AppState>,
) -> Result<Option<ModuleDescriptor>, String> {
    if module_id.is_empty() {
        return Err("Module ID cannot be empty".into());
    }
    Ok(state.registry.get(&module_id).await)
}

/// Compile using the polyglot compiler.
#[tauri::command]
pub async fn core_compile(
    target: CompilerTarget,
    _state: State<'_, AppState>,
) -> Result<CompileResult, String> {
    PolyglotCompiler::compile(target)
        .await
        .map_err(|e| e.to_string())
}

/// Return the list of languages supported by the polyglot compiler.
#[tauri::command]
pub async fn core_supported_languages(
    _state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    let langs: Vec<String> = PolyglotCompiler::supported_languages()
        .iter()
        .map(|l| format!("{:?}", l).to_lowercase())
        .collect();
    Ok(langs)
}
