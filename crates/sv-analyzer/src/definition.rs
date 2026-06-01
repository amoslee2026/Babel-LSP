//! 跳转到定义引擎

use thanosLSP_core::symbol::{Location, Position, Symbol};

/// 定义跳转引擎
pub struct DefinitionEngine;

impl DefinitionEngine {
    pub fn new() -> Self {
        Self
    }

    /// 查找第一个匹配名称的符号定义位置
    pub fn find_definition(&self, symbols: &[Symbol], name: &str) -> Option<Location> {
        symbols
            .iter()
            .find(|s| s.name == name)
            .map(|s| s.location.clone())
    }

    /// 查找所有匹配名称的符号定义位置（处理重载等情况）
    pub fn find_all_definitions(&self, symbols: &[Symbol], name: &str) -> Vec<Location> {
        symbols
            .iter()
            .filter(|s| s.name == name)
            .map(|s| s.location.clone())
            .collect()
    }

    /// 从文本内容和光标位置提取当前单词（标识符）
    ///
    /// `pos.line` 和 `pos.column` 均为 0-based。
    /// 光标必须位于标识符字符上才返回单词，位于空格/符号上返回 None。
    pub fn word_at_position(&self, content: &str, pos: Position) -> Option<String> {
        let line_text = content.lines().nth(pos.line as usize)?;
        let col = pos.column as usize;

        // 边界检查
        if col >= line_text.len() {
            return None;
        }

        let bytes = line_text.as_bytes();

        // 光标位置必须是标识符字符
        if !is_ident(bytes[col]) {
            return None;
        }

        // 向左找单词起始位置
        let mut start = col;
        while start > 0 && is_ident(bytes[start - 1]) {
            start -= 1;
        }

        // 向右找单词结束位置
        let mut end = col;
        while end < bytes.len() && is_ident(bytes[end]) {
            end += 1;
        }

        Some(line_text[start..end].to_string())
    }
}

impl Default for DefinitionEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// 标识符字符判断（字母、数字、下划线、$）
fn is_ident(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_' || b == b'$'
}

#[cfg(test)]
mod tests {
    use super::*;
    use smol_str::SmolStr;
    use thanosLSP_core::symbol::SymbolKind;

    fn make_sym(name: &str, line: u32) -> Symbol {
        Symbol::new(
            SmolStr::new(name),
            SymbolKind::Module,
            Location {
                uri: "file:///test.sv".to_string(),
                start: Position::new(line, 0),
                end: Position::new(line, name.len() as u32),
            },
        )
    }

    #[test]
    fn test_find_definition_by_name() {
        let engine = DefinitionEngine::new();
        let symbols = vec![make_sym("top_module", 1), make_sym("sub_module", 10)];
        let loc = engine.find_definition(&symbols, "sub_module");
        assert!(loc.is_some());
        assert_eq!(loc.unwrap().start.line, 10);
    }

    #[test]
    fn test_find_definition_not_found() {
        let engine = DefinitionEngine::new();
        let symbols = vec![make_sym("foo", 1)];
        assert!(engine.find_definition(&symbols, "bar").is_none());
    }

    #[test]
    fn test_find_all_definitions() {
        let engine = DefinitionEngine::new();
        let symbols = vec![
            make_sym("clk", 1),
            make_sym("clk", 5), // 多次定义（如在不同文件中）
            make_sym("rst", 2),
        ];
        let locs = engine.find_all_definitions(&symbols, "clk");
        assert_eq!(locs.len(), 2);
    }

    #[test]
    fn test_word_at_position_middle() {
        let engine = DefinitionEngine::new();
        let content = "  wire my_signal;\n";
        // 光标在 my_signal 的中间（列 8 在 'i' 上）
        let word = engine.word_at_position(content, Position::new(0, 8));
        assert_eq!(word.as_deref(), Some("my_signal"));
    }

    #[test]
    fn test_word_at_position_start() {
        let engine = DefinitionEngine::new();
        let content = "module top();\n";
        // 光标在 "top" 的起始位置（列 7）
        let word = engine.word_at_position(content, Position::new(0, 7));
        assert_eq!(word.as_deref(), Some("top"));
    }

    #[test]
    fn test_word_at_position_on_space() {
        let engine = DefinitionEngine::new();
        let content = "  wire  signal;\n";
        // 光标在空格上（列 6）
        let word = engine.word_at_position(content, Position::new(0, 6));
        assert_eq!(word, None);
    }

    #[test]
    fn test_word_at_position_system_task() {
        let engine = DefinitionEngine::new();
        let content = "  $display(\"hi\");\n";
        // 光标在 $display 内（列 3）
        let word = engine.word_at_position(content, Position::new(0, 3));
        assert_eq!(word.as_deref(), Some("$display"));
    }

    #[test]
    fn test_word_at_position_out_of_bounds() {
        let engine = DefinitionEngine::new();
        let content = "abc\n";
        let word = engine.word_at_position(content, Position::new(5, 0)); // 超出行数
        assert!(word.is_none());
    }
}
