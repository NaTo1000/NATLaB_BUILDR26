// NATLaB BUILDR26 — IDE & Platform Integration Engine
// Integrates external development environments into the admin mesh:
// PyCharm, Juno, JetBrains, VS Studio, Qodana, Google AI Studio, HP Studio.
// Backend-only orchestration — no user UI interference.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::TaskPriority;

// ── Supported Platforms ──────────────────────────────────────────────────────

/// All integrated IDE and development studio platforms.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum IntegrationPlatform {
    PyCharm,
    Juno,
    JetBrainsFleet,
    JetBrainsIntelliJ,
    JetBrainsRider,
    JetBrainsWebStorm,
    JetBrainsClion,
    VsStudio,
    VsCode,
    Qodana,
    GoogleAiStudio,
    HpStudio,
}

impl IntegrationPlatform {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::PyCharm => "JetBrains PyCharm",
            Self::Juno => "Juno IDE",
            Self::JetBrainsFleet => "JetBrains Fleet",
            Self::JetBrainsIntelliJ => "JetBrains IntelliJ IDEA",
            Self::JetBrainsRider => "JetBrains Rider",
            Self::JetBrainsWebStorm => "JetBrains WebStorm",
            Self::JetBrainsClion => "JetBrains CLion",
            Self::VsStudio => "Visual Studio",
            Self::VsCode => "Visual Studio Code",
            Self::Qodana => "JetBrains Qodana",
            Self::GoogleAiStudio => "Google AI Studio",
            Self::HpStudio => "HP Studio",
        }
    }

    pub fn category(&self) -> PlatformCategory {
        match self {
            Self::PyCharm | Self::Juno | Self::JetBrainsFleet
            | Self::JetBrainsIntelliJ | Self::JetBrainsRider
            | Self::JetBrainsWebStorm | Self::JetBrainsClion
            | Self::VsStudio | Self::VsCode => PlatformCategory::Ide,
            Self::Qodana => PlatformCategory::QualityAnalysis,
            Self::GoogleAiStudio => PlatformCategory::AiPlatform,
            Self::HpStudio => PlatformCategory::DevOps,
        }
    }

    /// All supported platforms for enumeration.
    pub fn all() -> Vec<Self> {
        vec![
            Self::PyCharm,
            Self::Juno,
            Self::JetBrainsFleet,
            Self::JetBrainsIntelliJ,
            Self::JetBrainsRider,
            Self::JetBrainsWebStorm,
            Self::JetBrainsClion,
            Self::VsStudio,
            Self::VsCode,
            Self::Qodana,
            Self::GoogleAiStudio,
            Self::HpStudio,
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PlatformCategory {
    Ide,
    QualityAnalysis,
    AiPlatform,
    DevOps,
}

// ── Integration Connection ───────────────────────────────────────────────────

/// Represents a live connection to an integrated platform.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformConnection {
    pub connection_id: String,
    pub platform: IntegrationPlatform,
    pub status: ConnectionStatus,
    pub endpoint: String,
    pub auth_method: AuthMethod,
    pub connected_at: Option<DateTime<Utc>>,
    pub last_sync: Option<DateTime<Utc>>,
    pub last_error: Option<String>,
    pub config: PlatformConfig,
    pub capabilities: Vec<PlatformCapability>,
    pub health_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ConnectionStatus {
    Connected,
    Disconnected,
    Authenticating,
    Syncing,
    Error,
    Degraded,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AuthMethod {
    ApiKey,
    OAuth2,
    ServiceAccount,
    Token,
    Certificate,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformConfig {
    pub auto_sync: bool,
    pub sync_interval_secs: u64,
    pub max_retries: u32,
    pub timeout_ms: u64,
    pub priority: TaskPriority,
    pub enabled_features: Vec<String>,
}

impl Default for PlatformConfig {
    fn default() -> Self {
        Self {
            auto_sync: true,
            sync_interval_secs: 300,
            max_retries: 3,
            timeout_ms: 30000,
            priority: TaskPriority::Green,
            enabled_features: vec![],
        }
    }
}

/// Capabilities that a platform integration provides.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PlatformCapability {
    CodeAnalysis,
    LiveLinting,
    BuildTrigger,
    TestExecution,
    DeploymentPipeline,
    AiCompletion,
    AiCodeReview,
    SecurityScan,
    PerformanceProfiling,
    DependencyAnalysis,
    ProjectSync,
    RemoteDebug,
    ContainerManagement,
    ModelTraining,
    DataPipeline,
}

// ── Sync Events ──────────────────────────────────────────────────────────────

/// A synchronisation event with an integrated platform.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncEvent {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub platform: IntegrationPlatform,
    pub event_type: SyncEventType,
    pub payload_summary: String,
    pub duration_ms: u64,
    pub success: bool,
    pub error: Option<String>,
    pub items_synced: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SyncEventType {
    FullSync,
    IncrementalSync,
    Push,
    Pull,
    Webhook,
    HealthCheck,
    ConfigUpdate,
}

// ── Integration Engine ───────────────────────────────────────────────────────

/// Manages all IDE and platform integrations for the admin mesh.
pub struct IntegrationEngine {
    connections: Arc<RwLock<HashMap<String, PlatformConnection>>>,
    sync_history: Arc<RwLock<Vec<SyncEvent>>>,
    max_sync_history: usize,
}

impl IntegrationEngine {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
            sync_history: Arc::new(RwLock::new(Vec::new())),
            max_sync_history: 1000,
        }
    }

    /// Register a new platform integration.
    pub async fn register_platform(
        &self,
        platform: IntegrationPlatform,
        endpoint: &str,
        auth_method: AuthMethod,
        config: PlatformConfig,
    ) -> PlatformConnection {
        let capabilities = Self::default_capabilities(&platform);

        let connection = PlatformConnection {
            connection_id: Uuid::new_v4().to_string(),
            platform,
            status: ConnectionStatus::Disconnected,
            endpoint: endpoint.to_string(),
            auth_method,
            connected_at: None,
            last_sync: None,
            last_error: None,
            config,
            capabilities,
            health_score: 100.0,
        };

        let mut conns = self.connections.write().await;
        conns.insert(connection.connection_id.clone(), connection.clone());
        connection
    }

    /// Connect to a registered platform.
    pub async fn connect(&self, connection_id: &str) -> Result<PlatformConnection, String> {
        let mut conns = self.connections.write().await;
        let conn = conns.get_mut(connection_id)
            .ok_or_else(|| format!("Connection '{}' not found", connection_id))?;

        conn.status = ConnectionStatus::Authenticating;
        // In production, this would perform actual auth handshake
        conn.status = ConnectionStatus::Connected;
        conn.connected_at = Some(Utc::now());
        conn.last_error = None;

        Ok(conn.clone())
    }

    /// Disconnect from a platform.
    pub async fn disconnect(&self, connection_id: &str) -> Result<PlatformConnection, String> {
        let mut conns = self.connections.write().await;
        let conn = conns.get_mut(connection_id)
            .ok_or_else(|| format!("Connection '{}' not found", connection_id))?;

        conn.status = ConnectionStatus::Disconnected;
        Ok(conn.clone())
    }

    /// Trigger a sync with a specific platform.
    pub async fn sync_platform(
        &self,
        connection_id: &str,
        event_type: SyncEventType,
    ) -> Result<SyncEvent, String> {
        let mut conns = self.connections.write().await;
        let conn = conns.get_mut(connection_id)
            .ok_or_else(|| format!("Connection '{}' not found", connection_id))?;

        if conn.status != ConnectionStatus::Connected {
            return Err(format!(
                "Platform '{}' is not connected (status: {:?})",
                conn.platform.display_name(), conn.status
            ));
        }

        conn.status = ConnectionStatus::Syncing;
        let start = std::time::Instant::now();

        // Simulate sync operation (in production, calls platform API)
        let sync_event = SyncEvent {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            platform: conn.platform.clone(),
            event_type,
            payload_summary: format!("Sync with {}", conn.platform.display_name()),
            duration_ms: start.elapsed().as_millis() as u64,
            success: true,
            error: None,
            items_synced: 0,
        };

        conn.status = ConnectionStatus::Connected;
        conn.last_sync = Some(Utc::now());
        drop(conns);

        let mut history = self.sync_history.write().await;
        if history.len() >= self.max_sync_history {
            history.remove(0);
        }
        history.push(sync_event.clone());

        Ok(sync_event)
    }

    /// List all registered platform connections.
    pub async fn list_connections(&self) -> Vec<PlatformConnection> {
        let conns = self.connections.read().await;
        conns.values().cloned().collect()
    }

    /// Get connections by platform category.
    pub async fn get_by_category(&self, category: PlatformCategory) -> Vec<PlatformConnection> {
        let conns = self.connections.read().await;
        conns.values()
            .filter(|c| c.platform.category() == category)
            .cloned()
            .collect()
    }

    /// Get sync history for a specific platform.
    pub async fn get_sync_history(
        &self,
        platform: Option<IntegrationPlatform>,
        limit: usize,
    ) -> Vec<SyncEvent> {
        let history = self.sync_history.read().await;
        match platform {
            Some(p) => history.iter()
                .rev()
                .filter(|e| e.platform == p)
                .take(limit)
                .cloned()
                .collect(),
            None => history.iter()
                .rev()
                .take(limit)
                .cloned()
                .collect(),
        }
    }

    /// Update connection health based on recent sync results.
    pub async fn update_health(&self, connection_id: &str) -> Option<f64> {
        let history = self.sync_history.read().await;
        let mut conns = self.connections.write().await;

        let conn = conns.get_mut(connection_id)?;
        let recent: Vec<_> = history.iter()
            .rev()
            .filter(|e| e.platform == conn.platform)
            .take(10)
            .collect();

        if recent.is_empty() {
            conn.health_score = 100.0;
            return Some(100.0);
        }

        let success_count = recent.iter().filter(|e| e.success).count();
        let health = (success_count as f64 / recent.len() as f64) * 100.0;
        conn.health_score = health;
        Some(health)
    }

    /// Remove a platform integration.
    pub async fn remove_platform(&self, connection_id: &str) -> bool {
        let mut conns = self.connections.write().await;
        conns.remove(connection_id).is_some()
    }

    /// Get default capabilities for each platform.
    fn default_capabilities(platform: &IntegrationPlatform) -> Vec<PlatformCapability> {
        match platform {
            IntegrationPlatform::PyCharm => vec![
                PlatformCapability::CodeAnalysis,
                PlatformCapability::LiveLinting,
                PlatformCapability::TestExecution,
                PlatformCapability::RemoteDebug,
                PlatformCapability::DependencyAnalysis,
            ],
            IntegrationPlatform::Juno => vec![
                PlatformCapability::CodeAnalysis,
                PlatformCapability::DataPipeline,
                PlatformCapability::ModelTraining,
                PlatformCapability::PerformanceProfiling,
            ],
            IntegrationPlatform::JetBrainsFleet => vec![
                PlatformCapability::CodeAnalysis,
                PlatformCapability::LiveLinting,
                PlatformCapability::ProjectSync,
                PlatformCapability::RemoteDebug,
            ],
            IntegrationPlatform::JetBrainsIntelliJ => vec![
                PlatformCapability::CodeAnalysis,
                PlatformCapability::LiveLinting,
                PlatformCapability::BuildTrigger,
                PlatformCapability::TestExecution,
                PlatformCapability::DependencyAnalysis,
                PlatformCapability::RemoteDebug,
            ],
            IntegrationPlatform::JetBrainsRider => vec![
                PlatformCapability::CodeAnalysis,
                PlatformCapability::LiveLinting,
                PlatformCapability::BuildTrigger,
                PlatformCapability::TestExecution,
                PlatformCapability::RemoteDebug,
            ],
            IntegrationPlatform::JetBrainsWebStorm => vec![
                PlatformCapability::CodeAnalysis,
                PlatformCapability::LiveLinting,
                PlatformCapability::BuildTrigger,
                PlatformCapability::TestExecution,
                PlatformCapability::DependencyAnalysis,
            ],
            IntegrationPlatform::JetBrainsClion => vec![
                PlatformCapability::CodeAnalysis,
                PlatformCapability::LiveLinting,
                PlatformCapability::BuildTrigger,
                PlatformCapability::PerformanceProfiling,
                PlatformCapability::RemoteDebug,
            ],
            IntegrationPlatform::VsStudio => vec![
                PlatformCapability::CodeAnalysis,
                PlatformCapability::LiveLinting,
                PlatformCapability::BuildTrigger,
                PlatformCapability::TestExecution,
                PlatformCapability::DeploymentPipeline,
                PlatformCapability::RemoteDebug,
                PlatformCapability::ContainerManagement,
            ],
            IntegrationPlatform::VsCode => vec![
                PlatformCapability::CodeAnalysis,
                PlatformCapability::LiveLinting,
                PlatformCapability::TestExecution,
                PlatformCapability::AiCompletion,
                PlatformCapability::RemoteDebug,
            ],
            IntegrationPlatform::Qodana => vec![
                PlatformCapability::CodeAnalysis,
                PlatformCapability::SecurityScan,
                PlatformCapability::AiCodeReview,
                PlatformCapability::DependencyAnalysis,
            ],
            IntegrationPlatform::GoogleAiStudio => vec![
                PlatformCapability::AiCompletion,
                PlatformCapability::AiCodeReview,
                PlatformCapability::ModelTraining,
                PlatformCapability::DataPipeline,
            ],
            IntegrationPlatform::HpStudio => vec![
                PlatformCapability::DeploymentPipeline,
                PlatformCapability::ContainerManagement,
                PlatformCapability::PerformanceProfiling,
                PlatformCapability::SecurityScan,
            ],
        }
    }
}

impl Default for IntegrationEngine {
    fn default() -> Self {
        Self::new()
    }
}
