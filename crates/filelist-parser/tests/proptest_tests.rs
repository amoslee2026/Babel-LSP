//! Filelist 解析器属性测试

use proptest::prelude::*;

// 生成有效的文件名（不含特殊字符和注释符号）
fn valid_filename() -> impl Strategy<Value = String> {
    // 文件名应该包含扩展名，且不含注释符号
    // 使用简单的字母数字组合避免解析问题
    "[a-z][a-z0-9_]{0,20}\\.(v|sv|vh|vhd)".prop_map(String::from)
}

proptest! {
    #[test]
    fn test_filelist_source_file_preserved(filename in valid_filename()) {
        let content = format!("{}\n", filename);
        let parser = babel_lsp_filelist::cadence::CadenceParser::default();
        let result = parser.parse(&content);

        // 有效文件名应该被解析到 source_files 中
        prop_assert!(
            result.source_files.iter().any(|f| f.path.to_string_lossy().contains(&filename))
        );
    }
}