//! TCL 行解析器（正则 + 括号计数，不依赖 tree-sitter-tcl）

/// TCL proc 定义
#[derive(Debug, Clone, PartialEq)]
pub struct TclProc {
    pub name: String,
    pub args: Vec<String>,
    pub start_line: u32,
    pub end_line: u32,
    pub body: String,
}

/// TCL 变量
#[derive(Debug, Clone, PartialEq)]
pub struct TclVariable {
    pub name: String,
    pub value: Option<String>,
    pub line: u32,
    /// "global" 或 proc 名称
    pub scope: String,
}

/// 解析结果
#[derive(Debug, Clone)]
pub struct ParseResult {
    pub procs: Vec<TclProc>,
    pub variables: Vec<TclVariable>,
    /// `source` 引入的文件
    pub sources: Vec<String>,
    /// `package require` 的包名
    pub packages: Vec<String>,
    /// (行号, 注释文本)
    pub comments: Vec<(u32, String)>,
}

impl ParseResult {
    pub fn new() -> Self {
        Self {
            procs: Vec::new(),
            variables: Vec::new(),
            sources: Vec::new(),
            packages: Vec::new(),
            comments: Vec::new(),
        }
    }
}

impl Default for ParseResult {
    fn default() -> Self {
        Self::new()
    }
}

/// TCL 解析器
pub struct TclParser;

impl TclParser {
    pub fn new() -> Self {
        Self
    }

    /// 解析 TCL 源代码，返回解析结果
    pub fn parse(&self, content: &str) -> ParseResult {
        let mut result = ParseResult::new();
        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0usize;

        while i < lines.len() {
            let line_no = i as u32;
            let trimmed = lines[i].trim();

            // 注释行
            if let Some(stripped) = trimmed.strip_prefix('#') {
                result.comments.push((line_no, stripped.trim().to_string()));
                i += 1;
                continue;
            }

            // proc 定义
            if let Some(rest) = strip_cmd(trimmed, "proc") {
                // 解析 proc name args body（可能跨多行）
                let (proc_def, consumed) = collect_proc(&lines, i, rest);
                if let Some(p) = proc_def {
                    result.procs.push(p);
                    i += consumed;
                } else {
                    i += 1;
                }
                continue;
            }

            // set varname value
            if let Some(rest) = strip_cmd(trimmed, "set") {
                let scope = current_scope_at_line(&result.procs, line_no);
                if let Some(var) = parse_set(rest, line_no, &scope) {
                    result.variables.push(var);
                }
                i += 1;
                continue;
            }

            // variable varname [value]
            if let Some(rest) = strip_cmd(trimmed, "variable") {
                let scope = current_scope_at_line(&result.procs, line_no);
                let parts = split_tokens(rest);
                if !parts.is_empty() {
                    let value = if parts.len() > 1 {
                        Some(parts[1..].join(" "))
                    } else {
                        None
                    };
                    result.variables.push(TclVariable {
                        name: parts[0].clone(),
                        value,
                        line: line_no,
                        scope,
                    });
                }
                i += 1;
                continue;
            }

            // global varname [varname ...]
            if let Some(rest) = strip_cmd(trimmed, "global") {
                let parts = split_tokens(rest);
                for name in parts {
                    if !name.is_empty() {
                        result.variables.push(TclVariable {
                            name,
                            value: None,
                            line: line_no,
                            scope: "global".to_string(),
                        });
                    }
                }
                i += 1;
                continue;
            }

            // source path
            if let Some(rest) = strip_cmd(trimmed, "source") {
                let path = rest
                    .trim()
                    .trim_matches('"')
                    .trim_matches('{')
                    .trim_matches('}')
                    .to_string();
                if !path.is_empty() {
                    result.sources.push(path);
                }
                i += 1;
                continue;
            }

            // package require pkgname [version]
            if let Some(rest) = strip_cmd(trimmed, "package") {
                let parts = split_tokens(rest);
                if parts.first().map(|s| s.as_str()) == Some("require") && parts.len() >= 2 {
                    result.packages.push(parts[1].clone());
                }
                i += 1;
                continue;
            }

            i += 1;
        }

        result
    }
}

impl Default for TclParser {
    fn default() -> Self {
        Self::new()
    }
}

// ───────────────────── helpers ─────────────────────

/// 检查行是否以 `cmd` 开头（大小写敏感），若是则返回其后的字符串
fn strip_cmd<'a>(line: &'a str, cmd: &str) -> Option<&'a str> {
    if let Some(rest) = line.strip_prefix(cmd) {
        if rest.is_empty() || rest.starts_with(' ') || rest.starts_with('\t') {
            return Some(rest.trim_start());
        }
    }
    None
}

/// 从 `set` 行的剩余部分解析变量名和值
fn parse_set(rest: &str, line: u32, scope: &str) -> Option<TclVariable> {
    let tokens = split_tokens(rest);
    if tokens.is_empty() {
        return None;
    }
    let name = tokens[0].clone();
    if name.is_empty() || name.starts_with('-') {
        return None;
    }
    let value = if tokens.len() > 1 {
        Some(tokens[1..].join(" "))
    } else {
        None
    };
    Some(TclVariable {
        name,
        value,
        line,
        scope: scope.to_string(),
    })
}

/// 简单 token 分割（不处理嵌套括号和引号内的空格，够用）
fn split_tokens(s: &str) -> Vec<String> {
    let s = s.trim();
    if s.is_empty() {
        return Vec::new();
    }

    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut depth_brace = 0i32;
    let mut depth_bracket = 0i32;
    let mut in_quote = false;

    for ch in s.chars() {
        match ch {
            '"' if depth_brace == 0 && depth_bracket == 0 => {
                in_quote = !in_quote;
            },
            '{' if !in_quote => {
                depth_brace += 1;
                current.push(ch);
            },
            '}' if !in_quote => {
                depth_brace -= 1;
                current.push(ch);
            },
            '[' if !in_quote => {
                depth_bracket += 1;
                current.push(ch);
            },
            ']' if !in_quote => {
                depth_bracket -= 1;
                current.push(ch);
            },
            ' ' | '\t' if !in_quote && depth_brace == 0 && depth_bracket == 0 => {
                if !current.is_empty() {
                    tokens.push(current.trim_matches('{').trim_matches('}').to_string());
                    current = String::new();
                }
            },
            _ => {
                current.push(ch);
            },
        }
    }
    if !current.is_empty() {
        tokens.push(current.trim_matches('{').trim_matches('}').to_string());
    }
    tokens
}

/// 计算行内花括号净增量
fn brace_delta(line: &str) -> i32 {
    let mut depth = 0i32;
    let mut in_quote = false;
    let mut prev_backslash = false;
    for ch in line.chars() {
        if prev_backslash {
            prev_backslash = false;
            continue;
        }
        match ch {
            '\\' => prev_backslash = true,
            '"' => in_quote = !in_quote,
            '{' if !in_quote => depth += 1,
            '}' if !in_quote => depth -= 1,
            _ => {},
        }
    }
    depth
}

/// 解析 proc 定义，可能跨多行（花括号计数）
/// 返回 (Option<TclProc>, consumed_lines_count)
fn collect_proc(lines: &[&str], start: usize, first_rest: &str) -> (Option<TclProc>, usize) {
    let tokens = split_tokens(first_rest);
    if tokens.is_empty() {
        return (None, 1);
    }
    let name = tokens[0].clone();

    // 从 first_rest 提取 args 字符串（第二个 token）
    let args_str = if tokens.len() > 1 {
        tokens[1].clone()
    } else {
        String::new()
    };
    let args: Vec<String> = split_tokens(&args_str);

    // 收集整个 proc 文本，直到花括号平衡
    let mut full_text = lines[start].to_string();
    let mut depth: i32 = brace_delta(lines[start]);
    let mut end = start;
    while depth > 0 && end + 1 < lines.len() {
        end += 1;
        full_text.push('\n');
        full_text.push_str(lines[end]);
        depth += brace_delta(lines[end]);
    }
    let consumed = end - start + 1;

    // 从完整文本中提取 body（跳过 name 和 args 块，取第三个 {…} 块）
    let body = extract_body_from_full(&full_text).unwrap_or_default();

    let proc = TclProc {
        name,
        args,
        start_line: start as u32,
        end_line: end as u32,
        body,
    };
    (Some(proc), consumed)
}

/// 从完整 proc 文本中提取 body 内容（第三个花括号块的内容）
///
/// 策略：跳过 "proc name " 之后，找第一个顶级 { } 块（args），
/// 再找第二个顶级 { } 块（body），返回其内部文本。
fn extract_body_from_full(full: &str) -> Option<String> {
    let chars: Vec<char> = full.chars().collect();
    let n = chars.len();
    let mut i = 0;

    // 跳过 "proc" 关键字
    while i < n && (chars[i] == ' ' || chars[i] == '\t' || chars[i] == '\n') {
        i += 1;
    }
    // 跳过 "proc"
    if i + 4 <= n && chars[i..i + 4].iter().collect::<String>() == "proc" {
        i += 4;
    }
    // 跳过空格
    while i < n && (chars[i] == ' ' || chars[i] == '\t') {
        i += 1;
    }
    // 跳过 proc name（非空白字符序列）
    while i < n && chars[i] != ' ' && chars[i] != '\t' && chars[i] != '\n' {
        i += 1;
    }
    // 跳过空格
    while i < n && (chars[i] == ' ' || chars[i] == '\t') {
        i += 1;
    }

    // 现在 i 指向 args 块（可能是 "{args}" 或 "arg1" 等）
    // 跳过 args 块（第一个顶级 { } 或单个 token）
    if i < n && chars[i] == '{' {
        // 找到匹配的 }
        let mut depth = 0i32;
        while i < n {
            match chars[i] {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        i += 1;
                        break;
                    }
                },
                _ => {},
            }
            i += 1;
        }
    } else {
        // 没有花括号的 args（如 "a b c"），跳到下一个空格
        while i < n && chars[i] != ' ' && chars[i] != '\t' && chars[i] != '\n' {
            i += 1;
        }
    }

    // 跳过空格和换行，找到 body 开始的 '{'
    while i < n && (chars[i] == ' ' || chars[i] == '\t' || chars[i] == '\n' || chars[i] == '\r') {
        i += 1;
    }

    if i >= n || chars[i] != '{' {
        return None;
    }

    // 现在 i 指向 body 的 '{'，提取内部内容
    let body_brace_start = i;
    i += 1; // skip opening '{'
    let body_content_start = i;

    let mut depth = 1i32;
    while i < n {
        match chars[i] {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    // chars[body_content_start..i] is the body
                    let body: String = chars[body_content_start..i].iter().collect();
                    let _ = body_brace_start; // suppress warning
                    return Some(body.trim().to_string());
                }
            },
            _ => {},
        }
        i += 1;
    }
    None
}

/// 根据已解析的 procs，判断某行号处于哪个 proc 的 scope（否则为 "global"）
fn current_scope_at_line(procs: &[TclProc], line: u32) -> String {
    for p in procs {
        if line >= p.start_line && line <= p.end_line {
            return p.name.clone();
        }
    }
    "global".to_string()
}

// ──────────────── proc body call extraction ────────────────

/// 从 proc body 中提取被调用的 proc 名列表。
/// 扫描每行的顶层命令和 `[cmd ...]` 命令替换内的命令名。
pub fn extract_calls_from_body(body: &str, known_procs: &[String]) -> Vec<String> {
    let mut calls = Vec::new();
    let known_set: std::collections::HashSet<&str> =
        known_procs.iter().map(|s| s.as_str()).collect();

    for line in body.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        // 提取该行中所有命令起始位置的第一个 token
        for candidate in extract_command_names(trimmed) {
            if known_set.contains(candidate.as_str()) {
                calls.push(candidate);
            }
        }
    }
    calls.sort();
    calls.dedup();
    calls
}

/// 从一行 TCL 代码中提取所有命令调用位置的命令名（顶层 + [cmd ...] 内）
fn extract_command_names(line: &str) -> Vec<String> {
    let mut result = Vec::new();
    let builtins: &[&str] = &[
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
        "lassign",
        "tailcall",
        "throw",
        "try",
        "return",
    ];

    let chars: Vec<char> = line.chars().collect();
    let n = chars.len();
    let mut i = 0;

    // 收集候选命令名的位置：行首 + 每个 '[' 之后
    while i < n {
        // 跳过空白
        while i < n && (chars[i] == ' ' || chars[i] == '\t') {
            i += 1;
        }
        if i >= n {
            break;
        }
        if chars[i] == '[' {
            // 进入命令替换：跳过 '[' 然后读取命令名
            i += 1;
            while i < n && (chars[i] == ' ' || chars[i] == '\t') {
                i += 1;
            }
            let start = i;
            while i < n && chars[i] != ' ' && chars[i] != '\t' && chars[i] != ']' && chars[i] != '['
            {
                i += 1;
            }
            let name: String = chars[start..i].iter().collect();
            if !name.is_empty() && !builtins.contains(&name.as_str()) {
                result.push(name);
            }
            // 继续扫描（不跳到 ']'，让外层循环继续）
        } else if chars[i] == ']' || chars[i] == '}' || chars[i] == '{' {
            i += 1;
        } else if chars[i] == '#' {
            break; // 注释，跳过本行
        } else {
            // 顶层命令：读取第一个 token
            let start = i;
            while i < n && chars[i] != ' ' && chars[i] != '\t' && chars[i] != '[' && chars[i] != ']'
            {
                i += 1;
            }
            let name: String = chars[start..i].iter().collect();
            if !name.is_empty()
                && !name.starts_with('$')
                && !name.starts_with('{')
                && !builtins.contains(&name.as_str())
            {
                result.push(name);
            }
            // 跳到行尾（只取第一个顶层命令名）
            // actually continue scanning for [ ... ] substitutions
        }
    }

    result
}
#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_TCL: &str = r#"
# This is a comment
proc add {a b} {
    return [expr {$a + $b}]
}

proc greet {name} {
    puts "Hello $name"
}

set x 42
set y "hello world"
global g_var
source utils.tcl
package require Vivado 1.1
variable myvar 100
"#;

    #[test]
    fn test_parse_comments() {
        let parser = TclParser::new();
        let result = parser.parse(SIMPLE_TCL);
        assert!(!result.comments.is_empty(), "should find comments");
        assert!(result.comments.iter().any(|(_, c)| c.contains("comment")));
    }

    #[test]
    fn test_parse_procs() {
        let parser = TclParser::new();
        let result = parser.parse(SIMPLE_TCL);
        assert_eq!(result.procs.len(), 2, "should find 2 procs");
        let names: Vec<&str> = result.procs.iter().map(|p| p.name.as_str()).collect();
        assert!(names.contains(&"add"));
        assert!(names.contains(&"greet"));
    }

    #[test]
    fn test_parse_proc_args() {
        let parser = TclParser::new();
        let result = parser.parse(SIMPLE_TCL);
        let add = result.procs.iter().find(|p| p.name == "add").unwrap();
        assert_eq!(add.args, vec!["a", "b"]);
    }

    #[test]
    fn test_parse_variable() {
        let parser = TclParser::new();
        let result = parser.parse(SIMPLE_TCL);
        let vars: Vec<&str> = result.variables.iter().map(|v| v.name.as_str()).collect();
        assert!(vars.contains(&"x"), "should find x");
        assert!(vars.contains(&"y"), "should find y");
        assert!(vars.contains(&"g_var"), "should find global g_var");
        assert!(vars.contains(&"myvar"), "should find myvar");
    }

    #[test]
    fn test_parse_set_value() {
        let parser = TclParser::new();
        let result = parser.parse(SIMPLE_TCL);
        let x = result.variables.iter().find(|v| v.name == "x").unwrap();
        assert_eq!(x.value, Some("42".to_string()));
    }

    #[test]
    fn test_parse_source() {
        let parser = TclParser::new();
        let result = parser.parse(SIMPLE_TCL);
        assert!(result.sources.contains(&"utils.tcl".to_string()));
    }

    #[test]
    fn test_parse_package() {
        let parser = TclParser::new();
        let result = parser.parse(SIMPLE_TCL);
        assert!(result.packages.contains(&"Vivado".to_string()));
    }

    #[test]
    fn test_multiline_proc() {
        let tcl =
            "proc complicated {x y z} {\n    set sum [expr {$x + $y + $z}]\n    return $sum\n}\n";
        let parser = TclParser::new();
        let result = parser.parse(tcl);
        assert_eq!(result.procs.len(), 1);
        let p = &result.procs[0];
        assert_eq!(p.name, "complicated");
        assert_eq!(p.args, vec!["x", "y", "z"]);
        assert!(
            p.start_line < p.end_line,
            "multi-line proc should span multiple lines"
        );
    }

    #[test]
    fn test_proc_line_numbers() {
        let tcl = "proc foo {} {\n    return 1\n}\n";
        let parser = TclParser::new();
        let result = parser.parse(tcl);
        assert_eq!(result.procs[0].start_line, 0);
        assert_eq!(result.procs[0].end_line, 2);
    }

    #[test]
    fn test_empty_input() {
        let parser = TclParser::new();
        let result = parser.parse("");
        assert!(result.procs.is_empty());
        assert!(result.variables.is_empty());
    }

    #[test]
    fn test_global_variable_scope() {
        let tcl = "global myvar\n";
        let parser = TclParser::new();
        let result = parser.parse(tcl);
        let v = result.variables.iter().find(|v| v.name == "myvar").unwrap();
        assert_eq!(v.scope, "global");
    }

    #[test]
    fn test_extract_calls_from_body() {
        let body = "    helper_proc $x\n    puts $result\n    another_proc 1 2\n";
        let known = vec![
            "helper_proc".to_string(),
            "another_proc".to_string(),
            "unused_proc".to_string(),
        ];
        let calls = extract_calls_from_body(body, &known);
        assert!(calls.contains(&"helper_proc".to_string()));
        assert!(calls.contains(&"another_proc".to_string()));
        assert!(!calls.contains(&"unused_proc".to_string()));
    }

    #[test]
    fn test_strip_cmd_with_tab() {
        // 测试 tab 分隔的命令
        let line = "proc\tfoo {}";
        let result = strip_cmd(line, "proc");
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "foo {}");
    }

    #[test]
    fn test_strip_cmd_without_space() {
        // 测试命令后没有空格的情况
        let line = "procfoo";
        let result = strip_cmd(line, "proc");
        assert!(result.is_none(), "procfoo 不应以 proc 命令解析");
    }

    #[test]
    fn test_strip_cmd_exact_match() {
        // 测试精确匹配
        let line = "proc";
        let result = strip_cmd(line, "proc");
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "");
    }

    #[test]
    fn test_parse_default_result() {
        let result = ParseResult::default();
        assert!(result.procs.is_empty());
        assert!(result.variables.is_empty());
        assert!(result.sources.is_empty());
        assert!(result.packages.is_empty());
        assert!(result.comments.is_empty());
    }
}
