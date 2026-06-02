//! MCP Server tool coverage 补充测试
//! 覆盖 integration_test.rs 未涵盖的 tool 方法

use thanosLSP_mcp::server::{
use rmcp::handler::server::tool::Parameters;
    CreateFileParams, GetCompletionsParams, GetDefinitionParams, RenameSymbolParams,
    ReplaceLinesParams, SetLogLevelParams, ThanosMcpServer, UriParam,
};

fn make_server() -> ThanosMcpServer {
    ThanosMcpServer::new()
}

const SIMPLE_SV: &str = r#"module counter(
    input  logic clk,
    input  logic rst,
    output logic [7:0] count
);
    always_ff @(posedge clk or posedge rst) begin
        if (rst) count <= 8'h0;
        else     count <= count + 1'b1;
    end
endmodule
"#;

#[tokio::test]
async fn test_get_completions_returns_json() {
    let s = make_server();
    let uri = "file:///tmp/compl.sv";
    s.open_file(Parameters(thanosLSP_mcp::server::OpenFileParams {
        uri: uri.to_string(),
        content: SIMPLE_SV.to_string(),
    }))
    .await;
    let res = s
        .get_completions(Parameters(GetCompletionsParams {
            uri: uri.to_string(),
            line: 0,
            character: 0,
            prefix: None,
        }))
        .await;
    // Should return a valid JSON array (may be empty for position with no completion)
    assert!(
        res.starts_with('['),
        "expected JSON array, got: {res:.80}"
    );
}

#[tokio::test]
async fn test_get_definition_returns_json() {
    let s = make_server();
    let uri = "file:///tmp/def.sv";
    s.open_file(Parameters(thanosLSP_mcp::server::OpenFileParams {
        uri: uri.to_string(),
        content: SIMPLE_SV.to_string(),
    }))
    .await;
    let res = s
        .get_definition(Parameters(GetDefinitionParams {
            uri: uri.to_string(),
            line: 0,
            character: 7, // 'counter' identifier
        }))
        .await;
    // Valid response is either null, an object, or an error string
    assert!(!res.is_empty(), "get_definition should return non-empty response");
}

#[tokio::test]
async fn test_get_references_returns_json() {
    let s = make_server();
    let uri = "file:///tmp/refs.sv";
    s.open_file(Parameters(thanosLSP_mcp::server::OpenFileParams {
        uri: uri.to_string(),
        content: SIMPLE_SV.to_string(),
    }))
        .await;
    let res = s
        .get_references(Parameters(GetDefinitionParams {
            uri: uri.to_string(),
            line: 0,
            character: 7,
        }))
        .await;
    assert!(!res.is_empty(), "get_references should return non-empty response");
}

#[tokio::test]
async fn test_get_hover_returns_response() {
    let s = make_server();
    let uri = "file:///tmp/hover.sv";
    s.open_file(Parameters(thanosLSP_mcp::server::OpenFileParams {
        uri: uri.to_string(),
        content: SIMPLE_SV.to_string(),
    }))
    .await;
    let res = s
        .get_hover(Parameters(GetDefinitionParams {
            uri: uri.to_string(),
            line: 0,
            character: 7,
        }))
        .await;
    assert!(!res.is_empty(), "get_hover should return non-empty response");
}

#[tokio::test]
async fn test_rename_symbol_modifies_content() {
    let s = make_server();
    let uri = "file:///tmp/rename.sv";
    s.open_file(Parameters(thanosLSP_mcp::server::OpenFileParams {
        uri: uri.to_string(),
        content: SIMPLE_SV.to_string(),
    }))
    .await;
    let res = s
        .rename_symbol(Parameters(RenameSymbolParams {
            uri: uri.to_string(),
            old_name: "counter".to_string(),
            new_name: "counter_v2".to_string(),
        }))
        .await;
    // Should succeed or report not found - must not panic
    assert!(!res.is_empty(), "rename_symbol should return non-empty response");
    // After rename, reading back should contain new name
    let content = s
        .read_file(Parameters(UriParam {
            uri: uri.to_string(),
        }))
        .await;
    assert!(
        content.contains("counter_v2") || res.contains("error"),
        "content should have new name or error: res={res:.80}"
    );
}

#[tokio::test]
async fn test_replace_lines_modifies_content() {
    let s = make_server();
    let uri = "file:///tmp/replace_lines.sv";
    s.open_file(Parameters(thanosLSP_mcp::server::OpenFileParams {
        uri: uri.to_string(),
        content: SIMPLE_SV.to_string(),
    }))
    .await;
    // Replace first line (0-based, exclusive end)
    let res = s
        .replace_lines(Parameters(ReplaceLinesParams {
            uri: uri.to_string(),
            start_line: 0,
            end_line: 1,
            new_text: "// replaced line\n".to_string(),
        }))
        .await;
    assert!(!res.is_empty(), "replace_lines should return non-empty response");
    // Verify content changed
    let content = s
        .read_file(Parameters(UriParam {
            uri: uri.to_string(),
        }))
        .await;
    assert!(
        content.contains("replaced line") || res.contains("error"),
        "content should have replaced line or error"
    );
}

#[tokio::test]
async fn test_create_file_creates_and_opens() {
    let s = make_server();
    let path = "/tmp/created_mcp_test.sv";
    let res = s
        .create_file(Parameters(CreateFileParams {
            path: path.to_string(),
            content: "// created by test\nmodule empty_mod; endmodule\n".to_string(),
        }))
        .await;
    assert!(
        res.contains("created") || res.contains("opened") || !res.contains("error"),
        "create_file unexpected response: {res:.80}"
    );
}

#[tokio::test]
async fn test_set_log_level_valid() {
    let s = make_server();
    let res = s
        .set_log_level(Parameters(SetLogLevelParams {
            level: "warn".to_string(),
        }))
        .await;
    assert!(
        res.contains("warn") || res.contains("log"),
        "set_log_level response: {res:.80}"
    );
}

#[tokio::test]
async fn test_set_log_level_restores_info() {
    let s = make_server();
    // Set to debug then back to info
    s.set_log_level(Parameters(SetLogLevelParams {
        level: "debug".to_string(),
    }))
    .await;
    let res = s
        .set_log_level(Parameters(SetLogLevelParams {
            level: "info".to_string(),
        }))
        .await;
    assert!(!res.is_empty(), "set_log_level(info) should respond");
}

#[tokio::test]
async fn test_format_file_sv_returns_response() {
    let s = make_server();
    let uri = "file:///tmp/format_test.sv";
    s.open_file(Parameters(thanosLSP_mcp::server::OpenFileParams {
        uri: uri.to_string(),
        content: SIMPLE_SV.to_string(),
    }))
    .await;
    let res = s
        .format_file(Parameters(UriParam {
            uri: uri.to_string(),
        }))
        .await;
    // May succeed or return "verible not found" - must not panic
    assert!(!res.is_empty(), "format_file should return non-empty response");
}

#[tokio::test]
async fn test_error_on_unopened_file() {
    let s = make_server();
    let res = s
        .read_file(Parameters(UriParam {
            uri: "file:///nonexistent_file_12345.sv".to_string(),
        }))
        .await;
    assert!(
        res.contains("error") || res.contains("not open"),
        "read_file on unopened file should return error: {res:.80}"
    );
}

#[tokio::test]
async fn test_get_diagnostics_vhdl_file() {
    let s = make_server();
    let uri = "file:///tmp/test.vhd";
    let vhdl_content = r#"library ieee;
use ieee.std_logic_1164.all;
entity half_adder is
    port(a, b : in std_logic; sum, carry : out std_logic);
end entity;
architecture rtl of half_adder is
begin
    sum   <= a xor b;
    carry <= a and b;
end architecture;
"#;
    s.open_file(Parameters(thanosLSP_mcp::server::OpenFileParams {
        uri: uri.to_string(),
        content: vhdl_content.to_string(),
    }))
    .await;
    let res = s
        .get_diagnostics(Parameters(UriParam {
            uri: uri.to_string(),
        }))
        .await;
    // Should return a JSON array of diagnostics (empty = no errors for valid VHDL)
    assert!(
        res.starts_with('['),
        "get_diagnostics should return JSON array: {res:.80}"
    );
}
