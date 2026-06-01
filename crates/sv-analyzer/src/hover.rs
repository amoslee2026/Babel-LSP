//! Hover 信息生成引擎

use thanosLSP_core::symbol::{Symbol, SymbolKind};

/// Hover 引擎：为符号生成 Markdown 格式的悬停说明
pub struct HoverEngine;

impl HoverEngine {
    pub fn new() -> Self {
        Self
    }

    /// 为符号生成 Markdown 格式的 Hover 文本
    ///
    /// 格式示例：
    /// ```markdown
    /// **module** `top`
    ///
    /// *Port declaration in module top*
    ///
    /// Declared at: file:///top.sv:1:0
    /// ```
    pub fn get_hover(&self, symbol: &Symbol) -> Option<String> {
        let kind_str = kind_display(symbol.kind);
        let mut parts = Vec::new();

        // 标题行：**kind** `name`
        parts.push(format!("**{}** `{}`", kind_str, symbol.name));

        // 详细信息
        if let Some(ref detail) = symbol.detail {
            parts.push(String::new()); // 空行
            parts.push(detail.clone());
        }

        // 文档注释
        if let Some(ref doc) = symbol.doc_comment {
            parts.push(String::new());
            parts.push(doc.clone());
        }

        // 声明位置
        parts.push(String::new());
        parts.push(format!(
            "Declared at: `{}:{}:{}`",
            symbol.location.uri,
            symbol.location.start.line + 1, // 转为 1-based 显示
            symbol.location.start.column + 1,
        ));

        Some(parts.join("\n"))
    }

    /// 为多个符号生成 Hover（取第一个）
    pub fn get_hover_for_name(&self, symbols: &[Symbol], name: &str) -> Option<String> {
        symbols
            .iter()
            .find(|s| s.name == name)
            .and_then(|s| self.get_hover(s))
    }
}

impl Default for HoverEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// 符号类型的可读名称
fn kind_display(kind: SymbolKind) -> &'static str {
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

#[cfg(test)]
mod tests {
    use super::*;
    use smol_str::SmolStr;
    use thanosLSP_core::symbol::{Location, Position, Symbol};

    fn make_sym(name: &str, kind: SymbolKind) -> Symbol {
        Symbol::new(
            SmolStr::new(name),
            kind,
            Location {
                uri: "file:///design.sv".to_string(),
                start: Position::new(4, 0), // 行 4（0-based）= 第 5 行
                end: Position::new(20, 10),
            },
        )
    }

    #[test]
    fn test_module_hover() {
        let engine = HoverEngine::new();
        let sym = make_sym("top", SymbolKind::Module);
        let hover = engine.get_hover(&sym).unwrap();
        assert!(hover.contains("**module**"), "应包含 **module**");
        assert!(hover.contains("`top`"), "应包含模块名");
        assert!(hover.contains("design.sv"), "应包含文件路径");
        assert!(hover.contains("5:1"), "应包含 1-based 行列号");
    }

    #[test]
    fn test_port_hover() {
        let engine = HoverEngine::new();
        let mut sym = make_sym("clk", SymbolKind::Port);
        sym.detail = Some("input logic clk".to_string());
        let hover = engine.get_hover(&sym).unwrap();
        assert!(hover.contains("**port**"));
        assert!(hover.contains("`clk`"));
        assert!(hover.contains("input logic clk"));
    }

    #[test]
    fn test_parameter_hover() {
        let engine = HoverEngine::new();
        let mut sym = make_sym("WIDTH", SymbolKind::Parameter);
        sym.detail = Some("parameter integer WIDTH = 8".to_string());
        let hover = engine.get_hover(&sym).unwrap();
        assert!(hover.contains("**parameter**"));
        assert!(hover.contains("WIDTH"));
    }

    #[test]
    fn test_hover_with_doc_comment() {
        let engine = HoverEngine::new();
        let mut sym = make_sym("my_func", SymbolKind::Function);
        sym.doc_comment = Some("/// Computes the checksum".to_string());
        let hover = engine.get_hover(&sym).unwrap();
        assert!(hover.contains("Computes the checksum"));
    }

    #[test]
    fn test_hover_for_name_found() {
        let engine = HoverEngine::new();
        let symbols = vec![
            make_sym("foo", SymbolKind::Module),
            make_sym("bar", SymbolKind::Signal),
        ];
        let hover = engine.get_hover_for_name(&symbols, "bar");
        assert!(hover.is_some());
        assert!(hover.unwrap().contains("`bar`"));
    }

    #[test]
    fn test_hover_for_name_not_found() {
        let engine = HoverEngine::new();
        let symbols = vec![make_sym("foo", SymbolKind::Module)];
        assert!(engine.get_hover_for_name(&symbols, "nonexistent").is_none());
    }

    #[test]
    fn test_all_symbol_kinds_display() {
        // 确保所有 SymbolKind 都有对应的显示文本（不 panic）
        let kinds = [
            SymbolKind::Module,
            SymbolKind::Port,
            SymbolKind::Signal,
            SymbolKind::Parameter,
            SymbolKind::Typedef,
            SymbolKind::Macro,
            SymbolKind::Function,
            SymbolKind::Task,
            SymbolKind::Interface,
            SymbolKind::Package,
            SymbolKind::Class,
            SymbolKind::Proc,
            SymbolKind::Variable,
            SymbolKind::Namespace,
            SymbolKind::Cell,
        ];
        let engine = HoverEngine::new();
        for kind in kinds {
            let sym = make_sym("test", kind);
            let hover = engine.get_hover(&sym);
            assert!(hover.is_some(), "{:?} 应生成 hover", kind);
        }
    }
}
