// App Repair Engine — Frontend Module
import { invoke } from "@tauri-apps/api/core";
import type { RepairTarget, RepairReport } from "../../types/modules";

export const AppRepairEngine = {
  /** Scan a project for issues. */
  scan: (target: RepairTarget) =>
    invoke<RepairReport>("repair_scan", { target }),

  /** Apply auto-fixes to a list of issues. */
  applyFixes: (targetPath: string, issueIds: string[]) =>
    invoke<RepairReport>("repair_apply_fixes", {
      targetPath,
      issueIds,
    }),
};
