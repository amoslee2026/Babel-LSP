//! 错误类型定义

use std::path::PathBuf;
use thiserror::Error;

/// 解析错误
#[derive(Debug, Error)]
pub enum ParseError {
    /// 循环引用检测
    #[error("Circular reference detected: {0}")]
    CircularReference(PathBuf),

    /// 文件未找到
    #[error("File not found: {0}")]
    FileNotFound(PathBuf),

    /// 嵌套 filelist 解析失败
    #[error("Nested filelist parse failed: {path}, reason: {reason}")]
    NestedFailed { path: PathBuf, reason: String },

    /// IO 错误
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// 超过最大递归深度
    #[error("Exceeded maximum recursion depth: {0}")]
    MaxDepthExceeded(u32),

    /// 环境变量展开失败
    #[error("Environment variable expansion failed: {0}")]
    EnvExpansionFailed(String),
}
