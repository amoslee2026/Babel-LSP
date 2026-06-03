//! Binary-level MCP stdio 协议集成测试
//!
//! 启动真实 babel-lsp binary（--mcp 模式），通过 stdin/stdout 发送/接收
//! JSON-RPC 2.0 消息，验证协议合规性和 Bug 回归。
//!
//! MCP 握手流程：
//!   client → initialize         → server responds
//!   client → notifications/initialized (no response)
//!   client → tools/list / tools/call  → server responds

use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

// ─── helper: binary path ───────────────────────────────────────────────────

/// 定位 babel-lsp binary。优先 debug，其次 release。
fn binary_path() -> PathBuf {
    let manifest = std::env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR not set");
    let root = PathBuf::from(manifest)
        .parent().unwrap() // crates
        .parent().unwrap() // project root
        .to_path_buf();

    let debug = root.join("target/debug/babel-lsp");
    if debug.exists() {
        return debug;
    }
    let release = root.join("target/release/babel-lsp");
    if release.exists() {
        return release;
    }
    debug // not found — tests will skip
}

// ─── helper: protocol ──────────────────────────────────────────────────────

fn send_and_recv(
    stdin: &mut std::process::ChildStdin,
    reader: &mut impl BufRead,
    msg: &str,
) -> String {
    stdin.write_all(format!("{msg}\n").as_bytes()).expect("write");
    stdin.flush().expect("flush");
    let mut buf = String::new();
    reader.read_line(&mut buf).expect("read_line");
    buf.trim().to_string()
}

fn send_no_resp(stdin: &mut std::process::ChildStdin, msg: &str) {
    stdin.write_all(format!("{msg}\n").as_bytes()).expect("write");
    stdin.flush().expect("flush");
}

fn init_msg() -> &'static str {
    r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-03-26","capabilities":{},"clientInfo":{"name":"test","version":"0.1"}}}"#
}

/// 完整 MCP 握手：initialize → notifications/initialized
/// 返回 initialize 响应的 JSON Value
fn do_handshake(
    stdin: &mut std::process::ChildStdin,
    reader: &mut impl BufRead,
) -> serde_json::Value {
    let raw = send_and_recv(stdin, reader, init_msg());
    send_no_resp(
        stdin,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
    );
    serde_json::from_str(&raw)
        .unwrap_or_else(|e| panic!("initialize response not valid JSON: {e}\nraw: {raw}"))
}

// ─── tests ─────────────────────────────────────────────────────────────────

#[test]
fn test_mcp_binary_exists() {
    let path = binary_path();
    assert!(
        path.exists(),
        "babel-lsp binary not found at {path:?}. Run `cargo build` first."
    );
}

#[test]
fn test_mcp_initialize_handshake() {
    let path = binary_path();
    if !path.exists() {
        eprintln!("SKIP: binary not found");
        return;
    }

    let mut child = Command::new(&path)
        .arg("--mcp")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawn");

    let mut stdin = child.stdin.take().unwrap();
    let stdout = child.stdout.take().unwrap();
    let mut reader = BufReader::new(stdout);

    let v = do_handshake(&mut stdin, &mut reader);
    drop(stdin);
    let _ = child.wait();

    assert_eq!(v["jsonrpc"], "2.0");
    assert_eq!(v["id"], 1);
    assert!(v["result"].is_object(), "result must be object");
    assert!(v["result"]["serverInfo"].is_object(), "serverInfo required");
    assert!(
        v["result"]["protocolVersion"].as_str().is_some_and(|s| !s.is_empty()),
        "protocolVersion required"
    );
}

#[test]
fn test_mcp_tools_list() {
    let path = binary_path();
    if !path.exists() {
        eprintln!("SKIP");
        return;
    }

    let mut child = Command::new(&path)
        .arg("--mcp")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawn");

    let mut stdin = child.stdin.take().unwrap();
    let stdout = child.stdout.take().unwrap();
    let mut reader = BufReader::new(stdout);

    let _ = do_handshake(&mut stdin, &mut reader);

    let raw = send_and_recv(
        &mut stdin,
        &mut reader,
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}"#,
    );
    drop(stdin);
    let _ = child.wait();

    let v: serde_json::Value = serde_json::from_str(&raw)
        .unwrap_or_else(|e| panic!("tools/list not valid JSON: {e}\nraw: {raw}"));

    assert_eq!(v["id"], 2);
    let tools = v["result"]["tools"].as_array().expect("tools array");
    assert!(!tools.is_empty(), "tools/list must return ≥ 1 tool");

    for tool in tools {
        assert!(tool["name"].is_string(), "tool missing name: {tool}");
        assert!(
            tool["inputSchema"].is_object(),
            "tool missing inputSchema: {tool}"
        );
    }
}

#[test]
fn test_mcp_tools_count() {
    let path = binary_path();
    if !path.exists() {
        eprintln!("SKIP");
        return;
    }

    let mut child = Command::new(&path)
        .arg("--mcp")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawn");

    let mut stdin = child.stdin.take().unwrap();
    let stdout = child.stdout.take().unwrap();
    let mut reader = BufReader::new(stdout);

    let _ = do_handshake(&mut stdin, &mut reader);
    let raw = send_and_recv(
        &mut stdin,
        &mut reader,
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}"#,
    );
    drop(stdin);
    let _ = child.wait();

    let v: serde_json::Value = serde_json::from_str(&raw).unwrap();
    let count = v["result"]["tools"].as_array().map(|a| a.len()).unwrap_or(0);
    assert!(count >= 10, "expected ≥ 10 MCP tools, got {count}");
}

/// BUG-1 回归：invalid JSON 不导致 crash（进程不以信号退出）
#[test]
fn test_mcp_invalid_json_does_not_crash() {
    let path = binary_path();
    if !path.exists() {
        eprintln!("SKIP");
        return;
    }

    let mut child = Command::new(&path)
        .arg("--mcp")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawn");

    let mut stdin = child.stdin.take().unwrap();
    stdin.write_all(b"this is not json at all\n").unwrap();
    stdin.flush().unwrap();
    drop(stdin);

    let status = child.wait().expect("wait");

    #[cfg(unix)]
    {
        use std::os::unix::process::ExitStatusExt;
        assert!(
            status.signal().is_none(),
            "BUG-1 REGRESSION: process killed by signal {:?}",
            status.signal()
        );
    }
    assert!(
        status.code().is_some(),
        "process must exit with code (not signal)"
    );
}

/// BUG-3 回归：stdout 只输出合法 JSON-RPC 行，无日志污染
#[test]
fn test_mcp_stdout_no_log_pollution() {
    let path = binary_path();
    if !path.exists() {
        eprintln!("SKIP");
        return;
    }

    let mut child = Command::new(&path)
        .arg("--mcp")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawn");

    let mut stdin = child.stdin.take().unwrap();
    let stdout = child.stdout.take().unwrap();
    let mut reader = BufReader::new(stdout);

    let _ = do_handshake(&mut stdin, &mut reader);
    drop(stdin);
    let output = child.wait_with_output().expect("wait_with_output");

    // stdout 中所有非空行必须是合法 JSON
    let stdout_str = String::from_utf8_lossy(&output.stdout);
    for line in stdout_str.lines() {
        if line.trim().is_empty() {
            continue;
        }
        assert!(
            serde_json::from_str::<serde_json::Value>(line).is_ok(),
            "BUG-3 REGRESSION: non-JSON line in stdout: {line}"
        );
    }
}

/// BUG-3 强化：启用 --log-level debug 时日志去 stderr，不污染 stdout
#[test]
fn test_mcp_logs_go_to_stderr_not_stdout() {
    let path = binary_path();
    if !path.exists() {
        eprintln!("SKIP");
        return;
    }

    let mut child = Command::new(&path)
        .args(["--mcp", "--log-level", "debug"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn");

    let mut stdin = child.stdin.take().unwrap();
    let stdout = child.stdout.take().unwrap();
    let mut reader = BufReader::new(stdout);

    let _ = do_handshake(&mut stdin, &mut reader);
    drop(stdin);
    let output = child.wait_with_output().expect("wait_with_output");

    let stdout_str = String::from_utf8_lossy(&output.stdout);
    for line in stdout_str.lines() {
        if line.trim().is_empty() {
            continue;
        }
        assert!(
            serde_json::from_str::<serde_json::Value>(line).is_ok(),
            "BUG-3 REGRESSION: log leaked to stdout: {line}"
        );
    }
}

#[test]
fn test_mcp_tools_call_get_diagnostics() {
    let path = binary_path();
    if !path.exists() {
        eprintln!("SKIP");
        return;
    }

    let mut child = Command::new(&path)
        .arg("--mcp")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawn");

    let mut stdin = child.stdin.take().unwrap();
    let stdout = child.stdout.take().unwrap();
    let mut reader = BufReader::new(stdout);

    let _ = do_handshake(&mut stdin, &mut reader);

    let raw = send_and_recv(
        &mut stdin,
        &mut reader,
        r#"{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"get_diagnostics","arguments":{"uri":"file:///tmp/test.sv"}}}"#,
    );
    drop(stdin);
    let _ = child.wait();

    let v: serde_json::Value = serde_json::from_str(&raw)
        .unwrap_or_else(|e| panic!("tools/call not valid JSON: {e}\nraw: {raw}"));

    assert_eq!(v["id"], 3);
    assert!(
        v["result"].is_object() || v["error"].is_object(),
        "must return result or error: {v}"
    );
}

/// BUG-2 回归：set_log_level 接受有效 level
#[test]
fn test_mcp_set_log_level_valid() {
    let path = binary_path();
    if !path.exists() {
        eprintln!("SKIP");
        return;
    }

    let mut child = Command::new(&path)
        .arg("--mcp")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawn");

    let mut stdin = child.stdin.take().unwrap();
    let stdout = child.stdout.take().unwrap();
    let mut reader = BufReader::new(stdout);

    let _ = do_handshake(&mut stdin, &mut reader);

    let raw = send_and_recv(
        &mut stdin,
        &mut reader,
        r#"{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"set_log_level","arguments":{"level":"debug"}}}"#,
    );
    drop(stdin);
    let _ = child.wait();

    let v: serde_json::Value = serde_json::from_str(&raw)
        .unwrap_or_else(|e| panic!("response not valid JSON: {e}"));

    assert_eq!(v["id"], 4);
    // 有效 level 应该成功，不返回 JSON-RPC error
    assert!(v["error"].is_null(), "valid level must not return error: {v}");
}

/// BUG-2 回归：set_log_level 拒绝无效 level，返回包含 "error"/"invalid" 的消息
#[test]
fn test_mcp_set_log_level_invalid() {
    let path = binary_path();
    if !path.exists() {
        eprintln!("SKIP");
        return;
    }

    let mut child = Command::new(&path)
        .arg("--mcp")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawn");

    let mut stdin = child.stdin.take().unwrap();
    let stdout = child.stdout.take().unwrap();
    let mut reader = BufReader::new(stdout);

    let _ = do_handshake(&mut stdin, &mut reader);

    let raw = send_and_recv(
        &mut stdin,
        &mut reader,
        r#"{"jsonrpc":"2.0","id":5,"method":"tools/call","params":{"name":"set_log_level","arguments":{"level":"invalid_level"}}}"#,
    );
    drop(stdin);
    let _ = child.wait();

    let v: serde_json::Value = serde_json::from_str(&raw)
        .unwrap_or_else(|e| panic!("response not valid JSON: {e}"));

    assert_eq!(v["id"], 5);

    // 提取 content 文本
    let text = v["result"]["content"]
        .as_array()
        .and_then(|arr| arr.iter().find_map(|c| c["text"].as_str()))
        .unwrap_or("");

    assert!(
        text.to_lowercase().contains("error") || text.to_lowercase().contains("invalid"),
        "BUG-2 REGRESSION: invalid level silently accepted, got: {text}"
    );
}

#[test]
fn test_mcp_sequential_requests() {
    let path = binary_path();
    if !path.exists() {
        eprintln!("SKIP");
        return;
    }

    let mut child = Command::new(&path)
        .arg("--mcp")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawn");

    let mut stdin = child.stdin.take().unwrap();
    let stdout = child.stdout.take().unwrap();
    let mut reader = BufReader::new(stdout);

    let _ = do_handshake(&mut stdin, &mut reader);

    // 连续发送 3 条 tools/list
    let r2 = send_and_recv(&mut stdin, &mut reader, r#"{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}"#);
    let r3 = send_and_recv(&mut stdin, &mut reader, r#"{"jsonrpc":"2.0","id":3,"method":"tools/list","params":{}}"#);
    let r4 = send_and_recv(&mut stdin, &mut reader, r#"{"jsonrpc":"2.0","id":4,"method":"tools/list","params":{}}"#);
    drop(stdin);
    let _ = child.wait();

    for (id, raw) in [(2, &r2), (3, &r3), (4, &r4)] {
        let v: serde_json::Value = serde_json::from_str(raw)
            .unwrap_or_else(|e| panic!("response #{id} not valid JSON: {e}\nraw: {raw}"));
        assert_eq!(v["id"], id, "id mismatch for response #{id}");
    }
}
