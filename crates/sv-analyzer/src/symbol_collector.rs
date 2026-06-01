//! 基于正则表达式的 SystemVerilog/Verilog 符号收集器
//!
//! 不依赖 AST（因为 slang 可能不可用），直接解析文本。

use regex::Regex;
use smol_str::SmolStr;
use std::sync::OnceLock;
use thanosLSP_core::symbol::{Location, Position, Symbol, SymbolKind};

/// 所有正则表达式的全局编译缓存
struct Patterns {
    module: Regex,
    interface: Regex,
    function: Regex,
    task: Regex,
    #[allow(dead_code)]
    port_inline: Regex, // 端口在 module 声明行内或端口列表
    port_body: Regex, // 模块体内的端口声明
    parameter: Regex,
    localparam: Regex,
    wire: Regex,
    logic: Regex,
    reg: Regex,
}

impl Patterns {
    fn new() -> Self {
        Self {
            // module <name> [ ... ] (...)
            module: Regex::new(r"(?m)^\s*module\s+(\w+)").unwrap(),
            // interface <name>
            interface: Regex::new(r"(?m)^\s*interface\s+(\w+)").unwrap(),
            // function [automatic] [return_type] <name>
            function: Regex::new(r"(?m)^\s*(?:automatic\s+)?function\s+(?:automatic\s+)?(?:\w+\s+)?(\w+)\s*[;\(]").unwrap(),
            // task [automatic] <name>
            task: Regex::new(r"(?m)^\s*(?:automatic\s+)?task\s+(?:automatic\s+)?(\w+)\s*[;\(]").unwrap(),
            // input/output/inout [logic/wire/reg] [dimensions] <name>
            port_inline: Regex::new(r"(?m)\b(input|output|inout)\b\s+(?:\w+\s+)*?(\w+)\s*(?:[,\)\n;])").unwrap(),
            port_body: Regex::new(r"(?m)^\s*(input|output|inout)\s+(?:logic|wire|reg|bit)?\s*(?:\[[^\]]*\])?\s*(?:\[[^\]]*\])?\s*(\w+)\s*[;,]").unwrap(),
            // parameter [type] <name> = ...
            parameter: Regex::new(r"(?m)\bparameter\s+(?:\w+\s+)?(?:\[[^\]]*\]\s*)?(\w+)\s*=").unwrap(),
            // localparam <name> = ...
            localparam: Regex::new(r"(?m)\blocalparam\s+(?:\w+\s+)?(?:\[[^\]]*\]\s*)?(\w+)\s*=").unwrap(),
            // wire [dimensions] <name>
            wire: Regex::new(r"(?m)^\s*wire\s+(?:\w+\s+)?(?:\[[^\]]*\]\s*)*(\w+)\s*[;,=]").unwrap(),
            // logic [dimensions] <name>
            logic: Regex::new(r"(?m)^\s*logic\s+(?:\[[^\]]*\]\s*)*(\w+)\s*[;,=]").unwrap(),
            // reg [dimensions] <name>
            reg: Regex::new(r"(?m)^\s*reg\s+(?:\[[^\]]*\]\s*)*(\w+)\s*[;,=]").unwrap(),
        }
    }
}

static PATTERNS: OnceLock<Patterns> = OnceLock::new();

fn patterns() -> &'static Patterns {
    PATTERNS.get_or_init(Patterns::new)
}

/// 符号收集器：从 SV/Verilog 源码文本中提取符号
pub struct SymbolCollector;

impl SymbolCollector {
    pub fn new() -> Self {
        Self
    }

    /// 从源码收集所有符号，`file_uri` 作为 Location 的 uri 字段
    pub fn collect_from_source(&self, content: &str, file_uri: &str) -> Vec<Symbol> {
        let mut symbols = Vec::new();
        let p = patterns();

        // 构建行偏移表，用于从字节偏移快速计算行列号
        let line_offsets = build_line_offsets(content);

        // 模块
        collect_matches(
            &p.module,
            content,
            &line_offsets,
            file_uri,
            SymbolKind::Module,
            &mut symbols,
        );
        // 接口
        collect_matches(
            &p.interface,
            content,
            &line_offsets,
            file_uri,
            SymbolKind::Interface,
            &mut symbols,
        );
        // 函数
        collect_matches(
            &p.function,
            content,
            &line_offsets,
            file_uri,
            SymbolKind::Function,
            &mut symbols,
        );
        // 任务
        collect_matches(
            &p.task,
            content,
            &line_offsets,
            file_uri,
            SymbolKind::Task,
            &mut symbols,
        );
        // 端口（模块体内声明）
        collect_matches(
            &p.port_body,
            content,
            &line_offsets,
            file_uri,
            SymbolKind::Port,
            &mut symbols,
        );
        // 参数
        collect_matches(
            &p.parameter,
            content,
            &line_offsets,
            file_uri,
            SymbolKind::Parameter,
            &mut symbols,
        );
        // 本地参数
        collect_matches(
            &p.localparam,
            content,
            &line_offsets,
            file_uri,
            SymbolKind::Parameter,
            &mut symbols,
        );
        // wire/logic/reg 变量（跳过已匹配为端口的）
        collect_matches(
            &p.wire,
            content,
            &line_offsets,
            file_uri,
            SymbolKind::Signal,
            &mut symbols,
        );
        collect_matches(
            &p.logic,
            content,
            &line_offsets,
            file_uri,
            SymbolKind::Signal,
            &mut symbols,
        );
        collect_matches(
            &p.reg,
            content,
            &line_offsets,
            file_uri,
            SymbolKind::Signal,
            &mut symbols,
        );

        // 去重（按名称+类型，保留第一次出现的）
        dedup_symbols(symbols)
    }
}

impl Default for SymbolCollector {
    fn default() -> Self {
        Self::new()
    }
}

/// 收集正则匹配到的符号（捕获组 1 为名称）
fn collect_matches(
    re: &Regex,
    content: &str,
    line_offsets: &[usize],
    file_uri: &str,
    kind: SymbolKind,
    out: &mut Vec<Symbol>,
) {
    for cap in re.captures_iter(content) {
        // 对于端口正则，名称在捕获组 2；其余在捕获组 1
        let (name_match, offset) = if cap.len() > 2 && cap.get(2).is_some() {
            let m = cap.get(2).unwrap();
            (m.as_str(), m.start())
        } else if let Some(m) = cap.get(1) {
            (m.as_str(), m.start())
        } else {
            continue;
        };

        // 过滤关键字和类型名
        if is_keyword_or_type(name_match) {
            continue;
        }

        let (line, col) = offset_to_line_col(offset, line_offsets);
        let loc = Location {
            uri: file_uri.to_string(),
            start: Position::new(line, col),
            end: Position::new(line, col + name_match.len() as u32),
        };
        out.push(Symbol::new(SmolStr::new(name_match), kind, loc));
    }
}

/// 构建每行的起始字节偏移表
fn build_line_offsets(content: &str) -> Vec<usize> {
    let mut offsets = vec![0usize];
    for (i, b) in content.bytes().enumerate() {
        if b == b'\n' {
            offsets.push(i + 1);
        }
    }
    offsets
}

/// 字节偏移 -> (行, 列)，均为 0-based
fn offset_to_line_col(offset: usize, line_offsets: &[usize]) -> (u32, u32) {
    let line = line_offsets
        .partition_point(|&o| o <= offset)
        .saturating_sub(1);
    let col = offset - line_offsets[line];
    (line as u32, col as u32)
}

/// 是否是需要过滤的 SV 关键字或类型名
fn is_keyword_or_type(name: &str) -> bool {
    matches!(
        name,
        "automatic"
            | "logic"
            | "wire"
            | "reg"
            | "bit"
            | "int"
            | "integer"
            | "byte"
            | "shortint"
            | "longint"
            | "real"
            | "realtime"
            | "time"
            | "signed"
            | "unsigned"
            | "input"
            | "output"
            | "inout"
            | "ref"
            | "parameter"
            | "localparam"
            | "module"
            | "endmodule"
            | "interface"
            | "endinterface"
            | "function"
            | "endfunction"
            | "task"
            | "endtask"
            | "begin"
            | "end"
            | "always"
            | "assign"
            | "if"
            | "else"
            | "case"
            | "endcase"
            | "default"
            | "for"
            | "while"
            | "forever"
            | "repeat"
            | "generate"
            | "endgenerate"
            | "genvar"
            | "initial"
            | "posedge"
            | "negedge"
            | "or"
            | "and"
            | "not"
            | "void"
            | "string"
            | "enum"
            | "struct"
            | "union"
            | "typedef"
            | "import"
            | "export"
            | "package"
            | "endpackage"
            | "class"
            | "endclass"
            | "virtual"
            | "extends"
            | "implements"
    )
}

/// 去除重复符号（名称+行号相同视为重复）
fn dedup_symbols(symbols: Vec<Symbol>) -> Vec<Symbol> {
    let mut seen = std::collections::HashSet::new();
    symbols
        .into_iter()
        .filter(|s| {
            seen.insert((
                s.name.clone(),
                s.location.start.line,
                s.location.start.column,
            ))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_MODULE: &str = r#"
module top (
    input  logic clk,
    input  logic rst_n,
    output logic [7:0] data_out
);
    parameter WIDTH = 8;
    localparam IDLE = 2'b00;
    wire   internal_wire;
    logic  [3:0] counter;
    reg    valid;

    function automatic logic my_func(input logic x);
        return x;
    endfunction

    task my_task(input logic a);
    endtask

endmodule
"#;

    const MULTI_MODULE: &str = r#"
module foo ();
endmodule

module bar ();
endmodule

interface my_if ();
endinterface
"#;

    #[test]
    fn test_collect_module() {
        let col = SymbolCollector::new();
        let syms = col.collect_from_source(SIMPLE_MODULE, "file:///top.sv");
        let modules: Vec<_> = syms
            .iter()
            .filter(|s| s.kind == SymbolKind::Module)
            .collect();
        assert_eq!(modules.len(), 1);
        assert_eq!(modules[0].name, "top");
    }

    #[test]
    fn test_collect_ports() {
        let col = SymbolCollector::new();
        let syms = col.collect_from_source(SIMPLE_MODULE, "file:///top.sv");
        let ports: Vec<_> = syms.iter().filter(|s| s.kind == SymbolKind::Port).collect();
        // 至少应有 clk, rst_n, data_out
        assert!(ports.len() >= 2, "期望至少 2 个端口，实际: {}", ports.len());
        let names: Vec<&str> = ports.iter().map(|s| s.name.as_str()).collect();
        assert!(names.contains(&"clk"), "clk 未找到，实际: {:?}", names);
        assert!(names.contains(&"rst_n"), "rst_n 未找到，实际: {:?}", names);
    }

    #[test]
    fn test_collect_parameters() {
        let col = SymbolCollector::new();
        let syms = col.collect_from_source(SIMPLE_MODULE, "file:///top.sv");
        let params: Vec<_> = syms
            .iter()
            .filter(|s| s.kind == SymbolKind::Parameter)
            .collect();
        let names: Vec<&str> = params.iter().map(|s| s.name.as_str()).collect();
        assert!(names.contains(&"WIDTH"), "WIDTH 未找到，实际: {:?}", names);
        assert!(names.contains(&"IDLE"), "IDLE 未找到，实际: {:?}", names);
    }

    #[test]
    fn test_collect_signals() {
        let col = SymbolCollector::new();
        let syms = col.collect_from_source(SIMPLE_MODULE, "file:///top.sv");
        let sigs: Vec<_> = syms
            .iter()
            .filter(|s| s.kind == SymbolKind::Signal)
            .collect();
        let names: Vec<&str> = sigs.iter().map(|s| s.name.as_str()).collect();
        assert!(
            names.contains(&"internal_wire")
                || names.contains(&"counter")
                || names.contains(&"valid"),
            "信号未找到，实际: {:?}",
            names
        );
    }

    #[test]
    fn test_collect_function_task() {
        let col = SymbolCollector::new();
        let syms = col.collect_from_source(SIMPLE_MODULE, "file:///top.sv");
        let funcs: Vec<_> = syms
            .iter()
            .filter(|s| s.kind == SymbolKind::Function)
            .collect();
        let tasks: Vec<_> = syms.iter().filter(|s| s.kind == SymbolKind::Task).collect();
        assert!(funcs.iter().any(|s| s.name == "my_func"), "my_func 未找到");
        assert!(tasks.iter().any(|s| s.name == "my_task"), "my_task 未找到");
    }

    #[test]
    fn test_collect_multi_module() {
        let col = SymbolCollector::new();
        let syms = col.collect_from_source(MULTI_MODULE, "file:///multi.sv");
        let modules: Vec<_> = syms
            .iter()
            .filter(|s| s.kind == SymbolKind::Module)
            .collect();
        let interfaces: Vec<_> = syms
            .iter()
            .filter(|s| s.kind == SymbolKind::Interface)
            .collect();
        assert_eq!(modules.len(), 2, "期望 2 个模块");
        assert_eq!(interfaces.len(), 1, "期望 1 个接口");
        let mod_names: Vec<&str> = modules.iter().map(|s| s.name.as_str()).collect();
        assert!(mod_names.contains(&"foo"));
        assert!(mod_names.contains(&"bar"));
    }

    #[test]
    fn test_location_is_set() {
        let col = SymbolCollector::new();
        let syms = col.collect_from_source(SIMPLE_MODULE, "file:///top.sv");
        for sym in &syms {
            assert_eq!(sym.location.uri, "file:///top.sv");
        }
    }

    #[test]
    fn test_collect_empty_source() {
        let col = SymbolCollector::new();
        let syms = col.collect_from_source("", "file:///empty.sv");
        assert!(syms.is_empty());
    }
}
