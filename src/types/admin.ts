// NATLaB BUILDR26 — Admin Audit & Runtime Monitor Types
// Backend-only ADMIN system type definitions.
// This system NEVER interferes with user-facing dynamics.

// ── Priority Hierarchy (Red → Green) ─────────────────────────────────────────

export type TaskPriority = "red" | "orange" | "yellow" | "green";

export const PRIORITY_CONFIG: Record<TaskPriority, { label: string; colour: string; urgent: boolean }> = {
  red: { label: "URGENT", colour: "#ef4444", urgent: true },
  orange: { label: "HIGH", colour: "#f97316", urgent: true },
  yellow: { label: "MEDIUM", colour: "#eab308", urgent: false },
  green: { label: "MONITOR", colour: "#22c55e", urgent: false },
};

// ── Audit Trail ──────────────────────────────────────────────────────────────

export type AuditCategory =
  | "dependency_update"
  | "security_patch"
  | "build_failure"
  | "test_regression"
  | "performance_degradation"
  | "system_health"
  | "process_allocation"
  | "kernel_protection"
  | "automation_event"
  | "research_note";

export interface AuditEntry {
  id: string;
  timestamp: string;
  category: AuditCategory;
  severity: TaskPriority;
  source_module: string;
  message: string;
  details: string | null;
  resolved: boolean;
  resolved_at: string | null;
  auto_resolved: boolean;
}

// ── Dependency Monitor ───────────────────────────────────────────────────────

export interface DependencyStatus {
  name: string;
  current_version: string;
  latest_version: string | null;
  ecosystem: string;
  priority: TaskPriority;
  has_security_advisory: boolean;
  last_checked: string;
  auto_updatable: boolean;
  update_scheduled: boolean;
}

export interface PriorityBreakdown {
  red: number;
  orange: number;
  yellow: number;
  green: number;
}

export interface DailySummary {
  date: string;
  total_dependencies: number;
  outdated_count: number;
  security_alerts: number;
  auto_updated: number;
  pending_review: number;
  system_health_score: number;
  tasks_by_priority: PriorityBreakdown;
}

// ── Process Allocator ────────────────────────────────────────────────────────

export type ProcessStatus = "running" | "idle" | "suspended" | "terminated" | "protected";

export interface ProcessAllocation {
  process_id: string;
  name: string;
  allocated_at: string;
  priority: TaskPriority;
  memory_limit_mb: number;
  cpu_quota_pct: number;
  status: ProcessStatus;
  protected: boolean;
}

// ── Kernel Protection ────────────────────────────────────────────────────────

export type KernelEventType =
  | "unauthorized_access"
  | "privilege_escalation"
  | "integrity_violation"
  | "resource_exhaustion"
  | "anomalous_pattern"
  | "routine_check";

export interface KernelProtectionEvent {
  id: string;
  timestamp: string;
  event_type: KernelEventType;
  source: string;
  threat_level: TaskPriority;
  description: string;
  action_taken: string;
  blocked: boolean;
}

// ── Automated Error Response ─────────────────────────────────────────────────

export type ResolutionAction =
  | "auto_fixed"
  | "retry"
  | "escalated"
  | "deferred"
  | "requires_manual"
  | "researching";

export interface ErrorResponse {
  id: string;
  timestamp: string;
  error_source: string;
  error_type: string;
  original_message: string;
  auto_diagnosis: string;
  resolution_action: ResolutionAction;
  resolved: boolean;
  resolution_duration_ms: number;
}

// ── AI Mesh / Runtime Monitor ────────────────────────────────────────────────

export type AiNodeRole =
  | "health_monitor"
  | "performance_tuner"
  | "dependency_manager"
  | "security_scanner"
  | "build_orchestrator"
  | "data_router"
  | "mesh_coordinator";

export type NodeStatus = "active" | "idle" | "tuning" | "overloaded" | "recovering" | "offline";

export type DataSection =
  | "build_pipeline"
  | "security_infra"
  | "dependency_graph"
  | "performance_metrics"
  | "audit_trail"
  | "user_profiles"
  | "plugin_registry"
  | "system_config";

export type MeshTopology = "full_mesh" | "star" | "hierarchical" | "ring";

export type TuningType =
  | "scale_up"
  | "scale_down"
  | "throttle_adjust"
  | "memory_realloc"
  | "priority_shift"
  | "route_optimize"
  | "cache_resize"
  | "batch_size_adjust";

export interface NodeConfig {
  max_throughput_ops: number;
  memory_budget_mb: number;
  cpu_affinity: number[];
  auto_scale: boolean;
  tuning_interval_ms: number;
  priority: TaskPriority;
}

export interface NodePerformance {
  ops_per_second: number;
  avg_latency_ms: number;
  p99_latency_ms: number;
  error_rate_pct: number;
  memory_used_mb: number;
  cpu_usage_pct: number;
  queue_depth: number;
  uptime_secs: number;
  last_tuned: string | null;
}

export interface AiNode {
  node_id: string;
  name: string;
  role: AiNodeRole;
  section: DataSection;
  status: NodeStatus;
  performance: NodePerformance;
  last_heartbeat: string;
  deployed_at: string;
  config: NodeConfig;
}

export interface TuningAction {
  id: string;
  timestamp: string;
  target_node: string;
  action_type: TuningType;
  parameter: string;
  old_value: string;
  new_value: string;
  reason: string;
  impact_score: number;
}

export interface DataFlowRecord {
  id: string;
  timestamp: string;
  source_section: DataSection;
  target_section: DataSection;
  payload_size_bytes: number;
  latency_ms: number;
  success: boolean;
  routed_via: string;
}

export interface SectionHealth {
  section: DataSection;
  throughput_ops: number;
  avg_latency_ms: number;
  error_rate_pct: number;
  storage_used_mb: number;
  assigned_nodes: string[];
  last_updated: string;
}

export interface MeshDeployment {
  deployment_id: string;
  created_at: string;
  updated_at: string;
  total_nodes: number;
  active_nodes: number;
  topology: MeshTopology;
  health_score: number;
  auto_scaling_enabled: boolean;
  min_nodes: number;
  max_nodes: number;
}

// ── IDE & Platform Integration ───────────────────────────────────────────────

export type IntegrationPlatform =
  | "py_charm"
  | "juno"
  | "jet_brains_fleet"
  | "jet_brains_intelli_j"
  | "jet_brains_rider"
  | "jet_brains_web_storm"
  | "jet_brains_clion"
  | "vs_studio"
  | "vs_code"
  | "qodana"
  | "google_ai_studio"
  | "hp_studio";

export type PlatformCategory = "ide" | "quality_analysis" | "ai_platform" | "dev_ops";

export type ConnectionStatus =
  | "connected"
  | "disconnected"
  | "authenticating"
  | "syncing"
  | "error"
  | "degraded";

export type AuthMethod = "api_key" | "oauth2" | "service_account" | "token" | "certificate" | "none";

export type PlatformCapability =
  | "code_analysis"
  | "live_linting"
  | "build_trigger"
  | "test_execution"
  | "deployment_pipeline"
  | "ai_completion"
  | "ai_code_review"
  | "security_scan"
  | "performance_profiling"
  | "dependency_analysis"
  | "project_sync"
  | "remote_debug"
  | "container_management"
  | "model_training"
  | "data_pipeline";

export type SyncEventType =
  | "full_sync"
  | "incremental_sync"
  | "push"
  | "pull"
  | "webhook"
  | "health_check"
  | "config_update";

export interface PlatformConfig {
  auto_sync: boolean;
  sync_interval_secs: number;
  max_retries: number;
  timeout_ms: number;
  priority: TaskPriority;
  enabled_features: string[];
}

export interface PlatformConnection {
  connection_id: string;
  platform: IntegrationPlatform;
  status: ConnectionStatus;
  endpoint: string;
  auth_method: AuthMethod;
  connected_at: string | null;
  last_sync: string | null;
  last_error: string | null;
  config: PlatformConfig;
  capabilities: PlatformCapability[];
  health_score: number;
}

export interface SyncEvent {
  id: string;
  timestamp: string;
  platform: IntegrationPlatform;
  event_type: SyncEventType;
  payload_summary: string;
  duration_ms: number;
  success: boolean;
  error: string | null;
  items_synced: number;
}

