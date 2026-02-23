// Sandboxed Plugin System — Tauri commands
// Loads, validates, sandboxes, and manages third-party plugins.

use serde::{Deserialize, Serialize};
use tauri::State;
use crate::AppState;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PluginStatus {
    Available,
    Loaded,
    Error,
    Disabled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub entry_point: String,
    pub permissions: Vec<String>,
    pub min_natlab_version: String,
    pub checksum_sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    pub manifest: PluginManifest,
    pub status: PluginStatus,
    pub loaded_at: Option<String>,
    pub sandbox_id: Option<String>,
}

/// List all installed plugins.
#[tauri::command]
pub async fn plugin_list(
    _state: State<'_, AppState>,
) -> Result<Vec<PluginInfo>, String> {
    // Stub: In production scans the plugins directory and validates manifests.
    Ok(vec![])
}

/// Install a plugin from a manifest file path.
#[tauri::command]
pub async fn plugin_install(
    manifest_path: String,
    _state: State<'_, AppState>,
) -> Result<PluginInfo, String> {
    if manifest_path.is_empty() {
        return Err("Manifest path cannot be empty".into());
    }

    // Stub: In production: validate manifest signature, check permissions,
    // spawn sandboxed WASM runtime, register with core engine.
    Err("Plugin installation not yet implemented".into())
}

/// Enable a previously disabled plugin.
#[tauri::command]
pub async fn plugin_enable(
    plugin_id: String,
    _state: State<'_, AppState>,
) -> Result<(), String> {
    if plugin_id.is_empty() {
        return Err("Plugin ID cannot be empty".into());
    }
    Ok(())
}

/// Disable (but do not uninstall) a plugin.
#[tauri::command]
pub async fn plugin_disable(
    plugin_id: String,
    _state: State<'_, AppState>,
) -> Result<(), String> {
    if plugin_id.is_empty() {
        return Err("Plugin ID cannot be empty".into());
    }
    Ok(())
}

/// Uninstall a plugin and remove its sandbox.
#[tauri::command]
pub async fn plugin_uninstall(
    plugin_id: String,
    _state: State<'_, AppState>,
) -> Result<(), String> {
    if plugin_id.is_empty() {
        return Err("Plugin ID cannot be empty".into());
    }
    Ok(())
}
