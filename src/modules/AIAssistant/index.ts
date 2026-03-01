// AI Assistant Engine — Frontend Module
import { invoke } from "@tauri-apps/api/core";
import type {
  ChatRequest,
  ChatResponse,
  CompletionRequest,
  CompletionResponse,
  ChatMessage,
} from "../../types/modules";

/** Build a user message with the current timestamp. */
export function userMessage(content: string): ChatMessage {
  return { role: "user", content, timestamp_ms: Date.now() };
}

export const AIAssistant = {
  /** Send a chat turn to the AI assistant. */
  chat: (request: ChatRequest) =>
    invoke<ChatResponse>("ai_chat", { request }),

  /** Request inline code completions. */
  complete: (request: CompletionRequest) =>
    invoke<CompletionResponse>("ai_complete", { request }),

  /** Convenience: start a new chat session with a single user message. */
  ask: (question: string, context?: string) =>
    invoke<ChatResponse>("ai_chat", {
      request: {
        session_id: crypto.randomUUID(),
        messages: [userMessage(question)],
        context: context ?? null,
        max_tokens: null,
        temperature: null,
      } satisfies ChatRequest,
    }),
};
