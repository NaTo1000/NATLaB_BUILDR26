// NATLaB BUILDR26 — App Shell
// Routes between modules and displays the Overseer dashboard.

import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { OverseerDashboard } from "./components/OverseerDashboard";
import type { ModuleDescriptor } from "./types/modules";
import clsx from "clsx";
import "./App.css";

const MODULE_ICONS: Record<string, string> = {
  core_engine: "⚙",
  app_repair: "🔧",
  xplatform_forge: "🏗",
  boot_forge: "💾",
  ai_logo_studio: "🎨",
  ai_assistant: "🤖",
  cloud_sync: "☁",
  plugin_system: "🧩",
  live_docs: "📄",
};

type ActiveView = "overseer" | string;

export default function App() {
  const [modules, setModules] = useState<ModuleDescriptor[]>([]);
  const [activeView, setActiveView] = useState<ActiveView>("overseer");
  const [loadingModules, setLoadingModules] = useState(false);

  useEffect(() => {
    loadModuleList();
  }, []);

  async function loadModuleList() {
    setLoadingModules(true);
    try {
      const list = await invoke<ModuleDescriptor[]>("core_list_modules");
      setModules(list.sort((a, b) => a.name.localeCompare(b.name)));
    } catch (err) {
      console.error("Failed to load module list:", err);
    } finally {
      setLoadingModules(false);
    }
  }

  return (
    <div className="app">
      {/* Sidebar */}
      <nav className="sidebar">
        <div className="sidebar__brand">
          <span className="sidebar__logo">⚗</span>
          <span className="sidebar__title">NATLaB</span>
          <span className="sidebar__subtitle">BUILDR26</span>
        </div>

        <ul className="sidebar__nav">
          <li>
            <button
              className={clsx("sidebar__item", activeView === "overseer" && "sidebar__item--active")}
              onClick={() => setActiveView("overseer")}
            >
              <span className="sidebar__item-icon">🛡</span>
              <span className="sidebar__item-label">Build Overseer</span>
            </button>
          </li>

          <li className="sidebar__section-label">Modules</li>

          {loadingModules ? (
            <li className="sidebar__loading">Loading modules…</li>
          ) : (
            modules.map((mod) => (
              <li key={mod.id}>
                <button
                  className={clsx(
                    "sidebar__item",
                    activeView === mod.id && "sidebar__item--active"
                  )}
                  onClick={() => setActiveView(mod.id)}
                >
                  <span className="sidebar__item-icon">
                    {MODULE_ICONS[mod.id] ?? "📦"}
                  </span>
                  <span className="sidebar__item-label">{mod.name}</span>
                  {mod.lazy && !mod.loaded && (
                    <span className="sidebar__lazy-badge" title="Lazy-loaded">
                      lazy
                    </span>
                  )}
                </button>
              </li>
            ))
          )}
        </ul>
      </nav>

      {/* Main Content */}
      <main className="main-content">
        {activeView === "overseer" ? (
          <OverseerDashboard />
        ) : (
          <ModulePlaceholder
            module={modules.find((m) => m.id === activeView) ?? null}
          />
        )}
      </main>
    </div>
  );
}

function ModulePlaceholder({ module }: { module: ModuleDescriptor | null }) {
  if (!module) {
    return <div className="placeholder">Select a module from the sidebar.</div>;
  }

  return (
    <div className="module-placeholder">
      <h2>
        {MODULE_ICONS[module.id] ?? "📦"} {module.name}
      </h2>
      <p className="module-placeholder__meta">
        v{module.version} · {module.category} ·{" "}
        {module.loaded ? "Loaded" : module.lazy ? "Lazy-loaded" : "Not loaded"}
      </p>
      <p className="module-placeholder__hint">
        Module UI for <strong>{module.name}</strong> will be implemented here.
      </p>
      {module.dependencies.length > 0 && (
        <div className="module-placeholder__deps">
          <strong>Depends on:</strong>{" "}
          {module.dependencies.join(", ")}
        </div>
      )}
      {module.permissions.length > 0 && (
        <div className="module-placeholder__perms">
          <strong>Permissions:</strong>{" "}
          {module.permissions.join(", ")}
        </div>
      )}
    </div>
  );
}
