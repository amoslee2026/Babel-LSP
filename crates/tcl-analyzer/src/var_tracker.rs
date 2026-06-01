//! TCL 变量 scope 追踪器

use crate::parser::ParseResult;
use std::collections::{HashMap, HashSet};

/// 单个变量的详细信息
#[derive(Debug, Clone)]
pub struct VarInfo {
    pub name: String,
    /// "global" 或 proc 名称
    pub scope: String,
    pub is_global: bool,
    /// (行号, 赋值内容)
    pub assignments: Vec<(u32, String)>,
}

/// 变量追踪器
pub struct VarTracker;

impl VarTracker {
    pub fn new() -> Self {
        Self
    }

    /// 对所有变量进行 scope 追踪
    /// key = "scope::name"，保证跨 scope 不混淆
    pub fn track(&self, parse_result: &ParseResult) -> HashMap<String, VarInfo> {
        let mut map: HashMap<String, VarInfo> = HashMap::new();

        // 处理顶层变量（global scope 和 proc 外部的 set）
        for var in &parse_result.variables {
            let key = format!("{}::{}", var.scope, var.name);
            let is_global = var.scope == "global";
            let entry = map.entry(key).or_insert_with(|| VarInfo {
                name: var.name.clone(),
                scope: var.scope.clone(),
                is_global,
                assignments: Vec::new(),
            });
            if let Some(val) = &var.value {
                entry.assignments.push((var.line, val.clone()));
            }
        }

        // 处理 proc body 内部的 set 调用
        for proc in &parse_result.procs {
            for (rel_line, body_line) in proc.body.lines().enumerate() {
                let line_no = proc.start_line + 1 + rel_line as u32;
                let trimmed = body_line.trim();
                if let Some(rest) = strip_cmd_str(trimmed, "set") {
                    let tokens = split_tokens_simple(rest);
                    if !tokens.is_empty() && !tokens[0].starts_with('-') {
                        let var_name = tokens[0].clone();
                        let val = if tokens.len() > 1 {
                            Some(tokens[1..].join(" "))
                        } else {
                            None
                        };
                        let key = format!("{}::{}", proc.name, var_name);
                        let entry = map.entry(key).or_insert_with(|| VarInfo {
                            name: var_name.clone(),
                            scope: proc.name.clone(),
                            is_global: false,
                            assignments: Vec::new(),
                        });
                        if let Some(v) = val {
                            entry.assignments.push((line_no, v));
                        }
                    }
                }
            }
        }

        map
    }

    /// 找出在 proc 内使用但未在该 scope 赋值/声明的变量
    /// 返回 (line, varname)
    pub fn find_undefined_vars(&self, parse_result: &ParseResult) -> Vec<(u32, String)> {
        // 收集每个 scope 中已声明/赋值的变量名（包括 proc body 内部 set）
        let tracked = self.track(parse_result);
        let mut scope_vars: HashMap<String, HashSet<String>> = HashMap::new();
        for info in tracked.values() {
            scope_vars
                .entry(info.scope.clone())
                .or_default()
                .insert(info.name.clone());
        }
        // 也包括 proc 参数
        for proc in &parse_result.procs {
            let entry = scope_vars.entry(proc.name.clone()).or_default();
            for arg in &proc.args {
                entry.insert(arg.clone());
            }
        }

        // 收集 global scope 变量
        let global_vars: HashSet<String> = scope_vars.get("global").cloned().unwrap_or_default();

        let mut undefined = Vec::new();

        // 扫描每个 proc 的 body，寻找 $var 引用
        for proc in &parse_result.procs {
            let local_vars = scope_vars.get(&proc.name).cloned().unwrap_or_default();
            for (rel_line, body_line) in proc.body.lines().enumerate() {
                let line_no = proc.start_line + 1 + rel_line as u32;
                for var_name in extract_var_refs(body_line) {
                    if !local_vars.contains(&var_name)
                        && !global_vars.contains(&var_name)
                        && !is_builtin_var(&var_name)
                    {
                        undefined.push((line_no, var_name));
                    }
                }
            }
        }

        undefined.sort();
        undefined.dedup();
        undefined
    }

    /// 找出已声明但从未在 proc body 中被读取（$var）的变量
    pub fn find_unused_vars(&self, parse_result: &ParseResult) -> Vec<String> {
        // 收集所有非 global 的赋值变量（来自 track，包括 proc body 内的 set）
        let tracked = self.track(parse_result);
        let assigned: Vec<(String, String)> = tracked
            .values()
            .filter(|v| !v.is_global)
            .map(|v| (v.scope.clone(), v.name.clone()))
            .collect();

        if assigned.is_empty() {
            return Vec::new();
        }

        // 对每个 proc 收集实际使用的 $var 引用
        let mut used: std::collections::HashSet<(String, String)> =
            std::collections::HashSet::new();
        for proc in &parse_result.procs {
            for body_line in proc.body.lines() {
                for var_name in extract_var_refs(body_line) {
                    used.insert((proc.name.clone(), var_name));
                }
            }
        }

        let mut unused: Vec<String> = assigned
            .iter()
            .filter(|(scope, name)| !used.contains(&(scope.clone(), name.clone())))
            .map(|(_, name)| name.clone())
            .collect();

        unused.sort();
        unused.dedup();
        unused
    }
}

impl Default for VarTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// 从一行 TCL 代码中提取所有 $varname 引用
fn extract_var_refs(line: &str) -> Vec<String> {
    let mut refs = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '$' && i + 1 < chars.len() {
            i += 1;
            // 处理 ${varname} 形式
            if chars[i] == '{' {
                i += 1;
                let start = i;
                while i < chars.len() && chars[i] != '}' {
                    i += 1;
                }
                let name: String = chars[start..i].iter().collect();
                if !name.is_empty() {
                    refs.push(name);
                }
            } else {
                // 普通 $varname（字母数字下划线）
                let start = i;
                while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }
                let name: String = chars[start..i].iter().collect();
                if !name.is_empty() {
                    refs.push(name);
                }
                continue; // 已经移动了 i
            }
        }
        i += 1;
    }

    refs
}

/// 一些 TCL 内建变量，不应被标记为 undefined
fn is_builtin_var(name: &str) -> bool {
    matches!(
        name,
        "argc"
            | "argv"
            | "argv0"
            | "env"
            | "errorInfo"
            | "errorCode"
            | "tcl_version"
            | "tcl_patchLevel"
            | "tcl_platform"
            | "tcl_library"
            | "_"
            | "auto_path"
    )
}

/// 检查字符串是否以 cmd 开头（后跟空格或为空）
fn strip_cmd_str<'a>(line: &'a str, cmd: &str) -> Option<&'a str> {
    if let Some(rest) = line.strip_prefix(cmd) {
        if rest.is_empty() || rest.starts_with(' ') || rest.starts_with('\t') {
            return Some(rest.trim_start());
        }
    }
    None
}

/// 简单 token 分割（用于 var_tracker 内部）
fn split_tokens_simple(s: &str) -> Vec<String> {
    let s = s.trim();
    if s.is_empty() {
        return Vec::new();
    }
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut depth = 0i32;
    let mut in_quote = false;
    for ch in s.chars() {
        match ch {
            '"' if depth == 0 => in_quote = !in_quote,
            '{' if !in_quote => {
                depth += 1;
                current.push(ch);
            },
            '}' if !in_quote => {
                depth -= 1;
                current.push(ch);
            },
            ' ' | '\t' if !in_quote && depth == 0 => {
                if !current.is_empty() {
                    tokens.push(current.trim_matches('{').trim_matches('}').to_string());
                    current = String::new();
                }
            },
            _ => current.push(ch),
        }
    }
    if !current.is_empty() {
        tokens.push(current.trim_matches('{').trim_matches('}').to_string());
    }
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::TclParser;

    const TCL_VARS: &str = r#"
set global_x 10
global global_x

proc compute {a b} {
    set result [expr {$a + $b}]
    return $result
}

proc unused_var_proc {} {
    set used_var 5
    set unused 99
    return $used_var
}
"#;

    #[test]
    fn test_track_variables() {
        let parser = TclParser::new();
        let result = parser.parse(TCL_VARS);
        let tracker = VarTracker::new();
        let vars = tracker.track(&result);
        assert!(!vars.is_empty(), "should track some variables");
    }

    #[test]
    fn test_track_global_variable() {
        let parser = TclParser::new();
        let result = parser.parse(TCL_VARS);
        let tracker = VarTracker::new();
        let vars = tracker.track(&result);
        let global_entry = vars.get("global::global_x");
        assert!(
            global_entry.is_some(),
            "should track global_x in global scope"
        );
        let entry = global_entry.unwrap();
        assert!(entry.is_global);
    }

    #[test]
    fn test_track_proc_variable() {
        let parser = TclParser::new();
        let result = parser.parse(TCL_VARS);
        let tracker = VarTracker::new();
        let vars = tracker.track(&result);
        let proc_var = vars.get("compute::result");
        assert!(proc_var.is_some(), "should track 'result' in compute scope");
        assert!(!proc_var.unwrap().is_global);
    }

    #[test]
    fn test_track_assignments() {
        let tcl = "proc f {} {\n    set x 1\n    set x 2\n}\n";
        let parser = TclParser::new();
        let result = parser.parse(tcl);
        let tracker = VarTracker::new();
        let vars = tracker.track(&result);
        // x should be in f scope
        let found = vars.values().any(|v| v.name == "x" && v.scope == "f");
        assert!(found, "should find x in scope f");
    }

    #[test]
    fn test_find_unused_vars() {
        let parser = TclParser::new();
        let result = parser.parse(TCL_VARS);
        let tracker = VarTracker::new();
        let unused = tracker.find_unused_vars(&result);
        assert!(
            unused.contains(&"unused".to_string()),
            "should find 'unused' var, got: {:?}",
            unused
        );
        assert!(
            !unused.contains(&"used_var".to_string()),
            "should not flag 'used_var' as unused"
        );
    }

    #[test]
    fn test_extract_var_refs() {
        let refs = extract_var_refs("set result [expr {$a + $b + ${c_val}}]");
        assert!(refs.contains(&"a".to_string()));
        assert!(refs.contains(&"b".to_string()));
        assert!(refs.contains(&"c_val".to_string()));
    }

    #[test]
    fn test_no_false_positives_for_builtins() {
        let refs = extract_var_refs("puts $argc $argv");
        // argc and argv are builtin vars - should not be flagged
        assert!(refs.contains(&"argc".to_string()));
        assert!(is_builtin_var("argc"));
        assert!(is_builtin_var("argv"));
    }

    #[test]
    fn test_empty_content() {
        let parser = TclParser::new();
        let result = parser.parse("");
        let tracker = VarTracker::new();
        let vars = tracker.track(&result);
        assert!(vars.is_empty());
        let unused = tracker.find_unused_vars(&result);
        assert!(unused.is_empty());
    }
}
