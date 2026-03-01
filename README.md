# NATLaB BUILDR26

> AI-driven, cross-platform builder with a premium-quality Build Overseer.

**Tech Stack:** Tauri 2 (Rust) + React 18 (TypeScript) + Monaco Editor + Zustand

---

## System Architecture

### Modules

| ID | Name | Category | Lazy |
|----|------|----------|------|
| `core_engine` | Core Engine Architecture | core_engine | No |
| `app_repair` | App Repair Engine | repair | Yes |
| `xplatform_forge` | XPlatform Forge | build | Yes |
| `boot_forge` | BootForge | media | Yes |
| `ai_logo_studio` | AI Logo Studio | ai | Yes |
| `ai_assistant` | AI Assistant Engine | ai | Yes |
| `cloud_sync` | Cloud Sync / Profiles | cloud | Yes |
| `plugin_system` | Sandboxed Plugin System | plugin | No |
| `live_docs` | Live Docs Generator | docs | Yes |

### Build Overseer

The **Build Overseer** is the quality-gate that evaluates every build against configurable thresholds and assigns an A–F grade:

| Code | Check | Threshold |
|------|-------|-----------|
| OVS-001 | Binary size | ≤ 50 MB |
| OVS-002 | Startup time | ≤ 800 ms |
| OVS-003 | Test coverage | ≥ 70% |
| OVS-004 | Security advisories | 0 |
| OVS-005 | Compile time | ≤ 120 s |
| OVS-006 | Idle memory | ≤ 150 MB |
| OVS-007 | Test failures | 0 |
| OVS-008 | Outdated deps | ≤ 5 |

Grades: **A** Premium · **B** Good · **C** Acceptable · **D** Needs Work · **F** Failing

### Directory Layout

```
NATLaB_BUILDR26/
├── src/                        # React / TypeScript frontend
│   ├── types/modules.ts        # Shared type definitions (all 9 modules)
│   ├── core/
│   │   ├── LazyLoader.ts       # Demand-driven module loader
│   │   ├── PluginRegistry.ts   # Plugin lifecycle manager
│   │   └── PolyglotCompiler.ts # Multi-language build bridge
│   ├── overseer/
│   │   └── BuildOverseer.ts    # Quality scoring + Zustand store
│   ├── modules/
│   │   ├── AppRepairEngine/
│   │   ├── XPlatformForge/
│   │   ├── BootForge/
│   │   ├── AILogoStudio/
│   │   ├── AIAssistant/
│   │   ├── CloudSync/
│   │   ├── PluginSystem/
│   │   └── LiveDocs/
│   ├── components/
│   │   ├── ModuleShell.tsx     # Shared module wrapper UI
│   │   └── OverseerDashboard.tsx
│   └── tests/
│       ├── buildOverseer.test.ts
│       ├── lazyLoader.test.ts
│       └── pluginRegistry.test.ts
├── src-tauri/                  # Rust / Tauri backend
│   └── src/
│       ├── lib.rs              # App setup + command registration
│       ├── core/mod.rs         # ModuleRegistry + PolyglotCompiler
│       ├── overseer/mod.rs     # BuildOverseer quality engine
│       └── commands/           # One file per module
│           ├── core_cmds.rs
│           ├── overseer_cmds.rs
│           ├── app_repair.rs
│           ├── xplatform_forge.rs
│           ├── boot_forge.rs
│           ├── ai_logo_studio.rs
│           ├── ai_assistant.rs
│           ├── cloud_sync.rs
│           ├── plugin_system.rs
│           └── live_docs.rs
├── package.json
├── vite.config.ts
└── tsconfig.json
```

---

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) ≥ 18
- [Rust](https://rustup.rs/) ≥ 1.75
- [Tauri CLI](https://tauri.app/start/prerequisites/) v2

### Install

```bash
npm install
```

### Development

```bash
npm run tauri:dev
```

### Build

```bash
npm run tauri:build
```

### Tests

```bash
# Frontend (Vitest)
npm test

# Backend (Rust)
cargo test --manifest-path src-tauri/Cargo.toml
```
