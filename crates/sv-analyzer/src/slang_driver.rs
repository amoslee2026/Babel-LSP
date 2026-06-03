//! slang/verilator CLI 驱动封装
//!
//! 优先尝试 slang，若不可用则回退到 verilator。

use std::process::Command;
use babel_lsp_core::diagnostic::Diagnostic;

use crate::diagnostics::DiagnosticParser;

/// slang + verilator 双后端驱动
pub struct SlangDriver {
    /// slang 可执行文件路径（可能不存在）
    slang_path: String,
    /// verilator 可执行文件路径
    verilator_path: String,
    parser: DiagnosticParser,
}

impl SlangDriver {
    pub fn new() -> Self {
        Self {
            slang_path: "slang".to_string(),
            verilator_path: "/usr/bin/verilator".to_string(),
            parser: DiagnosticParser::new(),
        }
    }

    pub fn with_paths(slang_path: String, verilator_path: String) -> Self {
        Self {
            slang_path,
            verilator_path,
            parser: DiagnosticParser::new(),
        }
    }

    /// 检查 slang 是否可用
    pub fn slang_available(&self) -> bool {
        Command::new(&self.slang_path)
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    /// 检查 verilator 是否可用
    pub fn verilator_available(&self) -> bool {
        Command::new(&self.verilator_path)
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    /// 对文件执行语法检查，返回诊断列表。
    /// 优先使用 slang，不可用时回退到 verilator。
    pub fn check_file(&self, path: &str) -> Vec<Diagnostic> {
        if self.slang_available() {
            self.check_with_slang(path)
        } else {
            self.check_with_verilator(path)
        }
    }

    /// 使用 slang 检查文件
    fn check_with_slang(&self, path: &str) -> Vec<Diagnostic> {
        let output = Command::new(&self.slang_path)
            .args(["--check-only", "--error-limit=0", path])
            .output();

        match output {
            Ok(out) => {
                let combined = format!(
                    "{}\n{}",
                    String::from_utf8_lossy(&out.stdout),
                    String::from_utf8_lossy(&out.stderr)
                );
                self.parser.parse_slang(&combined)
            },
            Err(e) => {
                tracing::warn!("slang 执行失败: {e}");
                vec![]
            },
        }
    }

    /// 使用 verilator 检查文件
    fn check_with_verilator(&self, path: &str) -> Vec<Diagnostic> {
        // verilator --lint-only 只做 lint，不生成代码
        let output = Command::new(&self.verilator_path)
            .args(["--lint-only", "-Wall", "--timing", path])
            .output();

        match output {
            Ok(out) => {
                let combined = format!(
                    "{}\n{}",
                    String::from_utf8_lossy(&out.stdout),
                    String::from_utf8_lossy(&out.stderr)
                );
                self.parser.parse_verilator(&combined)
            },
            Err(e) => {
                tracing::warn!("verilator 执行失败: {e}");
                vec![]
            },
        }
    }

    /// 使用 slang 生成 AST JSON（仅 slang 支持）。
    /// 若 slang 不可用则返回 None。
    pub fn parse_ast_json(&self, path: &str) -> Option<String> {
        if !self.slang_available() {
            return None;
        }

        let output = Command::new(&self.slang_path)
            .args(["--ast-json", "-", path])
            .output()
            .ok()?;

        if output.status.success() {
            Some(String::from_utf8_lossy(&output.stdout).into_owned())
        } else {
            None
        }
    }

    /// 解析 verilator 输出字符串为诊断列表（公开用于测试）
    pub fn parse_verilator_output(&self, output: &str) -> Vec<Diagnostic> {
        self.parser.parse_verilator(output)
    }

    /// 解析 slang 输出字符串为诊断列表（公开用于测试）
    pub fn parse_slang_output(&self, output: &str) -> Vec<Diagnostic> {
        self.parser.parse_slang(output)
    }
}

impl Default for SlangDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use babel_lsp_core::diagnostic::DiagnosticSeverity;

    #[test]
    fn test_parse_verilator_output() {
        let driver = SlangDriver::new();
        let output = "%Warning-UNDRIVEN: top.sv:3:1: Signal 'q' is not driven\n\
                      %Error: top.sv:7:4: Syntax error\n";
        let diags = driver.parse_verilator_output(output);
        assert_eq!(diags.len(), 2);
        assert_eq!(diags[0].severity, DiagnosticSeverity::Warning);
        assert_eq!(diags[1].severity, DiagnosticSeverity::Error);
        assert_eq!(diags[0].range.uri, "top.sv");
        assert_eq!(diags[0].range.start.line, 2); // 0-based
    }

    #[test]
    fn test_parse_slang_output() {
        let driver = SlangDriver::new();
        let output = "alu.sv:12:3: error: cannot assign to constant\n\
                      alu.sv:20:1: warning: implicit net 'x'\n";
        let diags = driver.parse_slang_output(output);
        assert_eq!(diags.len(), 2);
        assert_eq!(diags[0].severity, DiagnosticSeverity::Error);
        assert_eq!(diags[1].severity, DiagnosticSeverity::Warning);
        assert_eq!(diags[0].range.start.line, 11); // 0-based
        assert_eq!(diags[0].range.start.column, 2); // 0-based
    }

    #[test]
    fn test_verilator_available() {
        let driver = SlangDriver::new();
        // verilator 应该已安装
        assert!(driver.verilator_available());
    }

    #[test]
    fn test_parse_verilator_with_typename() {
        let driver = SlangDriver::new();
        let output = "%Warning-DECLFILENAME: obj_dir/Vtop.cpp:1:1: File 'Vtop.cpp' should be named 'Vtop.cpp'\n";
        let diags = driver.parse_verilator_output(output);
        // 该行格式不完全匹配标准，允许为空
        // 关键是不 panic
        let _ = diags;
    }

    #[test]
    fn test_parse_empty() {
        let driver = SlangDriver::new();
        assert!(driver.parse_verilator_output("").is_empty());
        assert!(driver.parse_slang_output("").is_empty());
    }
}
