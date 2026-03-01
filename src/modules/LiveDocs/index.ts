// Live Docs Generator — Frontend Module
import { invoke } from "@tauri-apps/api/core";
import type { DocGenRequest, DocGenResult } from "../../types/modules";

export const LiveDocs = {
  /** Generate documentation for a project. */
  generate: (request: DocGenRequest) =>
    invoke<DocGenResult>("docs_generate", { request }),

  /** Start watching a project directory for live doc regeneration. */
  watchStart: (projectPath: string, outputPath: string) =>
    invoke<string>("docs_watch_start", { projectPath, outputPath }),

  /** Stop a running docs watcher. */
  watchStop: (watcherId: string) =>
    invoke<void>("docs_watch_stop", { watcherId }),
};
