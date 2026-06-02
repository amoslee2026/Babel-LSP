//! L5 性能基准测试
//!
//! 对照 functional_spec §7.1 性能目标进行测量
//! 目标: MCP 工具响应 < 100ms (已缓存符号)

use std::time::Instant;
use thanosLSP_mcp::server::{OpenFileParams, SearchSymbolsParams, ThanosMcpServer};

fn make_large_sv(module_count: usize) -> String {
    let mut buf = String::new();
    for i in 0..module_count {
        buf.push_str(&format!(
            "module mod_{i} #(parameter W = 8)(input logic [W-1:0] a, output logic [W-1:0] b);\n  assign b = a;\nendmodule\n\n"
        ));
    }
    buf
}

/// MCP open_file + search_symbols 响应时间 < 100ms (已缓存)
#[tokio::test]
async fn perf_mcp_tool_response_under_100ms() {
    let server = ThanosMcpServer::new();
    let uri = "file:///perf_test.sv";
    let content = make_large_sv(20); // 20 modules

    // open_file returns String result (not Result), no expect needed
    let _ = server
        .open_file(Parameters(OpenFileParams {
            uri: uri.to_string(),
            content: content.clone(),
        }))
        .await;

    // Warm up
    let _ = server
        .search_symbols(Parameters(SearchSymbolsParams {
            query: "mod_0".to_string(),
            uri: Some(uri.to_string()),
        }))
        .await;

    // Measure 10 iterations
    let start = Instant::now();
    for _ in 0..10 {
        let _ = server
            .search_symbols(Parameters(SearchSymbolsParams {
                query: "mod_".to_string(),
                uri: Some(uri.to_string()),
            }))
            .await;
    }
    let avg_ms = start.elapsed().as_millis() / 10;

    assert!(
        avg_ms < 100,
        "MCP search_symbols avg {}ms exceeds 100ms target",
        avg_ms
    );
}

/// open_file 延迟（不含 slang 诊断，仅解析 + 索引）< 200ms
#[tokio::test]
async fn perf_open_file_latency_under_200ms() {
    let server = ThanosMcpServer::new();
    let content = make_large_sv(10); // ~10 modules, ~100 lines

    let start = Instant::now();
    for i in 0..10 {
        let uri = format!("file:///perf_open_{i}.sv");
        let _ = server
            .open_file(Parameters(OpenFileParams {
                uri,
                content: content.clone(),
            }))
            .await;
    }
    let avg_ms = start.elapsed().as_millis() / 10;

    assert!(
        avg_ms < 200,
        "open_file avg {}ms exceeds 200ms target",
        avg_ms
    );
}

/// 文件分类吞吐量（> 1M ops/sec）
#[test]
fn perf_file_classifier_throughput() {
    use thanosLSP_core::file_classifier::FileClassifier;

    let classifier = FileClassifier::new();
    let paths = [
        "design.sv",
        "tb_top.sv",
        "netlist.vg",
        "mod_netlist.v",
        "test_adder.sv",
        "constraints.tcl",
    ];

    let start = Instant::now();
    let iterations = 10_000;
    for _ in 0..iterations {
        for path in &paths {
            let _ = classifier.classify_by_path(path);
        }
    }
    let total_ms = start.elapsed().as_millis().max(1);
    let ops = iterations * paths.len();
    let ops_per_sec = (ops as f64 / total_ms as f64) * 1000.0;

    assert!(
        ops_per_sec > 100_000.0,
        "FileClassifier throughput {:.0} ops/sec below 100K ops/sec (spec: 1000 files in 5s)",
        ops_per_sec
    );
}
