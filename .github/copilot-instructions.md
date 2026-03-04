# Copilot Instructions for NATLaB BUILDR26

## Project Overview

**NATLaB BUILDR26** is an AI-driven, cross-platform desktop builder application.

- **Tech Stack:** Tauri 2 (Rust backend) + React 18 (TypeScript frontend) + Monaco Editor + Zustand
- **Node.js** ≥ 18 and **Rust** ≥ 1.75 are required.

---

## Repository Structure

```
NATLaB_BUILDR26/
├── src/                        # React / TypeScript frontend
│   ├── types/modules.ts        # Shared type definitions for all 9 modules
│   ├── core/
│   │   ├── LazyLoader.ts       # Demand-driven module loader
│   │   ├── PluginRegistry.ts   # Plugin lifecycle manager
│   │   └── PolyglotCompiler.ts # Multi-language build bridge
│   ├── overseer/
│   │   └── BuildOverseer.ts    # Quality scoring + Zustand store
│   ├── modules/                # One sub-directory per feature module
│   ├── components/             # Shared UI components
│   └── tests/                  # Vitest unit tests
├── src-tauri/                  # Rust / Tauri 2 backend
│   └── src/
│       ├── lib.rs              # App setup + Tauri command registration
│       ├── core/mod.rs         # ModuleRegistry + PolyglotCompiler
│       ├── overseer/mod.rs     # BuildOverseer quality engine
│       └── commands/           # One file per module (Tauri IPC commands)
├── package.json
├── vite.config.ts
└── tsconfig.json
```

---

## Modules

There are 9 modules. Each module has a TypeScript frontend entry in `src/modules/` and a corresponding Rust command file in `src-tauri/src/commands/`.

| ID | Name | Category | Lazy-loaded |
|----|------|----------|-------------|
| `core_engine` | Core Engine Architecture | core_engine | No |
| `app_repair` | App Repair Engine | repair | Yes |
| `xplatform_forge` | XPlatform Forge | build | Yes |
| `boot_forge` | BootForge | media | Yes |
| `ai_logo_studio` | AI Logo Studio | ai | Yes |
| `ai_assistant` | AI Assistant Engine | ai | Yes |
| `cloud_sync` | Cloud Sync / Profiles | cloud | Yes |
| `plugin_system` | Sandboxed Plugin System | plugin | No |
| `live_docs` | Live Docs Generator | docs | Yes |

Shared type definitions for all modules live in `src/types/modules.ts`. Always update this file when adding or changing module interfaces.

---

## Build Overseer

The **Build Overseer** is the quality gate that grades every build A–F. It lives in:
- Frontend: `src/overseer/BuildOverseer.ts` (Zustand store + scoring logic)
- Backend: `src-tauri/src/overseer/mod.rs`

Quality checks and thresholds:

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

---

## Development Commands

```bash
# Install dependencies
npm install

# Run frontend in dev mode (with Tauri)
npm run tauri:dev

# Production build
npm run tauri:build

# Frontend unit tests (Vitest)
npm test

# Frontend tests with coverage
npm run test:coverage

# TypeScript type-check (no emit)
npm run typecheck

# ESLint
npm run lint

# Rust/backend tests
cargo test --manifest-path src-tauri/Cargo.toml
```

---

## Coding Conventions

### TypeScript / React
- Use **TypeScript strict mode** (`tsconfig.json` has `strict: true`).
- Prefer **functional components** with React hooks.
- Use **Zustand** for global state management; avoid prop-drilling.
- Import shared types from `src/types/modules.ts`.
- Lazy-loadable modules must export a default React component and be loaded via `LazyLoader`.
- Tests live in `src/tests/` and use **Vitest** with `jsdom` environment.
- Test file naming: `<subject>.test.ts` or `<subject>.test.tsx`.

### Rust / Tauri
- Each module's IPC commands live in a single file under `src-tauri/src/commands/`.
- Register new commands in `src-tauri/src/lib.rs` inside the `.invoke_handler()` builder call.
- Follow standard Rust naming conventions: `snake_case` for functions and variables, `PascalCase` for types and structs.
- Return `Result<T, String>` from Tauri commands so errors surface cleanly in the frontend.
- Backend tests live in `src-tauri/tests/`.

### General
- Keep frontend (`src/`) and backend (`src-tauri/`) concerns strictly separated; communicate only through Tauri's IPC layer (`invoke`).
- The Build Overseer thresholds are the acceptance criteria for every build — do not lower them without an explicit decision.
- Do not commit secrets or API keys; use Tauri's environment variable or secrets plugin instead.
