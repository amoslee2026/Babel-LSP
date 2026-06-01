//! TCL proc 调用图分析器

use crate::parser::{extract_calls_from_body, ParseResult};
use std::collections::HashMap;

/// 单个 proc 的调用图节点
#[derive(Debug, Clone)]
pub struct CallGraph {
    /// 本 proc 的名称
    pub callee: String,
    /// 哪些 proc 调用了本 proc
    pub callers: Vec<String>,
    /// 本 proc 调用了哪些 proc
    pub callees: Vec<String>,
}

/// Proc 分析器
pub struct ProcAnalyzer;

impl ProcAnalyzer {
    pub fn new() -> Self {
        Self
    }

    /// 构建调用图：key = proc name，value = CallGraph
    pub fn analyze(&self, parse_result: &ParseResult) -> HashMap<String, CallGraph> {
        let proc_names: Vec<String> = parse_result.procs.iter().map(|p| p.name.clone()).collect();

        // 初始化所有节点
        let mut graph: HashMap<String, CallGraph> = proc_names
            .iter()
            .map(|name| {
                (
                    name.clone(),
                    CallGraph {
                        callee: name.clone(),
                        callers: Vec::new(),
                        callees: Vec::new(),
                    },
                )
            })
            .collect();

        // 对每个 proc body 提取调用
        for proc in &parse_result.procs {
            let calls = extract_calls_from_body(&proc.body, &proc_names);
            if let Some(node) = graph.get_mut(&proc.name) {
                node.callees = calls.clone();
            }
            // 更新被调用者的 callers
            for called in &calls {
                if let Some(called_node) = graph.get_mut(called) {
                    if !called_node.callers.contains(&proc.name) {
                        called_node.callers.push(proc.name.clone());
                    }
                }
            }
        }

        graph
    }

    /// 找出调用了不存在的 proc 的调用（未定义调用）
    pub fn find_undefined_calls(&self, parse_result: &ParseResult) -> Vec<(String, String)> {
        // 返回 (caller_name, undefined_callee_name)
        let proc_names: Vec<String> = parse_result.procs.iter().map(|p| p.name.clone()).collect();
        let builtins = builtin_commands();
        let mut result = Vec::new();

        for proc in &parse_result.procs {
            // 扫描 body 寻找可能的调用
            for line in proc.body.lines() {
                let trimmed = line.trim();
                if trimmed.is_empty() || trimmed.starts_with('#') {
                    continue;
                }
                let token = trimmed.split_whitespace().next().unwrap_or("");
                if token.is_empty() {
                    continue;
                }
                // 不是内建命令、不是已知 proc、不含特殊字符
                if !builtins.contains(&token)
                    && !proc_names.contains(&token.to_string())
                    && !token.starts_with('$')
                    && !token.starts_with('[')
                    && !token.starts_with('{')
                    && token
                        .chars()
                        .all(|c| c.is_alphanumeric() || c == '_' || c == ':')
                {
                    result.push((proc.name.clone(), token.to_string()));
                }
            }
        }

        result.sort();
        result.dedup();
        result
    }

    /// 找出未被任何其他 proc 调用的 proc（孤立 proc）
    pub fn find_unused_procs(&self, parse_result: &ParseResult) -> Vec<String> {
        let graph = self.analyze(parse_result);
        let mut unused: Vec<String> = graph
            .values()
            .filter(|node| node.callers.is_empty())
            .map(|node| node.callee.clone())
            .collect();
        unused.sort();
        unused
    }
}

impl Default for ProcAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

fn builtin_commands() -> Vec<&'static str> {
    vec![
        "set",
        "if",
        "else",
        "elseif",
        "while",
        "for",
        "foreach",
        "return",
        "break",
        "continue",
        "puts",
        "expr",
        "lappend",
        "lindex",
        "llength",
        "lset",
        "lrange",
        "lreplace",
        "lsearch",
        "lsort",
        "list",
        "string",
        "regexp",
        "regsub",
        "incr",
        "append",
        "format",
        "scan",
        "array",
        "dict",
        "info",
        "error",
        "catch",
        "uplevel",
        "upvar",
        "namespace",
        "package",
        "source",
        "global",
        "variable",
        "proc",
        "rename",
        "unset",
        "file",
        "open",
        "close",
        "read",
        "gets",
        "flush",
        "eof",
        "tell",
        "seek",
        "fconfigure",
        "encoding",
        "binary",
        "exec",
        "eval",
        "subst",
        "after",
        "update",
        "vwait",
        "clock",
        "cd",
        "pwd",
        "glob",
        "exit",
        "pid",
        "socket",
        "fileevent",
        "load",
        "tcl_findLibrary",
        "lassign",
        "tailcall",
        "throw",
        "try",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::TclParser;

    const TCL_WITH_CALLS: &str = r#"
proc helper {x} {
    return [expr {$x * 2}]
}

proc worker {a b} {
    set doubled [helper $a]
    return $doubled
}

proc main {} {
    worker 3 4
    puts "done"
}

proc orphan {} {
    return 0
}
"#;

    #[test]
    fn test_call_graph_callee() {
        let parser = TclParser::new();
        let result = parser.parse(TCL_WITH_CALLS);
        let analyzer = ProcAnalyzer::new();
        let graph = analyzer.analyze(&result);

        assert!(graph.contains_key("helper"));
        assert!(graph.contains_key("worker"));
        assert!(graph.contains_key("main"));
    }

    #[test]
    fn test_call_graph_callees() {
        let parser = TclParser::new();
        let result = parser.parse(TCL_WITH_CALLS);
        let analyzer = ProcAnalyzer::new();
        let graph = analyzer.analyze(&result);

        let worker_node = &graph["worker"];
        assert!(
            worker_node.callees.contains(&"helper".to_string()),
            "worker should call helper, got: {:?}",
            worker_node.callees
        );
    }

    #[test]
    fn test_call_graph_callers() {
        let parser = TclParser::new();
        let result = parser.parse(TCL_WITH_CALLS);
        let analyzer = ProcAnalyzer::new();
        let graph = analyzer.analyze(&result);

        let helper_node = &graph["helper"];
        assert!(
            helper_node.callers.contains(&"worker".to_string()),
            "helper should be called by worker, got: {:?}",
            helper_node.callers
        );
    }

    #[test]
    fn test_find_undefined() {
        let tcl = "proc test_proc {} {\n    undefined_proc 1 2\n    puts hello\n}\n";
        let parser = TclParser::new();
        let result = parser.parse(tcl);
        let analyzer = ProcAnalyzer::new();
        let undefined = analyzer.find_undefined_calls(&result);
        assert!(
            undefined
                .iter()
                .any(|(_, callee)| callee == "undefined_proc"),
            "should detect undefined_proc, got: {:?}",
            undefined
        );
    }

    #[test]
    fn test_find_unused_procs() {
        let parser = TclParser::new();
        let result = parser.parse(TCL_WITH_CALLS);
        let analyzer = ProcAnalyzer::new();
        let unused = analyzer.find_unused_procs(&result);
        // main and orphan are not called by any other proc
        assert!(
            unused.contains(&"main".to_string()) || unused.contains(&"orphan".to_string()),
            "should find unused procs, got: {:?}",
            unused
        );
    }

    #[test]
    fn test_empty_parse_result() {
        let parser = TclParser::new();
        let result = parser.parse("");
        let analyzer = ProcAnalyzer::new();
        let graph = analyzer.analyze(&result);
        assert!(graph.is_empty());
    }
}
