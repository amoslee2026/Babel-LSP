//! MCP 协议处理层
#![allow(non_snake_case)]

pub mod server;
pub mod tools;

pub use server::{run_stdio, BabelMcpServer};
