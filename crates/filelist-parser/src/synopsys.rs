//! Synopsys .f 文件解析

use std::path::{Path, PathBuf};

use crate::types::{MacroDefine, ParsedLine};

/// 解析单行 Synopsys .f 内容
///
/// # Arguments
/// * `line` - 预处理后的单行内容
/// * `base_path` - 基础路径（用于解析相对路径）
///
/// # Returns
/// * `ParsedLine` - 解析结果
pub fn parse_line(line: &str, base_path: &Path) -> ParsedLine {
    let trimmed = line.trim();

    // 空行跳过
    if trimmed.is_empty() {
        return ParsedLine::Empty;
    }

    // 源文件（无前缀）
    if !trimmed.starts_with('-') && !trimmed.starts_with('+') {
        let path = resolve_path(trimmed, base_path);
        return ParsedLine::SourceFile(path);
    }

    // -f 嵌套
    if let Some(rest) = trimmed.strip_prefix("-f ") {
        let path = resolve_path(rest.trim(), base_path);
        return ParsedLine::NestedFilelist(path);
    }
    if let Some(rest) = trimmed.strip_prefix("-f\t") {
        let path = resolve_path(rest.trim(), base_path);
        return ParsedLine::NestedFilelist(path);
    }

    // -v 库文件
    if let Some(rest) = trimmed.strip_prefix("-v ") {
        let path = resolve_path(rest.trim(), base_path);
        return ParsedLine::LibraryFile(path);
    }
    if let Some(rest) = trimmed.strip_prefix("-v\t") {
        let path = resolve_path(rest.trim(), base_path);
        return ParsedLine::LibraryFile(path);
    }

    // -y 库目录
    if let Some(rest) = trimmed.strip_prefix("-y ") {
        let path = resolve_path(rest.trim(), base_path);
        return ParsedLine::LibraryDir(path);
    }
    if let Some(rest) = trimmed.strip_prefix("-y\t") {
        let path = resolve_path(rest.trim(), base_path);
        return ParsedLine::LibraryDir(path);
    }

    // +incdir+ include 目录
    if let Some(rest) = trimmed.strip_prefix("+incdir+") {
        let path_str = rest.trim();
        // 处理多个 +incdir+ 选项（用 + 分隔）
        if path_str.contains('+') && !path_str.ends_with('+') {
            // 单个 +incdir+ 后面的路径
            let end_pos = path_str.find('+').unwrap_or(path_str.len());
            let single_path = &path_str[..end_pos];
            let path = resolve_path(single_path.trim(), base_path);
            return ParsedLine::IncludeDir(path);
        }
        let path = resolve_path(path_str, base_path);
        return ParsedLine::IncludeDir(path);
    }

    // +define+ 宏定义
    if let Some(rest) = trimmed.strip_prefix("+define+") {
        let define = parse_define(rest.trim());
        return ParsedLine::MacroDefine(define);
    }

    // +libext+ 库扩展名
    if let Some(rest) = trimmed.strip_prefix("+libext+") {
        let extensions = parse_libext(rest.trim());
        return ParsedLine::LibExtensions(extensions);
    }

    // -makelib (Cadence 扩展)
    if let Some(rest) = trimmed.strip_prefix("-makelib") {
        let lib_name = rest.trim();
        return ParsedLine::MakeLib(lib_name.to_string());
    }

    // -endlib (Cadence 扩展)
    if trimmed == "-endlib" {
        return ParsedLine::EndLib;
    }

    // 未知选项，作为源文件处理
    ParsedLine::UnknownOption(trimmed.to_string())
}

/// 解析宏定义
///
/// 格式: NAME 或 NAME=value 或 NAME="value with spaces"
fn parse_define(s: &str) -> MacroDefine {
    // 处理多个 +define+ 选项（用 + 分隔）
    let define_part = if s.contains('+') {
        let end_pos = s.find('+').unwrap_or(s.len());
        &s[..end_pos]
    } else {
        s
    };

    let trimmed = define_part.trim();

    // 处理带引号的值
    if let Some(eq_pos) = trimmed.find('=') {
        let name = trimmed[..eq_pos].trim().to_string();
        let value_str = &trimmed[eq_pos + 1..];

        // 处理引号包裹的值
        let value = if value_str.starts_with('"') && value_str.ends_with('"') && value_str.len() > 1
        {
            Some(value_str[1..value_str.len() - 1].to_string())
        } else {
            Some(value_str.trim().to_string())
        };

        MacroDefine { name, value }
    } else {
        MacroDefine {
            name: trimmed.to_string(),
            value: None,
        }
    }
}

/// 解析库扩展名
///
/// 格式: +libext+.v+.vh+.sv (多个扩展名用 + 分隔)
fn parse_libext(s: &str) -> Vec<String> {
    s.split('+')
        .filter(|ext| !ext.trim().is_empty())
        .map(|ext| ext.trim().to_string())
        .collect()
}

/// 解析路径（相对路径基于 base_path）
fn resolve_path(path_str: &str, base_path: &Path) -> PathBuf {
    let path = PathBuf::from(path_str);

    if path.is_absolute() {
        path
    } else {
        base_path.join(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_source_file() {
        let base = PathBuf::from("/project");
        let result = parse_line("src/top.v", &base);
        match result {
            ParsedLine::SourceFile(p) => {
                assert_eq!(p, PathBuf::from("/project/src/top.v"));
            },
            _ => panic!("Expected SourceFile"),
        }
    }

    #[test]
    fn test_parse_nested_filelist() {
        let base = PathBuf::from("/project");
        let result = parse_line("-f sub/filelist.f", &base);
        match result {
            ParsedLine::NestedFilelist(p) => {
                assert_eq!(p, PathBuf::from("/project/sub/filelist.f"));
            },
            _ => panic!("Expected NestedFilelist"),
        }
    }

    #[test]
    fn test_parse_include_dir() {
        let base = PathBuf::from("/project");
        let result = parse_line("+incdir+./include", &base);
        match result {
            ParsedLine::IncludeDir(p) => {
                assert_eq!(p, PathBuf::from("/project/include"));
            },
            _ => panic!("Expected IncludeDir"),
        }
    }

    #[test]
    fn test_parse_define_with_value() {
        let base = PathBuf::from("/project");
        let result = parse_line("+define+DEBUG=1", &base);
        match result {
            ParsedLine::MacroDefine(define) => {
                assert_eq!(define.name, "DEBUG");
                assert_eq!(define.value, Some("1".to_string()));
            },
            _ => panic!("Expected MacroDefine"),
        }
    }

    #[test]
    fn test_parse_define_without_value() {
        let base = PathBuf::from("/project");
        let result = parse_line("+define+ENABLE", &base);
        match result {
            ParsedLine::MacroDefine(define) => {
                assert_eq!(define.name, "ENABLE");
                assert_eq!(define.value, None);
            },
            _ => panic!("Expected MacroDefine"),
        }
    }

    #[test]
    fn test_parse_define_with_quoted_value() {
        let base = PathBuf::from("/project");
        let result = parse_line("+define+MSG=\"Hello World\"", &base);
        match result {
            ParsedLine::MacroDefine(define) => {
                assert_eq!(define.name, "MSG");
                assert_eq!(define.value, Some("Hello World".to_string()));
            },
            _ => panic!("Expected MacroDefine"),
        }
    }

    #[test]
    fn test_parse_libext() {
        let base = PathBuf::from("/project");
        let result = parse_line("+libext+.v+.vh+.sv", &base);
        match result {
            ParsedLine::LibExtensions(exts) => {
                assert_eq!(exts.len(), 3);
                assert_eq!(exts[0], ".v");
                assert_eq!(exts[1], ".vh");
                assert_eq!(exts[2], ".sv");
            },
            _ => panic!("Expected LibExtensions"),
        }
    }

    #[test]
    fn test_parse_library_file() {
        let base = PathBuf::from("/project");
        let result = parse_line("-v lib/tech.v", &base);
        match result {
            ParsedLine::LibraryFile(p) => {
                assert_eq!(p, PathBuf::from("/project/lib/tech.v"));
            },
            _ => panic!("Expected LibraryFile"),
        }
    }

    #[test]
    fn test_parse_library_dir() {
        let base = PathBuf::from("/project");
        let result = parse_line("-y lib", &base);
        match result {
            ParsedLine::LibraryDir(p) => {
                assert_eq!(p, PathBuf::from("/project/lib"));
            },
            _ => panic!("Expected LibraryDir"),
        }
    }

    #[test]
    fn test_parse_makelib() {
        let base = PathBuf::from("/project");
        let result = parse_line("-makelib tech_lib", &base);
        match result {
            ParsedLine::MakeLib(name) => {
                assert_eq!(name, "tech_lib");
            },
            _ => panic!("Expected MakeLib"),
        }
    }

    #[test]
    fn test_parse_endlib() {
        let base = PathBuf::from("/project");
        let result = parse_line("-endlib", &base);
        match result {
            ParsedLine::EndLib => (),
            _ => panic!("Expected EndLib"),
        }
    }

    #[test]
    fn test_parse_empty_line() {
        let base = PathBuf::from("/project");
        let result = parse_line("", &base);
        match result {
            ParsedLine::Empty => (),
            _ => panic!("Expected Empty"),
        }
    }

    #[test]
    fn test_parse_whitespace_only() {
        let base = PathBuf::from("/project");
        let result = parse_line("   \t  ", &base);
        match result {
            ParsedLine::Empty => (),
            _ => panic!("Expected Empty"),
        }
    }

    #[test]
    fn test_parse_absolute_path() {
        let base = PathBuf::from("/project");
        let result = parse_line("/absolute/path/file.v", &base);
        match result {
            ParsedLine::SourceFile(p) => {
                assert_eq!(p, PathBuf::from("/absolute/path/file.v"));
            },
            _ => panic!("Expected SourceFile"),
        }
    }
}
