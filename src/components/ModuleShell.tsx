// Module Shell — wraps each module with a consistent header and lazy-load state.

import React, { ReactNode } from "react";
import clsx from "clsx";
import type { ModuleDescriptor } from "../types/modules";

interface ModuleShellProps {
  descriptor: ModuleDescriptor;
  children: ReactNode;
  loading?: boolean;
  error?: string | null;
  className?: string;
}

const CATEGORY_ICONS: Record<string, string> = {
  core_engine: "⚙",
  repair: "🔧",
  build: "🏗",
  ai: "🤖",
  media: "💾",
  cloud: "☁",
  plugin: "🧩",
  docs: "📄",
};

export function ModuleShell({
  descriptor,
  children,
  loading = false,
  error = null,
  className,
}: ModuleShellProps) {
  const icon = CATEGORY_ICONS[descriptor.category] ?? "📦";

  return (
    <section className={clsx("module-shell", className)}>
      <header className="module-shell__header">
        <span className="module-shell__icon" role="img" aria-label={descriptor.category}>
          {icon}
        </span>
        <div className="module-shell__meta">
          <h3 className="module-shell__name">{descriptor.name}</h3>
          <span className="module-shell__version">v{descriptor.version}</span>
        </div>
        <div className="module-shell__status">
          {descriptor.loaded ? (
            <span className="module-shell__badge module-shell__badge--loaded">Loaded</span>
          ) : (
            <span className="module-shell__badge module-shell__badge--lazy">Lazy</span>
          )}
        </div>
      </header>

      <div className="module-shell__body">
        {loading ? (
          <div className="module-shell__loading">Loading module…</div>
        ) : error ? (
          <div className="module-shell__error">⚠ {error}</div>
        ) : (
          children
        )}
      </div>
    </section>
  );
}
