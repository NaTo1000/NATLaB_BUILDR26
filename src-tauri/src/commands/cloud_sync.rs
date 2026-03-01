// Cloud Sync / Profiles — Tauri commands
// Manages user profiles, project settings, and cloud synchronisation.

use serde::{Deserialize, Serialize};
use tauri::State;
use crate::AppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: String,
    pub display_name: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub preferences: serde_json::Value,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncStatus {
    pub last_sync: Option<String>,
    pub pending_changes: u32,
    pub in_progress: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResult {
    pub pushed: u32,
    pub pulled: u32,
    pub conflicts: u32,
    pub duration_ms: u64,
    pub success: bool,
}

/// Get the current user profile.
#[tauri::command]
pub async fn cloud_get_profile(
    _state: State<'_, AppState>,
) -> Result<Option<UserProfile>, String> {
    // Stub: In production reads from local secure store and merges with cloud.
    Ok(None)
}

/// Update user preferences.
#[tauri::command]
pub async fn cloud_update_preferences(
    preferences: serde_json::Value,
    _state: State<'_, AppState>,
) -> Result<(), String> {
    if !preferences.is_object() {
        return Err("Preferences must be a JSON object".into());
    }
    // Stub: In production writes to local store and queues a cloud sync.
    Ok(())
}

/// Trigger a cloud sync operation.
#[tauri::command]
pub async fn cloud_sync_now(
    _state: State<'_, AppState>,
) -> Result<SyncResult, String> {
    let start = std::time::Instant::now();

    // Stub: In production performs a 3-way merge of local and remote state.
    Ok(SyncResult {
        pushed: 0,
        pulled: 0,
        conflicts: 0,
        duration_ms: start.elapsed().as_millis() as u64,
        success: true,
    })
}

/// Get the current sync status.
#[tauri::command]
pub async fn cloud_sync_status(
    _state: State<'_, AppState>,
) -> Result<SyncStatus, String> {
    Ok(SyncStatus {
        last_sync: None,
        pending_changes: 0,
        in_progress: false,
        error: None,
    })
}
