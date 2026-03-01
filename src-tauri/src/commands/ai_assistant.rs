// AI Assistant Engine — Tauri commands
// Provides contextual AI chat, code completion, and project advice.

use serde::{Deserialize, Serialize};
use tauri::State;
use crate::AppState;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    System,
    User,
    Assistant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: MessageRole,
    pub content: String,
    pub timestamp_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    pub session_id: String,
    pub messages: Vec<ChatMessage>,
    pub context: Option<String>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    pub session_id: String,
    pub reply: ChatMessage,
    pub tokens_used: u32,
    pub model: String,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionRequest {
    pub language: String,
    pub prefix: String,
    pub suffix: Option<String>,
    pub max_tokens: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionResponse {
    pub completions: Vec<String>,
    pub model: String,
    pub duration_ms: u64,
}

/// Send a chat message to the AI assistant.
#[tauri::command]
pub async fn ai_chat(
    request: ChatRequest,
    _state: State<'_, AppState>,
) -> Result<ChatResponse, String> {
    if request.messages.is_empty() {
        return Err("Message history cannot be empty".into());
    }
    if let Some(t) = request.temperature {
        if !(0.0..=2.0).contains(&t) {
            return Err("Temperature must be between 0.0 and 2.0".into());
        }
    }

    let start = std::time::Instant::now();
    let now_ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64;

    // Stub: In production calls a local (Ollama/llama.cpp) or remote (OpenAI) model.
    Ok(ChatResponse {
        session_id: request.session_id,
        reply: ChatMessage {
            role: MessageRole::Assistant,
            content: "I'm the NATLaB BUILDR26 AI assistant. How can I help?".into(),
            timestamp_ms: now_ms,
        },
        tokens_used: 0,
        model: "natlab-assistant-v1".into(),
        duration_ms: start.elapsed().as_millis() as u64,
    })
}

/// Request an inline code completion.
#[tauri::command]
pub async fn ai_complete(
    request: CompletionRequest,
    _state: State<'_, AppState>,
) -> Result<CompletionResponse, String> {
    if request.prefix.is_empty() {
        return Err("Code prefix cannot be empty".into());
    }

    let start = std::time::Instant::now();

    Ok(CompletionResponse {
        completions: vec![],
        model: "natlab-codegen-v1".into(),
        duration_ms: start.elapsed().as_millis() as u64,
    })
}
