#![allow(non_snake_case)]
//! SystemVerilog/Verilog 语言分析器

pub mod ast_json;
pub mod completion;
pub mod definition;
pub mod diagnostics;
pub mod formatter;
pub mod hover;
pub mod slang_driver;
pub mod slang_ffi;
pub mod symbol_collector;
pub mod synth_checker;
