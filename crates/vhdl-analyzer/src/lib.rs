#![allow(non_snake_case)]
//! VHDL 语言分析器
//!
//! 提供对 VHDL 源文件的解析、符号提取和诊断检查能力。

pub mod diagnostics;
pub mod parser;
pub mod symbol_collector;

use babel_lsp_core::{diagnostic::Diagnostic, symbol::Symbol};

use diagnostics::VhdlDiagnostics;
use parser::VhdlParser;
use symbol_collector::VhdlSymbolCollector;

/// 顶层 VHDL 分析器，组合解析、符号收集和诊断三大模块。
pub struct VhdlAnalyzer {
    pub parser: VhdlParser,
    pub symbol_collector: VhdlSymbolCollector,
    pub diagnostics: VhdlDiagnostics,
}

impl VhdlAnalyzer {
    pub fn new() -> Self {
        Self {
            parser: VhdlParser::new(),
            symbol_collector: VhdlSymbolCollector::new(),
            diagnostics: VhdlDiagnostics::new(),
        }
    }

    /// 一次性分析 VHDL 文本，返回 (符号列表, 诊断列表)
    pub fn analyze(&self, content: &str, file_uri: &str) -> (Vec<Symbol>, Vec<Diagnostic>) {
        let parse_result = self.parser.parse(content);
        let symbols = self.symbol_collector.collect(&parse_result, file_uri);
        let diags = self.diagnostics.check(&parse_result, content, file_uri);
        (symbols, diags)
    }
}

impl Default for VhdlAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// 集成测试
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use babel_lsp_core::symbol::SymbolKind;

    const FILE_URI: &str = "file:///test.vhd";

    const FULL_EXAMPLE: &str = r#"
library ieee;
use ieee.std_logic_1164.all;

entity uart_tx is
    generic (
        BAUD_DIV : integer := 868
    );
    port (
        clk     : in  std_logic;
        rst_n   : in  std_logic;
        tx_data : in  std_logic_vector(7 downto 0);
        tx_en   : in  std_logic;
        tx_out  : out std_logic;
        tx_busy : out std_logic
    );
end entity uart_tx;

architecture rtl of uart_tx is
    signal baud_cnt : integer range 0 to 999;
    signal shift_reg : std_logic_vector(7 downto 0);
begin
    baud_proc : process(clk, rst_n)
    begin
        if rst_n = '0' then
            baud_cnt <= 0;
        elsif rising_edge(clk) then
            baud_cnt <= baud_cnt + 1;
        end if;
    end process;

    shift_reg <= tx_data;
end architecture rtl;
"#;

    #[test]
    fn test_analyze_returns_symbols_and_diagnostics() {
        let analyzer = VhdlAnalyzer::new();
        let (symbols, _diags) = analyzer.analyze(FULL_EXAMPLE, FILE_URI);
        assert!(!symbols.is_empty(), "should return symbols");
        let module_sym = symbols.iter().find(|s| s.kind == SymbolKind::Module);
        assert!(module_sym.is_some(), "should have entity as Module");
        assert_eq!(module_sym.unwrap().name, "uart_tx");
    }

    #[test]
    fn test_analyze_entity_ports_and_generics() {
        let analyzer = VhdlAnalyzer::new();
        let (symbols, _diags) = analyzer.analyze(FULL_EXAMPLE, FILE_URI);
        let entity = symbols
            .iter()
            .find(|s| s.kind == SymbolKind::Module)
            .unwrap();

        let ports: Vec<_> = entity
            .children
            .iter()
            .filter(|s| s.kind == SymbolKind::Port)
            .collect();
        let params: Vec<_> = entity
            .children
            .iter()
            .filter(|s| s.kind == SymbolKind::Parameter)
            .collect();
        assert_eq!(ports.len(), 6, "uart_tx has 6 ports");
        assert_eq!(params.len(), 1, "uart_tx has 1 generic");
    }

    #[test]
    fn test_analyze_architecture_signals() {
        let analyzer = VhdlAnalyzer::new();
        let (symbols, _diags) = analyzer.analyze(FULL_EXAMPLE, FILE_URI);
        let arch = symbols
            .iter()
            .find(|s| s.kind == SymbolKind::Namespace)
            .unwrap();
        let signals: Vec<_> = arch
            .children
            .iter()
            .filter(|s| s.kind == SymbolKind::Variable)
            .collect();
        assert_eq!(signals.len(), 2, "rtl has 2 signals");
    }
}
