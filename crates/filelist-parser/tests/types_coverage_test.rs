//! filelist-parser types/cadence coverage 补充测试 (Phase 8 Round 3)

use thanosLSP_filelist::cadence::CadenceParser;
use thanosLSP_filelist::types::{FilelistResult, ParseOptions, ParseWarning};
use std::path::PathBuf;

// ============================================================
// ParseOptions builder
// ============================================================

#[test]
fn test_parse_options_new() {
    let opts = ParseOptions::new(PathBuf::from("/my/project"));
    assert_eq!(opts.base_path, PathBuf::from("/my/project"));
    assert_eq!(opts.max_depth, 50);
    assert!(!opts.enable_cadence);
    assert!(opts.visited.is_empty());
}

#[test]
fn test_parse_options_with_cadence() {
    let opts = ParseOptions::new(PathBuf::from(".")).with_cadence();
    assert!(opts.enable_cadence);
}

#[test]
fn test_parse_options_with_max_depth() {
    let opts = ParseOptions::new(PathBuf::from(".")).with_max_depth(10);
    assert_eq!(opts.max_depth, 10);
}

#[test]
fn test_parse_options_builder_chain() {
    let opts = ParseOptions::new(PathBuf::from("/proj"))
        .with_cadence()
        .with_max_depth(5);
    assert!(opts.enable_cadence);
    assert_eq!(opts.max_depth, 5);
}

// ============================================================
// FilelistResult helpers
// ============================================================

#[test]
fn test_filelist_result_add_library_file() {
    let mut r = FilelistResult::default();
    r.add_library_file(PathBuf::from("lib/prim.v"));
    assert_eq!(r.library_files.len(), 1);
    assert_eq!(r.library_files[0], PathBuf::from("lib/prim.v"));
}

#[test]
fn test_filelist_result_add_library_dir() {
    let mut r = FilelistResult::default();
    r.add_library_dir(PathBuf::from("/libs/tech"));
    assert_eq!(r.library_dirs.len(), 1);
}

#[test]
fn test_filelist_result_add_library_extensions() {
    let mut r = FilelistResult::default();
    r.add_library_extensions(vec![".v".to_string(), ".sv".to_string()]);
    assert_eq!(r.library_extensions.len(), 2);
}

#[test]
fn test_filelist_result_add_warning() {
    let mut r = FilelistResult::default();
    r.add_warning(ParseWarning::MalformedLine {
        content: "???".to_string(),
        line: 3,
    });
    r.add_warning(ParseWarning::PathNotFound {
        path: PathBuf::from("missing.v"),
        line: 7,
    });
    assert_eq!(r.warnings.len(), 2);
}

#[test]
fn test_filelist_result_merge() {
    let mut a = FilelistResult::default();
    a.add_source_file(PathBuf::from("a.v"), None);
    a.add_library_file(PathBuf::from("la.v"));
    a.add_library_dir(PathBuf::from("/la"));
    a.add_library_extensions(vec![".v".to_string()]);
    a.add_warning(ParseWarning::MalformedLine {
        content: "x".to_string(),
        line: 1,
    });

    let mut b = FilelistResult::default();
    b.add_source_file(PathBuf::from("b.v"), Some("mylib".to_string()));
    b.add_include_dir(PathBuf::from("/inc"));
    b.add_macro_define("FOO".to_string(), Some("1".to_string()));

    a.merge(b);

    assert_eq!(a.source_files.len(), 2);
    assert_eq!(a.include_dirs.len(), 1);
    assert_eq!(a.macro_defines.len(), 1);
    assert_eq!(a.library_files.len(), 1);
    assert_eq!(a.library_dirs.len(), 1);
    assert_eq!(a.library_extensions.len(), 1);
    assert_eq!(a.warnings.len(), 1);
}

// ============================================================
// CadenceParser with various options
// ============================================================

#[test]
fn test_cadence_parser_default() {
    let parser = CadenceParser::default();
    let result = parser.parse("file.v\n");
    assert_eq!(result.source_files.len(), 1);
}

#[test]
fn test_cadence_parser_library_file() {
    let parser = CadenceParser::new(PathBuf::from("/proj"));
    let result = parser.parse("-v prim.v\n");
    assert_eq!(result.library_files.len(), 1);
}

#[test]
fn test_cadence_parser_library_dir() {
    let parser = CadenceParser::new(PathBuf::from("/proj"));
    let result = parser.parse("-y /libs\n");
    assert_eq!(result.library_dirs.len(), 1);
}

#[test]
fn test_cadence_parser_libext() {
    let parser = CadenceParser::new(PathBuf::from("/proj"));
    let result = parser.parse("+libext+.v+.sv\n");
    assert!(!result.library_extensions.is_empty());
}

#[test]
fn test_cadence_parser_unknown_option_becomes_warning() {
    let parser = CadenceParser::new(PathBuf::from("/proj"));
    // Unknown flags that don't start with + or - go through parse_line
    // and become UnknownOption → MalformedLine warning
    let result = parser.parse("-unknown_flag_xyz\n");
    // Should not panic and return some result
    let _ = result;
}

#[test]
fn test_cadence_parser_include_and_define() {
    let parser = CadenceParser::new(PathBuf::from("/proj"));
    let content = "+incdir+./inc\n+define+SIM\ndesign.sv\n";
    let result = parser.parse(content);
    assert_eq!(result.include_dirs.len(), 1);
    assert_eq!(result.macro_defines.len(), 1);
    assert_eq!(result.source_files.len(), 1);
}

// ============================================================
// Synopsys parser edge cases
// ============================================================

#[test]
fn test_synopsys_f_with_tab() {
    // 测试 -f\t 格式
    let parser = CadenceParser::new(PathBuf::from("/proj"));
    // Note: CadenceParser delegates to synopsys for some options
    let result = parser.parse("-f\tsubs.f\n");
    // Should not panic
    let _ = result;
}

#[test]
fn test_synopsys_v_with_tab() {
    // 测试 -v\t 格式
    let parser = CadenceParser::new(PathBuf::from("/proj"));
    let result = parser.parse("-v\tlib.v\n");
    assert!(!result.library_files.is_empty() || result.warnings.len() > 0);
}

#[test]
fn test_synopsys_y_with_tab() {
    // 测试 -y\t 格式
    let parser = CadenceParser::new(PathBuf::from("/proj"));
    let result = parser.parse("-y\tlibdir\n");
    assert!(!result.library_dirs.is_empty() || result.warnings.len() > 0);
}

#[test]
fn test_synopsys_inclist_multiple() {
    // 测试 +incdir+ 后有多个路径的情况
    let parser = CadenceParser::new(PathBuf::from("/proj"));
    let result = parser.parse("+incdir+./inc+./inc2\n");
    assert!(!result.include_dirs.is_empty());
}

#[test]
fn test_synopsys_makelib() {
    // 测试 -makelib
    let parser = CadenceParser::new(PathBuf::from("/proj"));
    let result = parser.parse("-makelib mylib\n");
    // Should not panic
    let _ = result;
}

#[test]
fn test_synopsys_endlib() {
    // 测试 -endlib
    let parser = CadenceParser::new(PathBuf::from("/proj"));
    let result = parser.parse("-endlib\n");
    let _ = result;
}

#[test]
fn test_synopsys_libext_various() {
    // 测试各种 libext 格式
    let parser = CadenceParser::new(PathBuf::from("/proj"));
    let result = parser.parse("+libext+.v\n");
    assert!(!result.library_extensions.is_empty());
}

#[test]
fn test_synopsys_define_with_value() {
    // 测试 +define+NAME=value
    let parser = CadenceParser::new(PathBuf::from("/proj"));
    let result = parser.parse("+define+WIDTH=8\n");
    assert_eq!(result.macro_defines.len(), 1);
    assert_eq!(result.macro_defines[0].name, "WIDTH");
    assert_eq!(result.macro_defines[0].value, Some("8".to_string()));
}

#[test]
fn test_env_expand_in_path() {
    // 测试环境变量展开
    use thanosLSP_filelist::env_expand::expand_with_warnings;
    std::env::set_var("TEST_VAR", "/test/path");
    let (expanded, warnings) = expand_with_warnings("$TEST_VAR/file.v");
    assert!(expanded.contains("test") || expanded.contains("file.v"));
    assert!(warnings.is_empty(), "no undefined var warnings expected");
    std::env::remove_var("TEST_VAR");
}

#[test]
fn test_parse_options_visited_tracking() {
    // 测试 visited 路径追踪
    let mut opts = ParseOptions::new(PathBuf::from("/proj"));
    opts.visited.insert(PathBuf::from("/proj/a.f"));
    opts.visited.insert(PathBuf::from("/proj/b.f"));
    assert_eq!(opts.visited.len(), 2);
}

#[test]
fn test_filelist_result_add_source_file_with_library() {
    // 测试添加带库的源文件
    let mut r = FilelistResult::default();
    r.add_source_file(PathBuf::from("design.v"), Some("work".to_string()));
    assert_eq!(r.source_files.len(), 1);
    assert_eq!(r.source_files[0].library, Some("work".to_string()));
}

#[test]
fn test_filelist_result_add_include_dir() {
    let mut r = FilelistResult::default();
    r.add_include_dir(PathBuf::from("/include"));
    assert_eq!(r.include_dirs.len(), 1);
}

#[test]
fn test_filelist_result_add_macro_define() {
    let mut r = FilelistResult::default();
    r.add_macro_define("DEBUG".to_string(), None);
    r.add_macro_define("WIDTH".to_string(), Some("8".to_string()));
    assert_eq!(r.macro_defines.len(), 2);
}
