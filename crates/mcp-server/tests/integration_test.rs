//! MCP Server 集成测试
//!
//! 测试完整的 Tool 调用流程，包括多 tool 组合使用场景

use thanosLSP_mcp::server::{
    OpenFileParams, ReplaceContentParams, SearchPatternParams, SearchSymbolsParams,
    ThanosMcpServer, UpdateFileParams, UriParam,
};

fn sv_test_server() -> ThanosMcpServer {
    ThanosMcpServer::new()
}

const SIMPLE_SV: &str = r#"
module adder #(
    parameter WIDTH = 8
)(
    input  logic [WIDTH-1:0] a,
    input  logic [WIDTH-1:0] b,
    output logic [WIDTH-1:0] sum
);
    assign sum = a + b;
endmodule
"#;

const RTL_WITH_ISSUES: &str = r#"
module bad_rtl (
    input clk,
    input rst_n
);
    initial begin
        $display("reset!");
        #10;
        $finish;
    end

    always @(posedge clk) begin
        force out = 1;
    end
endmodule
"#;

#[tokio::test]
async fn test_open_update_read_cycle() {
    let server = sv_test_server();
    let uri = "file:///adder.sv";

    // Open
    let res = server
        .open_file(OpenFileParams {
            uri: uri.to_string(),
            content: SIMPLE_SV.to_string(),
        })
        .await;
    assert!(res.contains("opened"), "open: {res}");

    // Update
    let new_content = SIMPLE_SV.replace("WIDTH = 8", "WIDTH = 16");
    let res = server
        .update_file(UpdateFileParams {
            uri: uri.to_string(),
            content: new_content,
        })
        .await;
    assert!(res.contains("updated"), "update: {res}");

    // Read back
    let content = server
        .read_file(UriParam {
            uri: uri.to_string(),
        })
        .await;
    assert!(content.contains("WIDTH = 16"), "read: {content}");
}

#[tokio::test]
async fn test_get_symbols_for_sv_module() {
    let server = sv_test_server();
    let uri = "file:///adder.sv";
    server
        .open_file(OpenFileParams {
            uri: uri.to_string(),
            content: SIMPLE_SV.to_string(),
        })
        .await;

    let symbols = server
        .get_symbols(UriParam {
            uri: uri.to_string(),
        })
        .await;
    // Should contain adder module
    assert!(symbols.contains("adder"), "symbols: {symbols}");
}

#[tokio::test]
async fn test_synthesizability_detects_issues() {
    let server = sv_test_server();
    let uri = "file:///bad_rtl.sv";
    server
        .open_file(OpenFileParams {
            uri: uri.to_string(),
            content: RTL_WITH_ISSUES.to_string(),
        })
        .await;

    let result = server
        .check_synthesizability(UriParam {
            uri: uri.to_string(),
        })
        .await;
    assert!(result.contains("SYN-V"), "expected synth issues: {result}");
    // Should detect initial block, $display, #10, $finish, force
    assert!(
        result.contains("SYN-V-001")
            || result.contains("SYN-V-002")
            || result.contains("SYN-V-005"),
        "expected specific codes: {result}"
    );
}

#[tokio::test]
async fn test_search_pattern_across_file() {
    let server = sv_test_server();
    let uri = "file:///adder.sv";
    server
        .open_file(OpenFileParams {
            uri: uri.to_string(),
            content: SIMPLE_SV.to_string(),
        })
        .await;

    let result = server
        .search_for_pattern(SearchPatternParams {
            pattern: "assign".to_string(),
            uri: Some(uri.to_string()),
        })
        .await;
    assert!(result.contains("assign"), "search: {result}");
}

#[tokio::test]
async fn test_replace_content_and_verify() {
    let server = sv_test_server();
    let uri = "file:///adder.sv";
    server
        .open_file(OpenFileParams {
            uri: uri.to_string(),
            content: SIMPLE_SV.to_string(),
        })
        .await;

    let result = server
        .replace_content(ReplaceContentParams {
            uri: uri.to_string(),
            old_text: "assign sum = a + b".to_string(),
            new_text: "assign sum = a ^ b".to_string(),
        })
        .await;
    assert_eq!(result, "replaced");

    let content = server
        .read_file(UriParam {
            uri: uri.to_string(),
        })
        .await;
    assert!(content.contains("a ^ b"), "content: {content}");
    assert!(!content.contains("a + b"), "old text still present");
}

#[tokio::test]
async fn test_close_and_verify_removed() {
    let server = sv_test_server();
    let uri = "file:///adder.sv";
    server
        .open_file(OpenFileParams {
            uri: uri.to_string(),
            content: SIMPLE_SV.to_string(),
        })
        .await;
    server
        .close_file(UriParam {
            uri: uri.to_string(),
        })
        .await;

    let content = server
        .read_file(UriParam {
            uri: uri.to_string(),
        })
        .await;
    assert!(content.starts_with("error"), "should be error: {content}");
}

#[tokio::test]
async fn test_search_symbols_multi_file() {
    let server = sv_test_server();

    server
        .open_file(OpenFileParams {
            uri: "file:///a.sv".to_string(),
            content: "module module_a;\nendmodule".to_string(),
        })
        .await;
    server
        .open_file(OpenFileParams {
            uri: "file:///b.sv".to_string(),
            content: "module module_b;\nendmodule".to_string(),
        })
        .await;

    let result = server
        .search_symbols(SearchSymbolsParams {
            query: "module".to_string(),
            uri: None,
        })
        .await;
    assert!(result.contains("module_a"), "search_symbols: {result}");
    assert!(result.contains("module_b"), "search_symbols: {result}");
}

#[tokio::test]
async fn test_project_memory_lists_files() {
    let server = sv_test_server();
    server
        .open_file(OpenFileParams {
            uri: "file:///top.sv".to_string(),
            content: "module top;\nendmodule".to_string(),
        })
        .await;

    let memory = server.get_project_memory().await;
    assert!(memory.contains("top.sv"), "memory: {memory}");
}

#[tokio::test]
async fn test_get_diagnostics_sv_synth() {
    let server = sv_test_server();
    let uri = "file:///rtl.sv";
    server
        .open_file(OpenFileParams {
            uri: uri.to_string(),
            content: "module foo;\n  initial begin\n    a = 1;\n  end\nendmodule".to_string(),
        })
        .await;

    let diags = server
        .get_diagnostics(UriParam {
            uri: uri.to_string(),
        })
        .await;
    // Should detect initial block
    assert!(diags.contains("SYN-V-001"), "diagnostics: {diags}");
}
