// Build Overseer — Tauri commands
// Exposes overseer evaluation, history, and threshold management to the frontend.

use serde::{Deserialize, Serialize};
use tauri::State;
use crate::AppState;
use crate::overseer::{
    BuildRecord, BuildMetrics, BuildGrade, QualityThresholds, BuildStage,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluateRequest {
    pub build_id: String,
    pub project: String,
    pub metrics: BuildMetrics,
}

/// Evaluate a build against the current quality thresholds.
#[tauri::command]
pub async fn overseer_evaluate(
    request: EvaluateRequest,
    state: State<'_, AppState>,
) -> Result<BuildRecord, String> {
    let record = BuildRecord {
        build_id: request.build_id,
        project: request.project,
        started_at: chrono::Utc::now(),
        finished_at: None,
        stage: BuildStage::QualityGate,
        metrics: request.metrics,
        violations: vec![],
        grade: None,
        perf_samples: vec![],
    };

    state
        .overseer
        .evaluate(record)
        .await
        .map_err(|e| e.to_string())
}

/// Get build history.
#[tauri::command]
pub async fn overseer_history(
    state: State<'_, AppState>,
) -> Result<Vec<BuildRecord>, String> {
    Ok(state.overseer.history().await)
}

/// Get current quality thresholds.
#[tauri::command]
pub async fn overseer_thresholds(
    state: State<'_, AppState>,
) -> Result<QualityThresholds, String> {
    Ok(state.overseer.thresholds().clone())
}

/// Get rolling quality score.
#[tauri::command]
pub async fn overseer_rolling_score(
    window: usize,
    state: State<'_, AppState>,
) -> Result<f64, String> {
    if window == 0 {
        return Err("Window must be greater than 0".into());
    }
    Ok(state.overseer.rolling_score(window).await)
}
