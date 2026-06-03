//! Gate-level Verilog 解析器测试
//!
//! 测试覆盖：
//! - module 定义提取
//! - 端口列表解析（方向、位宽）
//! - UDP primitive 处理
//! - specify block 跳过
//! - 注释提取

use babel_lsp_cell::parser::{CellParser, PortDirection};

/// 基础 module 解析测试
#[test]
fn parse_simple_module() {
    let verilog = r#"
module sky130_fd_sc_hd__and2 (X, A, B);
  output X;
  input A, B;
  and (X, A, B);
endmodule
"#;

    let parser = CellParser::new();
    let cells = parser.parse(verilog).expect("parse should succeed");

    assert_eq!(cells.len(), 1);
    let cell = &cells[0];
    assert_eq!(cell.name, "sky130_fd_sc_hd__and2");
    assert_eq!(cell.ports.len(), 3);
}

/// 端口方向解析测试
#[test]
fn parse_port_directions() {
    let verilog = r#"
module test_cell (OUT, IN1, IN2, BIDI);
  output OUT;
  input IN1, IN2;
  inout BIDI;
endmodule
"#;

    let parser = CellParser::new();
    let cells = parser.parse(verilog).expect("parse should succeed");

    let cell = &cells[0];
    assert_eq!(cell.ports.len(), 4);

    // 验证端口方向
    let out_port = cell.find_port("OUT").expect("OUT port should exist");
    assert_eq!(out_port.direction, PortDirection::Output);

    let in1_port = cell.find_port("IN1").expect("IN1 port should exist");
    assert_eq!(in1_port.direction, PortDirection::Input);

    let bidi_port = cell.find_port("BIDI").expect("BIDI port should exist");
    assert_eq!(bidi_port.direction, PortDirection::Inout);
}

/// 端口位宽解析测试
#[test]
fn parse_port_width() {
    let verilog = r#"
module wide_cell (OUT, IN);
  output [7:0] OUT;
  input [3:0] IN;
endmodule
"#;

    let parser = CellParser::new();
    let cells = parser.parse(verilog).expect("parse should succeed");

    let cell = &cells[0];
    let out_port = cell.find_port("OUT").expect("OUT port should exist");
    assert_eq!(out_port.width, 8);

    let in_port = cell.find_port("IN").expect("IN port should exist");
    assert_eq!(in_port.width, 4);
}

/// 多 module 解析测试
#[test]
fn parse_multiple_modules() {
    let verilog = r#"
module cell1 (X, A);
  output X;
  input A;
endmodule

module cell2 (Y, B, C);
  output Y;
  input B, C;
endmodule
"#;

    let parser = CellParser::new();
    let cells = parser.parse(verilog).expect("parse should succeed");

    assert_eq!(cells.len(), 2);
    assert!(cells.iter().any(|c| c.name == "cell1"));
    assert!(cells.iter().any(|c| c.name == "cell2"));
}

/// specify block 跳过测试
#[test]
fn skip_specify_block() {
    let verilog = r#"
module timing_cell (X, A, B);
  output X;
  input A, B;

  specify
    (A => X) = (0.5, 0.6);
    (B => X) = (0.5, 0.6);
  endspecify

  and (X, A, B);
endmodule
"#;

    let parser = CellParser::new();
    let cells = parser.parse(verilog).expect("parse should succeed");

    // 应正常解析，specify block 不影响端口提取
    assert_eq!(cells.len(), 1);
    let cell = &cells[0];
    assert_eq!(cell.ports.len(), 3);
}

/// UDP primitive 解析测试
#[test]
fn parse_udp_primitive() {
    let verilog = r#"
primitive udp_mux (Y, A, B, S);
  output Y;
  input A, B, S;

  table
    0 ? 0 : 0;
    1 ? 0 : 1;
  endtable
endprimitive
"#;

    let parser = CellParser::new();
    let cells = parser.parse(verilog).expect("parse should succeed");

    assert_eq!(cells.len(), 1);
    let cell = &cells[0];
    assert_eq!(cell.name, "udp_mux");
    assert_eq!(cell.ports.len(), 4);
    // table 内容应被跳过
}

/// 注释提取测试
#[test]
fn extract_description_from_comment() {
    let verilog = r#"
// AND2 gate: 2-input AND
module sky130_fd_sc_hd__and2 (X, A, B);
  output X;
  input A, B;
endmodule
"#;

    let parser = CellParser::new();
    let cells = parser.parse(verilog).expect("parse should succeed");

    let cell = &cells[0];
    assert_eq!(cell.description, Some("AND2 gate: 2-input AND".to_string()));
}

/// 端口声明在 module 头部测试
#[test]
fn parse_ports_in_header() {
    let verilog = r#"
module header_style (
  output X,
  input A,
  input B
);
  and (X, A, B);
endmodule
"#;

    let parser = CellParser::new();
    let cells = parser.parse(verilog).expect("parse should succeed");

    let cell = &cells[0];
    assert_eq!(cell.ports.len(), 3);
    assert!(cell.find_port("X").is_some());
    assert!(cell.find_port("A").is_some());
    assert!(cell.find_port("B").is_some());
}

/// 空 Verilog 测试
#[test]
fn parse_empty_verilog() {
    let parser = CellParser::new();
    let cells = parser.parse("").expect("parse should succeed");
    assert_eq!(cells.len(), 0);
}

/// 源文件路径和行号测试
#[test]
fn parse_source_location() {
    let verilog = r#"// comment
module test_cell (X, A);
  output X;
  input A;
endmodule
"#;

    let parser = CellParser::new();
    let cells = parser
        .parse_with_path(verilog, "test.v")
        .expect("parse should succeed");

    let cell = &cells[0];
    assert_eq!(cell.source_file.to_str(), Some("test.v"));
    // module 定义在第 2 行（注释后面）
    assert_eq!(cell.line, 2);
}

/// 错误处理测试 - 无效 Verilog
#[test]
fn handle_invalid_verilog() {
    let verilog = "this is not verilog";
    let parser = CellParser::new();
    // 应不崩溃，返回空结果或错误
    let result = parser.parse(verilog);
    // 可以返回空列表或错误，但不能 panic
    assert!(result.is_ok() || result.is_err());
}
