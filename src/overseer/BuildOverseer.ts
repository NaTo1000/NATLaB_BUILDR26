// Build Overseer — Frontend Bridge
// Wraps the backend overseer Tauri commands with a typed TypeScript API,
// and provides a reactive store for the dashboard.

import { invoke } from "@tauri-apps/api/core";
import { create } from "zustand";
import type {
  BuildRecord,
  BuildMetrics,
  QualityThresholds,
  BuildGrade,
  ViolationSeverity,
} from "../types/modules";

// ── API Bridge ────────────────────────────────────────────────────────────────

export interface EvaluateRequest {
  build_id: string;
  project: string;
  metrics: BuildMetrics;
}

export const OverseerAPI = {
  evaluate: (req: EvaluateRequest) =>
    invoke<BuildRecord>("overseer_evaluate", { request: req }),

  history: () => invoke<BuildRecord[]>("overseer_history"),

  thresholds: () => invoke<QualityThresholds>("overseer_thresholds"),

  rollingScore: (window: number) =>
    invoke<number>("overseer_rolling_score", { window }),
};

// ── Grade Helpers ─────────────────────────────────────────────────────────────

export const GRADE_COLOURS: Record<BuildGrade, string> = {
  A: "#22c55e",
  B: "#84cc16",
  C: "#eab308",
  D: "#f97316",
  F: "#ef4444",
};

export const GRADE_LABELS: Record<BuildGrade, string> = {
  A: "Premium",
  B: "Good",
  C: "Acceptable",
  D: "Needs Work",
  F: "Failing",
};

export const SEVERITY_COLOURS: Record<ViolationSeverity, string> = {
  info: "#60a5fa",
  warning: "#fbbf24",
  error: "#f97316",
  critical: "#ef4444",
};

// ── Overseer Store ────────────────────────────────────────────────────────────

interface OverseerState {
  history: BuildRecord[];
  thresholds: QualityThresholds | null;
  rollingScore: number;
  loading: boolean;
  error: string | null;
  fetchHistory: () => Promise<void>;
  fetchThresholds: () => Promise<void>;
  fetchRollingScore: (window?: number) => Promise<void>;
  evaluate: (req: EvaluateRequest) => Promise<BuildRecord>;
}

const DEFAULT_THRESHOLDS: QualityThresholds = {
  max_binary_size_bytes: 50 * 1024 * 1024,
  max_startup_ms: 800,
  min_coverage_pct: 70,
  max_security_advisories: 0,
  max_compile_secs: 120,
  max_idle_memory_mb: 150,
};

export const useOverseerStore = create<OverseerState>((set, get) => ({
  history: [],
  thresholds: null,
  rollingScore: 100,
  loading: false,
  error: null,

  fetchHistory: async () => {
    set({ loading: true, error: null });
    try {
      const history = await OverseerAPI.history();
      set({ history, loading: false });
    } catch (err) {
      set({ error: String(err), loading: false });
    }
  },

  fetchThresholds: async () => {
    try {
      const thresholds = await OverseerAPI.thresholds();
      set({ thresholds });
    } catch {
      set({ thresholds: DEFAULT_THRESHOLDS });
    }
  },

  fetchRollingScore: async (window = 10) => {
    try {
      const rollingScore = await OverseerAPI.rollingScore(window);
      set({ rollingScore });
    } catch {
      // keep current value
    }
  },

  evaluate: async (req) => {
    const record = await OverseerAPI.evaluate(req);
    set((state) => ({ history: [record, ...state.history] }));
    // Refresh rolling score after each new evaluation
    get().fetchRollingScore();
    return record;
  },
}));

// ── Quality Score Calculator ──────────────────────────────────────────────────

/** Compute a 0–100 quality score from a BuildMetrics object. */
export function computeQualityScore(
  metrics: BuildMetrics,
  thresholds: QualityThresholds
): number {
  let score = 100;

  if (metrics.binary_size_bytes > thresholds.max_binary_size_bytes) score -= 5;
  if (metrics.startup_ms > thresholds.max_startup_ms) score -= 15;
  if (metrics.coverage_pct < thresholds.min_coverage_pct) {
    const gap = thresholds.min_coverage_pct - metrics.coverage_pct;
    score -= Math.min(20, gap * 0.5);
  }
  if (metrics.security_advisories > thresholds.max_security_advisories) {
    score -= metrics.security_advisories * 20;
  }
  if (metrics.compile_secs > thresholds.max_compile_secs) score -= 5;
  if (metrics.idle_memory_mb > thresholds.max_idle_memory_mb) score -= 5;
  if (metrics.test_failures > 0) score -= metrics.test_failures * 15;
  if (metrics.outdated_dependencies > 5) score -= 5;
  if (metrics.lint_warnings > 10) score -= 5;

  return Math.max(0, Math.min(100, score));
}
