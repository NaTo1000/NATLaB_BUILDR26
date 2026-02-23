// NATLaB BUILDR26 — Tauri Application Library
// Initialises the core engine, build overseer, and registers all command handlers.

pub mod core;
pub mod overseer;
pub mod commands;

use std::sync::Arc;
use core::ModuleRegistry;
use overseer::BuildOverseer;

/// Global application state shared across all Tauri commands.
pub struct AppState {
    pub registry: Arc<ModuleRegistry>,
    pub overseer: Arc<BuildOverseer>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let registry = Arc::new(ModuleRegistry::new());
    let overseer = Arc::new(BuildOverseer::default());

    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .manage(AppState { registry, overseer })
        .invoke_handler(tauri::generate_handler![
            // Core Engine
            commands::core_cmds::core_list_modules,
            commands::core_cmds::core_load_module,
            commands::core_cmds::core_get_module,
            commands::core_cmds::core_compile,
            commands::core_cmds::core_supported_languages,
            // Build Overseer
            commands::overseer_cmds::overseer_evaluate,
            commands::overseer_cmds::overseer_history,
            commands::overseer_cmds::overseer_thresholds,
            commands::overseer_cmds::overseer_rolling_score,
            // App Repair Engine
            commands::app_repair::repair_scan,
            commands::app_repair::repair_apply_fixes,
            // XPlatform Forge
            commands::xplatform_forge::forge_build,
            commands::xplatform_forge::forge_list_platforms,
            // BootForge
            commands::boot_forge::bootforge_create,
            commands::boot_forge::bootforge_verify,
            // AI Logo Studio
            commands::ai_logo_studio::logo_generate,
            commands::ai_logo_studio::logo_export,
            // AI Assistant
            commands::ai_assistant::ai_chat,
            commands::ai_assistant::ai_complete,
            // Cloud Sync
            commands::cloud_sync::cloud_get_profile,
            commands::cloud_sync::cloud_update_preferences,
            commands::cloud_sync::cloud_sync_now,
            commands::cloud_sync::cloud_sync_status,
            // Plugin System
            commands::plugin_system::plugin_list,
            commands::plugin_system::plugin_install,
            commands::plugin_system::plugin_enable,
            commands::plugin_system::plugin_disable,
            commands::plugin_system::plugin_uninstall,
            // Live Docs
            commands::live_docs::docs_generate,
            commands::live_docs::docs_watch_start,
            commands::live_docs::docs_watch_stop,
        ])
        .run(tauri::generate_context!())
        .expect("error while running NATLaB BUILDR26");
}
