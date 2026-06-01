//! thanosLSP 核心共享层
//!
//! 提供配置解析、文档状态管理、符号类型、诊断、文件存储、项目索引等核心功能。
#![allow(non_snake_case)]

pub mod config;
pub mod cross_lang;
pub mod diagnostic;
pub mod document;
pub mod file_classifier;
pub mod file_store;
pub mod logging;
pub mod project_index;
pub mod project_memory;
pub mod symbol;

// 重导出核心类型
pub use config::{HdlConfig, ProjectConfig, TclConfig, VhdlConfig};
pub use diagnostic::{Diagnostic, DiagnosticSeverity};
pub use document::{DocumentState, FileClass, Language};
pub use symbol::{Location, Symbol, SymbolKind};
