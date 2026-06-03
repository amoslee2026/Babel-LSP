//! LSP 集成测试框架
//!
//! 使用 tower::Service trait 直接调用 LSP 服务

use std::sync::Arc;
use tower::Service;
use tower_lsp::{LspService, lsp_types::*};
use babel_lsp_lsp::backend::BabelLspBackend;

/// LSP 测试客户端
pub struct LspTestClient {
    service: Arc<tokio::sync::Mutex<LspService<BabelLspBackend>>>,
    request_id: std::sync::atomic::AtomicI64,
}

impl LspTestClient {
    /// 创建新的测试客户端
    pub fn new() -> Self {
        let (service, _socket) = LspService::new(BabelLspBackend::new);
        Self {
            service: Arc::new(tokio::sync::Mutex::new(service)),
            request_id: std::sync::atomic::AtomicI64::new(1),
        }
    }

    /// 发送 initialize 请求
    pub async fn initialize(&mut self) -> tower_lsp::jsonrpc::Result<InitializeResult> {
        let params = InitializeParams::default();
        self.call_request("initialize", params).await
    }

    /// 发送 initialized 通知
    pub async fn initialized(&mut self) {
        let params = InitializedParams {};
        self.call_notification("initialized", params).await;
    }

    /// 发送 shutdown 请求
    pub async fn shutdown(&mut self) -> tower_lsp::jsonrpc::Result<()> {
        self.call_request_no_params("shutdown").await
    }

    /// 打开文档（自动从 URI 推断 language_id）
    pub async fn did_open(&mut self, uri: &str, content: &str) {
        let lang = if uri.ends_with(".vhd") || uri.ends_with(".vhdl") {
            "vhdl"
        } else if uri.ends_with(".tcl") {
            "tcl"
        } else if uri.ends_with(".v") {
            "verilog"
        } else {
            "systemverilog"
        };
        let params = DidOpenTextDocumentParams {
            text_document: TextDocumentItem {
                uri: Url::parse(uri).unwrap(),
                language_id: lang.to_string(),
                version: 1,
                text: content.to_string(),
            },
        };
        self.call_notification("textDocument/didOpen", params).await;
    }

    /// 修改文档
    pub async fn did_change(&mut self, uri: &str, content: &str, version: i32) {
        let params = DidChangeTextDocumentParams {
            text_document: VersionedTextDocumentIdentifier {
                uri: Url::parse(uri).unwrap(),
                version,
            },
            content_changes: vec![TextDocumentContentChangeEvent {
                range: None,
                range_length: None,
                text: content.to_string(),
            }],
        };
        self.call_notification("textDocument/didChange", params).await;
    }

    /// 保存文档
    pub async fn did_save(&mut self, uri: &str) {
        let params = DidSaveTextDocumentParams {
            text_document: TextDocumentIdentifier {
                uri: Url::parse(uri).unwrap(),
            },
            text: None,
        };
        self.call_notification("textDocument/didSave", params).await;
    }

    /// 关闭文档
    pub async fn did_close(&mut self, uri: &str) {
        let params = DidCloseTextDocumentParams {
            text_document: TextDocumentIdentifier {
                uri: Url::parse(uri).unwrap(),
            },
        };
        self.call_notification("textDocument/didClose", params).await;
    }

    /// 获取补全
    pub async fn completion(
        &mut self,
        uri: &str,
        line: u32,
        character: u32,
    ) -> tower_lsp::jsonrpc::Result<Option<CompletionResponse>> {
        let params = CompletionParams {
            text_document_position: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier {
                    uri: Url::parse(uri).unwrap(),
                },
                position: Position { line, character },
            },
            work_done_progress_params: WorkDoneProgressParams::default(),
            partial_result_params: PartialResultParams::default(),
            context: None,
        };
        self.call_request("textDocument/completion", params).await
    }

    /// 跳转到定义
    pub async fn goto_definition(
        &mut self,
        uri: &str,
        line: u32,
        character: u32,
    ) -> tower_lsp::jsonrpc::Result<Option<GotoDefinitionResponse>> {
        let params = GotoDefinitionParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier {
                    uri: Url::parse(uri).unwrap(),
                },
                position: Position { line, character },
            },
            work_done_progress_params: WorkDoneProgressParams::default(),
            partial_result_params: PartialResultParams::default(),
        };
        self.call_request("textDocument/definition", params).await
    }

    /// 悬停提示
    pub async fn hover(
        &mut self,
        uri: &str,
        line: u32,
        character: u32,
    ) -> tower_lsp::jsonrpc::Result<Option<Hover>> {
        let params = HoverParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier {
                    uri: Url::parse(uri).unwrap(),
                },
                position: Position { line, character },
            },
            work_done_progress_params: WorkDoneProgressParams::default(),
        };
        self.call_request("textDocument/hover", params).await
    }

    /// 查找引用
    pub async fn references(
        &mut self,
        uri: &str,
        line: u32,
        character: u32,
    ) -> tower_lsp::jsonrpc::Result<Option<Vec<Location>>> {
        let params = ReferenceParams {
            text_document_position: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier {
                    uri: Url::parse(uri).unwrap(),
                },
                position: Position { line, character },
            },
            work_done_progress_params: WorkDoneProgressParams::default(),
            partial_result_params: PartialResultParams::default(),
            context: ReferenceContext {
                include_declaration: true,
            },
        };
        self.call_request("textDocument/references", params).await
    }

    /// 文档符号
    pub async fn document_symbol(
        &mut self,
        uri: &str,
    ) -> tower_lsp::jsonrpc::Result<Option<DocumentSymbolResponse>> {
        let params = DocumentSymbolParams {
            text_document: TextDocumentIdentifier {
                uri: Url::parse(uri).unwrap(),
            },
            work_done_progress_params: WorkDoneProgressParams::default(),
            partial_result_params: PartialResultParams::default(),
        };
        self.call_request("textDocument/documentSymbol", params).await
    }

    /// 格式化文档
    pub async fn formatting(
        &mut self,
        uri: &str,
    ) -> tower_lsp::jsonrpc::Result<Option<Vec<TextEdit>>> {
        let params = DocumentFormattingParams {
            text_document: TextDocumentIdentifier {
                uri: Url::parse(uri).unwrap(),
            },
            options: FormattingOptions::default(),
            work_done_progress_params: WorkDoneProgressParams::default(),
        };
        self.call_request("textDocument/formatting", params).await
    }

    /// 工作区符号
    pub async fn workspace_symbol(
        &mut self,
        query: &str,
    ) -> tower_lsp::jsonrpc::Result<Option<Vec<SymbolInformation>>> {
        let params = WorkspaceSymbolParams {
            query: query.to_string(),
            work_done_progress_params: WorkDoneProgressParams::default(),
            partial_result_params: PartialResultParams::default(),
        };
        self.call_request("workspace/symbol", params).await
    }

    /// 发送请求（无参数）
    async fn call_request_no_params<R>(
        &mut self,
        method: &'static str,
    ) -> tower_lsp::jsonrpc::Result<R>
    where
        R: serde::de::DeserializeOwned,
    {
        use std::sync::atomic::Ordering;

        let request = tower_lsp::jsonrpc::Request::build(method)
            .id(self.request_id.fetch_add(1, Ordering::SeqCst))
            .finish();

        let mut service = self.service.lock().await;
        tokio::task::yield_now().await;
        let response = service.call(request).await.unwrap();

        match response {
            Some(r) => {
                if let Some(error) = r.error() {
                    return Err(error.clone());
                }
                let result = r.result().expect("Expected result for successful response");
                Ok(serde_json::from_value(result.clone()).unwrap())
            }
            None => panic!("Expected response for request: {}", method),
        }
    }

    /// 发送请求
    async fn call_request<P, R>(
        &mut self,
        method: &'static str,
        params: P,
    ) -> tower_lsp::jsonrpc::Result<R>
    where
        P: serde::Serialize,
        R: serde::de::DeserializeOwned,
    {
        use std::sync::atomic::Ordering;

        let request = tower_lsp::jsonrpc::Request::build(method)
            .id(self.request_id.fetch_add(1, Ordering::SeqCst))
            .params(serde_json::to_value(params).unwrap())
            .finish();

        let mut service = self.service.lock().await;
        tokio::task::yield_now().await;
        let response = service.call(request).await.unwrap();

        match response {
            Some(r) => {
                if let Some(error) = r.error() {
                    return Err(error.clone());
                }
                let result = r.result().expect("Expected result for successful response");
                Ok(serde_json::from_value(result.clone()).unwrap())
            }
            None => panic!("Expected response for request: {}", method),
        }
    }

    /// 发送通知
    async fn call_notification<P>(&mut self, method: &'static str, params: P)
    where
        P: serde::Serialize,
    {
        let request = tower_lsp::jsonrpc::Request::build(method)
            .params(serde_json::to_value(params).unwrap())
            .finish();

        let mut service = self.service.lock().await;
        tokio::task::yield_now().await;
        let response = service.call(request).await.unwrap();
        assert!(response.is_none(), "Notification should not have response");
    }
}

impl Default for LspTestClient {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================
// 辅助函数
// ============================================================

/// 从 DocumentSymbolResponse 中提取符号列表
///
/// tower_lsp 将空数组 `[]` 反序列化为 `Flat([])` 而非 `Nested([])`，
/// 因为两者序列化结果相同，反序列化时无法区分。
fn nested_symbols(resp: Option<DocumentSymbolResponse>) -> Vec<DocumentSymbol> {
    match resp {
        Some(DocumentSymbolResponse::Nested(v)) => v,
        // Empty array is deserialized as Flat due to JSON ambiguity — treat as empty
        Some(DocumentSymbolResponse::Flat(v)) if v.is_empty() => vec![],
        Some(DocumentSymbolResponse::Flat(_)) => panic!("Expected Nested, got non-empty Flat"),
        None => panic!("Expected Some(DocumentSymbolResponse), got None"),
    }
}

// ============================================================
// 集成测试
// ============================================================

#[tokio::test]
async fn test_full_lsp_lifecycle() {
    let mut client = LspTestClient::new();

    // 初始化：验证服务能力和服务名称
    let result = client.initialize().await.unwrap();
    assert!(result.capabilities.completion_provider.is_some());
    assert_eq!(result.server_info.unwrap().name, "babel-lsp");

    client.initialized().await;

    // 打开包含一个模块的文件
    client
        .did_open("file:///test.sv", "module my_module; endmodule")
        .await;

    // 补全：必须返回 SV 关键字
    let completions = client.completion("file:///test.sv", 0, 0).await.unwrap();
    assert!(completions.is_some(), "completion should return Some");
    if let Some(CompletionResponse::Array(items)) = completions {
        assert!(!items.is_empty(), "completion items should not be empty");
        assert!(
            items.iter().any(|i| i.label == "module"),
            "completion should include 'module' keyword"
        );
    }

    // 文档符号：必须返回 my_module
    let symbols = nested_symbols(client.document_symbol("file:///test.sv").await.unwrap());
    assert_eq!(symbols.len(), 1, "Should extract exactly one module symbol");
    assert_eq!(symbols[0].name, "my_module");
    assert_eq!(symbols[0].kind, SymbolKind::MODULE);

    client.did_close("file:///test.sv").await;
    client.shutdown().await.unwrap();
}

#[tokio::test]
async fn test_did_change_updates_content() {
    let mut client = LspTestClient::new();
    client.initialize().await.unwrap();
    client.initialized().await;

    client
        .did_open("file:///test.sv", "module old; endmodule")
        .await;
    client
        .did_change("file:///test.sv", "module new_mod; endmodule", 2)
        .await;

    // 变更后符号缓存应更新为 new_mod
    let symbols = nested_symbols(client.document_symbol("file:///test.sv").await.unwrap());
    assert_eq!(symbols.len(), 1);
    assert_eq!(
        symbols[0].name, "new_mod",
        "Symbol should update to new module name after did_change"
    );
}

#[tokio::test]
async fn test_hover_on_module() {
    let mut client = LspTestClient::new();
    client.initialize().await.unwrap();
    client.initialized().await;

    // "module counter;\nendmodule" — counter 在 line 0, col 7
    client
        .did_open("file:///test.sv", "module counter;\nendmodule")
        .await;

    let hover = client
        .hover("file:///test.sv", 0, 7)
        .await
        .unwrap()
        .expect("Hover should return Some for known module name at col 7");

    if let HoverContents::Markup(markup) = &hover.contents {
        assert!(
            markup.value.contains("module"),
            "Hover should show symbol kind 'module'"
        );
        assert!(
            markup.value.contains("counter"),
            "Hover should show symbol name 'counter'"
        );
    } else {
        panic!("Expected Markup hover content");
    }
}

#[tokio::test]
async fn test_hover_returns_none_for_whitespace() {
    let mut client = LspTestClient::new();
    client.initialize().await.unwrap();
    client.initialized().await;

    // "module  top;" — col 6 is a space between 'module' and 'top'
    client
        .did_open("file:///test.sv", "module  top;")
        .await;

    let hover = client.hover("file:///test.sv", 0, 6).await.unwrap();
    // word_at_position on whitespace returns None → handle_hover returns None
    assert!(
        hover.is_none(),
        "Hover on whitespace should return None"
    );
}

#[tokio::test]
async fn test_goto_definition() {
    let mut client = LspTestClient::new();
    client.initialize().await.unwrap();
    client.initialized().await;

    // adder 在 line 0, sub 在 line 2
    client
        .did_open(
            "file:///test.sv",
            "module adder;\nendmodule\nmodule sub;\nendmodule",
        )
        .await;

    // goto_definition at (0, 7) → word "adder" → found at line 0
    let def = client
        .goto_definition("file:///test.sv", 0, 7)
        .await
        .unwrap()
        .expect("goto_definition should find 'adder'");

    if let GotoDefinitionResponse::Scalar(loc) = def {
        assert_eq!(loc.range.start.line, 0, "adder is defined on line 0");
    } else {
        panic!("Expected Scalar GotoDefinitionResponse");
    }
}

#[tokio::test]
async fn test_goto_definition_second_symbol() {
    let mut client = LspTestClient::new();
    client.initialize().await.unwrap();
    client.initialized().await;

    client
        .did_open(
            "file:///test.sv",
            "module adder;\nendmodule\nmodule sub;\nendmodule",
        )
        .await;

    // goto_definition at (2, 7) → word "sub" → found at line 2
    let def = client
        .goto_definition("file:///test.sv", 2, 7)
        .await
        .unwrap()
        .expect("goto_definition should find 'sub'");

    if let GotoDefinitionResponse::Scalar(loc) = def {
        assert_eq!(loc.range.start.line, 2, "sub is defined on line 2");
    } else {
        panic!("Expected Scalar GotoDefinitionResponse");
    }
}

#[tokio::test]
async fn test_goto_definition_not_found() {
    let mut client = LspTestClient::new();
    client.initialize().await.unwrap();
    client.initialized().await;

    // "wire clk;" — no module symbols
    client.did_open("file:///test.sv", "wire clk;").await;

    // Position on 'clk' — word found but not in symbol cache
    let def = client
        .goto_definition("file:///test.sv", 0, 5)
        .await
        .unwrap();
    assert!(def.is_none(), "goto_definition should return None for non-module 'clk'");
}

#[tokio::test]
async fn test_references() {
    let mut client = LspTestClient::new();
    client.initialize().await.unwrap();
    client.initialized().await;

    // clk 出现两次：line 0 col 5, line 1 col 7
    client
        .did_open("file:///test.sv", "wire clk;\nassign clk = 1;")
        .await;

    let refs = client
        .references("file:///test.sv", 0, 5)
        .await
        .unwrap()
        .expect("references should find occurrences of 'clk'");

    assert_eq!(refs.len(), 2, "clk appears on 2 lines");
    // First reference at line 0
    assert_eq!(refs[0].range.start.line, 0);
    assert_eq!(refs[0].range.start.character, 5);
    // Second reference at line 1
    assert_eq!(refs[1].range.start.line, 1);
    assert_eq!(refs[1].range.start.character, 7);
}

#[tokio::test]
async fn test_references_on_punctuation_returns_none() {
    let mut client = LspTestClient::new();
    client.initialize().await.unwrap();
    client.initialized().await;

    // ';' at position 0 — no identifier chars adjacent → word_at_position returns None
    client.did_open("file:///test.sv", ";wire clk;").await;

    let refs = client
        .references("file:///test.sv", 0, 0)
        .await
        .unwrap();
    assert!(refs.is_none(), "references on ';' (no adjacent identifier) should return None");
}

#[tokio::test]
async fn test_formatting() {
    let mut client = LspTestClient::new();
    client.initialize().await.unwrap();
    client.initialized().await;

    client
        .did_open("file:///test.sv", "module  top  ; endmodule")
        .await;

    let edits = client.formatting("file:///test.sv").await.unwrap();
    // formatting always returns Ok(Some(...)) — empty if no verible, edits if available
    assert!(edits.is_some(), "formatting should always return Some");
}

#[tokio::test]
async fn test_workspace_symbol() {
    let mut client = LspTestClient::new();
    client.initialize().await.unwrap();
    client.initialized().await;

    client
        .did_open("file:///test.sv", "module my_module; endmodule")
        .await;

    let symbols = client
        .workspace_symbol("my_module")
        .await
        .unwrap()
        .expect("workspace_symbol should return Some");

    assert!(!symbols.is_empty(), "Should find 'my_module' symbol");
    assert_eq!(symbols[0].name, "my_module");
    assert_eq!(symbols[0].kind, SymbolKind::MODULE);
}

#[tokio::test]
async fn test_workspace_symbol_empty_query_returns_all() {
    let mut client = LspTestClient::new();
    client.initialize().await.unwrap();
    client.initialized().await;

    client
        .did_open("file:///a.sv", "module mod_a; endmodule")
        .await;
    client
        .did_open("file:///b.sv", "module mod_b; endmodule")
        .await;

    let all = client
        .workspace_symbol("")
        .await
        .unwrap()
        .expect("workspace_symbol with empty query should return Some");

    assert_eq!(all.len(), 2, "Empty query should return both modules");
    let names: Vec<&str> = all.iter().map(|s| s.name.as_str()).collect();
    assert!(names.contains(&"mod_a"));
    assert!(names.contains(&"mod_b"));
}

#[tokio::test]
async fn test_workspace_symbol_filter() {
    let mut client = LspTestClient::new();
    client.initialize().await.unwrap();
    client.initialized().await;

    client
        .did_open("file:///a.sv", "module alu_top; endmodule")
        .await;
    client
        .did_open("file:///b.sv", "module fifo; endmodule")
        .await;
    client
        .did_open("file:///c.sv", "module alu_ctrl; endmodule")
        .await;

    let alu_syms = client
        .workspace_symbol("alu")
        .await
        .unwrap()
        .expect("Should return Some for 'alu' query");

    assert_eq!(alu_syms.len(), 2, "Should match alu_top and alu_ctrl");
    for s in &alu_syms {
        assert!(s.name.contains("alu"), "All results should contain 'alu'");
    }
}

#[tokio::test]
async fn test_multiple_files() {
    let mut client = LspTestClient::new();
    client.initialize().await.unwrap();
    client.initialized().await;

    client.did_open("file:///a.sv", "module a; endmodule").await;
    client.did_open("file:///b.sv", "module b; endmodule").await;

    let syms_a = nested_symbols(client.document_symbol("file:///a.sv").await.unwrap());
    let syms_b = nested_symbols(client.document_symbol("file:///b.sv").await.unwrap());

    assert_eq!(syms_a.len(), 1);
    assert_eq!(syms_a[0].name, "a");
    assert_eq!(syms_b.len(), 1);
    assert_eq!(syms_b[0].name, "b");
}

#[tokio::test]
async fn test_multiple_modules_in_one_file() {
    let mut client = LspTestClient::new();
    client.initialize().await.unwrap();
    client.initialized().await;

    client
        .did_open(
            "file:///test.sv",
            "module top;\nendmodule\nmodule sub_mod;\nendmodule",
        )
        .await;

    let syms = nested_symbols(client.document_symbol("file:///test.sv").await.unwrap());
    assert_eq!(syms.len(), 2, "Should extract both modules");
    assert_eq!(syms[0].name, "top");
    assert_eq!(syms[1].name, "sub_mod");
}

#[tokio::test]
async fn test_vhdl_file() {
    let mut client = LspTestClient::new();
    client.initialize().await.unwrap();
    client.initialized().await;

    // VHDL entity — extract_symbols_basic only handles SV 'module' keyword
    client
        .did_open(
            "file:///test.vhd",
            "entity counter is\n  port (clk : in bit);\nend entity;",
        )
        .await;

    let syms = nested_symbols(client.document_symbol("file:///test.vhd").await.unwrap());
    // No 'module' keyword → no symbols extracted (expected limitation)
    assert_eq!(
        syms.len(),
        0,
        "extract_symbols_basic cannot parse VHDL entity declarations"
    );
}

#[tokio::test]
async fn test_tcl_file() {
    let mut client = LspTestClient::new();
    client.initialize().await.unwrap();
    client.initialized().await;

    // TCL proc — extract_symbols_basic only handles SV 'module' keyword
    client
        .did_open(
            "file:///test.tcl",
            "proc my_proc {arg} {\n  puts $arg\n}",
        )
        .await;

    let syms = nested_symbols(client.document_symbol("file:///test.tcl").await.unwrap());
    // No 'module' keyword → no symbols extracted (expected limitation)
    assert_eq!(
        syms.len(),
        0,
        "extract_symbols_basic cannot parse TCL proc declarations"
    );
}

#[tokio::test]
async fn test_did_save() {
    let mut client = LspTestClient::new();
    client.initialize().await.unwrap();
    client.initialized().await;

    client
        .did_open("file:///test.sv", "module test; endmodule")
        .await;
    // did_save should not crash and file should still be queryable
    client.did_save("file:///test.sv").await;

    let syms = nested_symbols(client.document_symbol("file:///test.sv").await.unwrap());
    assert_eq!(syms.len(), 1, "Symbols should persist after did_save");
    assert_eq!(syms[0].name, "test");
}

#[tokio::test]
async fn test_did_close_clears_symbols() {
    let mut client = LspTestClient::new();
    client.initialize().await.unwrap();
    client.initialized().await;

    client
        .did_open("file:///test.sv", "module my_mod; endmodule")
        .await;

    // Verify symbol exists before close
    let before = nested_symbols(client.document_symbol("file:///test.sv").await.unwrap());
    assert_eq!(before.len(), 1);

    client.did_close("file:///test.sv").await;

    // After close, symbol cache is cleared → empty symbols
    let after = nested_symbols(client.document_symbol("file:///test.sv").await.unwrap());
    assert_eq!(
        after.len(),
        0,
        "Symbols should be cleared after did_close"
    );
}

#[tokio::test]
async fn test_document_operations_order() {
    let mut client = LspTestClient::new();
    client.initialize().await.unwrap();
    client.initialized().await;

    client
        .did_open("file:///test.sv", "module v1; endmodule")
        .await;
    client
        .did_change("file:///test.sv", "module v2; endmodule", 2)
        .await;
    client.did_save("file:///test.sv").await;
    client
        .did_change("file:///test.sv", "module v3; endmodule", 3)
        .await;

    // Final state should reflect v3
    let syms = nested_symbols(client.document_symbol("file:///test.sv").await.unwrap());
    assert_eq!(syms.len(), 1);
    assert_eq!(syms[0].name, "v3", "Final symbol should be v3 after all changes");

    client.did_close("file:///test.sv").await;
}

#[tokio::test]
async fn test_completion_after_no_open() {
    let mut client = LspTestClient::new();
    client.initialize().await.unwrap();
    client.initialized().await;

    // No did_open — symbol cache is empty, but SV keywords always returned
    let completions = client.completion("file:///missing.sv", 0, 0).await.unwrap();
    assert!(completions.is_some(), "completion should still return keywords even for unknown file");
    if let Some(CompletionResponse::Array(items)) = completions {
        assert!(!items.is_empty(), "SV keywords should always be in completion list");
    }
}
