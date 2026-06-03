//! 补全处理器

use babel_lsp_core::symbol::{Symbol, SymbolKind};
use tower_lsp::lsp_types::*;

/// 将 SymbolKind 转换为 LSP CompletionItemKind
fn symbol_kind_to_completion_kind(kind: SymbolKind) -> CompletionItemKind {
    match kind {
        SymbolKind::Module => CompletionItemKind::MODULE,
        SymbolKind::Port => CompletionItemKind::FIELD,
        SymbolKind::Signal => CompletionItemKind::VARIABLE,
        SymbolKind::Parameter => CompletionItemKind::CONSTANT,
        SymbolKind::Typedef => CompletionItemKind::CLASS,
        SymbolKind::Macro => CompletionItemKind::SNIPPET,
        SymbolKind::Function => CompletionItemKind::FUNCTION,
        SymbolKind::Task => CompletionItemKind::FUNCTION,
        SymbolKind::Interface => CompletionItemKind::INTERFACE,
        SymbolKind::Package => CompletionItemKind::MODULE,
        SymbolKind::Class => CompletionItemKind::CLASS,
        SymbolKind::Proc => CompletionItemKind::FUNCTION,
        SymbolKind::Variable => CompletionItemKind::VARIABLE,
        SymbolKind::Namespace => CompletionItemKind::MODULE,
        SymbolKind::Cell => CompletionItemKind::CLASS,
    }
}

/// SV 关键字补全列表
const SV_KEYWORDS: &[&str] = &[
    "module",
    "endmodule",
    "input",
    "output",
    "inout",
    "wire",
    "logic",
    "reg",
    "parameter",
    "localparam",
    "always",
    "always_ff",
    "always_comb",
    "always_latch",
    "begin",
    "end",
    "if",
    "else",
    "case",
    "casez",
    "casex",
    "endcase",
    "for",
    "while",
    "assign",
    "initial",
    "function",
    "endfunction",
    "task",
    "endtask",
    "generate",
    "endgenerate",
    "interface",
    "endinterface",
    "package",
    "endpackage",
    "class",
    "endclass",
    "typedef",
    "struct",
    "enum",
    "posedge",
    "negedge",
    "or",
    "and",
    "not",
];

/// 生成补全列表
pub fn handle_completion(
    symbols: &[Symbol],
    prefix: &str,
    include_keywords: bool,
) -> Vec<CompletionItem> {
    let prefix_lower = prefix.to_lowercase();
    let mut items: Vec<CompletionItem> = Vec::new();

    // 从符号表补全
    for sym in symbols {
        let name = sym.name.as_str();
        if name.to_lowercase().starts_with(&prefix_lower) {
            let kind = symbol_kind_to_completion_kind(sym.kind);
            let detail = sym
                .detail
                .clone()
                .or_else(|| Some(format!("{:?}", sym.kind)));
            items.push(CompletionItem {
                label: name.to_string(),
                kind: Some(kind),
                detail,
                documentation: sym.doc_comment.as_ref().map(|d| {
                    Documentation::MarkupContent(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: d.clone(),
                    })
                }),
                ..Default::default()
            });
        }
    }

    // 关键字补全
    if include_keywords {
        for kw in SV_KEYWORDS {
            if kw.starts_with(&prefix_lower) {
                items.push(CompletionItem {
                    label: kw.to_string(),
                    kind: Some(CompletionItemKind::KEYWORD),
                    ..Default::default()
                });
            }
        }
    }

    // 按标签排序
    items.sort_by(|a, b| a.label.cmp(&b.label));
    items
}

#[cfg(test)]
mod tests {
    use super::*;
    use smol_str::SmolStr;
    use babel_lsp_core::symbol::{Location, Position};

    fn make_symbol(name: &str, kind: SymbolKind) -> Symbol {
        Symbol::new(
            SmolStr::from(name),
            kind,
            Location {
                uri: "file:///test.sv".to_string(),
                start: Position::new(0, 0),
                end: Position::new(0, 10),
            },
        )
    }

    #[test]
    fn test_completion_prefix_filter() {
        let symbols = vec![
            make_symbol("my_module", SymbolKind::Module),
            make_symbol("my_signal", SymbolKind::Signal),
            make_symbol("other_signal", SymbolKind::Signal),
        ];
        let items = handle_completion(&symbols, "my_", false);
        assert_eq!(items.len(), 2);
        let labels: Vec<_> = items.iter().map(|i| i.label.as_str()).collect();
        assert!(labels.contains(&"my_module"));
        assert!(labels.contains(&"my_signal"));
    }

    #[test]
    fn test_completion_case_insensitive() {
        let symbols = vec![make_symbol("MyModule", SymbolKind::Module)];
        let items = handle_completion(&symbols, "my", false);
        assert_eq!(items.len(), 1);
    }

    #[test]
    fn test_completion_keywords() {
        let items = handle_completion(&[], "mod", true);
        assert!(items.iter().any(|i| i.label == "module"));
    }

    #[test]
    fn test_completion_empty_prefix() {
        let symbols = vec![
            make_symbol("a", SymbolKind::Signal),
            make_symbol("b", SymbolKind::Signal),
        ];
        let items = handle_completion(&symbols, "", false);
        assert_eq!(items.len(), 2);
    }
}
