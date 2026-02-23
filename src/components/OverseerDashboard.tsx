// Overseer Dashboard — React component
// Displays the rolling quality score, recent build history, and grade breakdown.

import React, { useEffect } from "react";
import { useOverseerStore, GRADE_COLOURS, GRADE_LABELS, SEVERITY_COLOURS } from "../overseer/BuildOverseer";
import type { BuildRecord, BuildGrade } from "../types/modules";
import clsx from "clsx";

function GradeBadge({ grade }: { grade: BuildGrade | null }) {
  if (!grade) return <span className="grade-badge grade-unknown">–</span>;
  return (
    <span
      className="grade-badge"
      style={{ backgroundColor: GRADE_COLOURS[grade] }}
      title={GRADE_LABELS[grade]}
    >
      {grade}
    </span>
  );
}

function BuildRow({ record }: { record: BuildRecord }) {
  const criticalCount = record.violations.filter(
    (v) => v.severity === "critical"
  ).length;
  const errorCount = record.violations.filter((v) => v.severity === "error").length;

  return (
    <tr className={clsx("build-row", record.stage === "failed" && "build-row--failed")}>
      <td className="build-cell build-cell--id" title={record.build_id}>
        {record.build_id.slice(0, 8)}…
      </td>
      <td className="build-cell">{record.project}</td>
      <td className="build-cell">
        <GradeBadge grade={record.grade} />
      </td>
      <td className="build-cell build-cell--stage">{record.stage}</td>
      <td className="build-cell">
        {record.violations.length === 0 ? (
          <span className="violations-none">✓ Clean</span>
        ) : (
          <span className="violations-summary">
            {criticalCount > 0 && (
              <span style={{ color: SEVERITY_COLOURS.critical }}>
                {criticalCount} crit{" "}
              </span>
            )}
            {errorCount > 0 && (
              <span style={{ color: SEVERITY_COLOURS.error }}>
                {errorCount} err
              </span>
            )}
          </span>
        )}
      </td>
      <td className="build-cell build-cell--date">
        {record.started_at
          ? new Date(record.started_at).toLocaleTimeString()
          : "–"}
      </td>
    </tr>
  );
}

export function OverseerDashboard() {
  const {
    history,
    rollingScore,
    thresholds,
    loading,
    error,
    fetchHistory,
    fetchThresholds,
    fetchRollingScore,
  } = useOverseerStore();

  useEffect(() => {
    fetchHistory();
    fetchThresholds();
    fetchRollingScore(10);
  }, [fetchHistory, fetchThresholds, fetchRollingScore]);

  const scoreColour =
    rollingScore >= 80
      ? GRADE_COLOURS.A
      : rollingScore >= 60
      ? GRADE_COLOURS.B
      : rollingScore >= 40
      ? GRADE_COLOURS.C
      : GRADE_COLOURS.F;

  return (
    <div className="overseer-dashboard">
      <header className="overseer-header">
        <h2 className="overseer-title">Build Overseer</h2>
        <button
          className="overseer-refresh"
          onClick={() => {
            fetchHistory();
            fetchRollingScore(10);
          }}
          disabled={loading}
        >
          {loading ? "Loading…" : "↺ Refresh"}
        </button>
      </header>

      {/* Rolling Score Card */}
      <div className="score-card">
        <div
          className="score-ring"
          style={{ "--score-colour": scoreColour } as React.CSSProperties}
        >
          <span className="score-value">{Math.round(rollingScore)}</span>
          <span className="score-label">/ 100</span>
        </div>
        <div className="score-info">
          <p className="score-heading">Rolling Quality Score</p>
          <p className="score-subtext">Based on last 10 builds</p>
          {thresholds && (
            <dl className="thresholds-list">
              <dt>Max startup</dt>
              <dd>{thresholds.max_startup_ms} ms</dd>
              <dt>Min coverage</dt>
              <dd>{thresholds.min_coverage_pct}%</dd>
              <dt>Max idle RAM</dt>
              <dd>{thresholds.max_idle_memory_mb} MB</dd>
            </dl>
          )}
        </div>
      </div>

      {error && <p className="overseer-error">⚠ {error}</p>}

      {/* Build History Table */}
      <div className="build-table-wrapper">
        <table className="build-table">
          <thead>
            <tr>
              <th>Build ID</th>
              <th>Project</th>
              <th>Grade</th>
              <th>Stage</th>
              <th>Violations</th>
              <th>Started</th>
            </tr>
          </thead>
          <tbody>
            {history.length === 0 ? (
              <tr>
                <td colSpan={6} className="build-empty">
                  No builds evaluated yet.
                </td>
              </tr>
            ) : (
              history.map((record) => (
                <BuildRow key={record.build_id} record={record} />
              ))
            )}
          </tbody>
        </table>
      </div>
    </div>
  );
}
