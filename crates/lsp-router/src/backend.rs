//! LSP Backend 实现
//!
//! tower-lsp LanguageServer trait 实现，路由 LSP 请求到各 handler 模块

use std::sync::Arc;

use dashmap::DashMap;
use thanosLSP_core::{diagnostic::Diagnostic, file_store::FileStore, symbol::Symbol};
use tokio::sync::Mutex;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use tracing::info;

use crate::handlers::{
    completion::handle_completion,
    definition::{handle_definition, word_at_position},
    formatting::handle_formatting,
    hover::handle_hover,
    lifecycle::build_server_capabilities,
    references::find_references_in_content,
    symbols::{handle_document_symbols, handle_workspace_symbols},
    sync::{handle_did_change, handle_did_close, handle_did_open, handle_did_save},
};

/// 共享状态
struct SharedState {
    /// 文件内容存储
    file_store: Arc<FileStore>,
    /// 诊断缓存 (uri -> diagnostics)
    diag_cache: Arc<DashMap<String, Vec<Diagnostic>>>,
    /// 符号缓存 (uri -> symbols)
    symbol_cache: Arc<DashMap<String, Vec<Symbol>>>,
}

impl SharedState {
    fn new() -> Self {
        Self {
            file_store: Arc::new(FileStore::new()),
            diag_cache: Arc::new(DashMap::new()),
            symbol_cache: Arc::new(DashMap::new()),
        }
    }
}

/// thanosLSP LSP Backend
pub struct ThanosLspBackend {
    client: Client,
    state: Arc<Mutex<SharedState>>,
}

impl ThanosLspBackend {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            state: Arc::new(Mutex::new(SharedState::new())),
        }
    }

    /// 触发文件分析（当前是基本的符号提取）
    async fn analyze_file(&self, uri: Url, content: String) {
        // 基本符号提取（不依赖 slang）
        let symbols = extract_symbols_basic(&uri, &content);
        let uri_str = uri.to_string();

        let state = self.state.lock().await;
        state.symbol_cache.insert(uri_str.clone(), symbols);

        // 推送空诊断（实际诊断需要 slang/verilator）
        let diags = state
            .diag_cache
            .get(&uri_str)
            .map(|d| convert_diagnostics(d.value()))
            .unwrap_or_default();
        drop(state);

        self.client.publish_diagnostics(uri, diags, None).await;
    }
}

/// 基本符号提取（不依赖外部工具）
pub fn extract_symbols_basic(uri: &Url, content: &str) -> Vec<Symbol> {
    use smol_str::SmolStr;
    use thanosLSP_core::symbol::{Location, Position, SymbolKind};

    let mut symbols = Vec::new();
    for (line_idx, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        // module declarations
        if let Some(rest) = trimmed.strip_prefix("module ") {
            let name = rest
                .split_whitespace()
                .next()
                .unwrap_or("")
                .trim_end_matches('(')
                .trim_end_matches(';');
            if !name.is_empty()
                && name
                    .chars()
                    .next()
                    .map(|c| c.is_alphabetic() || c == '_')
                    .unwrap_or(false)
            {
                let loc = Location {
                    uri: uri.to_string(),
                    start: Position::new(line_idx as u32, 0),
                    end: Position::new(line_idx as u32, line.len() as u32),
                };
                symbols.push(Symbol::new(SmolStr::new(name), SymbolKind::Module, loc));
            }
        }
    }
    symbols
}

/// 将内部 Diagnostic 转换为 LSP Diagnostic
pub fn convert_diagnostics(diags: &[Diagnostic]) -> Vec<tower_lsp::lsp_types::Diagnostic> {
    use thanosLSP_core::diagnostic::DiagnosticSeverity as DS;
    diags
        .iter()
        .map(|d| {
            let severity = match d.severity {
                DS::Error => DiagnosticSeverity::ERROR,
                DS::Warning => DiagnosticSeverity::WARNING,
                DS::Information => DiagnosticSeverity::INFORMATION,
                DS::Hint => DiagnosticSeverity::HINT,
            };
            tower_lsp::lsp_types::Diagnostic {
                range: Range {
                    start: Position {
                        line: d.range.start.line,
                        character: d.range.start.column,
                    },
                    end: Position {
                        line: d.range.end.line,
                        character: d.range.end.column,
                    },
                },
                severity: Some(severity),
                code: d.code.as_ref().map(|c| NumberOrString::String(c.clone())),
                source: Some(d.source.clone()),
                message: d.message.clone(),
                ..Default::default()
            }
        })
        .collect()
}

#[tower_lsp::async_trait]
impl LanguageServer for ThanosLspBackend {
    async fn initialize(
        &self,
        _params: InitializeParams,
    ) -> tower_lsp::jsonrpc::Result<InitializeResult> {
        info!("thanosLSP initializing");
        Ok(InitializeResult {
            capabilities: build_server_capabilities(),
            server_info: Some(ServerInfo {
                name: "thanosLSP".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        info!("thanosLSP initialized");
    }

    async fn shutdown(&self) -> tower_lsp::jsonrpc::Result<()> {
        info!("thanosLSP shutting down");
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        let content = params.text_document.text.clone();
        let version = params.text_document.version;

        let state = self.state.lock().await;
        handle_did_open(&state.file_store, &uri, content.clone(), version);
        drop(state);

        self.analyze_file(uri, content).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        let version = params.text_document.version;
        if let Some(change) = params.content_changes.into_iter().last() {
            let content = change.text.clone();
            let state = self.state.lock().await;
            handle_did_change(&state.file_store, &uri, content.clone(), version);
            drop(state);
            self.analyze_file(uri, content).await;
        }
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        let state = self.state.lock().await;
        handle_did_save(&state.file_store, &uri);
        drop(state);
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        let state = self.state.lock().await;
        handle_did_close(&state.file_store, &uri);
        let uri_str = uri.to_string();
        state.symbol_cache.remove(&uri_str);
        state.diag_cache.remove(&uri_str);
        drop(state);
    }

    async fn completion(
        &self,
        params: CompletionParams,
    ) -> tower_lsp::jsonrpc::Result<Option<CompletionResponse>> {
        let uri_str = params.text_document_position.text_document.uri.to_string();
        let state = self.state.lock().await;
        let symbols: Vec<Symbol> = state
            .symbol_cache
            .get(&uri_str)
            .map(|s| s.value().clone())
            .unwrap_or_default();
        drop(state);

        let items = handle_completion(&symbols, "", true);
        Ok(Some(CompletionResponse::Array(items)))
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> tower_lsp::jsonrpc::Result<Option<GotoDefinitionResponse>> {
        let uri = &params.text_document_position_params.text_document.uri;
        let pos = params.text_document_position_params.position;
        let uri_str = uri.to_string();

        let state = self.state.lock().await;
        let symbols: Vec<Symbol> = state
            .symbol_cache
            .get(&uri_str)
            .map(|s| s.value().clone())
            .unwrap_or_default();
        let content = state
            .file_store
            .get(uri)
            .map(|d| d.content.to_string())
            .unwrap_or_default();
        drop(state);

        let word = word_at_position(&content, pos.line, pos.character);
        if let Some(name) = word {
            if let Some(loc) = handle_definition(&symbols, &name) {
                return Ok(Some(GotoDefinitionResponse::Scalar(loc)));
            }
        }
        Ok(None)
    }

    async fn hover(&self, params: HoverParams) -> tower_lsp::jsonrpc::Result<Option<Hover>> {
        let uri = &params.text_document_position_params.text_document.uri;
        let pos = params.text_document_position_params.position;
        let uri_str = uri.to_string();

        let state = self.state.lock().await;
        let symbols: Vec<Symbol> = state
            .symbol_cache
            .get(&uri_str)
            .map(|s| s.value().clone())
            .unwrap_or_default();
        let content = state
            .file_store
            .get(uri)
            .map(|d| d.content.to_string())
            .unwrap_or_default();
        drop(state);

        let word = word_at_position(&content, pos.line, pos.character);
        if let Some(name) = word {
            return Ok(handle_hover(&symbols, &name));
        }
        Ok(None)
    }

    async fn references(
        &self,
        params: ReferenceParams,
    ) -> tower_lsp::jsonrpc::Result<Option<Vec<Location>>> {
        let uri = &params.text_document_position.text_document.uri;
        let pos = params.text_document_position.position;
        let uri_str = uri.to_string();

        let state = self.state.lock().await;
        let content = state
            .file_store
            .get(uri)
            .map(|d| d.content.to_string())
            .unwrap_or_default();
        drop(state);

        let word = word_at_position(&content, pos.line, pos.character);
        if let Some(name) = word {
            let refs = find_references_in_content(&content, &uri_str, &name);
            return Ok(Some(refs));
        }
        Ok(None)
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> tower_lsp::jsonrpc::Result<Option<DocumentSymbolResponse>> {
        let uri_str = params.text_document.uri.to_string();
        let state = self.state.lock().await;
        let symbols: Vec<Symbol> = state
            .symbol_cache
            .get(&uri_str)
            .map(|s| s.value().clone())
            .unwrap_or_default();
        drop(state);

        let doc_syms = handle_document_symbols(&symbols);
        Ok(Some(DocumentSymbolResponse::Nested(doc_syms)))
    }

    async fn symbol(
        &self,
        params: WorkspaceSymbolParams,
    ) -> tower_lsp::jsonrpc::Result<Option<Vec<SymbolInformation>>> {
        let state = self.state.lock().await;
        let all_symbols: Vec<Symbol> = state
            .symbol_cache
            .iter()
            .flat_map(|entry| entry.value().clone())
            .collect();
        drop(state);

        let ws_syms = handle_workspace_symbols(&all_symbols, &params.query);
        // Convert WorkspaceSymbol -> SymbolInformation for older LSP compat
        #[allow(deprecated)]
        let si: Vec<SymbolInformation> = ws_syms
            .into_iter()
            .filter_map(|ws| match ws.location {
                OneOf::Left(loc) => Some(SymbolInformation {
                    name: ws.name,
                    kind: ws.kind,
                    tags: None,
                    deprecated: None,
                    location: loc,
                    container_name: ws.container_name,
                }),
                _ => None,
            })
            .collect();

        Ok(Some(si))
    }

    async fn formatting(
        &self,
        params: DocumentFormattingParams,
    ) -> tower_lsp::jsonrpc::Result<Option<Vec<TextEdit>>> {
        let uri = &params.text_document.uri;
        let state = self.state.lock().await;
        let content = state
            .file_store
            .get(uri)
            .map(|d| d.content.to_string())
            .unwrap_or_default();
        drop(state);

        let edits = handle_formatting(&content);
        Ok(Some(edits))
    }
}

/// 启动 LSP 服务（stdio 模式）
pub async fn run_stdio() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(ThanosLspBackend::new);
    Server::new(stdin, stdout, socket).serve(service).await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use thanosLSP_core::diagnostic::Diagnostic as CoreDiag;
    use thanosLSP_core::symbol::{Location, Position};

    #[test]
    fn test_extract_symbols_basic() {
        let uri = Url::parse("file:///test.sv").unwrap();
        let content = "module my_mod (\n    input logic clk\n);\nendmodule";
        let symbols = extract_symbols_basic(&uri, content);
        assert!(!symbols.is_empty());
        let names: Vec<_> = symbols.iter().map(|s| s.name.as_str()).collect();
        assert!(names.contains(&"my_mod"));
    }

    #[test]
    fn test_extract_symbols_multiple_modules() {
        let uri = Url::parse("file:///test.sv").unwrap();
        let content = "module mod_a;\nendmodule\nmodule mod_b;\nendmodule";
        let symbols = extract_symbols_basic(&uri, content);
        assert_eq!(symbols.len(), 2);
        let names: Vec<_> = symbols.iter().map(|s| s.name.as_str()).collect();
        assert!(names.contains(&"mod_a"));
        assert!(names.contains(&"mod_b"));
    }

    #[test]
    fn test_extract_symbols_empty_content() {
        let uri = Url::parse("file:///test.sv").unwrap();
        let content = "";
        let symbols = extract_symbols_basic(&uri, content);
        assert!(symbols.is_empty());
    }

    #[test]
    fn test_extract_symbols_no_modules() {
        let uri = Url::parse("file:///test.sv").unwrap();
        let content = "// just a comment\nwire a;";
        let symbols = extract_symbols_basic(&uri, content);
        assert!(symbols.is_empty());
    }

    #[test]
    fn test_extract_symbols_with_parentheses() {
        let uri = Url::parse("file:///test.sv").unwrap();
        let content = "module my_module(\n    input clk\n);\nendmodule";
        let symbols = extract_symbols_basic(&uri, content);
        assert_eq!(symbols.len(), 1);
        assert_eq!(symbols[0].name, "my_module");
    }

    #[test]
    fn test_extract_symbols_with_semicolon() {
        let uri = Url::parse("file:///test.sv").unwrap();
        let content = "module top; wire a; endmodule";
        let symbols = extract_symbols_basic(&uri, content);
        assert_eq!(symbols.len(), 1);
        assert_eq!(symbols[0].name, "top");
    }

    #[test]
    fn test_extract_symbols_invalid_names() {
        let uri = Url::parse("file:///test.sv").unwrap();
        // module name starting with number should be ignored
        let content = "module 123invalid;\nendmodule\nmodule valid_mod;\nendmodule";
        let symbols = extract_symbols_basic(&uri, content);
        let names: Vec<_> = symbols.iter().map(|s| s.name.as_str()).collect();
        assert!(names.contains(&"valid_mod"));
        assert!(!names.contains(&"123invalid"));
    }

    #[test]
    fn test_convert_diagnostics_empty() {
        let diags = convert_diagnostics(&[]);
        assert!(diags.is_empty());
    }

    #[test]
    fn test_convert_diagnostics_severity() {
        let loc = Location {
            uri: "file:///test.sv".to_string(),
            start: Position::new(0, 0),
            end: Position::new(0, 10),
        };
        let diags = vec![
            CoreDiag::error(loc.clone(), "error msg".to_string()),
            CoreDiag::warning(loc.clone(), "warn msg".to_string()),
        ];
        let lsp_diags = convert_diagnostics(&diags);
        assert_eq!(lsp_diags.len(), 2);
        assert_eq!(lsp_diags[0].severity, Some(DiagnosticSeverity::ERROR));
        assert_eq!(lsp_diags[1].severity, Some(DiagnosticSeverity::WARNING));
    }

    #[test]
    fn test_convert_diagnostics_all_severities() {
        use thanosLSP_core::diagnostic::DiagnosticSeverity as CoreSeverity;
        let loc = Location {
            uri: "file:///test.sv".to_string(),
            start: Position::new(0, 0),
            end: Position::new(0, 10),
        };
        let diags = vec![
            CoreDiag::error(loc.clone(), "error".to_string()),
            CoreDiag::warning(loc.clone(), "warn".to_string()),
            CoreDiag::new(loc.clone(), CoreSeverity::Information, "info".to_string()),
            CoreDiag::new(loc.clone(), CoreSeverity::Hint, "hint".to_string()),
        ];
        let lsp_diags = convert_diagnostics(&diags);
        assert_eq!(lsp_diags.len(), 4);
        // Use tower_lsp DiagnosticSeverity for assertion (already imported in super)
        assert_eq!(lsp_diags[0].severity, Some(DiagnosticSeverity::ERROR));
        assert_eq!(lsp_diags[1].severity, Some(DiagnosticSeverity::WARNING));
        assert_eq!(lsp_diags[2].severity, Some(DiagnosticSeverity::INFORMATION));
        assert_eq!(lsp_diags[3].severity, Some(DiagnosticSeverity::HINT));
    }

    #[test]
    fn test_convert_diagnostics_with_code_and_source() {
        let loc = Location {
            uri: "file:///test.sv".to_string(),
            start: Position::new(0, 0),
            end: Position::new(0, 10),
        };
        let mut diag = CoreDiag::error(loc, "test error".to_string());
        diag.code = Some("E001".to_string());
        diag.source = "thanosLSP".to_string();
        let lsp_diags = convert_diagnostics(&[diag]);
        assert_eq!(lsp_diags.len(), 1);
        assert_eq!(
            lsp_diags[0].code,
            Some(NumberOrString::String("E001".to_string()))
        );
        assert_eq!(lsp_diags[0].source, Some("thanosLSP".to_string()));
    }

    #[test]
    fn test_convert_diagnostics_range() {
        let loc = Location {
            uri: "file:///test.sv".to_string(),
            start: Position::new(5, 10),
            end: Position::new(8, 20),
        };
        let diag = CoreDiag::error(loc, "test".to_string());
        let lsp_diags = convert_diagnostics(&[diag]);
        assert_eq!(lsp_diags[0].range.start.line, 5);
        assert_eq!(lsp_diags[0].range.start.character, 10);
        assert_eq!(lsp_diags[0].range.end.line, 8);
        assert_eq!(lsp_diags[0].range.end.character, 20);
    }

    #[test]
    fn test_shared_state_new() {
        let state = SharedState::new();
        assert!(Arc::strong_count(&state.file_store) >= 1);
        assert!(state.diag_cache.is_empty());
        assert!(state.symbol_cache.is_empty());
    }

    #[test]
    fn test_thanos_lsp_backend_new() {
        // We can't easily create a Client without a full LSP setup,
        // but we can at least verify the new function compiles correctly
        // by checking the struct size
        use std::mem::size_of;
        let backend_size = size_of::<ThanosLspBackend>();
        assert!(backend_size > 0);
    }
}
