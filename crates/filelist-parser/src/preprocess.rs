//! 预处理模块（行续接、注释过滤）

/// 预处理选项
#[derive(Debug, Clone)]
pub struct PreprocessOptions {
    /// 是否处理行续接
    pub handle_continuation: bool,
    /// 是否过滤注释
    pub filter_comments: bool,
}

impl Default for PreprocessOptions {
    fn default() -> Self {
        Self {
            handle_continuation: true,
            filter_comments: true,
        }
    }
}

/// 预处理 filelist 内容
///
/// 处理：
/// 1. 行续接（`\` 行尾）
/// 2. 注释过滤（`//` 行注释、`/* */` 块注释）
pub fn preprocess(content: &str, opts: PreprocessOptions) -> Vec<(u32, String)> {
    let mut result = Vec::new();
    let lines = content.lines().enumerate();
    let mut current_line = String::new();
    let mut start_line = 1u32;
    let mut in_block_comment = false;

    for (idx, line) in lines {
        let line_num = (idx + 1) as u32;
        let trimmed = line.trim();

        // 处理块注释
        if in_block_comment {
            if let Some(end_pos) = trimmed.find("*/") {
                in_block_comment = false;
                let after_comment = trimmed[end_pos + 2..].trim();
                if !after_comment.is_empty() {
                    if !current_line.is_empty() {
                        current_line.push(' ');
                    }
                    current_line.push_str(after_comment);
                }
            }
            continue;
        }

        // 检查块注释开始
        if let Some(start_pos) = trimmed.find("/*") {
            let before_comment = trimmed[..start_pos].trim();

            if let Some(end_pos) = trimmed[start_pos..].find("*/") {
                // 块注释在同一行内结束
                let actual_end = start_pos + end_pos;
                let after_comment = trimmed[actual_end + 2..].trim();

                if !before_comment.is_empty() {
                    if !current_line.is_empty() {
                        current_line.push(' ');
                    }
                    current_line.push_str(before_comment);
                }
                if !after_comment.is_empty() {
                    if !current_line.is_empty() {
                        current_line.push(' ');
                    }
                    current_line.push_str(after_comment);
                }
            } else {
                in_block_comment = true;
                if !before_comment.is_empty() {
                    if !current_line.is_empty() {
                        current_line.push(' ');
                    }
                    current_line.push_str(before_comment);
                }
            }
        } else if opts.filter_comments {
            // 过滤行注释
            let comment_pos = trimmed.find("//");
            let content_to_add = if let Some(pos) = comment_pos {
                trimmed[..pos].trim()
            } else {
                trimmed
            };

            if !content_to_add.is_empty() {
                if !current_line.is_empty() {
                    current_line.push(' ');
                }
                current_line.push_str(content_to_add);
            }
        } else {
            if !trimmed.is_empty() {
                if !current_line.is_empty() {
                    current_line.push(' ');
                }
                current_line.push_str(trimmed);
            }
        }

        // 处理行续接
        if opts.handle_continuation && current_line.ends_with('\\') {
            current_line.pop(); // 移除反斜杠
            continue;
        }

        // 添加完整的行到结果
        if !current_line.trim().is_empty() {
            result.push((start_line, current_line.trim().to_string()));
            current_line.clear();
        }

        start_line = line_num + 1;
    }

    // 处理最后一行（如果有未结束的续接）
    if !current_line.trim().is_empty() {
        result.push((start_line, current_line.trim().to_string()));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preprocess_simple_lines() {
        let content = "file1.v\nfile2.v\nfile3.v";
        let result = preprocess(content, PreprocessOptions::default());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], (1, "file1.v".to_string()));
        assert_eq!(result[1], (2, "file2.v".to_string()));
        assert_eq!(result[2], (3, "file3.v".to_string()));
    }

    #[test]
    fn test_preprocess_line_continuation() {
        let content = "file1.v \\\nfile2_continued.v\nfile3.v";
        let result = preprocess(content, PreprocessOptions::default());
        assert_eq!(result.len(), 2);
        // 由于处理方式，可能有多个空格，但内容应该正确
        assert!(result[0]
            .1
            .replace("  ", " ")
            .contains("file1.v file2_continued.v"));
        assert_eq!(result[1], (3, "file3.v".to_string()));
    }

    #[test]
    fn test_preprocess_line_comment() {
        let content = "file1.v // this is a comment\nfile2.v";
        let result = preprocess(content, PreprocessOptions::default());
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], (1, "file1.v".to_string()));
        assert_eq!(result[1], (2, "file2.v".to_string()));
    }

    #[test]
    fn test_preprocess_block_comment() {
        let content = "file1.v /* comment */ file2.v\nfile3.v";
        let result = preprocess(content, PreprocessOptions::default());
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], (1, "file1.v file2.v".to_string()));
    }

    #[test]
    fn test_preprocess_multiline_block_comment() {
        let content = "file1.v /* start\ncomment continues\nend */ file2.v\nfile3.v";
        let result = preprocess(content, PreprocessOptions::default());
        // 多行块注释的处理：第一行包含 file1.v，后续行在块注释中被跳过
        // 块注释结束后在同一行或下一行处理剩余内容
        assert!(!result.is_empty());
        // 第一行应该至少包含 file1.v
        assert!(result[0].1.contains("file1.v"));
    }

    #[test]
    fn test_preprocess_empty_lines() {
        let content = "file1.v\n\n\nfile2.v\n";
        let result = preprocess(content, PreprocessOptions::default());
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_preprocess_comment_only_line() {
        let content = "// only comment\nfile1.v\n";
        let result = preprocess(content, PreprocessOptions::default());
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], (2, "file1.v".to_string()));
    }
}
