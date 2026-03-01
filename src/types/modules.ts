// NATLaB BUILDR26 — Module Type Definitions
// Single source of truth for all module interfaces shared between
// the frontend (TypeScript) and backend (Rust/Tauri commands).

// ── Module Registry ───────────────────────────────────────────────────────────

export type ModuleCategory =
  | "core_engine"
  | "repair"
  | "build"
  | "ai"
  | "media"
  | "cloud"
  | "plugin"
  | "docs";

export interface ModuleDescriptor {
  id: string;
  name: string;
  version: string;
  category: ModuleCategory;
  lazy: boolean;
  dependencies: string[];
  permissions: string[];
  loaded: boolean;
  loaded_at: string | null;
}

// ── Build Overseer ────────────────────────────────────────────────────────────

export type BuildStage =
  | "init"
  | "lint"
  | "test"
  | "compile"
  | "bundle"
  | "security_scan"
  | "quality_gate"
  | "package"
  | "complete"
  | "failed";

export type ViolationSeverity = "info" | "warning" | "error" | "critical";
export type BuildGrade = "A" | "B" | "C" | "D" | "F";

export interface QualityViolation {
  code: string;
  severity: ViolationSeverity;
  message: string;
  suggestion: string;
}

export interface BuildMetrics {
  binary_size_bytes: number;
  startup_ms: number;
  coverage_pct: number;
  security_advisories: number;
  compile_secs: number;
  idle_memory_mb: number;
  lint_warnings: number;
  test_count: number;
  test_failures: number;
  bundle_chunks: number;
  total_dependencies: number;
  outdated_dependencies: number;
}

export interface PerfSample {
  timestamp: string;
  cpu_percent: number;
  memory_mb: number;
  thread_count: number;
  io_read_kb: number;
  io_write_kb: number;
}

export interface BuildRecord {
  build_id: string;
  project: string;
  started_at: string;
  finished_at: string | null;
  stage: BuildStage;
  metrics: BuildMetrics;
  violations: QualityViolation[];
  grade: BuildGrade | null;
  perf_samples: PerfSample[];
}

export interface QualityThresholds {
  max_binary_size_bytes: number;
  max_startup_ms: number;
  min_coverage_pct: number;
  max_security_advisories: number;
  max_compile_secs: number;
  max_idle_memory_mb: number;
}

// ── App Repair Engine ─────────────────────────────────────────────────────────

export interface RepairTarget {
  path: string;
  project_type: string;
  deep_scan: boolean;
}

export interface RepairIssue {
  id: string;
  severity: string;
  category: string;
  description: string;
  file_path: string | null;
  line: number | null;
  auto_fixable: boolean;
  fix_description: string | null;
}

export interface RepairReport {
  target_path: string;
  scan_duration_ms: number;
  issues_found: number;
  issues_fixed: number;
  issues: RepairIssue[];
  status: string;
}

// ── XPlatform Forge ───────────────────────────────────────────────────────────

export type TargetPlatform = "windows" | "macos" | "linux" | "android" | "ios" | "wasm";

export interface ForgeRequest {
  project_path: string;
  platforms: TargetPlatform[];
  optimisation: number;
  sign: boolean;
  notarise: boolean;
}

export interface PlatformBuildResult {
  platform: TargetPlatform;
  success: boolean;
  artifact_path: string | null;
  duration_ms: number;
  errors: string[];
  warnings: string[];
}

export interface ForgeResult {
  request_id: string;
  platform_results: PlatformBuildResult[];
  total_duration_ms: number;
  overall_success: boolean;
}

// ── BootForge ─────────────────────────────────────────────────────────────────

export type MediaFormat = "iso" | "usb" | "pxe" | "vhd";

export interface BootForgeRequest {
  source_path: string;
  output_path: string;
  format: MediaFormat;
  label: string;
  compression: boolean;
  verify_checksum: boolean;
}

export interface BootForgeResult {
  job_id: string;
  output_path: string;
  format: MediaFormat;
  size_bytes: number;
  sha256: string;
  duration_ms: number;
  success: boolean;
  error: string | null;
}

// ── AI Logo Studio ────────────────────────────────────────────────────────────

export interface LogoPrompt {
  brand_name: string;
  style: string;
  colour_palette: string[];
  aspect_ratio: string;
  negative_prompt: string | null;
  seed: number | null;
}

export interface LogoVariant {
  variant_id: string;
  base64_png: string;
  width: number;
  height: number;
  seed: number;
}

export interface LogoGenerationResult {
  request_id: string;
  variants: LogoVariant[];
  duration_ms: number;
  model_used: string;
}

// ── AI Assistant Engine ───────────────────────────────────────────────────────

export type MessageRole = "system" | "user" | "assistant";

export interface ChatMessage {
  role: MessageRole;
  content: string;
  timestamp_ms: number;
}

export interface ChatRequest {
  session_id: string;
  messages: ChatMessage[];
  context: string | null;
  max_tokens: number | null;
  temperature: number | null;
}

export interface ChatResponse {
  session_id: string;
  reply: ChatMessage;
  tokens_used: number;
  model: string;
  duration_ms: number;
}

export interface CompletionRequest {
  language: string;
  prefix: string;
  suffix: string | null;
  max_tokens: number | null;
}

export interface CompletionResponse {
  completions: string[];
  model: string;
  duration_ms: number;
}

// ── Cloud Sync / Profiles ─────────────────────────────────────────────────────

export interface UserProfile {
  id: string;
  display_name: string;
  email: string;
  avatar_url: string | null;
  preferences: Record<string, unknown>;
  created_at: string;
  updated_at: string;
}

export interface SyncStatus {
  last_sync: string | null;
  pending_changes: number;
  in_progress: boolean;
  error: string | null;
}

export interface SyncResult {
  pushed: number;
  pulled: number;
  conflicts: number;
  duration_ms: number;
  success: boolean;
}

// ── Plugin System ─────────────────────────────────────────────────────────────

export type PluginStatus = "available" | "loaded" | "error" | "disabled";

export interface PluginManifest {
  id: string;
  name: string;
  version: string;
  author: string;
  description: string;
  entry_point: string;
  permissions: string[];
  min_natlab_version: string;
  checksum_sha256: string;
}

export interface PluginInfo {
  manifest: PluginManifest;
  status: PluginStatus;
  loaded_at: string | null;
  sandbox_id: string | null;
}

// ── Live Docs Generator ───────────────────────────────────────────────────────

export type DocFormat = "markdown" | "html" | "pdf" | "docx";

export interface DocGenRequest {
  project_path: string;
  format: DocFormat;
  include_private: boolean;
  include_examples: boolean;
  output_path: string;
  template: string | null;
}

export interface DocGenResult {
  job_id: string;
  output_path: string;
  pages_generated: number;
  symbols_documented: number;
  duration_ms: number;
  warnings: string[];
  success: boolean;
}

// ── Polyglot Compiler ─────────────────────────────────────────────────────────

export type SupportedLanguage =
  | "rust"
  | "typescript"
  | "javascript"
  | "python"
  | "go"
  | "c"
  | "cpp"
  | "swift"
  | "kotlin"
  | "java";

export interface CompilerTarget {
  id: string;
  language: SupportedLanguage;
  source_path: string;
  output_path: string;
  optimisation_level: number;
  target_platforms: TargetPlatform[];
}

export interface CompileResult {
  target_id: string;
  success: boolean;
  output_path: string | null;
  duration_ms: number;
  warnings: string[];
  errors: string[];
}
