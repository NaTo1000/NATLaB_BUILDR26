// Unit tests: BuildOverseer quality score computation and grade assignment.
// These tests run purely in the JS runtime — no Tauri IPC needed.

import { describe, it, expect } from "vitest";
import {
  computeQualityScore,
  GRADE_COLOURS,
  GRADE_LABELS,
} from "../overseer/BuildOverseer";
import type { BuildMetrics, QualityThresholds, BuildGrade } from "../types/modules";

const DEFAULT_THRESHOLDS: QualityThresholds = {
  max_binary_size_bytes: 50 * 1024 * 1024,
  max_startup_ms: 800,
  min_coverage_pct: 70,
  max_security_advisories: 0,
  max_compile_secs: 120,
  max_idle_memory_mb: 150,
};

const PERFECT_METRICS: BuildMetrics = {
  binary_size_bytes: 10 * 1024 * 1024,
  startup_ms: 300,
  coverage_pct: 95,
  security_advisories: 0,
  compile_secs: 45,
  idle_memory_mb: 80,
  lint_warnings: 0,
  test_count: 200,
  test_failures: 0,
  bundle_chunks: 4,
  total_dependencies: 50,
  outdated_dependencies: 0,
};

describe("computeQualityScore", () => {
  it("returns 100 for a perfect build", () => {
    expect(computeQualityScore(PERFECT_METRICS, DEFAULT_THRESHOLDS)).toBe(100);
  });

  it("deducts 15 points when startup_ms exceeds threshold", () => {
    const metrics = { ...PERFECT_METRICS, startup_ms: 1000 };
    expect(computeQualityScore(metrics, DEFAULT_THRESHOLDS)).toBe(85);
  });

  it("deducts points proportionally for low coverage", () => {
    // coverage gap = 70 - 50 = 20; deduction = min(20, 20 * 0.5) = 10
    const metrics = { ...PERFECT_METRICS, coverage_pct: 50 };
    expect(computeQualityScore(metrics, DEFAULT_THRESHOLDS)).toBe(90);
  });

  it("deducts 20 points per security advisory", () => {
    const metrics = { ...PERFECT_METRICS, security_advisories: 2 };
    expect(computeQualityScore(metrics, DEFAULT_THRESHOLDS)).toBe(60);
  });

  it("deducts 15 points per test failure", () => {
    const metrics = { ...PERFECT_METRICS, test_failures: 2 };
    expect(computeQualityScore(metrics, DEFAULT_THRESHOLDS)).toBe(70);
  });

  it("clamps score to a minimum of 0", () => {
    const metrics: BuildMetrics = {
      ...PERFECT_METRICS,
      security_advisories: 10,
      test_failures: 5,
      startup_ms: 5000,
    };
    expect(computeQualityScore(metrics, DEFAULT_THRESHOLDS)).toBe(0);
  });

  it("deducts 5 points for binary size over limit", () => {
    const metrics = { ...PERFECT_METRICS, binary_size_bytes: 60 * 1024 * 1024 };
    expect(computeQualityScore(metrics, DEFAULT_THRESHOLDS)).toBe(95);
  });

  it("deducts 5 points for outdated dependencies > 5", () => {
    const metrics = { ...PERFECT_METRICS, outdated_dependencies: 10 };
    expect(computeQualityScore(metrics, DEFAULT_THRESHOLDS)).toBe(95);
  });

  it("deducts 5 points for lint warnings > 10", () => {
    const metrics = { ...PERFECT_METRICS, lint_warnings: 15 };
    expect(computeQualityScore(metrics, DEFAULT_THRESHOLDS)).toBe(95);
  });

  it("does not deduct for lint warnings <= 10", () => {
    const metrics = { ...PERFECT_METRICS, lint_warnings: 10 };
    expect(computeQualityScore(metrics, DEFAULT_THRESHOLDS)).toBe(100);
  });

  it("accumulates multiple independent deductions", () => {
    const metrics = {
      ...PERFECT_METRICS,
      startup_ms: 1000,        // -15
      binary_size_bytes: 60 * 1024 * 1024, // -5
      outdated_dependencies: 10, // -5
    };
    expect(computeQualityScore(metrics, DEFAULT_THRESHOLDS)).toBe(75);
  });
});

describe("Grade constants", () => {
  const grades: BuildGrade[] = ["A", "B", "C", "D", "F"];

  it("has a colour for every grade", () => {
    grades.forEach((g) => {
      expect(GRADE_COLOURS[g]).toBeDefined();
      expect(GRADE_COLOURS[g]).toMatch(/^#[0-9a-fA-F]{6}$/);
    });
  });

  it("has a label for every grade", () => {
    grades.forEach((g) => {
      expect(GRADE_LABELS[g]).toBeDefined();
      expect(typeof GRADE_LABELS[g]).toBe("string");
      expect(GRADE_LABELS[g].length).toBeGreaterThan(0);
    });
  });
});
