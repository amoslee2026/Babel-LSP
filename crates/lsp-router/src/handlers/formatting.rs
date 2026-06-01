//! 代码格式化处理器

use tower_lsp::lsp_types::*;

/// 尝试使用 verible-verilog-format 格式化
pub fn try_verible_format(content: &str) -> Option<String> {
    use std::io::Write;
    use std::process::{Command, Stdio};

    let mut child = Command::new("verible-verilog-format")
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .ok()?;

    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(content.as_bytes());
    }

    let output = child.wait_with_output().ok()?;
    if output.status.success() {
        String::from_utf8(output.stdout).ok()
    } else {
        None
    }
}

/// 将格式化结果转为 LSP TextEdit 列表
pub fn content_to_edits(original: &str, formatted: &str) -> Vec<TextEdit> {
    if original == formatted {
        return vec![];
    }
    let line_count = original.lines().count() as u32;
    let last_line_len = original.lines().last().unwrap_or("").len() as u32;
    vec![TextEdit {
        range: Range {
            start: Position {
                line: 0,
                character: 0,
            },
            end: Position {
                line: line_count,
                character: last_line_len,
            },
        },
        new_text: formatted.to_string(),
    }]
}

/// 处理格式化请求
pub fn handle_formatting(content: &str) -> Vec<TextEdit> {
    match try_verible_format(content) {
        Some(formatted) => content_to_edits(content, &formatted),
        None => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content_to_edits_no_change() {
        let edits = content_to_edits("same", "same");
        assert!(edits.is_empty());
    }

    #[test]
    fn test_content_to_edits_changed() {
        let original = "line1\nline2\n";
        let formatted = "line1\n  line2\n";
        let edits = content_to_edits(original, formatted);
        assert_eq!(edits.len(), 1);
        assert_eq!(edits[0].new_text, formatted);
    }

    #[test]
    fn test_format_no_panic() {
        // verible is not installed in test env - should not panic
        let _result = try_verible_format("module foo; endmodule");
    }
}
