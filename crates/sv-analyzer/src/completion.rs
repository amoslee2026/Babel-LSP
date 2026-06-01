//! 补全引擎：基于符号表和 SV 关键字提供代码补全

use thanosLSP_core::symbol::{Position, Symbol, SymbolKind};

/// 补全项类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionItemKind {
    Module,
    Port,
    Parameter,
    Variable,
    Function,
    Task,
    Interface,
    Keyword,
}

impl CompletionItemKind {
    fn from_symbol_kind(kind: SymbolKind) -> Self {
        match kind {
            SymbolKind::Module => Self::Module,
            SymbolKind::Port => Self::Port,
            SymbolKind::Parameter => Self::Parameter,
            SymbolKind::Signal => Self::Variable,
            SymbolKind::Function => Self::Function,
            SymbolKind::Task => Self::Task,
            SymbolKind::Interface => Self::Interface,
            _ => Self::Variable,
        }
    }
}

/// 补全项
#[derive(Debug, Clone)]
pub struct CompletionItem {
    pub label: String,
    pub kind: CompletionItemKind,
    pub detail: Option<String>,
}

/// SystemVerilog 常用关键字
const SV_KEYWORDS: &[&str] = &[
    "module",
    "endmodule",
    "interface",
    "endinterface",
    "package",
    "endpackage",
    "class",
    "endclass",
    "function",
    "endfunction",
    "task",
    "endtask",
    "always",
    "always_ff",
    "always_comb",
    "always_latch",
    "initial",
    "begin",
    "end",
    "if",
    "else",
    "case",
    "casez",
    "casex",
    "endcase",
    "for",
    "foreach",
    "while",
    "forever",
    "repeat",
    "do",
    "input",
    "output",
    "inout",
    "ref",
    "logic",
    "wire",
    "reg",
    "bit",
    "int",
    "integer",
    "byte",
    "parameter",
    "localparam",
    "typedef",
    "enum",
    "struct",
    "union",
    "assign",
    "generate",
    "endgenerate",
    "genvar",
    "posedge",
    "negedge",
    "or",
    "and",
    "not",
    "signed",
    "unsigned",
    "automatic",
    "virtual",
    "static",
    "fork",
    "join",
    "join_any",
    "join_none",
    "import",
    "export",
    "extends",
    "implements",
    "assert",
    "assume",
    "cover",
    "property",
    "sequence",
    "$display",
    "$monitor",
    "$finish",
    "$stop",
    "$time",
    "$realtime",
    "$random",
    "$urandom",
    "$signed",
    "$unsigned",
    "$cast",
    "$bits",
    "$size",
    "$high",
    "$low",
    "$left",
    "$right",
    "$clog2",
    "$floor",
    "$ceil",
    "$pow",
    "disable",
];

/// 补全引擎
pub struct CompletionEngine;

impl CompletionEngine {
    pub fn new() -> Self {
        Self
    }

    /// 根��前缀过滤符号和关键字，返回补全列表（不区分大小写）
    pub fn complete(
        &self,
        symbols: &[Symbol],
        prefix: &str,
        _position: Position,
    ) -> Vec<CompletionItem> {
        let prefix_lower = prefix.to_ascii_lowercase();
        let mut items: Vec<CompletionItem> = Vec::new();
        let mut seen = std::collections::HashSet::new();

        // 符号补全
        for sym in symbols {
            let name_lower = sym.name.to_ascii_lowercase();
            if name_lower.starts_with(&prefix_lower) && seen.insert(sym.name.to_string()) {
                let detail = sym
                    .detail
                    .clone()
                    .or_else(|| Some(format!("{:?}", sym.kind)));
                items.push(CompletionItem {
                    label: sym.name.to_string(),
                    kind: CompletionItemKind::from_symbol_kind(sym.kind),
                    detail,
                });
            }
        }

        // 关键字补全
        for kw in SV_KEYWORDS {
            if kw.to_ascii_lowercase().starts_with(&prefix_lower) && seen.insert(kw.to_string()) {
                items.push(CompletionItem {
                    label: kw.to_string(),
                    kind: CompletionItemKind::Keyword,
                    detail: Some("keyword".to_string()),
                });
            }
        }

        // 按 label 排序，关键字排在符号后面
        items.sort_by(|a, b| {
            let a_kw = a.kind == CompletionItemKind::Keyword;
            let b_kw = b.kind == CompletionItemKind::Keyword;
            match (a_kw, b_kw) {
                (false, true) => std::cmp::Ordering::Less,
                (true, false) => std::cmp::Ordering::Greater,
                _ => a.label.cmp(&b.label),
            }
        });

        items
    }
}

impl Default for CompletionEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smol_str::SmolStr;
    use thanosLSP_core::symbol::{Location, Symbol};

    fn make_symbol(name: &str, kind: SymbolKind) -> Symbol {
        Symbol::new(
            SmolStr::new(name),
            kind,
            Location {
                uri: "file:///test.sv".to_string(),
                start: Position::new(0, 0),
                end: Position::new(0, 0),
            },
        )
    }

    #[test]
    fn test_prefix_filtering() {
        let engine = CompletionEngine::new();
        let symbols = vec![
            make_symbol("clk_in", SymbolKind::Port),
            make_symbol("clk_out", SymbolKind::Port),
            make_symbol("data_bus", SymbolKind::Signal),
            make_symbol("my_module", SymbolKind::Module),
        ];
        let pos = Position::new(0, 0);
        let items = engine.complete(&symbols, "clk", pos);
        let labels: Vec<&str> = items.iter().map(|i| i.label.as_str()).collect();
        assert!(labels.contains(&"clk_in"), "clk_in 未出现");
        assert!(labels.contains(&"clk_out"), "clk_out 未出现");
        assert!(!labels.contains(&"data_bus"), "data_bus 不应出现");
    }

    #[test]
    fn test_case_insensitive() {
        let engine = CompletionEngine::new();
        let symbols = vec![
            make_symbol("DataBus", SymbolKind::Signal),
            make_symbol("DATAOUT", SymbolKind::Port),
        ];
        let pos = Position::new(0, 0);
        let items = engine.complete(&symbols, "data", pos);
        let labels: Vec<&str> = items.iter().map(|i| i.label.as_str()).collect();
        assert!(
            labels.contains(&"DataBus"),
            "DataBus 未出现（大小写不敏感）"
        );
        assert!(labels.contains(&"DATAOUT"), "DATAOUT 未出现");
    }

    #[test]
    fn test_keywords_included() {
        let engine = CompletionEngine::new();
        let pos = Position::new(0, 0);
        let items = engine.complete(&[], "always", pos);
        let labels: Vec<&str> = items.iter().map(|i| i.label.as_str()).collect();
        assert!(labels.contains(&"always"), "always 未出现");
        assert!(labels.contains(&"always_ff"), "always_ff 未出现");
        assert!(labels.contains(&"always_comb"), "always_comb 未出现");
    }

    #[test]
    fn test_empty_prefix_returns_all() {
        let engine = CompletionEngine::new();
        let symbols = vec![
            make_symbol("foo", SymbolKind::Module),
            make_symbol("bar", SymbolKind::Signal),
        ];
        let pos = Position::new(0, 0);
        let items = engine.complete(&symbols, "", pos);
        // 应该包含所有符号和所有关键字
        assert!(items.len() >= symbols.len() + SV_KEYWORDS.len());
    }

    #[test]
    fn test_symbols_before_keywords() {
        let engine = CompletionEngine::new();
        let symbols = vec![make_symbol("always_custom", SymbolKind::Signal)];
        let pos = Position::new(0, 0);
        let items = engine.complete(&symbols, "always", pos);
        // 符号应排在关键字之前
        let first = &items[0];
        assert_eq!(
            first.kind,
            CompletionItemKind::Variable,
            "符号应排在关键字前"
        );
    }

    #[test]
    fn test_no_duplicate_items() {
        let engine = CompletionEngine::new();
        let symbols = vec![
            make_symbol("my_sig", SymbolKind::Signal),
            make_symbol("my_sig", SymbolKind::Signal), // 重复
        ];
        let pos = Position::new(0, 0);
        let items = engine.complete(&symbols, "my", pos);
        let count = items.iter().filter(|i| i.label == "my_sig").count();
        assert_eq!(count, 1, "重复的 my_sig 应只出现一次");
    }
}
