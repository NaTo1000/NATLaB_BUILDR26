// Rust unit tests for the Build Overseer.
// Run with: cargo test --manifest-path src-tauri/Cargo.toml

#[cfg(test)]
mod overseer_tests {
    use crate::overseer::{
        BuildGrade, BuildMetrics, BuildOverseer, BuildRecord, BuildStage, QualityThresholds,
    };
    use chrono::Utc;

    fn default_record(id: &str, metrics: BuildMetrics) -> BuildRecord {
        BuildRecord {
            build_id: id.to_string(),
            project: "test-project".to_string(),
            started_at: Utc::now(),
            finished_at: None,
            stage: BuildStage::QualityGate,
            metrics,
            violations: vec![],
            grade: None,
            perf_samples: vec![],
        }
    }

    fn perfect_metrics() -> BuildMetrics {
        BuildMetrics {
            binary_size_bytes: 10 * 1024 * 1024,
            startup_ms: 300,
            coverage_pct: 95.0,
            security_advisories: 0,
            compile_secs: 45,
            idle_memory_mb: 80,
            lint_warnings: 0,
            test_count: 200,
            test_failures: 0,
            bundle_chunks: 4,
            total_dependencies: 50,
            outdated_dependencies: 0,
        }
    }

    #[tokio::test]
    async fn grade_a_for_perfect_build() {
        let overseer = BuildOverseer::default();
        let record = default_record("build-001", perfect_metrics());
        let result = overseer.evaluate(record).await.unwrap();
        assert_eq!(result.grade, Some(BuildGrade::A));
        assert!(result.violations.is_empty());
        assert_eq!(result.stage, BuildStage::Complete);
    }

    #[tokio::test]
    async fn grade_f_for_security_advisory() {
        let overseer = BuildOverseer::default();
        let mut metrics = perfect_metrics();
        metrics.security_advisories = 1;
        let record = default_record("build-002", metrics);
        let result = overseer.evaluate(record).await.unwrap();
        assert_eq!(result.grade, Some(BuildGrade::F));
        assert!(result.violations.iter().any(|v| v.code == "OVS-004"));
    }

    #[tokio::test]
    async fn grade_f_for_test_failures() {
        let overseer = BuildOverseer::default();
        let mut metrics = perfect_metrics();
        metrics.test_failures = 3;
        let record = default_record("build-003", metrics);
        let result = overseer.evaluate(record).await.unwrap();
        assert_eq!(result.grade, Some(BuildGrade::F));
        assert_eq!(result.stage, BuildStage::Failed);
    }

    #[tokio::test]
    async fn grade_d_for_slow_startup() {
        let overseer = BuildOverseer::default();
        let mut metrics = perfect_metrics();
        metrics.startup_ms = 1500;
        let record = default_record("build-004", metrics);
        let result = overseer.evaluate(record).await.unwrap();
        assert_eq!(result.grade, Some(BuildGrade::D));
        assert!(result.violations.iter().any(|v| v.code == "OVS-002"));
    }

    #[tokio::test]
    async fn grade_b_for_single_warning() {
        let overseer = BuildOverseer::default();
        let mut metrics = perfect_metrics();
        metrics.binary_size_bytes = 60 * 1024 * 1024; // triggers OVS-001 (warning)
        let record = default_record("build-005", metrics);
        let result = overseer.evaluate(record).await.unwrap();
        assert_eq!(result.grade, Some(BuildGrade::B));
    }

    #[tokio::test]
    async fn rolling_score_100_with_no_history() {
        let overseer = BuildOverseer::default();
        let score = overseer.rolling_score(10).await;
        assert_eq!(score, 100.0);
    }

    #[tokio::test]
    async fn history_grows_and_is_returned_most_recent_first() {
        let overseer = BuildOverseer::default();
        for i in 0..3 {
            let record = default_record(&format!("build-{:03}", i), perfect_metrics());
            overseer.evaluate(record).await.unwrap();
        }
        let hist = overseer.history().await;
        assert_eq!(hist.len(), 3);
        // Most recent should be last evaluated (build-002)
        assert_eq!(hist[0].build_id, "build-002");
    }

    #[tokio::test]
    async fn rolling_score_reflects_failing_builds() {
        let overseer = BuildOverseer::default();

        // 5 perfect builds
        for i in 0..5 {
            let record = default_record(&format!("good-{}", i), perfect_metrics());
            overseer.evaluate(record).await.unwrap();
        }

        // 5 failing builds
        for i in 0..5 {
            let mut m = perfect_metrics();
            m.test_failures = 1;
            let record = default_record(&format!("bad-{}", i), m);
            overseer.evaluate(record).await.unwrap();
        }

        // 5 out of 10 are premium — expect 50 %
        let score = overseer.rolling_score(10).await;
        assert_eq!(score, 50.0);
    }
}

#[cfg(test)]
mod core_tests {
    use crate::core::{
        ModuleRegistry, PolyglotCompiler, CompilerTarget, SupportedLanguage, TargetPlatform,
    };

    #[tokio::test]
    async fn registry_contains_all_builtin_modules() {
        let registry = ModuleRegistry::new();
        let modules = registry.list().await;
        let ids: Vec<&str> = modules.iter().map(|m| m.id.as_str()).collect();
        assert!(ids.contains(&"core_engine"));
        assert!(ids.contains(&"app_repair"));
        assert!(ids.contains(&"xplatform_forge"));
        assert!(ids.contains(&"boot_forge"));
        assert!(ids.contains(&"ai_logo_studio"));
        assert!(ids.contains(&"ai_assistant"));
        assert!(ids.contains(&"cloud_sync"));
        assert!(ids.contains(&"plugin_system"));
        assert!(ids.contains(&"live_docs"));
        assert_eq!(modules.len(), 9);
    }

    #[tokio::test]
    async fn loading_a_module_marks_it_loaded() {
        let registry = ModuleRegistry::new();
        // Must load core_engine first since app_repair depends on it
        registry.load("core_engine").await.unwrap();
        let module = registry.load("app_repair").await.unwrap();
        assert!(module.loaded);
        assert!(module.loaded_at.is_some());
    }

    #[tokio::test]
    async fn loading_module_without_dependency_fails() {
        let registry = ModuleRegistry::new();
        // app_repair depends on core_engine, which is not loaded
        let result = registry.load("app_repair").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn loading_nonexistent_module_returns_error() {
        let registry = ModuleRegistry::new();
        let result = registry.load("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn polyglot_compiler_returns_success_for_valid_target() {
        let target = CompilerTarget {
            id: "test-compile".to_string(),
            language: SupportedLanguage::Rust,
            source_path: "/some/path".to_string(),
            output_path: "/some/out".to_string(),
            optimisation_level: 2,
            target_platforms: vec![TargetPlatform::Linux],
        };
        let result = PolyglotCompiler::compile(target).await.unwrap();
        assert!(result.success);
        assert_eq!(result.output_path, Some("/some/out".to_string()));
    }

    #[tokio::test]
    async fn polyglot_compiler_fails_for_empty_paths() {
        let target = CompilerTarget {
            id: "test-empty".to_string(),
            language: SupportedLanguage::TypeScript,
            source_path: String::new(),
            output_path: String::new(),
            optimisation_level: 0,
            target_platforms: vec![TargetPlatform::Wasm],
        };
        let result = PolyglotCompiler::compile(target).await.unwrap();
        assert!(!result.success);
    }

    #[tokio::test]
    async fn supported_languages_contains_rust_and_typescript() {
        let langs = PolyglotCompiler::supported_languages();
        assert!(langs.contains(&SupportedLanguage::Rust));
        assert!(langs.contains(&SupportedLanguage::TypeScript));
    }
}
