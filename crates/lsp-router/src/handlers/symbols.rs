//! 文档符号处理器

use thanosLSP_core::symbol::{Symbol, SymbolKind};
use tower_lsp::lsp_types::*;

/// 将内部 SymbolKind 映射为 LSP SymbolKind
fn to_lsp_symbol_kind(kind: SymbolKind) -> tower_lsp::lsp_types::SymbolKind {
    match kind {
        SymbolKind::Module => tower_lsp::lsp_types::SymbolKind::MODULE,
        SymbolKind::Port => tower_lsp::lsp_types::SymbolKind::FIELD,
        SymbolKind::Signal => tower_lsp::lsp_types::SymbolKind::VARIABLE,
        SymbolKind::Parameter => tower_lsp::lsp_types::SymbolKind::CONSTANT,
        SymbolKind::Typedef => tower_lsp::lsp_types::SymbolKind::CLASS,
        SymbolKind::Macro => tower_lsp::lsp_types::SymbolKind::VARIABLE,
        SymbolKind::Function => tower_lsp::lsp_types::SymbolKind::FUNCTION,
        SymbolKind::Task => tower_lsp::lsp_types::SymbolKind::FUNCTION,
        SymbolKind::Interface => tower_lsp::lsp_types::SymbolKind::INTERFACE,
        SymbolKind::Package => tower_lsp::lsp_types::SymbolKind::PACKAGE,
        SymbolKind::Class => tower_lsp::lsp_types::SymbolKind::CLASS,
        SymbolKind::Proc => tower_lsp::lsp_types::SymbolKind::FUNCTION,
        SymbolKind::Variable => tower_lsp::lsp_types::SymbolKind::VARIABLE,
        SymbolKind::Namespace => tower_lsp::lsp_types::SymbolKind::NAMESPACE,
        SymbolKind::Cell => tower_lsp::lsp_types::SymbolKind::CLASS,
    }
}

/// 将内部 Symbol 转换为 LSP DocumentSymbol
fn to_document_symbol(sym: &Symbol) -> DocumentSymbol {
    let range = Range {
        start: Position {
            line: sym.location.start.line,
            character: sym.location.start.column,
        },
        end: Position {
            line: sym.location.end.line,
            character: sym.location.end.column,
        },
    };
    let children: Vec<DocumentSymbol> = sym.children.iter().map(to_document_symbol).collect();

    #[allow(deprecated)]
    DocumentSymbol {
        name: sym.name.to_string(),
        detail: sym.detail.clone(),
        kind: to_lsp_symbol_kind(sym.kind),
        range,
        selection_range: range,
        children: if children.is_empty() {
            None
        } else {
            Some(children)
        },
        tags: None,
        deprecated: None,
    }
}

/// 处理 documentSymbol 请求
pub fn handle_document_symbols(symbols: &[Symbol]) -> Vec<DocumentSymbol> {
    symbols.iter().map(to_document_symbol).collect()
}

/// 将内部 Symbol 转换为 LSP WorkspaceSymbol
pub fn to_workspace_symbol(sym: &Symbol) -> WorkspaceSymbol {
    let uri =
        Url::parse(&sym.location.uri).unwrap_or_else(|_| Url::parse("file:///unknown").unwrap());
    WorkspaceSymbol {
        name: sym.name.to_string(),
        kind: to_lsp_symbol_kind(sym.kind),
        tags: None,
        container_name: None,
        location: OneOf::Left(Location {
            uri,
            range: Range {
                start: Position {
                    line: sym.location.start.line,
                    character: sym.location.start.column,
                },
                end: Position {
                    line: sym.location.end.line,
                    character: sym.location.end.column,
                },
            },
        }),
        data: None,
    }
}

/// 处理 workspaceSymbol 请求（按查询过滤）
pub fn handle_workspace_symbols(all_symbols: &[Symbol], query: &str) -> Vec<WorkspaceSymbol> {
    let query_lower = query.to_lowercase();
    all_symbols
        .iter()
        .filter(|s| query.is_empty() || s.name.to_lowercase().contains(&query_lower))
        .map(to_workspace_symbol)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use smol_str::SmolStr;
    use thanosLSP_core::symbol::{Location, Position};

    fn make_symbol(name: &str, kind: SymbolKind) -> Symbol {
        Symbol::new(
            SmolStr::from(name),
            kind,
            Location {
                uri: "file:///test.sv".to_string(),
                start: Position::new(0, 0),
                end: Position::new(10, 0),
            },
        )
    }

    #[test]
    fn test_document_symbols() {
        let symbols = vec![
            make_symbol("my_module", SymbolKind::Module),
            make_symbol("clk", SymbolKind::Port),
        ];
        let doc_syms = handle_document_symbols(&symbols);
        assert_eq!(doc_syms.len(), 2);
        assert_eq!(doc_syms[0].name, "my_module");
        assert_eq!(doc_syms[0].kind, tower_lsp::lsp_types::SymbolKind::MODULE);
    }

    #[test]
    fn test_workspace_symbols_filter() {
        let symbols = vec![
            make_symbol("alu_top", SymbolKind::Module),
            make_symbol("fifo", SymbolKind::Module),
            make_symbol("alu_ctrl", SymbolKind::Module),
        ];
        let ws_syms = handle_workspace_symbols(&symbols, "alu");
        assert_eq!(ws_syms.len(), 2);
    }

    #[test]
    fn test_workspace_symbols_empty_query() {
        let symbols = vec![
            make_symbol("a", SymbolKind::Signal),
            make_symbol("b", SymbolKind::Signal),
        ];
        let ws_syms = handle_workspace_symbols(&symbols, "");
        assert_eq!(ws_syms.len(), 2);
    }
}
