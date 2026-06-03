//! VHDL 诊断生成器
//!
//! 基于文本的结构性诊断，不依赖外部工具。

use babel_lsp_core::{
    diagnostic::Diagnostic,
    symbol::{Location, Position},
};

use crate::parser::VhdlParseResult;

pub struct VhdlDiagnostics;

impl VhdlDiagnostics {
    pub fn new() -> Self {
        Self
    }

    /// 对解析结果和原始文本进行静态诊断检查
    pub fn check(
        &self,
        parse_result: &VhdlParseResult,
        content: &str,
        file_uri: &str,
    ) -> Vec<Diagnostic> {
        let mut diags = Vec::new();

        self.check_entity_end_name(parse_result, file_uri, &mut diags);
        self.check_empty_sensitivity_list(parse_result, file_uri, &mut diags);
        self.check_unused_signals(parse_result, content, file_uri, &mut diags);
        self.check_unneeded_use_clauses(parse_result, file_uri, &mut diags);

        diags
    }

    // ── VHD-E-001: entity end 名称不一致 ────────��────────────────────────────

    fn check_entity_end_name(
        &self,
        parse_result: &VhdlParseResult,
        file_uri: &str,
        diags: &mut Vec<Diagnostic>,
    ) {
        for entity in &parse_result.entities {
            if let Some(end_name) = &entity.end_name {
                // end_name 已经是小写（来自 parser 的 lowercase 处理）
                if !end_name.is_empty() && *end_name != entity.name.to_lowercase() {
                    let loc = Location {
                        uri: file_uri.to_string(),
                        start: Position::new(entity.end_line, 0),
                        end: Position::new(entity.end_line, 0),
                    };
                    diags.push(
                        Diagnostic::error(
                            loc,
                            format!(
                                "VHD-E-001: entity name mismatch: declared as '{}', but 'end {}' found",
                                entity.name, end_name
                            ),
                        )
                        .with_code("VHD-E-001".to_string())
                        .with_source("babel-lsp-vhdl".to_string()),
                    );
                }
            }
        }
    }

    // ── VHD-W-002: 空敏感列表 ─────────────────────────────────────────────────

    fn check_empty_sensitivity_list(
        &self,
        parse_result: &VhdlParseResult,
        file_uri: &str,
        diags: &mut Vec<Diagnostic>,
    ) {
        for arch in &parse_result.architectures {
            for proc in &arch.processes {
                if proc.sensitivity_list.is_empty() {
                    let loc = Location {
                        uri: file_uri.to_string(),
                        start: Position::new(proc.start_line, 0),
                        end: Position::new(proc.start_line, 0),
                    };
                    let label_info = proc
                        .label
                        .as_ref()
                        .map(|l| format!(" '{}'", l))
                        .unwrap_or_default();
                    diags.push(
                        Diagnostic::warning(
                            loc,
                            format!(
                                "VHD-W-002: process{} has empty sensitivity list",
                                label_info
                            ),
                        )
                        .with_code("VHD-W-002".to_string())
                        .with_source("babel-lsp-vhdl".to_string()),
                    );
                }
            }
        }
    }

    // ── VHD-W-001: 声明的信号未被赋值 ────────────────────────────────────────

    fn check_unused_signals(
        &self,
        parse_result: &VhdlParseResult,
        content: &str,
        file_uri: &str,
        diags: &mut Vec<Diagnostic>,
    ) {
        let lower_content = content.to_lowercase();

        for arch in &parse_result.architectures {
            for signal in &arch.signals {
                let sig_lower = signal.name.to_lowercase();
                // 检查信号名 + " <=" 是否出现（赋值）
                let assign_pattern = format!("{} <=", sig_lower);
                let assign_compact = format!("{}<=", sig_lower);
                if !lower_content.contains(&assign_pattern)
                    && !lower_content.contains(&assign_compact)
                {
                    let loc = Location {
                        uri: file_uri.to_string(),
                        start: Position::new(signal.line, 0),
                        end: Position::new(signal.line, 0),
                    };
                    diags.push(
                        Diagnostic::warning(
                            loc,
                            format!(
                                "VHD-W-001: signal '{}' is declared but never assigned",
                                signal.name
                            ),
                        )
                        .with_code("VHD-W-001".to_string())
                        .with_source("babel-lsp-vhdl".to_string()),
                    );
                }
            }
        }
    }

    // ── VHD-W-003: 不常用库（ieee.std_logic_misc）────────────────────────────

    fn check_unneeded_use_clauses(
        &self,
        parse_result: &VhdlParseResult,
        file_uri: &str,
        diags: &mut Vec<Diagnostic>,
    ) {
        for clause in &parse_result.use_clauses {
            if clause.contains("std_logic_misc") {
                let loc = Location {
                    uri: file_uri.to_string(),
                    start: Position::new(0, 0),
                    end: Position::new(0, 0),
                };
                diags.push(
                    Diagnostic::warning(
                        loc,
                        format!(
                            "VHD-W-003: '{}' references ieee.std_logic_misc which is rarely needed and non-standard",
                            clause
                        ),
                    )
                    .with_code("VHD-W-003".to_string())
                    .with_source("babel-lsp-vhdl".to_string()),
                );
            }
        }
    }
}

impl Default for VhdlDiagnostics {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// 测试
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::VhdlParser;
    use babel_lsp_core::diagnostic::DiagnosticSeverity;

    const FILE_URI: &str = "file:///test.vhd";

    #[test]
    fn test_no_diagnostics_clean_code() {
        let vhdl = r#"
library ieee;
use ieee.std_logic_1164.all;

entity clean is
    port (
        clk : in  std_logic;
        q   : out std_logic
    );
end entity clean;

architecture rtl of clean is
    signal data : std_logic;
begin
    data <= clk;
    q <= data;
end architecture rtl;
"#;
        let parser = VhdlParser::new();
        let diags_checker = VhdlDiagnostics::new();
        let result = parser.parse(vhdl);
        let diags = diags_checker.check(&result, vhdl, FILE_URI);

        // 不应有任何 Error 诊断
        let errors: Vec<_> = diags
            .iter()
            .filter(|d| d.severity == DiagnosticSeverity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "clean code should have no errors: {:?}",
            errors
        );
    }

    #[test]
    fn test_entity_name_mismatch() {
        let vhdl = r#"
entity foo is
    port (
        clk : in std_logic
    );
end entity bar;
"#;
        let parser = VhdlParser::new();
        let diags_checker = VhdlDiagnostics::new();
        let result = parser.parse(vhdl);
        let diags = diags_checker.check(&result, vhdl, FILE_URI);

        let mismatch: Vec<_> = diags
            .iter()
            .filter(|d| d.code.as_deref() == Some("VHD-E-001"))
            .collect();
        assert_eq!(mismatch.len(), 1, "should detect entity name mismatch");
        assert!(mismatch[0].message.contains("foo"));
        assert!(mismatch[0].message.contains("bar"));
    }

    #[test]
    fn test_empty_sensitivity_list() {
        let vhdl = r#"
architecture rtl of x is
begin
    process
    begin
        null;
    end process;
end architecture rtl;
"#;
        let parser = VhdlParser::new();
        let diags_checker = VhdlDiagnostics::new();
        let result = parser.parse(vhdl);
        let diags = diags_checker.check(&result, vhdl, FILE_URI);

        let w002: Vec<_> = diags
            .iter()
            .filter(|d| d.code.as_deref() == Some("VHD-W-002"))
            .collect();
        assert_eq!(w002.len(), 1, "should warn about empty sensitivity list");
    }

    #[test]
    fn test_unused_signal_warning() {
        let vhdl = r#"
architecture rtl of y is
    signal unused_sig : std_logic;
    signal used_sig   : std_logic;
begin
    used_sig <= '1';
end architecture rtl;
"#;
        let parser = VhdlParser::new();
        let diags_checker = VhdlDiagnostics::new();
        let result = parser.parse(vhdl);
        let diags = diags_checker.check(&result, vhdl, FILE_URI);

        let w001: Vec<_> = diags
            .iter()
            .filter(|d| d.code.as_deref() == Some("VHD-W-001"))
            .collect();
        assert_eq!(w001.len(), 1, "should warn about 1 unused signal");
        assert!(w001[0].message.contains("unused_sig"));
    }

    #[test]
    fn test_unneeded_library_warning() {
        let vhdl = r#"
library ieee;
use ieee.std_logic_1164.all;
use ieee.std_logic_misc.all;

entity e is end entity e;
"#;
        let parser = VhdlParser::new();
        let diags_checker = VhdlDiagnostics::new();
        let result = parser.parse(vhdl);
        let diags = diags_checker.check(&result, vhdl, FILE_URI);

        let w003: Vec<_> = diags
            .iter()
            .filter(|d| d.code.as_deref() == Some("VHD-W-003"))
            .collect();
        assert_eq!(w003.len(), 1, "should warn about std_logic_misc");
    }
}
