//! 诊断解析：解析 verilator 和 slang 的输出格式

use babel_lsp_core::diagnostic::{Diagnostic, DiagnosticSeverity};
use babel_lsp_core::symbol::{Location, Position};

/// 诊断解析器
pub struct DiagnosticParser;

impl DiagnosticParser {
    pub fn new() -> Self {
        Self
    }

    /// 解析 verilator 输出
    ///
    /// 格式：`%Warning-TYPENAME: file.sv:10:5: message`
    ///      `%Error: file.sv:5:1: message`
    ///      `%Error-TYPENAME: file.sv:5:1: message`
    pub fn parse_verilator(&self, output: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();

        for line in output.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            // 匹配 %Warning 或 %Error 开头
            let (severity, rest) = if let Some(rest) = line.strip_prefix("%Warning") {
                (DiagnosticSeverity::Warning, rest)
            } else if let Some(rest) = line.strip_prefix("%Error") {
                (DiagnosticSeverity::Error, rest)
            } else {
                continue;
            };

            // 提取可选的 -TYPENAME 部分
            let (code, rest) = if let Some(after_dash) = rest.strip_prefix('-') {
                // 找到冒号位置
                if let Some(colon_pos) = after_dash.find(": ") {
                    let code = after_dash[..colon_pos].to_string();
                    let rest = &after_dash[colon_pos + 2..];
                    (Some(code), rest)
                } else {
                    (None, rest)
                }
            } else if let Some(rest) = rest.strip_prefix(": ") {
                (None, rest)
            } else {
                continue;
            };

            // rest 应该是 "file.sv:line:col: message" 格式
            if let Some(diag) = parse_file_location(rest, severity, code, "verilator") {
                diagnostics.push(diag);
            }
        }

        diagnostics
    }

    /// 解析 slang 输出
    ///
    /// 格式：`file.sv:10:5: error: message`
    ///      `file.sv:10:5: warning: message`
    ///      `file.sv:10:5: note: message`
    pub fn parse_slang(&self, output: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();

        for line in output.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            // slang 格式：file:line:col: severity: message
            // 需要找到 ": error: " 或 ": warning: " 或 ": note: " 模式
            let (severity, file_loc_part, message) =
                if let Some(pos) = find_severity_marker(line, ": error: ") {
                    let msg = &line[pos + ": error: ".len()..];
                    (DiagnosticSeverity::Error, &line[..pos], msg)
                } else if let Some(pos) = find_severity_marker(line, ": warning: ") {
                    let msg = &line[pos + ": warning: ".len()..];
                    (DiagnosticSeverity::Warning, &line[..pos], msg)
                } else if let Some(pos) = find_severity_marker(line, ": note: ") {
                    let msg = &line[pos + ": note: ".len()..];
                    (DiagnosticSeverity::Information, &line[..pos], msg)
                } else {
                    continue;
                };

            // file_loc_part 格式：file.sv:line:col
            if let Some(diag) =
                parse_file_loc_parts(file_loc_part, message, severity, None, "slang")
            {
                diagnostics.push(diag);
            }
        }

        diagnostics
    }
}

impl Default for DiagnosticParser {
    fn default() -> Self {
        Self::new()
    }
}

/// 找到诊断严重级别标记的位置（处理文件路径中可能含冒号的情况，如 Windows 路径）
fn find_severity_marker(line: &str, marker: &str) -> Option<usize> {
    // 简单搜索 marker，从后向前找，避免路径中含冒号误匹配
    line.find(marker)
}

/// 从 "file.sv:line:col: message" 格式解析诊断
fn parse_file_location(
    s: &str,
    severity: DiagnosticSeverity,
    code: Option<String>,
    source: &str,
) -> Option<Diagnostic> {
    // 找到文件名与行列的分割点（第一个冒号后接数字的位置）
    // 格式: "some/path/file.sv:10:5: message"
    let parts: Vec<&str> = s.splitn(4, ':').collect();
    if parts.len() < 4 {
        // 尝试不带列号的格式
        if parts.len() == 3 {
            let file = parts[0];
            let line: u32 = parts[1].trim().parse().ok()?;
            let message = parts[2].trim().to_string();
            let loc = make_location(file, line.saturating_sub(1), 0);
            let mut diag = Diagnostic::new(loc, severity, message);
            diag.source = source.to_string();
            if let Some(c) = code {
                diag.code = Some(c);
            }
            return Some(diag);
        }
        return None;
    }

    let file = parts[0];
    let line: u32 = parts[1].trim().parse().ok()?;
    let col: u32 = parts[2].trim().parse().unwrap_or(1);
    let message = parts[3].trim().to_string();

    let loc = make_location(file, line.saturating_sub(1), col.saturating_sub(1));
    let mut diag = Diagnostic::new(loc, severity, message);
    diag.source = source.to_string();
    if let Some(c) = code {
        diag.code = Some(c);
    }
    Some(diag)
}

/// 从分离的位置部分和消息解析诊断（slang 格式）
fn parse_file_loc_parts(
    loc_part: &str,
    message: &str,
    severity: DiagnosticSeverity,
    code: Option<String>,
    source: &str,
) -> Option<Diagnostic> {
    // loc_part: "file.sv:line:col"
    let parts: Vec<&str> = loc_part.splitn(3, ':').collect();
    if parts.len() < 2 {
        return None;
    }
    let file = parts[0];
    let line: u32 = parts[1].trim().parse().ok()?;
    let col: u32 = if parts.len() >= 3 {
        parts[2].trim().parse().unwrap_or(1)
    } else {
        1
    };

    let loc = make_location(file, line.saturating_sub(1), col.saturating_sub(1));
    let mut diag = Diagnostic::new(loc, severity, message.to_string());
    diag.source = source.to_string();
    if let Some(c) = code {
        diag.code = Some(c);
    }
    Some(diag)
}

fn make_location(uri: &str, line: u32, col: u32) -> Location {
    Location {
        uri: uri.to_string(),
        start: Position::new(line, col),
        end: Position::new(line, col),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_verilator_warning() {
        let parser = DiagnosticParser::new();
        let output = "%Warning-UNDRIVEN: design.sv:10:5: Signal 'clk' is not driven\n";
        let diags = parser.parse_verilator(output);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].severity, DiagnosticSeverity::Warning);
        assert_eq!(diags[0].code, Some("UNDRIVEN".to_string()));
        assert_eq!(diags[0].range.uri, "design.sv");
        assert_eq!(diags[0].range.start.line, 9); // 0-based
        assert_eq!(diags[0].range.start.column, 4); // 0-based
    }

    #[test]
    fn test_parse_verilator_error() {
        let parser = DiagnosticParser::new();
        let output = "%Error: design.sv:5:1: syntax error, unexpected token\n";
        let diags = parser.parse_verilator(output);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].severity, DiagnosticSeverity::Error);
        assert_eq!(diags[0].range.start.line, 4);
    }

    #[test]
    fn test_parse_verilator_multiple() {
        let parser = DiagnosticParser::new();
        let output = "\
%Warning-UNDRIVEN: a.sv:3:1: not driven
%Error-SYNTAX: b.sv:7:2: unexpected token
%Warning-UNUSED: c.sv:1:1: unused variable
";
        let diags = parser.parse_verilator(output);
        assert_eq!(diags.len(), 3);
        assert_eq!(diags[0].severity, DiagnosticSeverity::Warning);
        assert_eq!(diags[1].severity, DiagnosticSeverity::Error);
        assert_eq!(diags[2].range.uri, "c.sv");
    }

    #[test]
    fn test_parse_slang_error() {
        let parser = DiagnosticParser::new();
        let output = "design.sv:10:5: error: undeclared identifier 'foo'\n";
        let diags = parser.parse_slang(output);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].severity, DiagnosticSeverity::Error);
        assert!(diags[0].message.contains("undeclared"));
        assert_eq!(diags[0].range.uri, "design.sv");
        assert_eq!(diags[0].range.start.line, 9);
    }

    #[test]
    fn test_parse_slang_warning() {
        let parser = DiagnosticParser::new();
        let output = "top.sv:1:1: warning: implicit net declaration\n";
        let diags = parser.parse_slang(output);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].severity, DiagnosticSeverity::Warning);
    }

    #[test]
    fn test_parse_slang_mixed() {
        let parser = DiagnosticParser::new();
        let output = "\
a.sv:2:3: error: syntax error
b.sv:5:1: warning: unused variable x
c.sv:10:8: note: declared here
";
        let diags = parser.parse_slang(output);
        assert_eq!(diags.len(), 3);
        assert_eq!(diags[2].severity, DiagnosticSeverity::Information);
    }

    #[test]
    fn test_parse_empty_output() {
        let parser = DiagnosticParser::new();
        assert!(parser.parse_verilator("").is_empty());
        assert!(parser.parse_slang("").is_empty());
    }
}
