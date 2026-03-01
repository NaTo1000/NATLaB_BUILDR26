// XPlatform Forge — Frontend Module
import { invoke } from "@tauri-apps/api/core";
import type { ForgeRequest, ForgeResult, TargetPlatform } from "../../types/modules";

export const XPlatformForge = {
  /** Start a cross-platform build. */
  build: (request: ForgeRequest) =>
    invoke<ForgeResult>("forge_build", { request }),

  /** List supported target platforms. */
  listPlatforms: () =>
    invoke<TargetPlatform[]>("forge_list_platforms"),
};
