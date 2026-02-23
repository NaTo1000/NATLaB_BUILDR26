// NATLaB BUILDR26 — Build Overseer
// The Overseer monitors every build pipeline stage, scores quality metrics,
// enforces performance thresholds, and surfaces actionable diagnostics.

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use anyhow::Result;

// ── Quality Thresholds ────────────────────────────────────────────────────────

/// Hard limits that a build must pass to be marked "premium quality".
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityThresholds {
    /// Maximum acceptable binary size in bytes (default 50 MB)
    pub max_binary_size_bytes: u64,
    /// Maximum cold-start time in milliseconds (default 800 ms)
    pub max_startup_ms: u64,
    /// Minimum test coverage percentage (default 70 %)
    pub min_coverage_pct: f64,
    /// Maximum number of high/critical security advisories (default 0)
    pub max_security_advisories: u32,
    /// Maximum compile time in seconds (default 120 s)
    pub max_compile_secs: u64,
    /// Maximum memory footprint at idle in MB (default 150 MB)
    pub max_idle_memory_mb: u64,
}

impl Default for QualityThresholds {
    fn default() -> Self {
        Self {
            max_binary_size_bytes: 50 * 1024 * 1024,
            max_startup_ms: 800,
            min_coverage_pct: 70.0,
            max_security_advisories: 0,
            max_compile_secs: 120,
            max_idle_memory_mb: 150,
        }
    }
}

// ── Performance Sample ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerfSample {
    pub timestamp: DateTime<Utc>,
    pub cpu_percent: f64,
    pub memory_mb: f64,
    pub thread_count: u32,
    pub io_read_kb: u64,
    pub io_write_kb: u64,
}

// ── Build Record ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BuildStage {
    Init,
    Lint,
    Test,
    Compile,
    Bundle,
    SecurityScan,
    QualityGate,
    Package,
    Complete,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildRecord {
    pub build_id: String,
    pub project: String,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
    pub stage: BuildStage,
    pub metrics: BuildMetrics,
    pub violations: Vec<QualityViolation>,
    pub grade: Option<BuildGrade>,
    pub perf_samples: Vec<PerfSample>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BuildMetrics {
    pub binary_size_bytes: u64,
    pub startup_ms: u64,
    pub coverage_pct: f64,
    pub security_advisories: u32,
    pub compile_secs: u64,
    pub idle_memory_mb: u64,
    pub lint_warnings: u32,
    pub test_count: u32,
    pub test_failures: u32,
    pub bundle_chunks: u32,
    pub total_dependencies: u32,
    pub outdated_dependencies: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityViolation {
    pub code: String,
    pub severity: ViolationSeverity,
    pub message: String,
    pub suggestion: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum ViolationSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// A-F grade assigned to each build by the Overseer.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum BuildGrade {
    /// All thresholds met, zero critical violations
    A,
    /// One or two warnings, no errors
    B,
    /// Minor threshold breaches, no critical violations
    C,
    /// Multiple threshold breaches or at least one error
    D,
    /// Critical violations — build should not be released
    F,
}

impl BuildGrade {
    pub fn label(&self) -> &'static str {
        match self {
            Self::A => "Premium",
            Self::B => "Good",
            Self::C => "Acceptable",
            Self::D => "Needs Work",
            Self::F => "Failing",
        }
    }
}

// ── Overseer ──────────────────────────────────────────────────────────────────

/// The Build Overseer: the authoritative quality gate for every build pipeline.
pub struct BuildOverseer {
    thresholds: QualityThresholds,
    history: Arc<RwLock<VecDeque<BuildRecord>>>,
    /// Maximum history entries retained in memory
    max_history: usize,
}

impl BuildOverseer {
    pub fn new(thresholds: QualityThresholds) -> Self {
        Self {
            thresholds,
            history: Arc::new(RwLock::new(VecDeque::new())),
            max_history: 200,
        }
    }

    /// Evaluate a completed build record and assign a grade.
    pub async fn evaluate(&self, mut record: BuildRecord) -> Result<BuildRecord> {
        let mut violations: Vec<QualityViolation> = Vec::new();
        let m = &record.metrics;

        // ── Binary Size ──────────────────────────────────────────────────────
        if m.binary_size_bytes > self.thresholds.max_binary_size_bytes {
            let mb = m.binary_size_bytes / (1024 * 1024);
            let limit_mb = self.thresholds.max_binary_size_bytes / (1024 * 1024);
            violations.push(QualityViolation {
                code: "OVS-001".into(),
                severity: ViolationSeverity::Warning,
                message: format!("Binary size {}MB exceeds limit {}MB", mb, limit_mb),
                suggestion: "Enable LTO + strip symbols in release profile".into(),
            });
        }

        // ── Startup Time ─────────────────────────────────────────────────────
        if m.startup_ms > self.thresholds.max_startup_ms {
            violations.push(QualityViolation {
                code: "OVS-002".into(),
                severity: ViolationSeverity::Error,
                message: format!(
                    "Startup time {}ms exceeds threshold {}ms",
                    m.startup_ms, self.thresholds.max_startup_ms
                ),
                suggestion: "Increase lazy-loading; defer non-critical modules".into(),
            });
        }

        // ── Test Coverage ────────────────────────────────────────────────────
        if m.coverage_pct < self.thresholds.min_coverage_pct {
            violations.push(QualityViolation {
                code: "OVS-003".into(),
                severity: ViolationSeverity::Error,
                message: format!(
                    "Coverage {:.1}% is below minimum {:.1}%",
                    m.coverage_pct, self.thresholds.min_coverage_pct
                ),
                suggestion: "Add unit tests for uncovered branches and public APIs".into(),
            });
        }

        // ── Security Advisories ──────────────────────────────────────────────
        if m.security_advisories > self.thresholds.max_security_advisories {
            violations.push(QualityViolation {
                code: "OVS-004".into(),
                severity: ViolationSeverity::Critical,
                message: format!(
                    "{} security advisories found (limit {})",
                    m.security_advisories, self.thresholds.max_security_advisories
                ),
                suggestion: "Run `cargo audit fix` / `npm audit fix` and update affected crates".into(),
            });
        }

        // ── Compile Time ─────────────────────────────────────────────────────
        if m.compile_secs > self.thresholds.max_compile_secs {
            violations.push(QualityViolation {
                code: "OVS-005".into(),
                severity: ViolationSeverity::Warning,
                message: format!(
                    "Compile time {}s exceeds threshold {}s",
                    m.compile_secs, self.thresholds.max_compile_secs
                ),
                suggestion: "Enable incremental compilation; split crates; use sccache".into(),
            });
        }

        // ── Idle Memory ──────────────────────────────────────────────────────
        if m.idle_memory_mb > self.thresholds.max_idle_memory_mb {
            violations.push(QualityViolation {
                code: "OVS-006".into(),
                severity: ViolationSeverity::Warning,
                message: format!(
                    "Idle memory {}MB exceeds threshold {}MB",
                    m.idle_memory_mb, self.thresholds.max_idle_memory_mb
                ),
                suggestion: "Profile heap usage; reduce static allocations; shrink lazy bundles".into(),
            });
        }

        // ── Test Failures ────────────────────────────────────────────────────
        if m.test_failures > 0 {
            violations.push(QualityViolation {
                code: "OVS-007".into(),
                severity: ViolationSeverity::Critical,
                message: format!("{} test(s) failed", m.test_failures),
                suggestion: "All tests must pass before a build can be graded above F".into(),
            });
        }

        // ── Outdated Dependencies ────────────────────────────────────────────
        if m.outdated_dependencies > 5 {
            violations.push(QualityViolation {
                code: "OVS-008".into(),
                severity: ViolationSeverity::Info,
                message: format!("{} outdated dependencies detected", m.outdated_dependencies),
                suggestion: "Run `npm outdated` / `cargo outdated` and update safe upgrades".into(),
            });
        }

        // ── Grade Assignment ─────────────────────────────────────────────────
        let grade = self.assign_grade(&violations);

        record.violations = violations;
        record.grade = Some(grade);
        record.stage = if record.metrics.test_failures == 0
            && record.grade.as_ref() != Some(&BuildGrade::F)
        {
            BuildStage::Complete
        } else {
            BuildStage::Failed
        };
        record.finished_at = Some(Utc::now());

        // Persist to history
        let mut hist = self.history.write().await;
        if hist.len() >= self.max_history {
            hist.pop_front();
        }
        hist.push_back(record.clone());

        Ok(record)
    }

    fn assign_grade(&self, violations: &[QualityViolation]) -> BuildGrade {
        let critical = violations.iter().filter(|v| v.severity == ViolationSeverity::Critical).count();
        let errors = violations.iter().filter(|v| v.severity == ViolationSeverity::Error).count();
        let warnings = violations.iter().filter(|v| v.severity == ViolationSeverity::Warning).count();

        if critical > 0 {
            BuildGrade::F
        } else if errors > 0 {
            BuildGrade::D
        } else if warnings > 2 {
            BuildGrade::C
        } else if warnings > 0 {
            BuildGrade::B
        } else {
            BuildGrade::A
        }
    }

    /// Return a copy of the build history (most-recent first).
    pub async fn history(&self) -> Vec<BuildRecord> {
        let hist = self.history.read().await;
        hist.iter().rev().cloned().collect()
    }

    /// Return the current quality thresholds.
    pub fn thresholds(&self) -> &QualityThresholds {
        &self.thresholds
    }

    /// Compute a rolling quality score (0–100) over the last N builds.
    pub async fn rolling_score(&self, window: usize) -> f64 {
        let hist = self.history.read().await;
        let recent: Vec<_> = hist.iter().rev().take(window).collect();
        if recent.is_empty() {
            return 100.0;
        }
        let passing = recent
            .iter()
            .filter(|r| matches!(r.grade, Some(BuildGrade::A) | Some(BuildGrade::B)))
            .count();
        (passing as f64 / recent.len() as f64) * 100.0
    }
}

impl Default for BuildOverseer {
    fn default() -> Self {
        Self::new(QualityThresholds::default())
    }
}
