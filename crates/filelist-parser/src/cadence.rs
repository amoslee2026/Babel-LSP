//! Cadence filelist 解析（-makelib/-endlib）

use crate::env_expand::expand_with_warnings;
use crate::preprocess::{preprocess, PreprocessOptions};
use crate::synopsys::parse_line;
use crate::types::{FilelistResult, ParsedLine};
use crate::ParseWarning;

use std::path::PathBuf;

/// Cadence 解析器
pub struct CadenceParser {
    /// 基础路径
    base_path: PathBuf,
}

impl CadenceParser {
    pub fn new(base_path: PathBuf) -> Self {
        Self { base_path }
    }

    /// 解析 Cadence 格式内容
    ///
    /// 支持：
    /// - -makelib <name> 进入 library scope
    /// - -endlib 退出 library scope
    /// - 所有 Synopsys .f 格式选项
    pub fn parse(&self, content: &str) -> FilelistResult {
        // 预处理
        let preprocessed = preprocess(content, PreprocessOptions::default());

        // 环境变量展开
        let expanded_lines: Vec<(u32, String, Vec<String>)> = preprocessed
            .into_iter()
            .map(|(line_num, line)| {
                let (expanded, warnings) = expand_with_warnings(&line);
                (line_num, expanded, warnings)
            })
            .collect();

        let mut result = FilelistResult::default();
        let mut current_library: Option<String> = None;

        for (line_num, line, env_warnings) in expanded_lines {
            // 收集环境变量警告
            for var_name in env_warnings {
                result.add_warning(ParseWarning::EnvUndefined {
                    var_name,
                    line: line_num,
                });
            }

            // 解析单行
            let parsed = parse_line(&line, &self.base_path);

            match parsed {
                ParsedLine::Empty => continue,

                ParsedLine::MakeLib(name) => {
                    current_library = Some(name);
                },

                ParsedLine::EndLib => {
                    current_library = None;
                },

                ParsedLine::SourceFile(path) => {
                    result.add_source_file(path, current_library.clone());
                },

                ParsedLine::LibraryFile(path) => {
                    result.add_library_file(path);
                },

                ParsedLine::LibraryDir(path) => {
                    result.add_library_dir(path);
                },

                ParsedLine::IncludeDir(path) => {
                    result.add_include_dir(path);
                },

                ParsedLine::MacroDefine(define) => {
                    result.add_macro_define(define.name, define.value);
                },

                ParsedLine::LibExtensions(exts) => {
                    result.add_library_extensions(exts);
                },

                ParsedLine::NestedFilelist(_) => {
                    // 嵌套 filelist 在主解析器处理
                    // 这里简化处理，不递归
                },

                ParsedLine::UnknownOption(content) => {
                    result.add_warning(ParseWarning::MalformedLine {
                        content,
                        line: line_num,
                    });
                },
            }
        }

        result
    }
}

impl Default for CadenceParser {
    fn default() -> Self {
        Self::new(PathBuf::from("."))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cadence_makelib_scope() {
        let content = r#"
-makelib tech_lib
lib_file1.v
lib_file2.v
-endlib
normal_file.v
"#;

        let parser = CadenceParser::new(PathBuf::from("/project"));
        let result = parser.parse(content);

        assert_eq!(result.source_files.len(), 3);

        // 前两个文件属于 tech_lib
        assert_eq!(result.source_files[0].library, Some("tech_lib".to_string()));
        assert_eq!(result.source_files[1].library, Some("tech_lib".to_string()));

        // 最后一个文件不属于任何库
        assert_eq!(result.source_files[2].library, None);
    }

    #[test]
    fn test_cadence_multiple_libraries() {
        let content = r#"
-makelib lib1
file1.v
-endlib
-makelib lib2
file2.v
-endlib
file3.v
"#;

        let parser = CadenceParser::new(PathBuf::from("/project"));
        let result = parser.parse(content);

        assert_eq!(result.source_files.len(), 3);
        assert_eq!(result.source_files[0].library, Some("lib1".to_string()));
        assert_eq!(result.source_files[1].library, Some("lib2".to_string()));
        assert_eq!(result.source_files[2].library, None);
    }

    #[test]
    fn test_cadence_with_synopsys_options() {
        let content = r#"
-makelib tech_lib
+incdir+./include
+define+DEBUG=1
lib_file.v
-endlib
"#;

        let parser = CadenceParser::new(PathBuf::from("/project"));
        let result = parser.parse(content);

        assert_eq!(result.source_files.len(), 1);
        assert_eq!(result.include_dirs.len(), 1);
        assert_eq!(result.macro_defines.len(), 1);
        assert_eq!(result.source_files[0].library, Some("tech_lib".to_string()));
    }

    #[test]
    fn test_cadence_nested_scope() {
        // 测试嵌套库（虽然不推荐，但应该正确处理）
        let content = r#"
-makelib outer_lib
outer_file.v
-makelib inner_lib
inner_file.v
-endlib
outer_file2.v
-endlib
"#;

        let parser = CadenceParser::new(PathBuf::from("/project"));
        let result = parser.parse(content);

        assert_eq!(result.source_files.len(), 3);
        // inner_file.v 属于 inner_lib
        assert_eq!(
            result.source_files[1].library,
            Some("inner_lib".to_string())
        );
    }

    #[test]
    fn test_cadence_no_endlib() {
        // 测试缺少 -endlib 的情况
        let content = r#"
-makelib tech_lib
lib_file.v
normal_file.v
"#;

        let parser = CadenceParser::new(PathBuf::from("/project"));
        let result = parser.parse(content);

        // 所有文件都属于 tech_lib（因为没有 -endlib）
        assert_eq!(result.source_files.len(), 2);
        assert_eq!(result.source_files[0].library, Some("tech_lib".to_string()));
        assert_eq!(result.source_files[1].library, Some("tech_lib".to_string()));
    }
}
