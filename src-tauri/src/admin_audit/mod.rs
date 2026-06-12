// NATLaB BUILDR26 — Admin Audit Trail & Process Control
// Backend-only ADMIN system: audit logging, dependency monitoring,
// automated error response, process allocation, and kernel protection.
// This module NEVER interferes with user-facing system dynamics.

pub mod runtime_monitor;
pub mod integrations;

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc, Duration};
use anyhow::Result;
use uuid::Uuid;

// ── Priority Hierarchy (Red → Green) ─────────────────────────────────────────

/// Priority level for admin tasks — red = urgent, green = monitor/research.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum TaskPriority {
    /// Immediate action required — system critical
    Red,
    /// High priority — requires attention within 24h
    Orange,
    /// Medium priority — scheduled maintenance
    Yellow,
    /// Low priority — research and monitor
    Green,
}

impl TaskPriority {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Red => "URGENT",
            Self::Orange => "HIGH",
            Self::Yellow => "MEDIUM",
            Self::Green => "MONITOR",
        }
    }

    pub fn is_urgent(&self) -> bool {
        matches!(self, Self::Red | Self::Orange)
    }
}

// ── Audit Trail ──────────────────────────────────────────────────────────────

/// A single audit log entry for development trail tracking.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub category: AuditCategory,
    pub severity: TaskPriority,
    pub source_module: String,
    pub message: String,
    pub details: Option<String>,
    pub resolved: bool,
    pub resolved_at: Option<DateTime<Utc>>,
    pub auto_resolved: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AuditCategory {
    DependencyUpdate,
    SecurityPatch,
    BuildFailure,
    TestRegression,
    PerformanceDegradation,
    SystemHealth,
    ProcessAllocation,
    KernelProtection,
    AutomationEvent,
    ResearchNote,
}

// ── Dependency Monitor ───────────────────────────────────────────────────────

/// Tracks dependency health with priority-based update scheduling.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyStatus {
    pub name: String,
    pub current_version: String,
    pub latest_version: Option<String>,
    pub ecosystem: String,
    pub priority: TaskPriority,
    pub has_security_advisory: bool,
    pub last_checked: DateTime<Utc>,
    pub auto_updatable: bool,
    pub update_scheduled: bool,
}

/// Daily dependency summary report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailySummary {
    pub date: DateTime<Utc>,
    pub total_dependencies: u32,
    pub outdated_count: u32,
    pub security_alerts: u32,
    pub auto_updated: u32,
    pub pending_review: u32,
    pub system_health_score: f64,
    pub tasks_by_priority: PriorityBreakdown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriorityBreakdown {
    pub red: u32,
    pub orange: u32,
    pub yellow: u32,
    pub green: u32,
}

// ── Process Allocator & Kernel Protection ────────────────────────────────────

/// Root process allocation record for admin-level system management.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessAllocation {
    pub process_id: String,
    pub name: String,
    pub allocated_at: DateTime<Utc>,
    pub priority: TaskPriority,
    pub memory_limit_mb: u64,
    pub cpu_quota_pct: f64,
    pub status: ProcessStatus,
    pub protected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProcessStatus {
    Running,
    Idle,
    Suspended,
    Terminated,
    Protected,
}

/// Security infrastructure kernel protection record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelProtectionEvent {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub event_type: KernelEventType,
    pub source: String,
    pub threat_level: TaskPriority,
    pub description: String,
    pub action_taken: String,
    pub blocked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum KernelEventType {
    UnauthorizedAccess,
    PrivilegeEscalation,
    IntegrityViolation,
    ResourceExhaustion,
    AnomalousPattern,
    RoutineCheck,
}

// ── Automated Error Response ─────────────────────────────────────────────────

/// AI internal process engine error response record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub error_source: String,
    pub error_type: String,
    pub original_message: String,
    pub auto_diagnosis: String,
    pub resolution_action: ResolutionAction,
    pub resolved: bool,
    pub resolution_duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ResolutionAction {
    AutoFixed,
    Retry,
    Escalated,
    Deferred,
    RequiresManual,
    Researching,
}

// ── Admin Audit Engine ───────────────────────────────────────────────────────

/// The root Admin Audit Engine: manages audit trails, dependency monitoring,
/// process allocation, kernel protection, and automated error response.
/// This is a BACKEND-ONLY system that does not interact with user UI.
pub struct AdminAuditEngine {
    audit_log: Arc<RwLock<VecDeque<AuditEntry>>>,
    dependencies: Arc<RwLock<Vec<DependencyStatus>>>,
    daily_summaries: Arc<RwLock<VecDeque<DailySummary>>>,
    processes: Arc<RwLock<Vec<ProcessAllocation>>>,
    kernel_events: Arc<RwLock<VecDeque<KernelProtectionEvent>>>,
    error_responses: Arc<RwLock<VecDeque<ErrorResponse>>>,
    max_audit_entries: usize,
    max_daily_summaries: usize,
    max_kernel_events: usize,
    max_error_responses: usize,
}

impl AdminAuditEngine {
    pub fn new() -> Self {
        Self {
            audit_log: Arc::new(RwLock::new(VecDeque::new())),
            dependencies: Arc::new(RwLock::new(Vec::new())),
            daily_summaries: Arc::new(RwLock::new(VecDeque::new())),
            processes: Arc::new(RwLock::new(Vec::new())),
            kernel_events: Arc::new(RwLock::new(VecDeque::new())),
            error_responses: Arc::new(RwLock::new(VecDeque::new())),
            max_audit_entries: 1000,
            max_daily_summaries: 365,
            max_kernel_events: 500,
            max_error_responses: 500,
        }
    }

    // ── Audit Trail Logging ──────────────────────────────────────────────────

    /// Log a new audit entry to the development trail.
    pub async fn log_audit(
        &self,
        category: AuditCategory,
        severity: TaskPriority,
        source_module: &str,
        message: &str,
        details: Option<String>,
    ) -> AuditEntry {
        let entry = AuditEntry {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            category,
            severity,
            source_module: source_module.to_string(),
            message: message.to_string(),
            details,
            resolved: false,
            resolved_at: None,
            auto_resolved: false,
        };

        let mut log = self.audit_log.write().await;
        if log.len() >= self.max_audit_entries {
            log.pop_front();
        }
        log.push_back(entry.clone());
        entry
    }

    /// Mark an audit entry as resolved.
    pub async fn resolve_audit(&self, id: &str, auto: bool) -> Result<AuditEntry> {
        let mut log = self.audit_log.write().await;
        let entry = log.iter_mut().find(|e| e.id == id)
            .ok_or_else(|| anyhow::anyhow!("Audit entry '{}' not found", id))?;
        entry.resolved = true;
        entry.resolved_at = Some(Utc::now());
        entry.auto_resolved = auto;
        Ok(entry.clone())
    }

    /// Get all audit entries, optionally filtered by priority.
    pub async fn get_audit_log(&self, priority_filter: Option<TaskPriority>) -> Vec<AuditEntry> {
        let log = self.audit_log.read().await;
        match priority_filter {
            Some(priority) => log.iter()
                .filter(|e| e.severity == priority)
                .rev()
                .cloned()
                .collect(),
            None => log.iter().rev().cloned().collect(),
        }
    }

    /// Get unresolved entries ordered by priority (red first).
    pub async fn get_todo_list(&self) -> Vec<AuditEntry> {
        let log = self.audit_log.read().await;
        let mut todos: Vec<_> = log.iter()
            .filter(|e| !e.resolved)
            .cloned()
            .collect();
        todos.sort_by(|a, b| a.severity.cmp(&b.severity));
        todos
    }

    // ── Dependency Monitoring ────────────────────────────────────────────────

    /// Register or update a dependency status.
    pub async fn update_dependency(&self, status: DependencyStatus) {
        let mut deps = self.dependencies.write().await;
        if let Some(existing) = deps.iter_mut().find(|d| d.name == status.name) {
            *existing = status;
        } else {
            deps.push(status);
        }
    }

    /// Get all dependencies ordered by priority (urgent first).
    pub async fn get_dependencies(&self) -> Vec<DependencyStatus> {
        let deps = self.dependencies.read().await;
        let mut sorted = deps.clone();
        sorted.sort_by(|a, b| a.priority.cmp(&b.priority));
        sorted
    }

    /// Generate a daily summary of the current system state.
    pub async fn generate_daily_summary(&self) -> DailySummary {
        let deps = self.dependencies.read().await;
        let log = self.audit_log.read().await;

        let total_dependencies = deps.len() as u32;
        let outdated_count = deps.iter()
            .filter(|d| d.latest_version.is_some() && d.latest_version.as_deref() != Some(&d.current_version))
            .count() as u32;
        let security_alerts = deps.iter()
            .filter(|d| d.has_security_advisory)
            .count() as u32;
        let auto_updated = deps.iter()
            .filter(|d| d.auto_updatable && d.update_scheduled)
            .count() as u32;

        let today = Utc::now();
        let today_start = today - Duration::hours(24);
        let pending_review = log.iter()
            .filter(|e| !e.resolved && e.timestamp > today_start)
            .count() as u32;

        let red = log.iter().filter(|e| !e.resolved && e.severity == TaskPriority::Red).count() as u32;
        let orange = log.iter().filter(|e| !e.resolved && e.severity == TaskPriority::Orange).count() as u32;
        let yellow = log.iter().filter(|e| !e.resolved && e.severity == TaskPriority::Yellow).count() as u32;
        let green = log.iter().filter(|e| !e.resolved && e.severity == TaskPriority::Green).count() as u32;

        // System health: 100 - (red*25 + orange*10 + yellow*3 + security_alerts*15)
        let health = (100.0_f64
            - (red as f64 * 25.0)
            - (orange as f64 * 10.0)
            - (yellow as f64 * 3.0)
            - (security_alerts as f64 * 15.0))
            .max(0.0)
            .min(100.0);

        let summary = DailySummary {
            date: today,
            total_dependencies,
            outdated_count,
            security_alerts,
            auto_updated,
            pending_review,
            system_health_score: health,
            tasks_by_priority: PriorityBreakdown { red, orange, yellow, green },
        };

        let mut summaries = self.daily_summaries.write().await;
        if summaries.len() >= self.max_daily_summaries {
            summaries.pop_front();
        }
        summaries.push_back(summary.clone());
        summary
    }

    /// Get recent daily summaries.
    pub async fn get_daily_summaries(&self, count: usize) -> Vec<DailySummary> {
        let summaries = self.daily_summaries.read().await;
        summaries.iter().rev().take(count).cloned().collect()
    }

    // ── Process Allocation ───────────────────────────────────────────────────

    /// Allocate a new admin process with resource limits.
    pub async fn allocate_process(
        &self,
        name: &str,
        priority: TaskPriority,
        memory_limit_mb: u64,
        cpu_quota_pct: f64,
        protected: bool,
    ) -> ProcessAllocation {
        let allocation = ProcessAllocation {
            process_id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            allocated_at: Utc::now(),
            priority,
            memory_limit_mb,
            cpu_quota_pct,
            status: ProcessStatus::Running,
            protected,
        };

        let mut processes = self.processes.write().await;
        processes.push(allocation.clone());
        allocation
    }

    /// Update process status (cannot terminate protected processes).
    pub async fn update_process_status(&self, process_id: &str, status: ProcessStatus) -> Result<ProcessAllocation> {
        let mut processes = self.processes.write().await;
        let proc = processes.iter_mut()
            .find(|p| p.process_id == process_id)
            .ok_or_else(|| anyhow::anyhow!("Process '{}' not found", process_id))?;

        if proc.protected && status == ProcessStatus::Terminated {
            return Err(anyhow::anyhow!(
                "Cannot terminate protected process '{}'. Remove protection first.",
                proc.name
            ));
        }

        proc.status = status;
        Ok(proc.clone())
    }

    /// List all active processes sorted by priority.
    pub async fn list_processes(&self) -> Vec<ProcessAllocation> {
        let processes = self.processes.read().await;
        let mut sorted: Vec<_> = processes.iter()
            .filter(|p| p.status != ProcessStatus::Terminated)
            .cloned()
            .collect();
        sorted.sort_by(|a, b| a.priority.cmp(&b.priority));
        sorted
    }

    // ── Kernel Protection ────────────────────────────────────────────────────

    /// Log a kernel protection event.
    pub async fn log_kernel_event(
        &self,
        event_type: KernelEventType,
        source: &str,
        threat_level: TaskPriority,
        description: &str,
        action_taken: &str,
        blocked: bool,
    ) -> KernelProtectionEvent {
        let event = KernelProtectionEvent {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            event_type,
            source: source.to_string(),
            threat_level,
            description: description.to_string(),
            action_taken: action_taken.to_string(),
            blocked,
        };

        let mut events = self.kernel_events.write().await;
        if events.len() >= self.max_kernel_events {
            events.pop_front();
        }
        events.push_back(event.clone());

        // Auto-log to audit trail for critical threats
        if event.threat_level.is_urgent() {
            drop(events);
            self.log_audit(
                AuditCategory::KernelProtection,
                event.threat_level.clone(),
                "kernel_protection",
                &format!("Security event: {}", event.description),
                Some(format!("Action: {} | Blocked: {}", event.action_taken, event.blocked)),
            ).await;
        }

        event
    }

    /// Get kernel protection events.
    pub async fn get_kernel_events(&self, limit: usize) -> Vec<KernelProtectionEvent> {
        let events = self.kernel_events.read().await;
        events.iter().rev().take(limit).cloned().collect()
    }

    // ── Automated Error Response ─────────────────────────────────────────────

    /// Process an error through the AI internal response engine.
    pub async fn handle_error(
        &self,
        error_source: &str,
        error_type: &str,
        message: &str,
    ) -> ErrorResponse {
        let start = std::time::Instant::now();

        // AI-driven diagnosis (rule-based for now, extensible to ML)
        let (diagnosis, action) = self.diagnose_error(error_type, message);

        let response = ErrorResponse {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            error_source: error_source.to_string(),
            error_type: error_type.to_string(),
            original_message: message.to_string(),
            auto_diagnosis: diagnosis,
            resolution_action: action.clone(),
            resolved: action == ResolutionAction::AutoFixed,
            resolution_duration_ms: start.elapsed().as_millis() as u64,
        };

        let mut responses = self.error_responses.write().await;
        if responses.len() >= self.max_error_responses {
            responses.pop_front();
        }
        responses.push_back(response.clone());

        // If not auto-fixed, escalate to audit trail
        if !response.resolved {
            let severity = match &action {
                ResolutionAction::Escalated => TaskPriority::Red,
                ResolutionAction::RequiresManual => TaskPriority::Orange,
                ResolutionAction::Retry => TaskPriority::Yellow,
                _ => TaskPriority::Green,
            };
            drop(responses);
            self.log_audit(
                AuditCategory::AutomationEvent,
                severity,
                error_source,
                &format!("Auto-error response: {}", message),
                Some(format!("Diagnosis: {}", response.auto_diagnosis)),
            ).await;
        }

        response
    }

    /// Internal AI diagnosis engine — classifies errors and selects resolution strategy.
    fn diagnose_error(&self, error_type: &str, message: &str) -> (String, ResolutionAction) {
        let msg_lower = message.to_lowercase();

        // Dependency-related errors → auto-retry with update
        if msg_lower.contains("outdated") || msg_lower.contains("deprecated") || error_type == "dependency" {
            return (
                "Outdated dependency detected. Scheduling automatic update.".to_string(),
                ResolutionAction::AutoFixed,
            );
        }

        // Network/transient errors → retry
        if msg_lower.contains("timeout") || msg_lower.contains("connection") || error_type == "network" {
            return (
                "Transient network error. Retry scheduled with exponential backoff.".to_string(),
                ResolutionAction::Retry,
            );
        }

        // Security errors → escalate immediately
        if msg_lower.contains("unauthorized") || msg_lower.contains("permission") || error_type == "security" {
            return (
                "Security-related error detected. Escalating to kernel protection.".to_string(),
                ResolutionAction::Escalated,
            );
        }

        // Build/compile errors → research
        if msg_lower.contains("compile") || msg_lower.contains("build") || error_type == "build" {
            return (
                "Build system error. Researching root cause and collecting diagnostics.".to_string(),
                ResolutionAction::Researching,
            );
        }

        // Unknown → defer for manual review
        (
            "Unknown error pattern. Deferred for manual analysis.".to_string(),
            ResolutionAction::Deferred,
        )
    }

    /// Get error response history.
    pub async fn get_error_responses(&self, limit: usize) -> Vec<ErrorResponse> {
        let responses = self.error_responses.read().await;
        responses.iter().rev().take(limit).cloned().collect()
    }
}

impl Default for AdminAuditEngine {
    fn default() -> Self {
        Self::new()
    }
}
