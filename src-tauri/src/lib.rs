// NATLaB BUILDR26 — Tauri Application Library
// Initialises the core engine, build overseer, and registers all command handlers.

pub mod core;
pub mod overseer;
pub mod admin_audit;
pub mod commands;

use std::sync::Arc;
use core::ModuleRegistry;
use overseer::BuildOverseer;
use admin_audit::AdminAuditEngine;
use admin_audit::runtime_monitor::RuntimeMonitor;
use admin_audit::integrations::IntegrationEngine;

/// Global application state shared across all Tauri commands.
pub struct AppState {
    pub registry: Arc<ModuleRegistry>,
    pub overseer: Arc<BuildOverseer>,
    pub admin_engine: Arc<AdminAuditEngine>,
    pub runtime_monitor: Arc<RuntimeMonitor>,
    pub integrations: Arc<IntegrationEngine>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let registry = Arc::new(ModuleRegistry::new());
    let overseer = Arc::new(BuildOverseer::default());
    let admin_engine = Arc::new(AdminAuditEngine::default());
    let runtime_monitor = Arc::new(RuntimeMonitor::default());
    let integrations = Arc::new(IntegrationEngine::default());

    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .manage(AppState { registry, overseer, admin_engine, runtime_monitor, integrations })
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
            // Admin Audit & Process Control (backend-only, no user UI interaction)
            commands::admin_cmds::admin_log_audit,
            commands::admin_cmds::admin_resolve_audit,
            commands::admin_cmds::admin_get_audit_log,
            commands::admin_cmds::admin_get_todo_list,
            commands::admin_cmds::admin_update_dependency,
            commands::admin_cmds::admin_get_dependencies,
            commands::admin_cmds::admin_generate_daily_summary,
            commands::admin_cmds::admin_get_daily_summaries,
            commands::admin_cmds::admin_handle_error,
            commands::admin_cmds::admin_get_error_responses,
            commands::admin_cmds::admin_allocate_process,
            commands::admin_cmds::admin_update_process,
            commands::admin_cmds::admin_list_processes,
            commands::admin_cmds::admin_log_kernel_event,
            commands::admin_cmds::admin_get_kernel_events,
            // Runtime Monitor / AI Mesh (backend-only)
            commands::admin_cmds::admin_deploy_node,
            commands::admin_cmds::admin_update_node_perf,
            commands::admin_cmds::admin_get_mesh_nodes,
            commands::admin_cmds::admin_remove_node,
            commands::admin_cmds::admin_auto_tune,
            commands::admin_cmds::admin_apply_tuning,
            commands::admin_cmds::admin_get_tuning_history,
            commands::admin_cmds::admin_record_data_flow,
            commands::admin_cmds::admin_get_section_health,
            commands::admin_cmds::admin_get_deployment,
            commands::admin_cmds::admin_set_topology,
            commands::admin_cmds::admin_compute_mesh_health,
            commands::admin_cmds::admin_auto_scale_check,
            // IDE & Platform Integrations (backend-only)
            commands::admin_cmds::admin_register_platform,
            commands::admin_cmds::admin_connect_platform,
            commands::admin_cmds::admin_disconnect_platform,
            commands::admin_cmds::admin_sync_platform,
            commands::admin_cmds::admin_list_integrations,
            commands::admin_cmds::admin_get_integrations_by_category,
            commands::admin_cmds::admin_get_sync_history,
            commands::admin_cmds::admin_remove_integration,
            commands::admin_cmds::admin_list_supported_platforms,
        ])
        .run(tauri::generate_context!())
        .expect("error while running NATLaB BUILDR26");
}
