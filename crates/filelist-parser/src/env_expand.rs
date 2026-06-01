//! 环境变量展开

use std::env;

/// 展开环境变量（返回展开后的字符串和警告列表）
///
/// 支持：
/// - `$VAR` 格式
/// - `${VAR}` 格式
///
/// # Arguments
/// * `input` - 含环境变量的字符串
///
/// # Returns
/// * `(String, Vec<String>)` - 展开后的字符串和未定义的变量名列表
pub fn expand_with_warnings(input: &str) -> (String, Vec<String>) {
    let mut result = input.to_string();
    let mut warnings = Vec::new();

    // 先展开 ${VAR} 格式（带括号）
    expand_braced_vars(&mut result, &mut warnings);

    // 再展开 $VAR 格式（无括号）
    expand_simple_vars(&mut result, &mut warnings);

    (result, warnings)
}

/// 展开 ${VAR} 格式的环境变量
fn expand_braced_vars(result: &mut String, warnings: &mut Vec<String>) {
    let mut offset = 0;
    while let Some(rel_start) = result[offset..].find("${") {
        let start = offset + rel_start;
        if let Some(rel_end) = result[start..].find('}') {
            let end = start + rel_end;
            let var_name = &result[start + 2..end];
            if is_valid_var_name(var_name) {
                if let Ok(value) = env::var(var_name) {
                    result.replace_range(start..=end, &value);
                    offset = start + value.len();
                } else {
                    // SD-1 fix: 未定义变量保留原始 ${VAR}，记录警告
                    warnings.push(var_name.to_string());
                    offset = end + 1; // skip past closing '}'
                }
            } else {
                offset = end + 1;
            }
        } else {
            break;
        }
    }
}

/// 展开 $VAR 格式的环境变量（无括号）
fn expand_simple_vars(result: &mut String, warnings: &mut Vec<String>) {
    // 使用迭代方式处理，避免索引问题
    let mut chars: Vec<char> = result.chars().collect();
    let mut i = 0;
    let mut new_warnings = Vec::new();

    while i < chars.len() {
        if chars[i] == '$' {
            // 检查是否是 ${VAR} 格式（已处理）
            if i + 1 < chars.len() && chars[i + 1] == '{' {
                i += 2;
                continue;
            }

            // 提取变量名（直到遇到非字母数字字符）
            let start = i;
            i += 1;

            let var_start = i;
            while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }

            if i > var_start {
                let var_name: String = chars[var_start..i].iter().collect();
                if is_valid_var_name(&var_name) {
                    if let Ok(value) = env::var(&var_name) {
                        // 替换变量
                        let value_chars: Vec<char> = value.chars().collect();
                        chars.splice(start..i, value_chars.iter().cloned());
                        i = start + value_chars.len();
                        continue;
                    } else {
                        new_warnings.push(var_name);
                        // SD-1 fix: 未定义变量保留原始 $VAR，i 已在变量名末尾，不修改 chars
                        continue;
                    }
                }
            }
        }
        i += 1;
    }

    *result = chars.into_iter().collect();
    warnings.extend(new_warnings);
}

/// 展开环境变量（简化版本，不返回警告）
pub fn expand_env_vars(s: &str) -> String {
    let (result, _) = expand_with_warnings(s);
    result
}

/// 验证变量名格式
///
/// 格式要求：[A-Za-z_][A-Za-z0-9_]*
fn is_valid_var_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    let first_char = name.chars().next().unwrap();
    if !first_char.is_ascii_alphabetic() && first_char != '_' {
        return false;
    }

    name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_braced_var() {
        std::env::set_var("TEST_VAR", "test_value");
        let (result, warnings) = expand_with_warnings("${TEST_VAR}/file.txt");
        assert_eq!(result, "test_value/file.txt");
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_expand_simple_var() {
        std::env::set_var("SIMPLE_VAR", "simple_value");
        let (result, warnings) = expand_with_warnings("$SIMPLE_VAR/file.txt");
        assert_eq!(result, "simple_value/file.txt");
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_expand_multiple_vars() {
        std::env::set_var("VAR1", "value1");
        std::env::set_var("VAR2", "value2");
        let (result, warnings) = expand_with_warnings("${VAR1}/$VAR2/file.txt");
        assert_eq!(result, "value1/value2/file.txt");
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_expand_undefined_var() {
        // SD-1 fix: 未定义变量应保留原始 ${VAR}，不替换为空
        std::env::remove_var("UNDEFINED_VAR");
        let (result, warnings) = expand_with_warnings("${UNDEFINED_VAR}/file.txt");
        assert_eq!(result, "${UNDEFINED_VAR}/file.txt");
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0], "UNDEFINED_VAR");
    }

    #[test]
    fn test_expand_var_in_middle() {
        std::env::set_var("MID_VAR", "middle");
        let (result, warnings) = expand_with_warnings("prefix_${MID_VAR}_suffix");
        assert_eq!(result, "prefix_middle_suffix");
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_valid_var_name() {
        assert!(is_valid_var_name("VALID_VAR"));
        assert!(is_valid_var_name("_valid_var"));
        assert!(is_valid_var_name("ValidVar123"));
        assert!(!is_valid_var_name(""));
        assert!(!is_valid_var_name("123invalid"));
        assert!(!is_valid_var_name("invalid-var"));
    }

    #[test]
    fn test_expand_preserves_other_text() {
        std::env::set_var("PROJECT", "myproject");
        let (result, _) = expand_with_warnings("/home/user/$PROJECT/src/top.v");
        assert_eq!(result, "/home/user/myproject/src/top.v");
    }

    #[test]
    fn test_expand_no_vars() {
        let (result, warnings) = expand_with_warnings("plain/path/file.txt");
        assert_eq!(result, "plain/path/file.txt");
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_expand_consecutive_vars() {
        std::env::set_var("A", "a");
        std::env::set_var("B", "b");
        let (result, _) = expand_with_warnings("$A$B");
        assert_eq!(result, "ab");
    }

    #[test]
    fn test_expand_simple_undefined_var() {
        // SD-1 fix: 未定义变量应保留原始 $VAR，不替换为空
        std::env::remove_var("UNDEFINED_SIMPLE");
        let (result, warnings) = expand_with_warnings("$UNDEFINED_SIMPLE/file.txt");
        assert_eq!(result, "$UNDEFINED_SIMPLE/file.txt");
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0], "UNDEFINED_SIMPLE");
    }
}
