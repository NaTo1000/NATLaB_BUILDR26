// NATLaB BUILDR26 — Runtime Monitoring & AI Mesh Orchestration
// Internal real-time performance tuning, multi-AI mesh deployment,
// and sectional data management. Backend-only ADMIN process.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::TaskPriority;

// ── AI Node Identity ─────────────────────────────────────────────────────────

/// Represents a single AI agent node in the mesh network.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiNode {
    pub node_id: String,
    pub name: String,
    pub role: AiNodeRole,
    pub section: DataSection,
    pub status: NodeStatus,
    pub performance: NodePerformance,
    pub last_heartbeat: DateTime<Utc>,
    pub deployed_at: DateTime<Utc>,
    pub config: NodeConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AiNodeRole {
    /// Monitors system health metrics
    HealthMonitor,
    /// Tunes performance parameters in real-time
    PerformanceTuner,
    /// Manages dependency updates and validation
    DependencyManager,
    /// Handles security scanning and threat detection
    SecurityScanner,
    /// Orchestrates build pipeline stages
    BuildOrchestrator,
    /// Manages data flow between sections
    DataRouter,
    /// Coordinates other AI nodes
    MeshCoordinator,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum NodeStatus {
    Active,
    Idle,
    Tuning,
    Overloaded,
    Recovering,
    Offline,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    pub max_throughput_ops: u64,
    pub memory_budget_mb: u64,
    pub cpu_affinity: Vec<u32>,
    pub auto_scale: bool,
    pub tuning_interval_ms: u64,
    pub priority: TaskPriority,
}

// ── Performance Metrics ──────────────────────────────────────────────────────

/// Real-time performance snapshot for a single AI node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodePerformance {
    pub ops_per_second: f64,
    pub avg_latency_ms: f64,
    pub p99_latency_ms: f64,
    pub error_rate_pct: f64,
    pub memory_used_mb: f64,
    pub cpu_usage_pct: f64,
    pub queue_depth: u32,
    pub uptime_secs: u64,
    pub last_tuned: Option<DateTime<Utc>>,
}

impl Default for NodePerformance {
    fn default() -> Self {
        Self {
            ops_per_second: 0.0,
            avg_latency_ms: 0.0,
            p99_latency_ms: 0.0,
            error_rate_pct: 0.0,
            memory_used_mb: 0.0,
            cpu_usage_pct: 0.0,
            queue_depth: 0,
            uptime_secs: 0,
            last_tuned: None,
        }
    }
}

/// A tuning action applied by the performance management system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuningAction {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub target_node: String,
    pub action_type: TuningType,
    pub parameter: String,
    pub old_value: String,
    pub new_value: String,
    pub reason: String,
    pub impact_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TuningType {
    ScaleUp,
    ScaleDown,
    ThrottleAdjust,
    MemoryRealloc,
    PriorityShift,
    RouteOptimize,
    CacheResize,
    BatchSizeAdjust,
}

// ── Sectional Data Management ────────────────────────────────────────────────

/// Defines a logical data section managed by the mesh.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum DataSection {
    BuildPipeline,
    SecurityInfra,
    DependencyGraph,
    PerformanceMetrics,
    AuditTrail,
    UserProfiles,
    PluginRegistry,
    SystemConfig,
}

/// Data flow record between sections.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataFlowRecord {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub source_section: DataSection,
    pub target_section: DataSection,
    pub payload_size_bytes: u64,
    pub latency_ms: f64,
    pub success: bool,
    pub routed_via: String,
}

/// Section health metrics for data management.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectionHealth {
    pub section: DataSection,
    pub throughput_ops: f64,
    pub avg_latency_ms: f64,
    pub error_rate_pct: f64,
    pub storage_used_mb: f64,
    pub assigned_nodes: Vec<String>,
    pub last_updated: DateTime<Utc>,
}

// ── Mesh Deployment ──────────────────────────────────────────────────────────

/// Deployment configuration for the AI mesh network.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshDeployment {
    pub deployment_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub total_nodes: u32,
    pub active_nodes: u32,
    pub topology: MeshTopology,
    pub health_score: f64,
    pub auto_scaling_enabled: bool,
    pub min_nodes: u32,
    pub max_nodes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MeshTopology {
    /// All nodes connected to all others
    FullMesh,
    /// Hub-and-spoke with coordinator at center
    Star,
    /// Hierarchical tree structure
    Hierarchical,
    /// Ring topology for sequential processing
    Ring,
}

// ── Runtime Monitor Engine ───────────────────────────────────────────────────

/// The AI Mesh Runtime Monitor: orchestrates multi-AI performance management
/// with real-time tuning and sectional data management.
pub struct RuntimeMonitor {
    nodes: Arc<RwLock<HashMap<String, AiNode>>>,
    tuning_history: Arc<RwLock<VecDeque<TuningAction>>>,
    data_flows: Arc<RwLock<VecDeque<DataFlowRecord>>>,
    section_health: Arc<RwLock<HashMap<DataSection, SectionHealth>>>,
    deployment: Arc<RwLock<MeshDeployment>>,
    max_tuning_history: usize,
    max_data_flows: usize,
}

impl RuntimeMonitor {
    pub fn new() -> Self {
        let deployment = MeshDeployment {
            deployment_id: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            total_nodes: 0,
            active_nodes: 0,
            topology: MeshTopology::Star,
            health_score: 100.0,
            auto_scaling_enabled: true,
            min_nodes: 3,
            max_nodes: 20,
        };

        Self {
            nodes: Arc::new(RwLock::new(HashMap::new())),
            tuning_history: Arc::new(RwLock::new(VecDeque::new())),
            data_flows: Arc::new(RwLock::new(VecDeque::new())),
            section_health: Arc::new(RwLock::new(HashMap::new())),
            deployment: Arc::new(RwLock::new(deployment)),
            max_tuning_history: 1000,
            max_data_flows: 2000,
        }
    }

    // ── Node Management ──────────────────────────────────────────────────────

    /// Deploy a new AI node into the mesh.
    pub async fn deploy_node(
        &self,
        name: &str,
        role: AiNodeRole,
        section: DataSection,
        config: NodeConfig,
    ) -> AiNode {
        let node = AiNode {
            node_id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            role,
            section: section.clone(),
            status: NodeStatus::Active,
            performance: NodePerformance::default(),
            last_heartbeat: Utc::now(),
            deployed_at: Utc::now(),
            config,
        };

        let mut nodes = self.nodes.write().await;
        nodes.insert(node.node_id.clone(), node.clone());

        // Update deployment stats
        let mut deploy = self.deployment.write().await;
        deploy.total_nodes = nodes.len() as u32;
        deploy.active_nodes = nodes.values()
            .filter(|n| n.status == NodeStatus::Active || n.status == NodeStatus::Tuning)
            .count() as u32;
        deploy.updated_at = Utc::now();

        node
    }

    /// Update node performance metrics (heartbeat).
    pub async fn update_node_performance(
        &self,
        node_id: &str,
        performance: NodePerformance,
    ) -> Option<AiNode> {
        let mut nodes = self.nodes.write().await;
        if let Some(node) = nodes.get_mut(node_id) {
            node.performance = performance;
            node.last_heartbeat = Utc::now();

            // Auto-detect overload and trigger status change
            if node.performance.cpu_usage_pct > 90.0 || node.performance.error_rate_pct > 5.0 {
                node.status = NodeStatus::Overloaded;
            } else if node.performance.cpu_usage_pct < 5.0 && node.performance.queue_depth == 0 {
                node.status = NodeStatus::Idle;
            } else {
                node.status = NodeStatus::Active;
            }

            Some(node.clone())
        } else {
            None
        }
    }

    /// Get all mesh nodes.
    pub async fn get_nodes(&self) -> Vec<AiNode> {
        let nodes = self.nodes.read().await;
        nodes.values().cloned().collect()
    }

    /// Get nodes by section.
    pub async fn get_nodes_by_section(&self, section: &DataSection) -> Vec<AiNode> {
        let nodes = self.nodes.read().await;
        nodes.values()
            .filter(|n| &n.section == section)
            .cloned()
            .collect()
    }

    /// Remove a node from the mesh.
    pub async fn remove_node(&self, node_id: &str) -> bool {
        let mut nodes = self.nodes.write().await;
        let removed = nodes.remove(node_id).is_some();

        if removed {
            let mut deploy = self.deployment.write().await;
            deploy.total_nodes = nodes.len() as u32;
            deploy.active_nodes = nodes.values()
                .filter(|n| n.status == NodeStatus::Active || n.status == NodeStatus::Tuning)
                .count() as u32;
            deploy.updated_at = Utc::now();
        }

        removed
    }

    // ── Real-Time Tuning ─────────────────────────────────────────────────────

    /// Apply a tuning action to optimise a node's performance.
    pub async fn apply_tuning(
        &self,
        target_node: &str,
        action_type: TuningType,
        parameter: &str,
        old_value: &str,
        new_value: &str,
        reason: &str,
    ) -> TuningAction {
        let action = TuningAction {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            target_node: target_node.to_string(),
            action_type,
            parameter: parameter.to_string(),
            old_value: old_value.to_string(),
            new_value: new_value.to_string(),
            reason: reason.to_string(),
            impact_score: 0.0,
        };

        // Mark node as tuning
        let mut nodes = self.nodes.write().await;
        if let Some(node) = nodes.get_mut(target_node) {
            node.status = NodeStatus::Tuning;
            node.performance.last_tuned = Some(Utc::now());
        }
        drop(nodes);

        let mut history = self.tuning_history.write().await;
        if history.len() >= self.max_tuning_history {
            history.pop_front();
        }
        history.push_back(action.clone());

        action
    }

    /// Auto-tune: analyse all nodes and apply optimisations where needed.
    pub async fn auto_tune(&self) -> Vec<TuningAction> {
        let nodes = self.nodes.read().await;
        let mut actions = Vec::new();

        for node in nodes.values() {
            // High latency → scale up
            if node.performance.avg_latency_ms > 100.0 && node.config.auto_scale {
                let action = TuningAction {
                    id: Uuid::new_v4().to_string(),
                    timestamp: Utc::now(),
                    target_node: node.node_id.clone(),
                    action_type: TuningType::ScaleUp,
                    parameter: "throughput_limit".to_string(),
                    old_value: node.config.max_throughput_ops.to_string(),
                    new_value: (node.config.max_throughput_ops * 2).to_string(),
                    reason: format!(
                        "Avg latency {:.1}ms exceeds 100ms threshold",
                        node.performance.avg_latency_ms
                    ),
                    impact_score: node.performance.avg_latency_ms / 100.0,
                };
                actions.push(action);
            }

            // Memory pressure → realloc
            if node.performance.memory_used_mb > node.config.memory_budget_mb as f64 * 0.85 {
                let new_budget = (node.config.memory_budget_mb as f64 * 1.5) as u64;
                let action = TuningAction {
                    id: Uuid::new_v4().to_string(),
                    timestamp: Utc::now(),
                    target_node: node.node_id.clone(),
                    action_type: TuningType::MemoryRealloc,
                    parameter: "memory_budget_mb".to_string(),
                    old_value: node.config.memory_budget_mb.to_string(),
                    new_value: new_budget.to_string(),
                    reason: format!(
                        "Memory usage {:.1}MB at {:.0}% of budget",
                        node.performance.memory_used_mb,
                        (node.performance.memory_used_mb / node.config.memory_budget_mb as f64) * 100.0
                    ),
                    impact_score: node.performance.memory_used_mb / node.config.memory_budget_mb as f64,
                };
                actions.push(action);
            }

            // High error rate → throttle
            if node.performance.error_rate_pct > 2.0 {
                let action = TuningAction {
                    id: Uuid::new_v4().to_string(),
                    timestamp: Utc::now(),
                    target_node: node.node_id.clone(),
                    action_type: TuningType::ThrottleAdjust,
                    parameter: "max_throughput_ops".to_string(),
                    old_value: node.config.max_throughput_ops.to_string(),
                    new_value: (node.config.max_throughput_ops / 2).to_string(),
                    reason: format!(
                        "Error rate {:.2}% exceeds 2% threshold — throttling to recover",
                        node.performance.error_rate_pct
                    ),
                    impact_score: node.performance.error_rate_pct / 2.0,
                };
                actions.push(action);
            }
        }
        drop(nodes);

        // Persist tuning actions
        let mut history = self.tuning_history.write().await;
        for action in &actions {
            if history.len() >= self.max_tuning_history {
                history.pop_front();
            }
            history.push_back(action.clone());
        }

        actions
    }

    /// Get tuning history.
    pub async fn get_tuning_history(&self, limit: usize) -> Vec<TuningAction> {
        let history = self.tuning_history.read().await;
        history.iter().rev().take(limit).cloned().collect()
    }

    // ── Sectional Data Management ────────────────────────────────────────────

    /// Record a data flow between sections.
    pub async fn record_data_flow(
        &self,
        source: DataSection,
        target: DataSection,
        payload_size_bytes: u64,
        latency_ms: f64,
        success: bool,
        routed_via: &str,
    ) -> DataFlowRecord {
        let record = DataFlowRecord {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            source_section: source,
            target_section: target,
            payload_size_bytes,
            latency_ms,
            success,
            routed_via: routed_via.to_string(),
        };

        let mut flows = self.data_flows.write().await;
        if flows.len() >= self.max_data_flows {
            flows.pop_front();
        }
        flows.push_back(record.clone());
        record
    }

    /// Update section health metrics.
    pub async fn update_section_health(&self, health: SectionHealth) {
        let mut sections = self.section_health.write().await;
        sections.insert(health.section.clone(), health);
    }

    /// Get health for all sections.
    pub async fn get_all_section_health(&self) -> Vec<SectionHealth> {
        let sections = self.section_health.read().await;
        sections.values().cloned().collect()
    }

    /// Get data flow records for a specific section.
    pub async fn get_section_flows(&self, section: &DataSection, limit: usize) -> Vec<DataFlowRecord> {
        let flows = self.data_flows.read().await;
        flows.iter()
            .rev()
            .filter(|f| &f.source_section == section || &f.target_section == section)
            .take(limit)
            .cloned()
            .collect()
    }

    // ── Mesh Deployment ──────────────────────────────────────────────────────

    /// Get current mesh deployment status.
    pub async fn get_deployment(&self) -> MeshDeployment {
        let deploy = self.deployment.read().await;
        deploy.clone()
    }

    /// Update mesh topology.
    pub async fn set_topology(&self, topology: MeshTopology) {
        let mut deploy = self.deployment.write().await;
        deploy.topology = topology;
        deploy.updated_at = Utc::now();
    }

    /// Compute overall mesh health score based on all nodes.
    pub async fn compute_mesh_health(&self) -> f64 {
        let nodes = self.nodes.read().await;
        if nodes.is_empty() {
            return 100.0;
        }

        let total = nodes.len() as f64;
        let active = nodes.values()
            .filter(|n| matches!(n.status, NodeStatus::Active | NodeStatus::Tuning | NodeStatus::Idle))
            .count() as f64;
        let avg_error_rate = nodes.values()
            .map(|n| n.performance.error_rate_pct)
            .sum::<f64>() / total;
        let overloaded = nodes.values()
            .filter(|n| n.status == NodeStatus::Overloaded)
            .count() as f64;

        let health = ((active / total) * 70.0)
            + ((1.0 - (avg_error_rate / 100.0).min(1.0)) * 20.0)
            + ((1.0 - (overloaded / total).min(1.0)) * 10.0);

        let mut deploy = self.deployment.write().await;
        deploy.health_score = health.max(0.0).min(100.0);
        deploy.health_score
    }

    /// Auto-scale: ensure node count is within bounds based on load.
    pub async fn auto_scale_check(&self) -> Option<String> {
        let deploy = self.deployment.read().await;
        if !deploy.auto_scaling_enabled {
            return None;
        }

        let nodes = self.nodes.read().await;
        let active_count = nodes.values()
            .filter(|n| n.status != NodeStatus::Offline)
            .count() as u32;
        let overloaded_count = nodes.values()
            .filter(|n| n.status == NodeStatus::Overloaded)
            .count() as u32;

        if overloaded_count > active_count / 3 && active_count < deploy.max_nodes {
            Some(format!(
                "SCALE_UP: {}/{} nodes overloaded. Recommend deploying {} additional nodes.",
                overloaded_count, active_count, (overloaded_count + 1).min(deploy.max_nodes - active_count)
            ))
        } else if overloaded_count == 0 && active_count > deploy.min_nodes {
            let idle_count = nodes.values()
                .filter(|n| n.status == NodeStatus::Idle)
                .count() as u32;
            if idle_count > active_count / 2 {
                Some(format!(
                    "SCALE_DOWN: {}/{} nodes idle. Recommend removing {} idle nodes.",
                    idle_count, active_count, (idle_count / 2).min(active_count - deploy.min_nodes)
                ))
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Default for RuntimeMonitor {
    fn default() -> Self {
        Self::new()
    }
}
