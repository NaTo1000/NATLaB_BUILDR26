// NATLaB BUILDR26 — Admin Process Engine (Backend Bridge)
// Internal runtime monitoring, AI mesh orchestration, and IDE integration.
// This store is ADMIN-ONLY and does NOT render user-facing UI.

import { invoke } from "@tauri-apps/api/core";
import { create } from "zustand";
import type {
  AuditEntry,
  DailySummary,
  DependencyStatus,
  ErrorResponse,
  ProcessAllocation,
  KernelProtectionEvent,
  TaskPriority,
  AiNode,
  TuningAction,
  MeshDeployment,
  SectionHealth,
  AuditCategory,
  KernelEventType,
  ProcessStatus,
  AiNodeRole,
  DataSection,
  NodeConfig,
  NodePerformance,
  MeshTopology,
  TuningType,
} from "../../types/admin";

// ── Admin Audit API Bridge ───────────────────────────────────────────────────

export const AdminAPI = {
  // Audit Trail
  logAudit: (req: {
    category: AuditCategory;
    severity: TaskPriority;
    source_module: string;
    message: string;
    details?: string;
  }) => invoke<AuditEntry>("admin_log_audit", { request: req }),

  resolveAudit: (id: string, autoResolved: boolean) =>
    invoke<AuditEntry>("admin_resolve_audit", { request: { id, auto_resolved: autoResolved } }),

  getAuditLog: (priority?: TaskPriority) =>
    invoke<AuditEntry[]>("admin_get_audit_log", { priority }),

  getTodoList: () => invoke<AuditEntry[]>("admin_get_todo_list"),

  // Dependencies
  updateDependency: (status: DependencyStatus) =>
    invoke<void>("admin_update_dependency", { status }),

  getDependencies: () => invoke<DependencyStatus[]>("admin_get_dependencies"),

  generateDailySummary: () => invoke<DailySummary>("admin_generate_daily_summary"),

  getDailySummaries: (count?: number) =>
    invoke<DailySummary[]>("admin_get_daily_summaries", { count }),

  // Error Response
  handleError: (req: { error_source: string; error_type: string; message: string }) =>
    invoke<ErrorResponse>("admin_handle_error", { request: req }),

  getErrorResponses: (limit?: number) =>
    invoke<ErrorResponse[]>("admin_get_error_responses", { limit }),

  // Process Allocation
  allocateProcess: (req: {
    name: string;
    priority: TaskPriority;
    memory_limit_mb: number;
    cpu_quota_pct: number;
    protected: boolean;
  }) => invoke<ProcessAllocation>("admin_allocate_process", { request: req }),

  updateProcess: (processId: string, status: ProcessStatus) =>
    invoke<ProcessAllocation>("admin_update_process", { request: { process_id: processId, status } }),

  listProcesses: () => invoke<ProcessAllocation[]>("admin_list_processes"),

  // Kernel Protection
  logKernelEvent: (req: {
    event_type: KernelEventType;
    source: string;
    threat_level: TaskPriority;
    description: string;
    action_taken: string;
    blocked: boolean;
  }) => invoke<KernelProtectionEvent>("admin_log_kernel_event", { request: req }),

  getKernelEvents: (limit?: number) =>
    invoke<KernelProtectionEvent[]>("admin_get_kernel_events", { limit }),

  // Mesh / Runtime Monitor
  deployNode: (req: {
    name: string;
    role: AiNodeRole;
    section: DataSection;
    config: NodeConfig;
  }) => invoke<AiNode>("admin_deploy_node", { request: req }),

  updateNodePerf: (nodeId: string, performance: NodePerformance) =>
    invoke<AiNode | null>("admin_update_node_perf", { request: { node_id: nodeId, performance } }),

  getMeshNodes: () => invoke<AiNode[]>("admin_get_mesh_nodes"),

  removeNode: (nodeId: string) => invoke<boolean>("admin_remove_node", { nodeId }),

  autoTune: () => invoke<TuningAction[]>("admin_auto_tune"),

  applyTuning: (req: {
    target_node: string;
    action_type: TuningType;
    parameter: string;
    old_value: string;
    new_value: string;
    reason: string;
  }) => invoke<TuningAction>("admin_apply_tuning", { request: req }),

  getTuningHistory: (limit?: number) =>
    invoke<TuningAction[]>("admin_get_tuning_history", { limit }),

  getSectionHealth: () => invoke<SectionHealth[]>("admin_get_section_health"),

  getDeployment: () => invoke<MeshDeployment>("admin_get_deployment"),

  setTopology: (topology: MeshTopology) =>
    invoke<void>("admin_set_topology", { topology }),

  computeMeshHealth: () => invoke<number>("admin_compute_mesh_health"),

  autoScaleCheck: () => invoke<string | null>("admin_auto_scale_check"),

  // IDE & Platform Integrations
  listSupportedPlatforms: () =>
    invoke<Array<{ id: string; name: string; category: string }>>("admin_list_supported_platforms"),

  listIntegrations: () => invoke("admin_list_integrations"),

  registerPlatform: (req: {
    platform: string;
    endpoint: string;
    auth_method: string;
    config: Record<string, unknown>;
  }) => invoke("admin_register_platform", { request: req }),

  connectPlatform: (connectionId: string) =>
    invoke("admin_connect_platform", { connectionId }),

  disconnectPlatform: (connectionId: string) =>
    invoke("admin_disconnect_platform", { connectionId }),

  syncPlatform: (connectionId: string, eventType: string) =>
    invoke("admin_sync_platform", { request: { connection_id: connectionId, event_type: eventType } }),

  removePlatform: (connectionId: string) =>
    invoke<boolean>("admin_remove_integration", { connectionId }),
};

// ── Admin Process Store ──────────────────────────────────────────────────────

interface AdminProcessState {
  // Audit
  auditLog: AuditEntry[];
  todoList: AuditEntry[];
  // Dependencies
  dependencies: DependencyStatus[];
  dailySummary: DailySummary | null;
  // Errors
  errorResponses: ErrorResponse[];
  // Processes
  processes: ProcessAllocation[];
  // Kernel
  kernelEvents: KernelProtectionEvent[];
  // Mesh
  meshNodes: AiNode[];
  deployment: MeshDeployment | null;
  sectionHealth: SectionHealth[];
  tuningHistory: TuningAction[];
  // State
  loading: boolean;
  error: string | null;
  // Actions
  refreshAll: () => Promise<void>;
  refreshAudit: () => Promise<void>;
  refreshMesh: () => Promise<void>;
  triggerAutoTune: () => Promise<TuningAction[]>;
  generateSummary: () => Promise<DailySummary>;
}

export const useAdminProcessStore = create<AdminProcessState>((set) => ({
  auditLog: [],
  todoList: [],
  dependencies: [],
  dailySummary: null,
  errorResponses: [],
  processes: [],
  kernelEvents: [],
  meshNodes: [],
  deployment: null,
  sectionHealth: [],
  tuningHistory: [],
  loading: false,
  error: null,

  refreshAll: async () => {
    set({ loading: true, error: null });
    try {
      const [auditLog, todoList, dependencies, processes, kernelEvents, meshNodes, deployment, sectionHealth] =
        await Promise.all([
          AdminAPI.getAuditLog(),
          AdminAPI.getTodoList(),
          AdminAPI.getDependencies(),
          AdminAPI.listProcesses(),
          AdminAPI.getKernelEvents(),
          AdminAPI.getMeshNodes(),
          AdminAPI.getDeployment(),
          AdminAPI.getSectionHealth(),
        ]);
      set({
        auditLog, todoList, dependencies, processes,
        kernelEvents, meshNodes, deployment, sectionHealth,
        loading: false,
      });
    } catch (err) {
      set({ error: String(err), loading: false });
    }
  },

  refreshAudit: async () => {
    try {
      const [auditLog, todoList, errorResponses] = await Promise.all([
        AdminAPI.getAuditLog(),
        AdminAPI.getTodoList(),
        AdminAPI.getErrorResponses(),
      ]);
      set({ auditLog, todoList, errorResponses });
    } catch (err) {
      set({ error: String(err) });
    }
  },

  refreshMesh: async () => {
    try {
      const [meshNodes, deployment, sectionHealth, tuningHistory] = await Promise.all([
        AdminAPI.getMeshNodes(),
        AdminAPI.getDeployment(),
        AdminAPI.getSectionHealth(),
        AdminAPI.getTuningHistory(),
      ]);
      set({ meshNodes, deployment, sectionHealth, tuningHistory });
    } catch (err) {
      set({ error: String(err) });
    }
  },

  triggerAutoTune: async () => {
    const actions = await AdminAPI.autoTune();
    const tuningHistory = await AdminAPI.getTuningHistory();
    set({ tuningHistory });
    return actions;
  },

  generateSummary: async () => {
    const summary = await AdminAPI.generateDailySummary();
    set({ dailySummary: summary });
    return summary;
  },
}));
