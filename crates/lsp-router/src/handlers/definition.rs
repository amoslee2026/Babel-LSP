//! 定义跳转处理器

use babel_lsp_core::symbol::Symbol;
use tower_lsp::lsp_types::*;

/// 将内部 Location 转换为 LSP Location
fn to_lsp_location(loc: &babel_lsp_core::symbol::Location) -> Option<Location> {
    let uri = Url::parse(&loc.uri).ok()?;
    Some(Location {
        uri,
        range: Range {
            start: Position {
                line: loc.start.line,
                character: loc.start.column,
            },
            end: Position {
                line: loc.end.line,
                character: loc.end.column,
            },
        },
    })
}

/// 在给定位置提取标识符（单词）
pub fn word_at_position(content: &str, line: u32, character: u32) -> Option<String> {
    let lines: Vec<&str> = content.lines().collect();
    let line_content = lines.get(line as usize)?;
    let chars: Vec<char> = line_content.chars().collect();
    let col = character as usize;

    if col > chars.len() {
        return None;
    }

    // Find word boundary
    let is_ident = |c: char| c.is_alphanumeric() || c == '_';

    let mut start = col;
    while start > 0 && is_ident(chars[start - 1]) {
        start -= 1;
    }

    let mut end = col;
    while end < chars.len() && is_ident(chars[end]) {
        end += 1;
    }

    if start == end {
        return None;
    }

    Some(chars[start..end].iter().collect())
}

/// 查找符号定义
pub fn handle_definition(symbols: &[Symbol], name: &str) -> Option<Location> {
    symbols
        .iter()
        .find(|s| s.name.as_str() == name)
        .and_then(|s| to_lsp_location(&s.location))
}

/// 查找所有同名定义（多文件）
pub fn handle_all_definitions(symbols: &[Symbol], name: &str) -> Vec<Location> {
    symbols
        .iter()
        .filter(|s| s.name.as_str() == name)
        .filter_map(|s| to_lsp_location(&s.location))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use smol_str::SmolStr;
    use babel_lsp_core::symbol::{Location as CoreLocation, Position as CorePos, SymbolKind};

    fn make_symbol(name: &str, uri: &str, line: u32) -> Symbol {
        Symbol::new(
            SmolStr::from(name),
            SymbolKind::Module,
            CoreLocation {
                uri: uri.to_string(),
                start: CorePos::new(line, 0),
                end: CorePos::new(line + 5, 0),
            },
        )
    }

    #[test]
    fn test_find_definition() {
        let symbols = vec![
            make_symbol("my_module", "file:///a.sv", 10),
            make_symbol("other", "file:///b.sv", 5),
        ];
        let loc = handle_definition(&symbols, "my_module");
        assert!(loc.is_some());
        let loc = loc.unwrap();
        assert_eq!(loc.range.start.line, 10);
    }

    #[test]
    fn test_definition_not_found() {
        let symbols = vec![make_symbol("foo", "file:///a.sv", 0)];
        assert!(handle_definition(&symbols, "bar").is_none());
    }

    #[test]
    fn test_word_at_position() {
        let content = "module my_module (";
        // "my_module" starts at col 7
        assert_eq!(
            word_at_position(content, 0, 10),
            Some("my_module".to_string())
        );
        assert_eq!(
            word_at_position(content, 0, 7),
            Some("my_module".to_string())
        );
    }

    #[test]
    fn test_word_at_position_empty() {
        let content = "module  (";
        // space at col 7
        assert_eq!(word_at_position(content, 0, 7), None);
    }

    #[test]
    fn test_all_definitions() {
        let symbols = vec![
            make_symbol("foo", "file:///a.sv", 1),
            make_symbol("foo", "file:///b.sv", 2),
            make_symbol("bar", "file:///c.sv", 3),
        ];
        let locs = handle_all_definitions(&symbols, "foo");
        assert_eq!(locs.len(), 2);
    }
}
