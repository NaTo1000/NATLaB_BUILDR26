// AI Logo Studio — Frontend Module
import { invoke } from "@tauri-apps/api/core";
import type { LogoPrompt, LogoGenerationResult } from "../../types/modules";

export const AILogoStudio = {
  /** Generate logo variants for a brand prompt. */
  generate: (prompt: LogoPrompt, variantCount: number = 4) =>
    invoke<LogoGenerationResult>("logo_generate", {
      prompt,
      variantCount,
    }),

  /** Export a variant to a file. */
  export: (variantId: string, outputPath: string, format: "png" | "svg" | "ico" | "webp") =>
    invoke<string>("logo_export", { variantId, outputPath, format }),
};
