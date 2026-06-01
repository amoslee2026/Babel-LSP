//! Core 类型属性测试

use proptest::prelude::*;
use thanosLSP_core::symbol::{Position, Location};
use thanosLSP_core::diagnostic::{Diagnostic, DiagnosticSeverity};

// ============================================================
// Position 属性测试
// ============================================================

proptest! {
    #[test]
    fn test_position_new_valid(line: u32, column: u32) {
        let pos = Position::new(line, column);
        assert_eq!(pos.line, line);
        assert_eq!(pos.column, column);
    }

    #[test]
    fn test_position_equality(line in 0u32..1000, col in 0u32..1000) {
        let p1 = Position::new(line, col);
        let p2 = Position::new(line, col);
        assert_eq!(p1, p2);
    }
}

// ============================================================
// Location 属性测试
// ============================================================

proptest! {
    #[test]
    fn test_location_uri_preserved(uri: String, line in 0u32..100, col in 0u32..100) {
        prop_assume!(!uri.contains('\0'));
        prop_assume!(!uri.is_empty());

        let loc = Location {
            uri: uri.clone(),
            start: Position::new(line, col),
            end: Position::new(line + 1, col),
        };
        assert_eq!(loc.uri, uri);
    }
}

// ============================================================
// Diagnostic 属性测试
// ============================================================

proptest! {
    #[test]
    fn test_diagnostic_severity_roundtrip(severity_idx in 0usize..4) {
        let severity = match severity_idx {
            0 => DiagnosticSeverity::Error,
            1 => DiagnosticSeverity::Warning,
            2 => DiagnosticSeverity::Information,
            _ => DiagnosticSeverity::Hint,
        };

        let loc = Location {
            uri: "file:///test.sv".to_string(),
            start: Position::new(0, 0),
            end: Position::new(1, 0),
        };

        let diag = Diagnostic::new(loc, severity.clone(), "test".to_string());
        assert_eq!(diag.severity, severity);
    }

    #[test]
    fn test_diagnostic_message_preserved(message: String) {
        prop_assume!(!message.is_empty());

        let loc = Location {
            uri: "file:///test.sv".to_string(),
            start: Position::new(0, 0),
            end: Position::new(1, 0),
        };

        let diag = Diagnostic::error(loc, message.clone());
        assert_eq!(diag.message, message);
    }

    #[test]
    fn test_diagnostic_code_preserved(code: String) {
        let loc = Location {
            uri: "file:///test.sv".to_string(),
            start: Position::new(0, 0),
            end: Position::new(1, 0),
        };

        let mut diag = Diagnostic::error(loc, "test".to_string());
        diag.code = Some(code.clone());
        assert_eq!(diag.code, Some(code));
    }
}