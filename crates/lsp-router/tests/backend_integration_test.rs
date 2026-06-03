//! LSP Backend 集成测试
//!
//! 测试 ThanosLspBackend 的辅助函数和内部逻辑

use babel_lsp_lsp::backend::{extract_symbols_basic, convert_diagnostics};
use tower_lsp::lsp_types::*;
use url::Url;

// ============================================================
// extract_symbols_basic 测试
// ============================================================

#[test]
fn test_extract_symbols_basic_module() {
    let uri = Url::parse("file:///test.sv").unwrap();
    let content = "module my_module;\nendmodule";
    let symbols = extract_symbols_basic(&uri, content);

    assert_eq!(symbols.len(), 1);
    assert_eq!(symbols[0].name.as_str(), "my_module");
}

#[test]
fn test_extract_symbols_multiple_modules() {
    let uri = Url::parse("file:///test.sv").unwrap();
    let content = "module a;\nendmodule\nmodule b;\nendmodule";
    let symbols = extract_symbols_basic(&uri, content);

    assert_eq!(symbols.len(), 2);
}

#[test]
fn test_extract_symbols_empty() {
    let uri = Url::parse("file:///test.sv").unwrap();
    let symbols = extract_symbols_basic(&uri, "");

    assert!(symbols.is_empty());
}

#[test]
fn test_extract_symbols_no_modules() {
    let uri = Url::parse("file:///test.sv").unwrap();
    let content = "// just comments\nwire a;";
    let symbols = extract_symbols_basic(&uri, content);

    assert!(symbols.is_empty());
}

#[test]
fn test_extract_symbols_with_parentheses() {
    let uri = Url::parse("file:///test.sv").unwrap();
    let content = "module my_module(\n    input clk\n);\nendmodule";
    let symbols = extract_symbols_basic(&uri, content);

    assert_eq!(symbols.len(), 1);
    assert_eq!(symbols[0].name.as_str(), "my_module");
}

#[test]
fn test_extract_symbols_with_semicolon() {
    let uri = Url::parse("file:///test.sv").unwrap();
    let content = "module top; wire a; endmodule";
    let symbols = extract_symbols_basic(&uri, content);

    assert_eq!(symbols.len(), 1);
    assert_eq!(symbols[0].name.as_str(), "top");
}

#[test]
fn test_extract_symbols_invalid_name_numeric_start() {
    let uri = Url::parse("file:///test.sv").unwrap();
    // 数字开头的模块名应被跳过
    let content = "module 123invalid;\nendmodule\nmodule valid;\nendmodule";
    let symbols = extract_symbols_basic(&uri, content);

    assert_eq!(symbols.len(), 1);
    assert_eq!(symbols[0].name.as_str(), "valid");
}

#[test]
fn test_extract_symbols_with_comments() {
    let uri = Url::parse("file:///test.sv").unwrap();
    let content = "// comment\nmodule a;\nendmodule\n// another comment";
    let symbols = extract_symbols_basic(&uri, content);

    assert_eq!(symbols.len(), 1);
}

#[test]
fn test_extract_symbols_underscores() {
    let uri = Url::parse("file:///test.sv").unwrap();
    let content = "module my_module_name;\nendmodule";
    let symbols = extract_symbols_basic(&uri, content);

    assert_eq!(symbols.len(), 1);
    assert_eq!(symbols[0].name.as_str(), "my_module_name");
}

#[test]
fn test_extract_symbols_position() {
    let uri = Url::parse("file:///test.sv").unwrap();
    let content = "module mod_a;\nendmodule\nmodule mod_b;\nendmodule";
    let symbols = extract_symbols_basic(&uri, content);

    // 验证位置信息
    assert_eq!(symbols[0].location.start.line, 0);
    assert!(symbols[0].location.end.line >= 0);
    assert_eq!(symbols[1].location.start.line, 2);
}

#[test]
fn test_extract_symbols_uri_in_result() {
    let uri = Url::parse("file:///path/to/test.sv").unwrap();
    let content = "module m;\nendmodule";
    let symbols = extract_symbols_basic(&uri, content);

    assert_eq!(symbols[0].location.uri, "file:///path/to/test.sv");
}

// ============================================================
// convert_diagnostics 测试
// ============================================================

#[test]
fn test_convert_diagnostics_empty() {
    let lsp_diags = convert_diagnostics(&[]);
    assert!(lsp_diags.is_empty());
}

#[test]
fn test_convert_diagnostics_error() {
    use babel_lsp_core::diagnostic::Diagnostic;
    use babel_lsp_core::symbol::{Location, Position};

    let loc = Location {
        uri: "file:///test.sv".to_string(),
        start: Position::new(0, 0),
        end: Position::new(0, 10),
    };
    let diags = vec![Diagnostic::error(loc, "test error".to_string())];
    let lsp_diags = convert_diagnostics(&diags);

    assert_eq!(lsp_diags.len(), 1);
    assert_eq!(lsp_diags[0].severity, Some(DiagnosticSeverity::ERROR));
    assert_eq!(lsp_diags[0].message, "test error");
}

#[test]
fn test_convert_diagnostics_warning() {
    use babel_lsp_core::diagnostic::Diagnostic;
    use babel_lsp_core::symbol::{Location, Position};

    let loc = Location {
        uri: "file:///test.sv".to_string(),
        start: Position::new(0, 0),
        end: Position::new(0, 10),
    };
    let diags = vec![Diagnostic::warning(loc, "test warning".to_string())];
    let lsp_diags = convert_diagnostics(&diags);

    assert_eq!(lsp_diags.len(), 1);
    assert_eq!(lsp_diags[0].severity, Some(DiagnosticSeverity::WARNING));
}

#[test]
fn test_convert_diagnostics_information() {
    use babel_lsp_core::diagnostic::Diagnostic;
    use babel_lsp_core::symbol::{Location, Position};

    let loc = Location {
        uri: "file:///test.sv".to_string(),
        start: Position::new(0, 0),
        end: Position::new(0, 10),
    };
    let diags = vec![Diagnostic::new(loc, babel_lsp_core::diagnostic::DiagnosticSeverity::Information, "info".to_string())];
    let lsp_diags = convert_diagnostics(&diags);

    assert_eq!(lsp_diags.len(), 1);
    assert_eq!(lsp_diags[0].severity, Some(DiagnosticSeverity::INFORMATION));
}

#[test]
fn test_convert_diagnostics_hint() {
    use babel_lsp_core::diagnostic::Diagnostic;
    use babel_lsp_core::symbol::{Location, Position};

    let loc = Location {
        uri: "file:///test.sv".to_string(),
        start: Position::new(0, 0),
        end: Position::new(0, 10),
    };
    let diags = vec![Diagnostic::new(loc, babel_lsp_core::diagnostic::DiagnosticSeverity::Hint, "hint".to_string())];
    let lsp_diags = convert_diagnostics(&diags);

    assert_eq!(lsp_diags.len(), 1);
    assert_eq!(lsp_diags[0].severity, Some(DiagnosticSeverity::HINT));
}

#[test]
fn test_convert_diagnostics_with_code() {
    use babel_lsp_core::diagnostic::Diagnostic;
    use babel_lsp_core::symbol::{Location, Position};

    let loc = Location {
        uri: "file:///test.sv".to_string(),
        start: Position::new(0, 0),
        end: Position::new(0, 10),
    };
    let mut diag = Diagnostic::error(loc, "test".to_string());
    diag.code = Some("E001".to_string());
    let lsp_diags = convert_diagnostics(&[diag]);

    assert_eq!(lsp_diags.len(), 1);
    assert!(lsp_diags[0].code.is_some());
}

#[test]
fn test_convert_diagnostics_with_source() {
    use babel_lsp_core::diagnostic::Diagnostic;
    use babel_lsp_core::symbol::{Location, Position};

    let loc = Location {
        uri: "file:///test.sv".to_string(),
        start: Position::new(0, 0),
        end: Position::new(0, 10),
    };
    let diag = Diagnostic {
        source: "babel-lsp".to_string(),
        ..Diagnostic::error(loc, "test".to_string())
    };
    let lsp_diags = convert_diagnostics(&[diag]);

    assert_eq!(lsp_diags.len(), 1);
    assert_eq!(lsp_diags[0].source, Some("babel-lsp".to_string()));
}

#[test]
fn test_convert_diagnostics_range() {
    use babel_lsp_core::diagnostic::Diagnostic;
    use babel_lsp_core::symbol::{Location, Position};

    let loc = Location {
        uri: "file:///test.sv".to_string(),
        start: Position::new(5, 10),
        end: Position::new(8, 20),
    };
    let diag = Diagnostic::error(loc, "test".to_string());
    let lsp_diags = convert_diagnostics(&[diag]);

    assert_eq!(lsp_diags[0].range.start.line, 5);
    assert_eq!(lsp_diags[0].range.start.character, 10);
    assert_eq!(lsp_diags[0].range.end.line, 8);
    assert_eq!(lsp_diags[0].range.end.character, 20);
}

#[test]
fn test_convert_diagnostics_multiple() {
    use babel_lsp_core::diagnostic::Diagnostic;
    use babel_lsp_core::symbol::{Location, Position};

    let loc = Location {
        uri: "file:///test.sv".to_string(),
        start: Position::new(0, 0),
        end: Position::new(0, 10),
    };
    let diags = vec![
        Diagnostic::error(loc.clone(), "error1".to_string()),
        Diagnostic::warning(loc.clone(), "warning1".to_string()),
        Diagnostic::error(loc.clone(), "error2".to_string()),
    ];
    let lsp_diags = convert_diagnostics(&diags);

    assert_eq!(lsp_diags.len(), 3);
}

#[test]
fn test_convert_diagnostics_code_number_or_string() {
    use babel_lsp_core::diagnostic::Diagnostic;
    use babel_lsp_core::symbol::{Location, Position};

    let loc = Location {
        uri: "file:///test.sv".to_string(),
        start: Position::new(0, 0),
        end: Position::new(0, 10),
    };
    let mut diag = Diagnostic::error(loc, "test".to_string());
    diag.code = Some("SYN-V-001".to_string());
    let lsp_diags = convert_diagnostics(&[diag]);

    // 验证 code 被正确转换为 NumberOrString::String
    if let Some(NumberOrString::String(code)) = &lsp_diags[0].code {
        assert_eq!(code, "SYN-V-001");
    } else {
        panic!("Expected String code");
    }
}