//! 引用查找处理器

use thanosLSP_core::symbol::Symbol;
use tower_lsp::lsp_types::*;

/// 在文本中查找标识符的所有出现位置
pub fn find_references_in_content(content: &str, uri: &str, name: &str) -> Vec<Location> {
    let mut locations = Vec::new();
    let uri = match Url::parse(uri) {
        Ok(u) => u,
        Err(_) => return locations,
    };

    for (line_idx, line) in content.lines().enumerate() {
        let mut start = 0;
        while let Some(pos) = line[start..].find(name) {
            let abs_pos = start + pos;
            let is_ident = |c: char| c.is_alphanumeric() || c == '_';
            let before_ok =
                abs_pos == 0 || !line.chars().nth(abs_pos - 1).map(is_ident).unwrap_or(false);
            let after_ok = abs_pos + name.len() >= line.len()
                || !line
                    .chars()
                    .nth(abs_pos + name.len())
                    .map(is_ident)
                    .unwrap_or(false);

            if before_ok && after_ok {
                locations.push(Location {
                    uri: uri.clone(),
                    range: Range {
                        start: Position {
                            line: line_idx as u32,
                            character: abs_pos as u32,
                        },
                        end: Position {
                            line: line_idx as u32,
                            character: (abs_pos + name.len()) as u32,
                        },
                    },
                });
            }
            start = abs_pos + 1;
        }
    }

    locations
}

/// 查找符号引用（从符号表中查找同名符号）
pub fn handle_references(symbols: &[Symbol], name: &str) -> Vec<Location> {
    symbols
        .iter()
        .filter(|s| s.name.as_str() == name)
        .filter_map(|s| {
            let uri = Url::parse(&s.location.uri).ok()?;
            Some(Location {
                uri,
                range: Range {
                    start: Position {
                        line: s.location.start.line,
                        character: s.location.start.column,
                    },
                    end: Position {
                        line: s.location.end.line,
                        character: s.location.end.column,
                    },
                },
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_references_in_content() {
        let content = "assign foo = bar;\nlogic foo;";
        let refs = find_references_in_content(content, "file:///test.sv", "foo");
        assert_eq!(refs.len(), 2);
        assert_eq!(refs[0].range.start.line, 0);
        assert_eq!(refs[1].range.start.line, 1);
    }

    #[test]
    fn test_no_partial_match() {
        let content = "assign foobar = 1;";
        let refs = find_references_in_content(content, "file:///test.sv", "foo");
        assert_eq!(refs.len(), 0);
    }

    #[test]
    fn test_references_column_positions() {
        let content = "  assign foo = 1;";
        let refs = find_references_in_content(content, "file:///test.sv", "foo");
        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0].range.start.character, 9);
    }
}
