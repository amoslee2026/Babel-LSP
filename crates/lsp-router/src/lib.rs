//! LSP 协议处理层
#![allow(non_snake_case)]

pub mod backend;
pub mod handlers;

pub use backend::ThanosLspBackend;
