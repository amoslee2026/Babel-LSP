#![allow(non_snake_case)]
//! Filelist 解析器（Synopsys .f / Cadence 格式）
//!
//! 提供工业标准 filelist 解析功能，支持：
//! - Synopsys .f 格式解析
//! - Cadence -makelib/-endlib 库分组
//! - 环境变量展开
//! - 嵌套 filelist 递归解析
//! - 循环引用检测

pub mod cadence;
pub mod env_expand;
pub mod error;
pub mod preprocess;
pub mod synopsys;
pub mod types;

use std::fs;
use std::path::Path;

use preprocess::{preprocess, PreprocessOptions};
use types::ParsedLine;

pub use error::ParseError;
pub use types::{FilelistResult, MacroDefine, ParseOptions, ParseWarning, SourceFileInfo};

/// 解析 filelist 文件
///
/// # Arguments
/// * `path` - filelist 文件路径
/// * `opts` - 解析选项
///
/// # Returns
/// * `Result<FilelistResult, ParseError>` - 解析结果或错误
pub fn parse(path: &Path, opts: ParseOptions) -> Result<FilelistResult, ParseError> {
    let canonical_path = path.canonicalize().map_err(ParseError::IoError)?;

    // 检测循环引用
    if opts.visited.contains(&canonical_path) {
        return Err(ParseError::CircularReference(canonical_path));
    }

    // 检查递归深度
    if opts.visited.len() >= opts.max_depth as usize {
        return Err(ParseError::MaxDepthExceeded(opts.max_depth));
    }

    // 读取文件内容
    let content = fs::read_to_string(&canonical_path)?;

    // 解析内容
    parse_string_with_path(&content, opts, &canonical_path)
}

/// 解析 filelist 字符串内容（无文件 IO）
///
/// # Arguments
/// * `content` - filelist 内容字符串
/// * `opts` - 解析选项
///
/// # Returns
/// * `Result<FilelistResult, ParseError>` - 解析结果或错误
pub fn parse_string(content: &str, opts: ParseOptions) -> Result<FilelistResult, ParseError> {
    let base_path = opts.base_path.clone();
    parse_string_with_path(content, opts, &base_path)
}

/// 解析 filelist 字符串内容（带文件路径上下文）
fn parse_string_with_path(
    content: &str,
    opts: ParseOptions,
    file_path: &Path,
) -> Result<FilelistResult, ParseError> {
    // 预处理：行续接、注释过滤
    let preprocessed = preprocess(content, PreprocessOptions::default());

    // 环境变量展开（带警告收集）
    let expanded_lines: Vec<(u32, String, Vec<String>)> = preprocessed
        .into_iter()
        .map(|(line_num, line)| {
            let (expanded, warnings) = env_expand::expand_with_warnings(&line);
            (line_num, expanded, warnings)
        })
        .collect();

    let mut result = FilelistResult::default();

    // 当前 library scope（用于 Cadence 扩展）
    let mut current_library: Option<String> = None;

    // 处理每行
    for (line_num, line, env_warnings) in expanded_lines {
        // 收集环境变量警告
        for var_name in env_warnings {
            result.add_warning(ParseWarning::EnvUndefined {
                var_name,
                line: line_num,
            });
        }

        // 解析单行
        let parsed = synopsys::parse_line(&line, &opts.base_path);

        match parsed {
            ParsedLine::Empty => continue,

            ParsedLine::SourceFile(path) => {
                result.add_source_file(path, current_library.clone());
            },

            ParsedLine::NestedFilelist(nested_path) => {
                // 处理嵌套 filelist
                let mut nested_opts = opts.clone();
                nested_opts.visited.insert(file_path.to_path_buf());

                match parse(&nested_path, nested_opts) {
                    Ok(nested_result) => {
                        result.merge(nested_result);
                    },
                    Err(ParseError::CircularReference(p)) => {
                        return Err(ParseError::CircularReference(p));
                    },
                    Err(e) => {
                        return Err(ParseError::NestedFailed {
                            path: nested_path,
                            reason: e.to_string(),
                        });
                    },
                }
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

            ParsedLine::MakeLib(name) if opts.enable_cadence => {
                current_library = Some(name);
            },

            ParsedLine::EndLib if opts.enable_cadence => {
                current_library = None;
            },

            ParsedLine::UnknownOption(content) => {
                result.add_warning(ParseWarning::MalformedLine {
                    content,
                    line: line_num,
                });
            },

            // Cadence 扩展未启用时的处理
            ParsedLine::MakeLib(_) | ParsedLine::EndLib => {
                result.add_warning(ParseWarning::MalformedLine {
                    content: line,
                    line: line_num,
                });
            },
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_parse_simple_filelist() {
        let temp_dir = TempDir::new().unwrap();
        let filelist_path = temp_dir.path().join("test.f");

        let content = r#"
// This is a comment
file1.v
file2.v
+incdir+./include
+define+DEBUG=1
"#;

        fs::write(&filelist_path, content).unwrap();

        let opts = ParseOptions::new(temp_dir.path().to_path_buf());
        let result = parse(&filelist_path, opts).unwrap();

        assert_eq!(result.source_files.len(), 2);
        assert_eq!(result.include_dirs.len(), 1);
        assert_eq!(result.macro_defines.len(), 1);
    }

    #[test]
    fn test_parse_with_nested_filelist() {
        let temp_dir = TempDir::new().unwrap();

        // 创建嵌套 filelist
        let nested_path = temp_dir.path().join("nested.f");
        fs::write(&nested_path, "nested_file1.v\nnested_file2.v").unwrap();

        // 创建主 filelist
        let main_path = temp_dir.path().join("main.f");
        fs::write(&main_path, "main_file.v\n-f nested.f\nmain_file2.v").unwrap();

        let opts = ParseOptions::new(temp_dir.path().to_path_buf());
        let result = parse(&main_path, opts).unwrap();

        // 主 filelist 有 2 个源文件，嵌套有 2 个，总共 4 个
        assert_eq!(result.source_files.len(), 4);
    }

    #[test]
    fn test_parse_with_library_options() {
        let temp_dir = TempDir::new().unwrap();
        let filelist_path = temp_dir.path().join("test.f");

        let content = r#"
-v lib_file.v
-y lib_dir
+libext+.v+.vh
"#;

        fs::write(&filelist_path, content).unwrap();

        let opts = ParseOptions::new(temp_dir.path().to_path_buf());
        let result = parse(&filelist_path, opts).unwrap();

        assert_eq!(result.library_files.len(), 1);
        assert_eq!(result.library_dirs.len(), 1);
        assert_eq!(result.library_extensions.len(), 2);
    }

    #[test]
    fn test_circular_reference_detection() {
        let temp_dir = TempDir::new().unwrap();

        // 创建循环引用：a.f -> b.f -> a.f
        let a_path = temp_dir.path().join("a.f");
        let b_path = temp_dir.path().join("b.f");

        fs::write(&a_path, "-f b.f\nfile_a.v").unwrap();
        fs::write(&b_path, "-f a.f\nfile_b.v").unwrap();

        let opts = ParseOptions::new(temp_dir.path().to_path_buf());
        let result = parse(&a_path, opts);

        assert!(result.is_err());
        match result.unwrap_err() {
            ParseError::CircularReference(_) => (),
            _ => panic!("Expected CircularReference error"),
        }
    }

    #[test]
    fn test_parse_string() {
        let content = "file1.v\nfile2.v\n+incdir+./include";
        let opts = ParseOptions::new(std::path::PathBuf::from("/project"));
        let result = parse_string(content, opts).unwrap();

        assert_eq!(result.source_files.len(), 2);
        assert_eq!(result.include_dirs.len(), 1);
    }

    #[test]
    fn test_cadence_library_grouping() {
        let temp_dir = TempDir::new().unwrap();
        let filelist_path = temp_dir.path().join("test.f");

        let content = r#"
-makelib tech_lib
lib_file1.v
lib_file2.v
-endlib
normal_file.v
"#;

        fs::write(&filelist_path, content).unwrap();

        let opts = ParseOptions::new(temp_dir.path().to_path_buf()).with_cadence();
        let result = parse(&filelist_path, opts).unwrap();

        assert_eq!(result.source_files.len(), 3);
        assert_eq!(result.source_files[0].library, Some("tech_lib".to_string()));
        assert_eq!(result.source_files[1].library, Some("tech_lib".to_string()));
        assert_eq!(result.source_files[2].library, None);
    }

    #[test]
    fn test_max_depth_exceeded() {
        let temp_dir = TempDir::new().unwrap();

        // 创建深度嵌套的 filelist
        let f0 = temp_dir.path().join("f0.f");
        let f1 = temp_dir.path().join("f1.f");
        let f2 = temp_dir.path().join("f2.f");

        fs::write(&f0, "-f f1.f\nfile0.v").unwrap();
        fs::write(&f1, "-f f2.f\nfile1.v").unwrap();
        fs::write(&f2, "file2.v").unwrap();

        // max_depth = 0 表示不允许任何嵌套
        let opts = ParseOptions::new(temp_dir.path().to_path_buf()).with_max_depth(0);
        let result = parse(&f0, opts);

        assert!(result.is_err());
        match result.unwrap_err() {
            ParseError::MaxDepthExceeded(0) => (),
            e => panic!("Expected MaxDepthExceeded(0) error, got: {:?}", e),
        }
    }

    #[test]
    fn test_line_continuation() {
        let temp_dir = TempDir::new().unwrap();
        let filelist_path = temp_dir.path().join("test.f");

        let content = r#"
file1.v \
file2_continued.v
file3.v
"#;

        fs::write(&filelist_path, content).unwrap();

        let opts = ParseOptions::new(temp_dir.path().to_path_buf());
        let result = parse(&filelist_path, opts).unwrap();

        assert_eq!(result.source_files.len(), 2);
        // 第一行应该是合并后的路径
        let first_path = result.source_files[0].path.to_string_lossy();
        assert!(first_path.contains("file1.v"));
        assert!(first_path.contains("file2_continued.v"));
    }

    #[test]
    fn test_env_var_expansion() {
        let temp_dir = TempDir::new().unwrap();
        let filelist_path = temp_dir.path().join("test.f");

        std::env::set_var("TEST_PROJECT", "testproj");

        let content = r#"
$TEST_PROJECT/file1.v
${TEST_PROJECT}/file2.v
"#;

        fs::write(&filelist_path, content).unwrap();

        let opts = ParseOptions::new(temp_dir.path().to_path_buf());
        let result = parse(&filelist_path, opts).unwrap();

        assert_eq!(result.source_files.len(), 2);
        // 路径应该包含展开后的变量值
        for file in &result.source_files {
            let path_str = file.path.to_string_lossy();
            assert!(path_str.contains("testproj"));
        }
    }

    #[test]
    fn test_undefined_env_var_warning() {
        std::env::remove_var("UNDEFINED_VAR");

        let content = "$UNDEFINED_VAR/file.v";
        let opts = ParseOptions::new(std::path::PathBuf::from("/project"));
        let result = parse_string(content, opts).unwrap();

        // 应该有环境变量未定义的警告
        assert!(!result.warnings.is_empty());
        match &result.warnings[0] {
            ParseWarning::EnvUndefined { var_name, .. } => {
                assert_eq!(var_name, "UNDEFINED_VAR");
            },
            _ => panic!("Expected EnvUndefined warning"),
        }
    }
}
