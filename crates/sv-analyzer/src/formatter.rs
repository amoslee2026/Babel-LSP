//! verible-verilog-format 封装
//!
//! 若 verible 不可用则原样返回输入内容。

use std::io::Write;
use std::process::{Command, Stdio};

/// 格式化器
pub struct Formatter {
    verible_path: String,
}

impl Formatter {
    pub fn new() -> Self {
        Self {
            verible_path: "verible-verilog-format".to_string(),
        }
    }

    pub fn with_path(path: String) -> Self {
        Self { verible_path: path }
    }

    /// 检查 verible-verilog-format 是否可用
    pub fn is_available(&self) -> bool {
        Command::new(&self.verible_path)
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    /// 格式化 SV/Verilog 源码
    ///
    /// - 若 verible 可用：写入临时文件 → 运行格式化 → 读取结果
    /// - 若 verible 不可用：原样返回内容
    pub fn format(&self, content: &str) -> anyhow::Result<String> {
        if !self.is_available() {
            tracing::debug!("verible-verilog-format 不可用，跳过格式化");
            return Ok(content.to_string());
        }

        // 写入临时文件（verible 不支持 stdin 格式化）
        let mut tmp = tempfile::Builder::new().suffix(".sv").tempfile()?;
        tmp.write_all(content.as_bytes())?;
        let tmp_path = tmp.path().to_owned();

        // 运行 verible-verilog-format --inplace
        let status = Command::new(&self.verible_path)
            .arg("--inplace")
            .arg(&tmp_path)
            .status()?;

        if !status.success() {
            // 格式化失败时返回原内容（不报错，保持容错性��
            tracing::warn!("verible-verilog-format 返回非零退出码，使用原始内容");
            return Ok(content.to_string());
        }

        // 读取格式化后的内容
        let formatted = std::fs::read_to_string(&tmp_path)?;
        Ok(formatted)
    }

    /// 通过 stdin/stdout 管道格式化（备用方案）
    pub fn format_via_pipe(&self, content: &str) -> anyhow::Result<String> {
        if !self.is_available() {
            return Ok(content.to_string());
        }

        let mut child = Command::new(&self.verible_path)
            .arg("-")  // 从 stdin 读取
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        if let Some(stdin) = child.stdin.take() {
            let mut stdin = stdin;
            stdin.write_all(content.as_bytes())?;
        }

        let output = child.wait_with_output()?;
        if output.status.success() && !output.stdout.is_empty() {
            Ok(String::from_utf8_lossy(&output.stdout).into_owned())
        } else {
            Ok(content.to_string())
        }
    }
}

impl Default for Formatter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_SV: &str = "module  top  ( input logic clk , output logic q ) ;\nendmodule\n";

    #[test]
    fn test_format_unavailable_returns_original() {
        // 使用不存在的路径，确保 is_available() 返回 false
        let formatter = Formatter::with_path("/nonexistent/verible-verilog-format".to_string());
        assert!(!formatter.is_available());
        let result = formatter.format(SAMPLE_SV).unwrap();
        assert_eq!(result, SAMPLE_SV);
    }

    #[test]
    fn test_format_empty_content() {
        let formatter = Formatter::with_path("/nonexistent/verible-verilog-format".to_string());
        let result = formatter.format("").unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_is_available_nonexistent() {
        let formatter = Formatter::with_path("/dev/null/nonexistent".to_string());
        assert!(!formatter.is_available());
    }

    #[test]
    fn test_format_with_verible_if_available() {
        let formatter = Formatter::new();
        // 不论 verible 是否可用，都应返回 Ok，内容非空
        let result = formatter.format(SAMPLE_SV);
        assert!(result.is_ok());
        let formatted = result.unwrap();
        assert!(!formatted.is_empty());
    }

    #[test]
    fn test_format_via_pipe_unavailable_returns_original() {
        let formatter = Formatter::with_path("/nonexistent/verible-verilog-format".to_string());
        let result = formatter.format_via_pipe(SAMPLE_SV).unwrap();
        assert_eq!(result, SAMPLE_SV);
    }

    #[test]
    fn test_format_via_pipe_empty_content() {
        let formatter = Formatter::with_path("/nonexistent/verible-verilog-format".to_string());
        let result = formatter.format_via_pipe("").unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_format_via_pipe_with_verible_if_available() {
        let formatter = Formatter::new();
        // 不论 verible 是否可用，都应返回 Ok
        let result = formatter.format_via_pipe(SAMPLE_SV);
        assert!(result.is_ok());
        let formatted = result.unwrap();
        assert!(!formatted.is_empty());
    }

    #[test]
    fn test_default_formatter() {
        let formatter = Formatter::default();
        assert_eq!(formatter.verible_path, "verible-verilog-format");
    }

    #[test]
    fn test_with_path() {
        let formatter = Formatter::with_path("/custom/path/verible".to_string());
        assert_eq!(formatter.verible_path, "/custom/path/verible");
    }
}
