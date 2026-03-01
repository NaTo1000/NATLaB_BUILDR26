// BootForge — Frontend Module
import { invoke } from "@tauri-apps/api/core";
import type { BootForgeRequest, BootForgeResult } from "../../types/modules";

export const BootForge = {
  /** Create a bootable media image. */
  create: (request: BootForgeRequest) =>
    invoke<BootForgeResult>("bootforge_create", { request }),

  /** Verify the integrity of an existing image. */
  verify: (imagePath: string, expectedSha256?: string) =>
    invoke<boolean>("bootforge_verify", {
      imagePath,
      expectedSha256: expectedSha256 ?? null,
    }),
};
