// Core Engine — Polyglot Compiler (Frontend Bridge)
// Wraps the backend polyglot compiler Tauri command with a typed TypeScript API.

import { invoke } from "@tauri-apps/api/core";
import type {
  CompilerTarget,
  CompileResult,
  SupportedLanguage,
} from "../types/modules";

export class PolyglotCompiler {
  /** Compile a target using the backend polyglot compiler. */
  static async compile(target: CompilerTarget): Promise<CompileResult> {
    return invoke<CompileResult>("core_compile", { target });
  }

  /** Return the list of languages supported by the backend compiler. */
  static async supportedLanguages(): Promise<SupportedLanguage[]> {
    return invoke<SupportedLanguage[]>("core_supported_languages");
  }

  /** Build a CompilerTarget with sensible defaults. */
  static buildTarget(
    overrides: Partial<CompilerTarget> & {
      source_path: string;
      output_path: string;
    }
  ): CompilerTarget {
    return {
      id: crypto.randomUUID(),
      language: "rust",
      optimisation_level: 2,
      target_platforms: ["linux"],
      ...overrides,
    };
  }
}
