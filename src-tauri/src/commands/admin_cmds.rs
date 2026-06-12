// NATLaB BUILDR26 — Admin Audit Tauri Commands
// Internal admin-only command handlers. These do NOT expose user-facing UI.

use tauri::State;
use crate::AppState;
use crate::admin_audit::{
    AuditCategory, TaskPriority, DependencyStatus,
    KernelEventType, ProcessStatus,
};
use crate::admin_audit::runtime_monitor::{
    AiNodeRole, DataSection, NodeConfig, NodePerformance, MeshTopology,
    TuningType, SectionHealth,
};
use crate::admin_audit::integrations::{
    IntegrationPlatform, AuthMethod, PlatformConfig, SyncEventType, PlatformCategory,
};
use serde::Deserialize;

// ── Request/Response DTOs ────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct LogAuditRequest {
    pub category: AuditCategory,
    pub severity: TaskPriority,
    pub source_module: String,
    pub message: String,
    pub details: Option<String>,
}

#[derive(Deserialize)]
pub struct ResolveAuditRequest {
    pub id: String,
    pub auto_resolved: bool,
}

#[derive(Deserialize)]
pub struct HandleErrorRequest {
    pub error_source: String,
    pub error_type: String,
    pub message: String,
}

#[derive(Deserialize)]
pub struct AllocateProcessRequest {
    pub name: String,
    pub priority: TaskPriority,
    pub memory_limit_mb: u64,
    pub cpu_quota_pct: f64,
    pub protected: bool,
}

#[derive(Deserialize)]
pub struct UpdateProcessRequest {
    pub process_id: String,
    pub status: ProcessStatus,
}

#[derive(Deserialize)]
pub struct KernelEventRequest {
    pub event_type: KernelEventType,
    pub source: String,
    pub threat_level: TaskPriority,
    pub description: String,
    pub action_taken: String,
    pub blocked: bool,
}

#[derive(Deserialize)]
pub struct DeployNodeRequest {
    pub name: String,
    pub role: AiNodeRole,
    pub section: DataSection,
    pub config: NodeConfig,
}

#[derive(Deserialize)]
pub struct UpdateNodePerfRequest {
    pub node_id: String,
    pub performance: NodePerformance,
}

#[derive(Deserialize)]
pub struct ApplyTuningRequest {
    pub target_node: String,
    pub action_type: TuningType,
    pub parameter: String,
    pub old_value: String,
    pub new_value: String,
    pub reason: String,
}

#[derive(Deserialize)]
pub struct RecordDataFlowRequest {
    pub source: DataSection,
    pub target: DataSection,
    pub payload_size_bytes: u64,
    pub latency_ms: f64,
    pub success: bool,
    pub routed_via: String,
}

// ── Audit Trail Commands ─────────────────────────────────────────────────────

#[tauri::command]
pub async fn admin_log_audit(
    state: State<'_, AppState>,
    request: LogAuditRequest,
) -> Result<serde_json::Value, String> {
    let entry = state.admin_engine.log_audit(
        request.category,
        request.severity,
        &request.source_module,
        &request.message,
        request.details,
    ).await;
    serde_json::to_value(entry).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn admin_resolve_audit(
    state: State<'_, AppState>,
    request: ResolveAuditRequest,
) -> Result<serde_json::Value, String> {
    let entry = state.admin_engine.resolve_audit(&request.id, request.auto_resolved)
        .await
        .map_err(|e| e.to_string())?;
    serde_json::to_value(entry).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn admin_get_audit_log(
    state: State<'_, AppState>,
    priority: Option<TaskPriority>,
) -> Result<serde_json::Value, String> {
    let log = state.admin_engine.get_audit_log(priority).await;
    serde_json::to_value(log).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn admin_get_todo_list(
    state: State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    let todos = state.admin_engine.get_todo_list().await;
    serde_json::to_value(todos).map_err(|e| e.to_string())
}

// ── Dependency Monitor Commands ──────────────────────────────────────────────

#[tauri::command]
pub async fn admin_update_dependency(
    state: State<'_, AppState>,
    status: DependencyStatus,
) -> Result<(), String> {
    state.admin_engine.update_dependency(status).await;
    Ok(())
}

#[tauri::command]
pub async fn admin_get_dependencies(
    state: State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    let deps = state.admin_engine.get_dependencies().await;
    serde_json::to_value(deps).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn admin_generate_daily_summary(
    state: State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    let summary = state.admin_engine.generate_daily_summary().await;
    serde_json::to_value(summary).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn admin_get_daily_summaries(
    state: State<'_, AppState>,
    count: Option<usize>,
) -> Result<serde_json::Value, String> {
    let summaries = state.admin_engine.get_daily_summaries(count.unwrap_or(30)).await;
    serde_json::to_value(summaries).map_err(|e| e.to_string())
}

// ── Error Response Commands ──────────────────────────────────────────────────

#[tauri::command]
pub async fn admin_handle_error(
    state: State<'_, AppState>,
    request: HandleErrorRequest,
) -> Result<serde_json::Value, String> {
    let response = state.admin_engine.handle_error(
        &request.error_source,
        &request.error_type,
        &request.message,
    ).await;
    serde_json::to_value(response).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn admin_get_error_responses(
    state: State<'_, AppState>,
    limit: Option<usize>,
) -> Result<serde_json::Value, String> {
    let responses = state.admin_engine.get_error_responses(limit.unwrap_or(50)).await;
    serde_json::to_value(responses).map_err(|e| e.to_string())
}

// ── Process Allocator Commands ───────────────────────────────────────────────

#[tauri::command]
pub async fn admin_allocate_process(
    state: State<'_, AppState>,
    request: AllocateProcessRequest,
) -> Result<serde_json::Value, String> {
    let alloc = state.admin_engine.allocate_process(
        &request.name,
        request.priority,
        request.memory_limit_mb,
        request.cpu_quota_pct,
        request.protected,
    ).await;
    serde_json::to_value(alloc).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn admin_update_process(
    state: State<'_, AppState>,
    request: UpdateProcessRequest,
) -> Result<serde_json::Value, String> {
    let proc = state.admin_engine.update_process_status(&request.process_id, request.status)
        .await
        .map_err(|e| e.to_string())?;
    serde_json::to_value(proc).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn admin_list_processes(
    state: State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    let procs = state.admin_engine.list_processes().await;
    serde_json::to_value(procs).map_err(|e| e.to_string())
}

// ── Kernel Protection Commands ───────────────────────────────────────────────

#[tauri::command]
pub async fn admin_log_kernel_event(
    state: State<'_, AppState>,
    request: KernelEventRequest,
) -> Result<serde_json::Value, String> {
    let event = state.admin_engine.log_kernel_event(
        request.event_type,
        &request.source,
        request.threat_level,
        &request.description,
        &request.action_taken,
        request.blocked,
    ).await;
    serde_json::to_value(event).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn admin_get_kernel_events(
    state: State<'_, AppState>,
    limit: Option<usize>,
) -> Result<serde_json::Value, String> {
    let events = state.admin_engine.get_kernel_events(limit.unwrap_or(50)).await;
    serde_json::to_value(events).map_err(|e| e.to_string())
}

// ── Runtime Monitor / Mesh Commands ──────────────────────────────────────────

#[tauri::command]
pub async fn admin_deploy_node(
    state: State<'_, AppState>,
    request: DeployNodeRequest,
) -> Result<serde_json::Value, String> {
    let node = state.runtime_monitor.deploy_node(
        &request.name,
        request.role,
        request.section,
        request.config,
    ).await;
    serde_json::to_value(node).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn admin_update_node_perf(
    state: State<'_, AppState>,
    request: UpdateNodePerfRequest,
) -> Result<serde_json::Value, String> {
    let node = state.runtime_monitor.update_node_performance(
        &request.node_id,
        request.performance,
    ).await;
    serde_json::to_value(node).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn admin_get_mesh_nodes(
    state: State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    let nodes = state.runtime_monitor.get_nodes().await;
    serde_json::to_value(nodes).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn admin_remove_node(
    state: State<'_, AppState>,
    node_id: String,
) -> Result<bool, String> {
    Ok(state.runtime_monitor.remove_node(&node_id).await)
}

#[tauri::command]
pub async fn admin_auto_tune(
    state: State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    let actions = state.runtime_monitor.auto_tune().await;
    serde_json::to_value(actions).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn admin_apply_tuning(
    state: State<'_, AppState>,
    request: ApplyTuningRequest,
) -> Result<serde_json::Value, String> {
    let action = state.runtime_monitor.apply_tuning(
        &request.target_node,
        request.action_type,
        &request.parameter,
        &request.old_value,
        &request.new_value,
        &request.reason,
    ).await;
    serde_json::to_value(action).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn admin_get_tuning_history(
    state: State<'_, AppState>,
    limit: Option<usize>,
) -> Result<serde_json::Value, String> {
    let history = state.runtime_monitor.get_tuning_history(limit.unwrap_or(50)).await;
    serde_json::to_value(history).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn admin_record_data_flow(
    state: State<'_, AppState>,
    request: RecordDataFlowRequest,
) -> Result<serde_json::Value, String> {
    let record = state.runtime_monitor.record_data_flow(
        request.source,
        request.target,
        request.payload_size_bytes,
        request.latency_ms,
        request.success,
        &request.routed_via,
    ).await;
    serde_json::to_value(record).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn admin_get_section_health(
    state: State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    let health = state.runtime_monitor.get_all_section_health().await;
    serde_json::to_value(health).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn admin_get_deployment(
    state: State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    let deploy = state.runtime_monitor.get_deployment().await;
    serde_json::to_value(deploy).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn admin_set_topology(
    state: State<'_, AppState>,
    topology: MeshTopology,
) -> Result<(), String> {
    state.runtime_monitor.set_topology(topology).await;
    Ok(())
}

#[tauri::command]
pub async fn admin_compute_mesh_health(
    state: State<'_, AppState>,
) -> Result<f64, String> {
    Ok(state.runtime_monitor.compute_mesh_health().await)
}

#[tauri::command]
pub async fn admin_auto_scale_check(
    state: State<'_, AppState>,
) -> Result<Option<String>, String> {
    Ok(state.runtime_monitor.auto_scale_check().await)
}

// ── IDE & Platform Integration Commands ──────────────────────────────────────

#[derive(Deserialize)]
pub struct RegisterPlatformRequest {
    pub platform: IntegrationPlatform,
    pub endpoint: String,
    pub auth_method: AuthMethod,
    pub config: PlatformConfig,
}

#[derive(Deserialize)]
pub struct SyncPlatformRequest {
    pub connection_id: String,
    pub event_type: SyncEventType,
}

#[tauri::command]
pub async fn admin_register_platform(
    state: State<'_, AppState>,
    request: RegisterPlatformRequest,
) -> Result<serde_json::Value, String> {
    let conn = state.integrations.register_platform(
        request.platform,
        &request.endpoint,
        request.auth_method,
        request.config,
    ).await;
    serde_json::to_value(conn).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn admin_connect_platform(
    state: State<'_, AppState>,
    connection_id: String,
) -> Result<serde_json::Value, String> {
    let conn = state.integrations.connect(&connection_id)
        .await
        .map_err(|e| e.to_string())?;
    serde_json::to_value(conn).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn admin_disconnect_platform(
    state: State<'_, AppState>,
    connection_id: String,
) -> Result<serde_json::Value, String> {
    let conn = state.integrations.disconnect(&connection_id)
        .await
        .map_err(|e| e.to_string())?;
    serde_json::to_value(conn).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn admin_sync_platform(
    state: State<'_, AppState>,
    request: SyncPlatformRequest,
) -> Result<serde_json::Value, String> {
    let event = state.integrations.sync_platform(&request.connection_id, request.event_type)
        .await
        .map_err(|e| e.to_string())?;
    serde_json::to_value(event).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn admin_list_integrations(
    state: State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    let conns = state.integrations.list_connections().await;
    serde_json::to_value(conns).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn admin_get_integrations_by_category(
    state: State<'_, AppState>,
    category: PlatformCategory,
) -> Result<serde_json::Value, String> {
    let conns = state.integrations.get_by_category(category).await;
    serde_json::to_value(conns).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn admin_get_sync_history(
    state: State<'_, AppState>,
    platform: Option<IntegrationPlatform>,
    limit: Option<usize>,
) -> Result<serde_json::Value, String> {
    let history = state.integrations.get_sync_history(platform, limit.unwrap_or(50)).await;
    serde_json::to_value(history).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn admin_remove_integration(
    state: State<'_, AppState>,
    connection_id: String,
) -> Result<bool, String> {
    Ok(state.integrations.remove_platform(&connection_id).await)
}

#[tauri::command]
pub async fn admin_list_supported_platforms(
) -> Result<serde_json::Value, String> {
    let platforms: Vec<_> = IntegrationPlatform::all().iter().map(|p| {
        serde_json::json!({
            "id": p,
            "name": p.display_name(),
            "category": p.category(),
        })
    }).collect();
    serde_json::to_value(platforms).map_err(|e| e.to_string())
}
