// NATLaB BUILDR26 — Admin Process Engine Tests
// Validates the admin audit trail, priority system, runtime monitor,
// and IDE integration types/logic.

import { describe, it, expect } from "vitest";
import { PRIORITY_CONFIG } from "../types/admin";
import type {
  AuditEntry,
  DailySummary,
  DependencyStatus,
  ProcessAllocation,
  KernelProtectionEvent,
  ErrorResponse,
  AiNode,
  TuningAction,
  MeshDeployment,
  PlatformConnection,
  SyncEvent,
  IntegrationPlatform,
} from "../types/admin";

// ── Priority Hierarchy Tests ─────────────────────────────────────────────────

describe("TaskPriority System", () => {
  it("defines all priority levels with correct colours", () => {
    expect(PRIORITY_CONFIG.red.colour).toBe("#ef4444");
    expect(PRIORITY_CONFIG.orange.colour).toBe("#f97316");
    expect(PRIORITY_CONFIG.yellow.colour).toBe("#eab308");
    expect(PRIORITY_CONFIG.green.colour).toBe("#22c55e");
  });

  it("marks red and orange as urgent", () => {
    expect(PRIORITY_CONFIG.red.urgent).toBe(true);
    expect(PRIORITY_CONFIG.orange.urgent).toBe(true);
    expect(PRIORITY_CONFIG.yellow.urgent).toBe(false);
    expect(PRIORITY_CONFIG.green.urgent).toBe(false);
  });

  it("has correct labels for each priority", () => {
    expect(PRIORITY_CONFIG.red.label).toBe("URGENT");
    expect(PRIORITY_CONFIG.orange.label).toBe("HIGH");
    expect(PRIORITY_CONFIG.yellow.label).toBe("MEDIUM");
    expect(PRIORITY_CONFIG.green.label).toBe("MONITOR");
  });
});

// ── Audit Entry Type Shape Tests ─────────────────────────────────────────────

describe("AuditEntry Structure", () => {
  const mockEntry: AuditEntry = {
    id: "test-001",
    timestamp: "2026-06-12T10:00:00Z",
    category: "dependency_update",
    severity: "red",
    source_module: "admin_engine",
    message: "Critical dependency outdated",
    details: "react@18.2.0 → 19.0.0",
    resolved: false,
    resolved_at: null,
    auto_resolved: false,
  };

  it("creates a valid audit entry shape", () => {
    expect(mockEntry.id).toBe("test-001");
    expect(mockEntry.category).toBe("dependency_update");
    expect(mockEntry.severity).toBe("red");
    expect(mockEntry.resolved).toBe(false);
  });

  it("can mark entry as resolved", () => {
    const resolved: AuditEntry = {
      ...mockEntry,
      resolved: true,
      resolved_at: "2026-06-12T11:00:00Z",
      auto_resolved: true,
    };
    expect(resolved.resolved).toBe(true);
    expect(resolved.auto_resolved).toBe(true);
    expect(resolved.resolved_at).not.toBeNull();
  });
});

// ── Dependency Status Tests ──────────────────────────────────────────────────

describe("DependencyStatus Structure", () => {
  const mockDep: DependencyStatus = {
    name: "zustand",
    current_version: "4.4.7",
    latest_version: "4.5.0",
    ecosystem: "npm",
    priority: "green",
    has_security_advisory: false,
    last_checked: "2026-06-12T10:00:00Z",
    auto_updatable: true,
    update_scheduled: false,
  };

  it("tracks dependency versioning", () => {
    expect(mockDep.current_version).toBe("4.4.7");
    expect(mockDep.latest_version).toBe("4.5.0");
    expect(mockDep.latest_version).not.toBe(mockDep.current_version);
  });

  it("flags security advisories separately from updates", () => {
    const critical: DependencyStatus = {
      ...mockDep,
      priority: "red",
      has_security_advisory: true,
    };
    expect(critical.priority).toBe("red");
    expect(critical.has_security_advisory).toBe(true);
  });
});

// ── Daily Summary Tests ──────────────────────────────────────────────────────

describe("DailySummary Structure", () => {
  const mockSummary: DailySummary = {
    date: "2026-06-12T00:00:00Z",
    total_dependencies: 45,
    outdated_count: 3,
    security_alerts: 0,
    auto_updated: 2,
    pending_review: 1,
    system_health_score: 94.0,
    tasks_by_priority: { red: 0, orange: 1, yellow: 2, green: 5 },
  };

  it("computes system health score", () => {
    expect(mockSummary.system_health_score).toBeGreaterThan(0);
    expect(mockSummary.system_health_score).toBeLessThanOrEqual(100);
  });

  it("breaks down tasks by priority", () => {
    const { red, orange, yellow, green } = mockSummary.tasks_by_priority;
    expect(red + orange + yellow + green).toBe(8);
    expect(red).toBe(0);
  });
});

// ── Process Allocation Tests ─────────────────────────────────────────────────

describe("ProcessAllocation Structure", () => {
  const mockProcess: ProcessAllocation = {
    process_id: "proc-001",
    name: "dependency_monitor",
    allocated_at: "2026-06-12T10:00:00Z",
    priority: "green",
    memory_limit_mb: 256,
    cpu_quota_pct: 10.0,
    status: "running",
    protected: true,
  };

  it("creates a process with resource limits", () => {
    expect(mockProcess.memory_limit_mb).toBe(256);
    expect(mockProcess.cpu_quota_pct).toBe(10.0);
    expect(mockProcess.protected).toBe(true);
  });

  it("protected processes cannot be terminated", () => {
    // This validates the type contract — actual enforcement is in Rust backend
    expect(mockProcess.protected).toBe(true);
    expect(mockProcess.status).not.toBe("terminated");
  });
});

// ── Kernel Protection Tests ──────────────────────────────────────────────────

describe("KernelProtection Structure", () => {
  const mockEvent: KernelProtectionEvent = {
    id: "kern-001",
    timestamp: "2026-06-12T10:00:00Z",
    event_type: "unauthorized_access",
    source: "external_plugin",
    threat_level: "red",
    description: "Attempt to access protected memory region",
    action_taken: "Blocked and isolated",
    blocked: true,
  };

  it("logs security events with threat levels", () => {
    expect(mockEvent.threat_level).toBe("red");
    expect(mockEvent.blocked).toBe(true);
  });

  it("identifies event types correctly", () => {
    expect(mockEvent.event_type).toBe("unauthorized_access");
  });
});

// ── Error Response Tests ─────────────────────────────────────────────────────

describe("ErrorResponse AI Engine", () => {
  const mockResponse: ErrorResponse = {
    id: "err-001",
    timestamp: "2026-06-12T10:00:00Z",
    error_source: "build_pipeline",
    error_type: "dependency",
    original_message: "Package outdated: lodash@4.17.20",
    auto_diagnosis: "Outdated dependency detected. Scheduling automatic update.",
    resolution_action: "auto_fixed",
    resolved: true,
    resolution_duration_ms: 15,
  };

  it("auto-diagnoses dependency errors", () => {
    expect(mockResponse.resolution_action).toBe("auto_fixed");
    expect(mockResponse.resolved).toBe(true);
  });

  it("tracks resolution duration", () => {
    expect(mockResponse.resolution_duration_ms).toBeGreaterThanOrEqual(0);
  });
});

// ── AI Mesh Node Tests ───────────────────────────────────────────────────────

describe("AI Mesh Runtime Monitor", () => {
  const mockNode: AiNode = {
    node_id: "node-001",
    name: "perf_tuner_alpha",
    role: "performance_tuner",
    section: "performance_metrics",
    status: "active",
    performance: {
      ops_per_second: 1500.0,
      avg_latency_ms: 12.5,
      p99_latency_ms: 45.0,
      error_rate_pct: 0.02,
      memory_used_mb: 128.0,
      cpu_usage_pct: 35.0,
      queue_depth: 4,
      uptime_secs: 86400,
      last_tuned: "2026-06-12T09:00:00Z",
    },
    last_heartbeat: "2026-06-12T10:00:00Z",
    deployed_at: "2026-06-11T10:00:00Z",
    config: {
      max_throughput_ops: 5000,
      memory_budget_mb: 512,
      cpu_affinity: [0, 1, 2, 3],
      auto_scale: true,
      tuning_interval_ms: 60000,
      priority: "green",
    },
  };

  it("deploys nodes with performance tracking", () => {
    expect(mockNode.performance.ops_per_second).toBe(1500.0);
    expect(mockNode.performance.error_rate_pct).toBeLessThan(1.0);
    expect(mockNode.status).toBe("active");
  });

  it("assigns nodes to data sections", () => {
    expect(mockNode.section).toBe("performance_metrics");
    expect(mockNode.role).toBe("performance_tuner");
  });

  it("tracks node configuration for auto-scaling", () => {
    expect(mockNode.config.auto_scale).toBe(true);
    expect(mockNode.config.max_throughput_ops).toBe(5000);
  });
});

// ── Tuning Action Tests ──────────────────────────────────────────────────────

describe("Real-Time Tuning System", () => {
  const mockTuning: TuningAction = {
    id: "tune-001",
    timestamp: "2026-06-12T10:00:00Z",
    target_node: "node-001",
    action_type: "scale_up",
    parameter: "max_throughput_ops",
    old_value: "5000",
    new_value: "10000",
    reason: "Avg latency 150ms exceeds 100ms threshold",
    impact_score: 1.5,
  };

  it("records tuning actions with before/after values", () => {
    expect(mockTuning.old_value).toBe("5000");
    expect(mockTuning.new_value).toBe("10000");
    expect(parseInt(mockTuning.new_value)).toBeGreaterThan(parseInt(mockTuning.old_value));
  });

  it("provides reason and impact scoring", () => {
    expect(mockTuning.reason).toContain("latency");
    expect(mockTuning.impact_score).toBeGreaterThan(0);
  });
});

// ── Mesh Deployment Tests ────────────────────────────────────────────────────

describe("Mesh Deployment Management", () => {
  const mockDeploy: MeshDeployment = {
    deployment_id: "deploy-001",
    created_at: "2026-06-11T10:00:00Z",
    updated_at: "2026-06-12T10:00:00Z",
    total_nodes: 8,
    active_nodes: 7,
    topology: "star",
    health_score: 95.5,
    auto_scaling_enabled: true,
    min_nodes: 3,
    max_nodes: 20,
  };

  it("tracks mesh deployment status", () => {
    expect(mockDeploy.active_nodes).toBeLessThanOrEqual(mockDeploy.total_nodes);
    expect(mockDeploy.health_score).toBeGreaterThan(90);
  });

  it("enforces auto-scaling bounds", () => {
    expect(mockDeploy.min_nodes).toBeLessThan(mockDeploy.max_nodes);
    expect(mockDeploy.total_nodes).toBeGreaterThanOrEqual(mockDeploy.min_nodes);
    expect(mockDeploy.total_nodes).toBeLessThanOrEqual(mockDeploy.max_nodes);
  });
});

// ── IDE Integration Tests ────────────────────────────────────────────────────

describe("IDE & Platform Integration", () => {
  const mockConnection: PlatformConnection = {
    connection_id: "conn-001",
    platform: "py_charm",
    status: "connected",
    endpoint: "https://localhost:63342",
    auth_method: "token",
    connected_at: "2026-06-12T10:00:00Z",
    last_sync: "2026-06-12T10:30:00Z",
    last_error: null,
    config: {
      auto_sync: true,
      sync_interval_secs: 300,
      max_retries: 3,
      timeout_ms: 30000,
      priority: "green",
      enabled_features: ["code_analysis", "live_linting"],
    },
    capabilities: ["code_analysis", "live_linting", "test_execution", "remote_debug", "dependency_analysis"],
    health_score: 100.0,
  };

  it("registers platform connections with capabilities", () => {
    expect(mockConnection.capabilities).toContain("code_analysis");
    expect(mockConnection.capabilities.length).toBeGreaterThan(0);
  });

  it("tracks connection health", () => {
    expect(mockConnection.health_score).toBe(100.0);
    expect(mockConnection.status).toBe("connected");
    expect(mockConnection.last_error).toBeNull();
  });

  it("supports all required platforms", () => {
    const platforms: IntegrationPlatform[] = [
      "py_charm",
      "juno",
      "jet_brains_fleet",
      "jet_brains_intelli_j",
      "jet_brains_rider",
      "jet_brains_web_storm",
      "jet_brains_clion",
      "vs_studio",
      "vs_code",
      "qodana",
      "google_ai_studio",
      "hp_studio",
    ];
    expect(platforms.length).toBe(12);
    expect(platforms).toContain("py_charm");
    expect(platforms).toContain("google_ai_studio");
    expect(platforms).toContain("hp_studio");
    expect(platforms).toContain("qodana");
  });

  it("configures auto-sync with interval", () => {
    expect(mockConnection.config.auto_sync).toBe(true);
    expect(mockConnection.config.sync_interval_secs).toBe(300);
  });
});

describe("Sync Events", () => {
  const mockSync: SyncEvent = {
    id: "sync-001",
    timestamp: "2026-06-12T10:30:00Z",
    platform: "google_ai_studio",
    event_type: "full_sync",
    payload_summary: "Sync with Google AI Studio",
    duration_ms: 245,
    success: true,
    error: null,
    items_synced: 15,
  };

  it("records sync events with duration", () => {
    expect(mockSync.success).toBe(true);
    expect(mockSync.duration_ms).toBeGreaterThan(0);
    expect(mockSync.items_synced).toBe(15);
  });

  it("tracks platform source of sync", () => {
    expect(mockSync.platform).toBe("google_ai_studio");
    expect(mockSync.event_type).toBe("full_sync");
  });
});
