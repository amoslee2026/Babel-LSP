//! babel-lsp 主入口
//!
//! 根据命令行参数选择运行模式：
//!   babel-lsp           — LSP stdio 模式（默认，供编辑器使用）
//!   babel-lsp --mcp     — MCP stdio 模式（供 Claude Code 使用）
//!
//! 日志输出：
//!   - 默认写入文件：.thanos/logs/babel-lsp.log（按日轮转）
//!   - BABEL_LOG=debug 启用 debug 级别
//!   - --no-log-file 禁用文件日志，输出到 stderr

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use babel_lsp_core::logging::{init_logging, LogMode, LoggingConfig};

#[derive(Parser)]
#[command(
    name = "babel-lsp",
    version,
    about = "HDL Language Server supporting SV/VHDL/TCL — LSP + MCP"
)]
struct Cli {
    /// 以 MCP 模式启动（默认为 LSP stdio 模式）
    #[arg(long)]
    mcp: bool,

    /// 日志级别 (error|warn|info|debug|trace)
    #[arg(long, default_value = "info")]
    log_level: String,

    /// 日志文件目录（默认：.thanos/logs/）
    #[arg(long)]
    log_dir: Option<PathBuf>,

    /// 禁用文件日志，输出到 stderr
    #[arg(long)]
    no_log_file: bool,
}

impl Cli {
    /// 解析日志级别，环境变量 BABEL_LOG 优先
    fn resolve_log_level(&self) -> String {
        std::env::var("BABEL_LOG")
            .or_else(|_| std::env::var("RUST_LOG"))
            .unwrap_or_else(|_| self.log_level.clone())
    }

    /// 解析日志目录
    fn resolve_log_dir(&self) -> Option<PathBuf> {
        if self.no_log_file {
            return None;
        }

        // 命令行参数优先
        if let Some(dir) = &self.log_dir {
            return Some(dir.clone());
        }

        // 默认：当前目录下的 .thanos/logs/
        let default_dir = PathBuf::from(".thanos/logs");
        if let Err(e) = std::fs::create_dir_all(&default_dir) {
            eprintln!("无法创建日志目录 {:?}: {}", default_dir, e);
            return None;
        }
        Some(default_dir)
    }

    /// 构建 LoggingConfig
    fn build_logging_config(&self) -> LoggingConfig {
        let level = self.resolve_log_level();
        let file_path = self.resolve_log_dir();
        let mode = if self.no_log_file {
            LogMode::Debug // 输出到终端
        } else {
            LogMode::Normal // 输出到文件
        };

        LoggingConfig {
            level,
            file_path: file_path.map(|p| p.to_string_lossy().to_string()),
            mode,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // 初始化日志系统
    let logging_config = cli.build_logging_config();
    init_logging(&logging_config)?;

    if cli.mcp {
        tracing::info!("babel-lsp starting in MCP mode");
        babel_lsp_mcp::run_stdio().await?;
    } else {
        tracing::info!("babel-lsp starting in LSP stdio mode");
        babel_lsp_lsp::backend::run_stdio().await;
    }

    Ok(())
}
