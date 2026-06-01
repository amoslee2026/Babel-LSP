//! core crate coverage 补充测试 (Phase 8 Round 3)
//! 覆盖 diagnostic.rs 和 config.rs 未覆盖路径

use thanosLSP_core::config::ProjectConfig;
use thanosLSP_core::diagnostic::{Diagnostic, DiagnosticCache, DiagnosticSeverity};
use thanosLSP_core::symbol::{Location, Position};

fn make_loc(uri: &str) -> Location {
    Location {
        uri: uri.to_string(),
        start: Position::new(0, 0),
        end: Position::new(1, 0),
    }
}

// ============================================================
// diagnostic.rs 未覆盖方法
// ============================================================

#[test]
fn test_diagnostic_with_code() {
    let d = Diagnostic::error(make_loc("file:///a.sv"), "bad".to_string())
        .with_code("SYN-001".to_string());
    assert_eq!(d.code, Some("SYN-001".to_string()));
}

#[test]
fn test_diagnostic_with_source() {
    let d = Diagnostic::warning(make_loc("file:///b.sv"), "warn".to_string())
        .with_source("thanosLSP-sv".to_string());
    assert_eq!(d.source, "thanosLSP-sv");
}

#[test]
fn test_diagnostic_add_related_info() {
    let d = Diagnostic::new(
        make_loc("file:///c.sv"),
        DiagnosticSeverity::Information,
        "info".to_string(),
    )
    .add_related_info(make_loc("file:///d.sv"), "see here".to_string());
    assert_eq!(d.related_info.len(), 1);
    assert_eq!(d.related_info[0].message, "see here");
}

#[test]
fn test_diagnostic_severity_hint() {
    let d = Diagnostic::new(
        make_loc("file:///e.sv"),
        DiagnosticSeverity::Hint,
        "hint".to_string(),
    );
    assert_eq!(d.severity, DiagnosticSeverity::Hint);
}

#[test]
fn test_diagnostic_cache_by_file() {
    let mut cache = DiagnosticCache::new();
    cache.add(Diagnostic::error(make_loc("file:///x.sv"), "e1".to_string()));
    cache.add(Diagnostic::error(make_loc("file:///y.sv"), "e2".to_string()));
    cache.add(Diagnostic::warning(make_loc("file:///x.sv"), "w1".to_string()));

    let x_diags = cache.by_file("file:///x.sv");
    assert_eq!(x_diags.len(), 2);

    let y_diags = cache.by_file("file:///y.sv");
    assert_eq!(y_diags.len(), 1);

    let z_diags = cache.by_file("file:///z.sv");
    assert!(z_diags.is_empty());
}

#[test]
fn test_diagnostic_cache_clear() {
    let mut cache = DiagnosticCache::default();
    cache.add(Diagnostic::error(make_loc("file:///f.sv"), "e".to_string()));
    assert_eq!(cache.diagnostics().len(), 1);
    cache.clear();
    assert!(cache.diagnostics().is_empty());
}

// ============================================================
// config.rs 未覆盖路径
// ============================================================

#[test]
fn test_config_from_file_nonexistent_returns_err() {
    use std::path::PathBuf;
    let result = ProjectConfig::from_file(&PathBuf::from("/nonexistent/path/config.json"));
    assert!(result.is_err());
}

#[test]
fn test_config_load_from_root_no_file_uses_default() {
    use std::path::Path;
    // /tmp is guaranteed to exist but has no thanosLSP.json
    let result = ProjectConfig::load_from_root(Path::new("/tmp"));
    assert!(result.is_ok());
    let config = result.unwrap();
    assert_eq!(config.server.mcp_port, 3000);
}



#[test]
fn test_config_default_for_root() {
    use std::path::Path;
    let config = ProjectConfig::default_for_root(Path::new("/my/project")).unwrap();
    assert_eq!(config.project_root.to_str().unwrap(), "/my/project");
    assert_eq!(config.vhdl.libraries, vec!["work".to_string()]);
    assert!(config.synth.enabled);
    assert_eq!(config.synth.rules, vec!["all".to_string()]);
}

#[test]
fn test_config_from_file_valid_json() {
    use std::io::Write;
    use std::path::PathBuf;

    let json = r#"{
        "project_root": "/tmp/proj",
        "hdl": {"filelists": [], "include_paths": [], "defines": []},
        "vhdl": {"libraries": ["work"], "standard": "2008"},
        "tcl": {"source_paths": [], "eda_tools": ["Vivado"]},
        "classification": {
            "rtl_patterns": ["**/*.v"],
            "tb_patterns": ["**/*_tb.*"],
            "netlist_patterns": ["**/*_netlist.*"]
        },
        "server": {"lsp_port": 6030, "mcp_port": 3000},
        "synth": {"enabled": true, "rules": ["all"]},
        "logging": {"level": "info", "file": null},
        "memory": {"scan_interval_secs": 300, "persist_path": "/tmp/.thanos"}
    }"#;

    let path = PathBuf::from("/tmp/thanosLSP_test_config.json");
    let mut f = std::fs::File::create(&path).unwrap();
    f.write_all(json.as_bytes()).unwrap();
    drop(f);

    let result = ProjectConfig::from_file(&path);
    assert!(result.is_ok(), "from_file failed: {:?}", result.err());
    let config = result.unwrap();
    assert_eq!(config.server.mcp_port, 3000);

    std::fs::remove_file(&path).ok();
}
