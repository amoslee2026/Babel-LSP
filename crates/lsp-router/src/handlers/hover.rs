//! Hover 信息处理器

use thanosLSP_core::symbol::{Symbol, SymbolKind};
use tower_lsp::lsp_types::*;

/// 格式化符号类型名称
fn kind_label(kind: SymbolKind) -> &'static str {
    match kind {
        SymbolKind::Module => "module",
        SymbolKind::Port => "port",
        SymbolKind::Signal => "signal",
        SymbolKind::Parameter => "parameter",
        SymbolKind::Typedef => "typedef",
        SymbolKind::Macro => "macro",
        SymbolKind::Function => "function",
        SymbolKind::Task => "task",
        SymbolKind::Interface => "interface",
        SymbolKind::Package => "package",
        SymbolKind::Class => "class",
        SymbolKind::Proc => "proc",
        SymbolKind::Variable => "variable",
        SymbolKind::Namespace => "namespace",
        SymbolKind::Cell => "cell",
    }
}

/// 生成符号 Hover 内容（Markdown）
pub fn format_hover(symbol: &Symbol) -> String {
    let mut md = format!("**{}** `{}`", kind_label(symbol.kind), symbol.name);

    if let Some(detail) = &symbol.detail {
        md.push_str(&format!("\n\n```systemverilog\n{}\n```", detail));
    }

    if let Some(doc) = &symbol.doc_comment {
        md.push_str(&format!("\n\n---\n{}", doc));
    }

    md
}

/// 处理 hover 请求
pub fn handle_hover(symbols: &[Symbol], name: &str) -> Option<Hover> {
    let symbol = symbols.iter().find(|s| s.name.as_str() == name)?;
    let md = format_hover(symbol);

    Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: md,
        }),
        range: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use smol_str::SmolStr;
    use thanosLSP_core::symbol::{Location, Position};

    fn make_symbol(name: &str, kind: SymbolKind) -> Symbol {
        let mut s = Symbol::new(
            SmolStr::from(name),
            kind,
            Location {
                uri: "file:///test.sv".to_string(),
                start: Position::new(0, 0),
                end: Position::new(10, 0),
            },
        );
        s.detail = Some(format!("logic [7:0] {}", name));
        s.doc_comment = Some("A test signal".to_string());
        s
    }

    #[test]
    fn test_hover_format() {
        let sym = make_symbol("my_sig", SymbolKind::Signal);
        let md = format_hover(&sym);
        assert!(md.contains("**signal**"));
        assert!(md.contains("`my_sig`"));
        assert!(md.contains("logic [7:0]"));
        assert!(md.contains("A test signal"));
    }

    #[test]
    fn test_hover_module() {
        let sym = make_symbol("my_mod", SymbolKind::Module);
        let md = format_hover(&sym);
        assert!(md.contains("**module**"));
    }

    #[test]
    fn test_handle_hover_found() {
        let symbols = vec![make_symbol("my_sig", SymbolKind::Signal)];
        let hover = handle_hover(&symbols, "my_sig");
        assert!(hover.is_some());
    }

    #[test]
    fn test_handle_hover_not_found() {
        let symbols: Vec<Symbol> = vec![];
        let hover = handle_hover(&symbols, "nonexistent");
        assert!(hover.is_none());
    }
}
