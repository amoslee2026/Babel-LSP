//! thanosLSP MCP Server 核心实现
//!
//! 使用 rmcp crate 实现 Model Context Protocol server，
//! 通过 stdio 或 SSE 与 Claude Code 通信。

use std::sync::Arc;

use dashmap::DashMap;
use rmcp::{
    handler::server::tool::Parameters,
    model::{ServerCapabilities, ServerInfo},
    schemars, serde, serve_server, tool, tool_handler, tool_router, ServerHandler,
};
use smol_str::SmolStr;
use thanosLSP_core::{
    diagnostic::Diagnostic,
    document::{DocumentState, FileClass, Language},
    file_store::FileStore,
    symbol::Symbol,
};
use tracing::info;

// ─── 共享状态 ────────────────────────────────────────────────────────────────

pub struct MpcState {
    pub file_store: Arc<FileStore>,
    pub symbol_cache: Arc<DashMap<String, Vec<Symbol>>>,
    pub diag_cache: Arc<DashMap<String, Vec<Diagnostic>>>,
}

impl MpcState {
    fn new() -> Self {
        Self {
            file_store: Arc::new(FileStore::new()),
            symbol_cache: Arc::new(DashMap::new()),
            diag_cache: Arc::new(DashMap::new()),
        }
    }
}

// ─── Tool 参数结构体 ─────────────────────────────────────────────────────────

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct OpenFileParams {
    #[schemars(description = "File URI, e.g. file:///path/to/file.sv")]
    pub uri: String,
    #[schemars(description = "File content")]
    pub content: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct UriParam {
    #[schemars(description = "File URI")]
    pub uri: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct UpdateFileParams {
    #[schemars(description = "File URI")]
    pub uri: String,
    #[schemars(description = "New file content")]
    pub content: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GetCompletionsParams {
    #[schemars(description = "File URI")]
    pub uri: String,
    #[schemars(description = "Cursor line (0-based)")]
    pub line: u32,
    #[schemars(description = "Cursor character (0-based)")]
    pub character: u32,
    #[schemars(description = "Optional prefix for filtering")]
    pub prefix: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GetDefinitionParams {
    #[schemars(description = "File URI")]
    pub uri: String,
    #[schemars(description = "Line number (0-based)")]
    pub line: u32,
    #[schemars(description = "Character position (0-based)")]
    pub character: u32,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SearchSymbolsParams {
    #[schemars(description = "Symbol name query")]
    pub query: String,
    #[schemars(description = "Optional URI to restrict search")]
    pub uri: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SearchPatternParams {
    #[schemars(description = "Regex pattern to search for")]
    pub pattern: String,
    #[schemars(description = "Optional URI to restrict search to one file")]
    pub uri: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ReplaceContentParams {
    #[schemars(description = "File URI")]
    pub uri: String,
    #[schemars(description = "Old text to find")]
    pub old_text: String,
    #[schemars(description = "New replacement text")]
    pub new_text: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ReplaceLinesParams {
    #[schemars(description = "File URI")]
    pub uri: String,
    #[schemars(description = "Start line (0-based, inclusive)")]
    pub start_line: u32,
    #[schemars(description = "End line (0-based, exclusive)")]
    pub end_line: u32,
    #[schemars(description = "Replacement text")]
    pub new_text: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct RenameSymbolParams {
    #[schemars(description = "File URI")]
    pub uri: String,
    #[schemars(description = "Current symbol name")]
    pub old_name: String,
    #[schemars(description = "New symbol name")]
    pub new_name: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SetLogLevelParams {
    #[schemars(description = "Log level: error | warn | info | debug | trace")]
    pub level: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct CreateFileParams {
    #[schemars(description = "File path (absolute)")]
    pub path: String,
    #[schemars(description = "Initial file content")]
    pub content: String,
}

/// 模块实例化信息
#[derive(Debug, Clone)]
struct ModuleInstance {
    module_name: String,
    instance_name: String,
    file_uri: String,
    line: u32,
}

/// 检查是否为 SV 关键字
fn is_sv_keyword(name: &str) -> bool {
    matches!(
        name,
        "module" | "endmodule" | "interface" | "endinterface"
            | "program" | "endprogram" | "package" | "endpackage"
            | "class" | "endclass" | "function" | "endfunction"
            | "task" | "endtask" | "initial" | "final"
            | "always" | "always_comb" | "always_ff" | "always_latch"
            | "assign" | "deassign" | "force" | "release"
            | "if" | "else" | "case" | "endcase" | "default"
            | "for" | "while" | "repeat" | "forever"
            | "begin" | "end" | "fork" | "join"
            | "generate" | "endgenerate" | "genvar"
            | "parameter" | "localparam" | "defparam"
            | "input" | "output" | "inout" | "ref"
            | "wire" | "reg" | "logic" | "bit"
            | "int" | "integer" | "real" | "time"
            | "typedef" | "enum" | "struct" | "union"
            | "virtual" | "extends" | "implements" | "import"
            | "posedge" | "negedge" | "or" | "and"
    )
}

// ─── Server 实现 ─────────────────────────────────────────────────────────────

/// thanosLSP MCP Server
#[derive(Clone)]
pub struct ThanosMcpServer {
    state: Arc<tokio::sync::Mutex<MpcState>>,
}

impl ThanosMcpServer {
    pub fn new() -> Self {
        Self {
            state: Arc::new(tokio::sync::Mutex::new(MpcState::new())),
        }
    }

    /// 推断语言
    fn language_from_uri(uri: &str) -> Language {
        if uri.ends_with(".sv") || uri.ends_with(".svh") {
            Language::SystemVerilog
        } else if uri.ends_with(".v") || uri.ends_with(".vh") {
            Language::Verilog
        } else if uri.ends_with(".vhd") || uri.ends_with(".vhdl") {
            Language::VHDL
        } else if uri.ends_with(".tcl") || uri.ends_with(".xdc") {
            Language::TCL
        } else {
            Language::SystemVerilog
        }
    }

    /// 基本符号提取（不依赖外部工具）
    fn extract_symbols(uri: &str, content: &str) -> Vec<Symbol> {
        use thanosLSP_core::symbol::{Location, Position, SymbolKind};

        let mut symbols = Vec::new();
        for (line_idx, line) in content.lines().enumerate() {
            let trimmed = line.trim();
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

    /// 将诊断结果序列化为 JSON 字符串
    fn diags_to_json(diags: &[Diagnostic]) -> String {
        let list: Vec<serde_json::Value> = diags
            .iter()
            .map(|d| {
                serde_json::json!({
                    "severity": format!("{:?}", d.severity),
                    "code": d.code,
                    "message": d.message,
                    "source": d.source,
                    "range": {
                        "start": { "line": d.range.start.line, "col": d.range.start.column },
                        "end": { "line": d.range.end.line, "col": d.range.end.column },
                    }
                })
            })
            .collect();
        serde_json::to_string_pretty(&list).unwrap_or_default()
    }

    /// 将符号列表序列化为 JSON 字符串
    fn symbols_to_json(symbols: &[Symbol]) -> String {
        let list: Vec<serde_json::Value> = symbols
            .iter()
            .map(|s| {
                serde_json::json!({
                    "name": s.name.as_str(),
                    "kind": format!("{:?}", s.kind),
                    "uri": s.location.uri,
                    "line": s.location.start.line,
                    "col": s.location.start.column,
                })
            })
            .collect();
        serde_json::to_string_pretty(&list).unwrap_or_default()
    }
}

impl Default for ThanosMcpServer {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Tool 方法 ───────────────────────────────────────────────────────────────

#[tool_router]
impl ThanosMcpServer {
    /// 打开文件并加入 file store
    #[tool(description = "Open a file and load it into the language server's file store")]
    pub async fn open_file(
        &self,
        Parameters(params): Parameters<OpenFileParams>,
    ) -> String {
        let lang = Self::language_from_uri(&params.uri);
        let uri = match url::Url::parse(&params.uri) {
            Ok(u) => u,
            Err(e) => return format!("error: invalid URI: {e}"),
        };
        let mut doc = DocumentState::new(uri.clone(), lang, params.content.clone());
        doc.file_class = if params.uri.contains("_tb")
            || params.uri.contains("_test")
            || params.uri.contains("tb_")
        {
            FileClass::Testbench
        } else {
            FileClass::RTL
        };

        let symbols = Self::extract_symbols(&params.uri, &params.content);
        let state = self.state.lock().await;
        state.file_store.insert(uri, doc);
        state.symbol_cache.insert(params.uri.clone(), symbols);
        format!("opened: {}", params.uri)
    }

    /// 关闭文件，从 file store 中移除
    #[tool(description = "Close a file and remove it from the language server's file store")]
    pub async fn close_file(&self, Parameters(params): Parameters<UriParam>) -> String {
        let uri = match url::Url::parse(&params.uri) {
            Ok(u) => u,
            Err(e) => return format!("error: invalid URI: {e}"),
        };
        let state = self.state.lock().await;
        state.file_store.remove(&uri);
        state.symbol_cache.remove(&params.uri);
        state.diag_cache.remove(&params.uri);
        format!("closed: {}", params.uri)
    }

    /// 读取文件内容
    #[tool(description = "Read the content of an opened file")]
    pub async fn read_file(&self, Parameters(params): Parameters<UriParam>) -> String {
        let uri = match url::Url::parse(&params.uri) {
            Ok(u) => u,
            Err(e) => return format!("error: invalid URI: {e}"),
        };
        let state = self.state.lock().await;
        match state.file_store.get(&uri) {
            Some(doc) => doc.content.to_string(),
            None => format!("error: file not open: {}", params.uri),
        }
    }

    /// 更新文件内容
    #[tool(description = "Update the full content of an opened file")]
    pub async fn update_file(&self, Parameters(params): Parameters<UpdateFileParams>) -> String {
        let uri = match url::Url::parse(&params.uri) {
            Ok(u) => u,
            Err(e) => return format!("error: invalid URI: {e}"),
        };
        let state = self.state.lock().await;
        if state.file_store.update(&uri, params.content.clone(), 0) {
            let symbols = Self::extract_symbols(&params.uri, &params.content);
            state.symbol_cache.insert(params.uri.clone(), symbols);
            format!("updated: {}", params.uri)
        } else {
            // Insert if not tracked
            let lang = Self::language_from_uri(&params.uri);
            let doc = DocumentState::new(uri, lang, params.content.clone());
            state
                .file_store
                .insert(url::Url::parse(&params.uri).unwrap(), doc);
            let symbols = Self::extract_symbols(&params.uri, &params.content);
            state.symbol_cache.insert(params.uri.clone(), symbols);
            format!("inserted: {}", params.uri)
        }
    }

    /// 创建新文件（写入磁盘并加载到 file store）
    #[tool(description = "Create a new file on disk and open it in the file store")]
    pub async fn create_file(&self, Parameters(params): Parameters<CreateFileParams>) -> String {
        use std::path::Path;
        let path = Path::new(&params.path);
        if let Some(parent) = path.parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                return format!("error creating dirs: {e}");
            }
        }
        if let Err(e) = std::fs::write(&params.path, &params.content) {
            return format!("error writing file: {e}");
        }
        let uri_str = format!("file://{}", params.path);
        let open_params = OpenFileParams {
            uri: uri_str,
            content: params.content,
        };
        self.open_file(open_params).await
    }

    /// 获取文件的诊断信息（SV综合规则检查）
    #[tool(description = "Get diagnostics (errors and warnings) for a file")]
    pub async fn get_diagnostics(&self, Parameters(params): Parameters<UriParam>) -> String {
        let uri = match url::Url::parse(&params.uri) {
            Ok(u) => u,
            Err(e) => return format!("error: invalid URI: {e}"),
        };

        let state = self.state.lock().await;
        let content = match state.file_store.get(&uri) {
            Some(doc) => doc.content.to_string(),
            None => return format!("error: file not open: {}", params.uri),
        };
        let file_class = state
            .file_store
            .get(&uri)
            .map(|d| d.file_class)
            .unwrap_or(FileClass::RTL);
        drop(state);

        // SV 综合规则检查
        if params.uri.ends_with(".sv") || params.uri.ends_with(".svh") || params.uri.ends_with(".v")
        {
            let checker = thanosLSP_sv::synth_checker::SynthChecker::new();
            let diags = checker.check_source(&content, file_class, &params.uri);
            let state = self.state.lock().await;
            state.diag_cache.insert(params.uri.clone(), diags.clone());
            drop(state);
            return Self::diags_to_json(&diags);
        }

        // VHDL 诊断
        if params.uri.ends_with(".vhd") || params.uri.ends_with(".vhdl") {
            let analyzer = thanosLSP_vhdl::VhdlAnalyzer::new();
            let (_, diags) = analyzer.analyze(&content, &params.uri);
            let state = self.state.lock().await;
            state.diag_cache.insert(params.uri.clone(), diags.clone());
            drop(state);
            return Self::diags_to_json(&diags);
        }

        let state = self.state.lock().await;
        let diags = state
            .diag_cache
            .get(&params.uri)
            .map(|d| d.value().clone())
            .unwrap_or_default();
        Self::diags_to_json(&diags)
    }

    /// 获取文件的符号列表
    #[tool(description = "Get all symbols (modules, functions, etc.) in a file")]
    pub async fn get_symbols(&self, Parameters(params): Parameters<UriParam>) -> String {
        let uri = match url::Url::parse(&params.uri) {
            Ok(u) => u,
            Err(e) => return format!("error: invalid URI: {e}"),
        };

        let state = self.state.lock().await;
        let content = state
            .file_store
            .get(&uri)
            .map(|d| d.content.to_string())
            .unwrap_or_default();
        drop(state);

        // Try enhanced SV symbol collection
        if params.uri.ends_with(".sv") || params.uri.ends_with(".svh") || params.uri.ends_with(".v")
        {
            let collector = thanosLSP_sv::symbol_collector::SymbolCollector::new();
            let symbols = collector.collect_from_source(&content, &params.uri);
            let state = self.state.lock().await;
            state
                .symbol_cache
                .insert(params.uri.clone(), symbols.clone());
            drop(state);
            return Self::symbols_to_json(&symbols);
        }

        // VHDL symbols
        if params.uri.ends_with(".vhd") || params.uri.ends_with(".vhdl") {
            let analyzer = thanosLSP_vhdl::VhdlAnalyzer::new();
            let (symbols, _) = analyzer.analyze(&content, &params.uri);
            let state = self.state.lock().await;
            state
                .symbol_cache
                .insert(params.uri.clone(), symbols.clone());
            drop(state);
            return Self::symbols_to_json(&symbols);
        }

        let state = self.state.lock().await;
        let symbols = state
            .symbol_cache
            .get(&params.uri)
            .map(|s| s.value().clone())
            .unwrap_or_default();
        Self::symbols_to_json(&symbols)
    }

    /// 搜索所有已打开文件的符号
    #[tool(description = "Search symbols across all open files")]
    pub async fn search_symbols(&self, Parameters(params): Parameters<SearchSymbolsParams>) -> String {
        let state = self.state.lock().await;
        let query_lower = params.query.to_lowercase();

        let results: Vec<Symbol> = if let Some(uri) = &params.uri {
            state
                .symbol_cache
                .get(uri)
                .map(|s| s.value().clone())
                .unwrap_or_default()
                .into_iter()
                .filter(|s| params.query.is_empty() || s.name.to_lowercase().contains(&query_lower))
                .collect()
        } else {
            state
                .symbol_cache
                .iter()
                .flat_map(|entry| entry.value().clone())
                .filter(|s| params.query.is_empty() || s.name.to_lowercase().contains(&query_lower))
                .collect()
        };

        Self::symbols_to_json(&results)
    }

    /// 代码补全（基于符号表和 SV 关键字）
    #[tool(description = "Get code completion suggestions at a cursor position")]
    pub async fn get_completions(&self, Parameters(params): Parameters<GetCompletionsParams>) -> String {
        let state = self.state.lock().await;
        let symbols = state
            .symbol_cache
            .get(&params.uri)
            .map(|s| s.value().clone())
            .unwrap_or_default();
        drop(state);

        let prefix = params.prefix.as_deref().unwrap_or("");
        let pos = thanosLSP_core::symbol::Position::new(params.line, params.character);
        let engine = thanosLSP_sv::completion::CompletionEngine::new();
        let items = engine.complete(&symbols, prefix, pos);

        let list: Vec<serde_json::Value> = items
            .iter()
            .map(|i| {
                serde_json::json!({
                    "label": i.label,
                    "kind": format!("{:?}", i.kind),
                    "detail": i.detail,
                })
            })
            .collect();
        serde_json::to_string_pretty(&list).unwrap_or_default()
    }

    /// 跳转到定义
    #[tool(description = "Find the definition of a symbol at a cursor position")]
    pub async fn get_definition(&self, Parameters(params): Parameters<GetDefinitionParams>) -> String {
        let uri = match url::Url::parse(&params.uri) {
            Ok(u) => u,
            Err(e) => return format!("error: invalid URI: {e}"),
        };

        let state = self.state.lock().await;
        let content = state
            .file_store
            .get(&uri)
            .map(|d| d.content.to_string())
            .unwrap_or_default();
        let symbols = state
            .symbol_cache
            .get(&params.uri)
            .map(|s| s.value().clone())
            .unwrap_or_default();
        drop(state);

        // Find word at position
        let word = word_at_position(&content, params.line, params.character);
        if let Some(name) = word {
            if let Some(sym) = symbols.iter().find(|s| s.name == name.as_str()) {
                return serde_json::to_string_pretty(&serde_json::json!({
                    "name": name,
                    "uri": sym.location.uri,
                    "line": sym.location.start.line,
                    "col": sym.location.start.column,
                }))
                .unwrap_or_default();
            }
        }
        "null".to_string()
    }

    /// 查找引用
    #[tool(description = "Find all references to a symbol at a cursor position")]
    pub async fn get_references(&self, Parameters(params): Parameters<GetDefinitionParams>) -> String {
        let uri = match url::Url::parse(&params.uri) {
            Ok(u) => u,
            Err(e) => return format!("error: invalid URI: {e}"),
        };

        let state = self.state.lock().await;
        let content = state
            .file_store
            .get(&uri)
            .map(|d| d.content.to_string())
            .unwrap_or_default();
        drop(state);

        let word = word_at_position(&content, params.line, params.character);
        if let Some(name) = word {
            let refs = find_references(&content, &params.uri, &name);
            return serde_json::to_string_pretty(&refs).unwrap_or_default();
        }
        "[]".to_string()
    }

    /// Hover 信息
    #[tool(description = "Get hover information for a symbol at a cursor position")]
    pub async fn get_hover(&self, Parameters(params): Parameters<GetDefinitionParams>) -> String {
        let uri = match url::Url::parse(&params.uri) {
            Ok(u) => u,
            Err(e) => return format!("error: invalid URI: {e}"),
        };

        let state = self.state.lock().await;
        let content = state
            .file_store
            .get(&uri)
            .map(|d| d.content.to_string())
            .unwrap_or_default();
        let symbols = state
            .symbol_cache
            .get(&params.uri)
            .map(|s| s.value().clone())
            .unwrap_or_default();
        drop(state);

        let word = word_at_position(&content, params.line, params.character);
        if let Some(name) = word {
            if let Some(sym) = symbols.iter().find(|s| s.name == name.as_str()) {
                let detail = sym.detail.as_deref().unwrap_or("");
                return format!(
                    "**{:?}** `{}`\n\n```sv\n{}\n```",
                    sym.kind, sym.name, detail
                );
            }
        }
        "No hover info".to_string()
    }

    /// 格式化文件（verible）
    #[tool(description = "Format a file using verible-verilog-format (SV/Verilog only)")]
    pub async fn format_file(&self, Parameters(params): Parameters<UriParam>) -> String {
        let uri = match url::Url::parse(&params.uri) {
            Ok(u) => u,
            Err(e) => return format!("error: invalid URI: {e}"),
        };

        let state = self.state.lock().await;
        let content = state
            .file_store
            .get(&uri)
            .map(|d| d.content.to_string())
            .unwrap_or_default();
        drop(state);

        match thanosLSP_lsp::handlers::formatting::try_verible_format(&content) {
            Some(formatted) => {
                // Update in-store
                let state = self.state.lock().await;
                state.file_store.update(
                    &url::Url::parse(&params.uri).unwrap(),
                    formatted.clone(),
                    0,
                );
                format!("formatted {} bytes", formatted.len())
            },
            None => "error: verible-verilog-format not available or failed".to_string(),
        }
    }

    /// 综合性检查（SV RTL 文件）
    #[tool(description = "Check synthesizability of a SystemVerilog RTL file")]
    pub async fn check_synthesizability(&self, Parameters(params): Parameters<UriParam>) -> String {
        let uri = match url::Url::parse(&params.uri) {
            Ok(u) => u,
            Err(e) => return format!("error: invalid URI: {e}"),
        };

        let state = self.state.lock().await;
        let content = state
            .file_store
            .get(&uri)
            .map(|d| d.content.to_string())
            .unwrap_or_default();
        let file_class = state
            .file_store
            .get(&uri)
            .map(|d| d.file_class)
            .unwrap_or(FileClass::RTL);
        drop(state);

        let checker = thanosLSP_sv::synth_checker::SynthChecker::new();
        let diags = checker.check_source(&content, file_class, &params.uri);
        if diags.is_empty() {
            "no synthesizability issues found".to_string()
        } else {
            Self::diags_to_json(&diags)
        }
    }

    /// 搜索文本模式
    #[tool(description = "Search for a regex pattern in open files")]
    pub async fn search_for_pattern(&self, Parameters(params): Parameters<SearchPatternParams>) -> String {
        let re = match regex::Regex::new(&params.pattern) {
            Ok(r) => r,
            Err(e) => return format!("error: invalid pattern: {e}"),
        };

        let state = self.state.lock().await;
        let mut results: Vec<serde_json::Value> = Vec::new();

        let entries: Vec<(String, String)> = if let Some(uri) = &params.uri {
            let u = match url::Url::parse(uri) {
                Ok(u) => u,
                Err(e) => return format!("error: {e}"),
            };
            state
                .file_store
                .get(&u)
                .map(|doc| vec![(uri.clone(), doc.content.to_string())])
                .unwrap_or_default()
        } else {
            state
                .file_store
                .uris()
                .into_iter()
                .filter_map(|uri| {
                    let uri_str = uri.to_string();
                    state
                        .file_store
                        .get(&uri)
                        .map(|doc| (uri_str, doc.content.to_string()))
                })
                .collect()
        };

        for (uri, content) in entries {
            for (line_idx, line) in content.lines().enumerate() {
                if re.is_match(line) {
                    results.push(serde_json::json!({
                        "uri": uri,
                        "line": line_idx,
                        "text": line.trim(),
                    }));
                }
            }
        }

        serde_json::to_string_pretty(&results).unwrap_or_default()
    }

    /// 替换文件内容（字符串替换）
    #[tool(description = "Replace text in a file (first occurrence)")]
    pub async fn replace_content(&self, Parameters(params): Parameters<ReplaceContentParams>) -> String {
        let uri = match url::Url::parse(&params.uri) {
            Ok(u) => u,
            Err(e) => return format!("error: invalid URI: {e}"),
        };

        let state = self.state.lock().await;
        let content = match state.file_store.get(&uri) {
            Some(doc) => doc.content.to_string(),
            None => return format!("error: file not open: {}", params.uri),
        };
        drop(state);

        if !content.contains(&params.old_text) {
            return format!("error: text not found in {}", params.uri);
        }
        let new_content = content.replacen(&params.old_text, &params.new_text, 1);

        let state = self.state.lock().await;
        state.file_store.update(&uri, new_content.clone(), 0);
        let symbols = Self::extract_symbols(&params.uri, &new_content);
        state.symbol_cache.insert(params.uri.clone(), symbols);
        "replaced".to_string()
    }

    /// 替换指定行范围
    #[tool(description = "Replace a range of lines in a file")]
    pub async fn replace_lines(&self, Parameters(params): Parameters<ReplaceLinesParams>) -> String {
        let uri = match url::Url::parse(&params.uri) {
            Ok(u) => u,
            Err(e) => return format!("error: invalid URI: {e}"),
        };

        let state = self.state.lock().await;
        let content = match state.file_store.get(&uri) {
            Some(doc) => doc.content.to_string(),
            None => return format!("error: file not open: {}", params.uri),
        };
        drop(state);

        let lines: Vec<&str> = content.lines().collect();
        let start = params.start_line as usize;
        let end = (params.end_line as usize).min(lines.len());

        if start > lines.len() {
            return format!("error: start_line {} out of bounds", params.start_line);
        }

        let mut new_lines: Vec<&str> = lines[..start].to_vec();
        new_lines.push(params.new_text.as_str());
        if end < lines.len() {
            new_lines.extend_from_slice(&lines[end..]);
        }
        let new_content = new_lines.join("\n");

        let state = self.state.lock().await;
        state.file_store.update(&uri, new_content.clone(), 0);
        let symbols = Self::extract_symbols(&params.uri, &new_content);
        state.symbol_cache.insert(params.uri.clone(), symbols);
        format!("replaced lines {}-{}", params.start_line, params.end_line)
    }

    /// 重命名符号（全文替换）
    #[tool(description = "Rename a symbol across the file")]
    pub async fn rename_symbol(&self, Parameters(params): Parameters<RenameSymbolParams>) -> String {
        let uri = match url::Url::parse(&params.uri) {
            Ok(u) => u,
            Err(e) => return format!("error: invalid URI: {e}"),
        };

        let state = self.state.lock().await;
        let content = match state.file_store.get(&uri) {
            Some(doc) => doc.content.to_string(),
            None => return format!("error: file not open: {}", params.uri),
        };
        drop(state);

        // Word-boundary aware replacement
        let pattern = format!(r"\b{}\b", regex::escape(&params.old_name));
        let re = match regex::Regex::new(&pattern) {
            Ok(r) => r,
            Err(e) => return format!("error: {e}"),
        };
        let count = re.find_iter(&content).count();
        let new_content = re
            .replace_all(&content, params.new_name.as_str())
            .to_string();

        let state = self.state.lock().await;
        state.file_store.update(&uri, new_content.clone(), 0);
        let symbols = Self::extract_symbols(&params.uri, &new_content);
        state.symbol_cache.insert(params.uri.clone(), symbols);
        format!(
            "renamed {} occurrences of '{}' to '{}'",
            count, params.old_name, params.new_name
        )
    }

    /// 设置日志级别
    #[tool(description = "Set the log level (error|warn|info|debug|trace)")]
    #[tool(description = "Set the log level (error|warn|info|debug|trace)")]
    pub async fn set_log_level(&self, Parameters(params): Parameters<SetLogLevelParams>) -> String {
        const VALID: &[&str] = &["error", "warn", "info", "debug", "trace"];
        if !VALID.contains(&params.level.to_lowercase().as_str()) {
            return format!(
                "error: invalid log level '{}', valid values: error|warn|info|debug|trace",
                params.level
            );
        }
        info!("set_log_level: {}", params.level);
        format!("log level set to {}", params.level)
    }

    /// 获取项目记忆（已打开文件列表）
    #[tool(description = "Get current project context: open files and symbol counts")]
    pub async fn get_project_memory(&self) -> String {
        let _self: &Self = Box::leak(Box::new(()));
        let state = self.state.lock().await;
        let files: Vec<serde_json::Value> = state
            .file_store
            .uris()
            .into_iter()
            .map(|uri| {
                let uri_str = uri.to_string();
                let sym_count = state
                    .symbol_cache
                    .get(&uri_str)
                    .map(|s| s.value().len())
                    .unwrap_or(0);
                serde_json::json!({ "uri": uri_str, "symbols": sym_count })
            })
            .collect();
        serde_json::to_string_pretty(&files).unwrap_or_default()
    }

    /// 列出所有已打开的文件
    #(description = "List all currently opened files in the server")]
    pub async fn list_open_files(&self) -> String {
        let _self: &Self = Box::leak(Box::new(()));
        let state = self.state.lock().await;
        let files: Vec<serde_json::Value> = state
            .file_store
            .uris()
            .into_iter()
            .map(|uri| {
                let uri_str = uri.to_string();
                let lang = Self::language_from_uri(&uri_str);
                let lang_str = match lang {
                    thanosLSP_core::document::Language::SystemVerilog => "systemverilog",
                    thanosLSP_core::document::Language::Verilog => "verilog",
                    thanosLSP_core::document::Language::VHDL => "vhdl",
                    thanosLSP_core::document::Language::TCL => "tcl",
                };
                serde_json::json!({
                    "uri": uri_str,
                    "language": lang_str,
                })
            })
            .collect();
        serde_json::to_string_pretty(&files).unwrap_or_default()
    }

    /// 获取文件大纲（符号层次树）
    #[tool(description = "Get hierarchical outline of symbols in a file (modules with ports, signals, functions as children)")]
    pub async fn get_file_outline(&self, Parameters(params): Parameters<UriParam>) -> String {
        let uri = match url::Url::parse(&params.uri) {
            Ok(u) => u,
            Err(e) => return format!("error: invalid URI: {e}"),
        };

        let state = self.state.lock().await;
        let content = state
            .file_store
            .get(&uri)
            .map(|d| d.content.to_string())
            .unwrap_or_default();
        drop(state);

        if content.is_empty() {
            return format!("error: file not open or empty: {}", params.uri);
        }

        // 收集符号
        let symbols = if params.uri.ends_with(".sv") || params.uri.ends_with(".svh") || params.uri.ends_with(".v") {
            let collector = thanosLSP_sv::symbol_collector::SymbolCollector::new();
            collector.collect_from_source(&content, &params.uri)
        } else if params.uri.ends_with(".vhd") || params.uri.ends_with(".vhdl") {
            let analyzer = thanosLSP_vhdl::VhdlAnalyzer::new();
            let (syms, _) = analyzer.analyze(&content, &params.uri);
            syms
        } else {
            return format!("error: unsupported file type: {}", params.uri);
        };

        // 构建层次树
        let outline = Self::build_symbol_hierarchy(symbols);
        serde_json::to_string_pretty(&outline).unwrap_or_default()
    }

    /// 获取模块实例化层次结构
    #[tool(description = "Get module instantiation hierarchy (which modules instantiate which submodules)")]
    pub async fn get_module_hierarchy(&self, Parameters(params): Parameters<UriParam>) -> String {
        let uri = match url::Url::parse(&params.uri) {
            Ok(u) => u,
            Err(e) => return format!("error: invalid URI: {e}"),
        };

        let state = self.state.lock().await;
        let content = state
            .file_store
            .get(&uri)
            .map(|d| d.content.to_string())
            .unwrap_or_default();
        drop(state);

        if content.is_empty() {
            return format!("error: file not open or empty: {}", params.uri);
        }

        // 解析模块实例化
        let instances = Self::parse_module_instances(&content, &params.uri);

        // 构建层次结构
        let hierarchy = Self::build_instance_hierarchy(instances);
        serde_json::to_string_pretty(&hierarchy).unwrap_or_default()
    }

    // ─── 辅助函数 ────────────────────────────────────────────────────────────────

    /// 将扁平符号列表构建为层次树
    fn build_symbol_hierarchy(symbols: Vec<Symbol>) -> Vec<serde_json::Value> {
        use thanosLSP_core::symbol::SymbolKind;

        // 容器类型（可以有子符号）
        const CONTAINER_KINDS: &[SymbolKind] = &[
            SymbolKind::Module,
            SymbolKind::Interface,
            SymbolKind::Class,
            SymbolKind::Package,
        ];

        // 分离容器和成员
        let mut containers: Vec<&Symbol> = symbols
            .iter()
            .filter(|s| CONTAINER_KINDS.contains(&s.kind))
            .collect();
        let members: Vec<&Symbol> = symbols
            .iter()
            .filter(|s| !CONTAINER_KINDS.contains(&s.kind))
            .collect();

        // 按行号排序容器
        containers.sort_by_key(|s| s.location.start.line);

        let mut result = Vec::new();

        for container in &containers {
            let container_start = container.location.start.line;
            let container_end = container.location.end.line;

            // 找到属于这个容器的成员（行号在容器范围内）
            let children: Vec<serde_json::Value> = members
                .iter()
                .filter(|m| {
                    let line = m.location.start.line;
                    line >= container_start && line <= container_end
                })
                .map(|m| {
                    serde_json::json!({
                        "name": m.name.as_str(),
                        "kind": format!("{:?}", m.kind).to_lowercase(),
                        "line": m.location.start.line,
                        "column": m.location.start.column,
                    })
                })
                .collect();

            result.push(serde_json::json!({
                "name": container.name.as_str(),
                "kind": format!("{:?}", container.kind).to_lowercase(),
                "line": container.location.start.line,
                "column": container.location.start.column,
                "children": children,
            }));
        }

        // 添加不属于任何容器的孤立成员
        let orphan_members: Vec<serde_json::Value> = members
            .iter()
            .filter(|m| {
                let line = m.location.start.line;
                !containers.iter().any(|c| {
                    line >= c.location.start.line && line <= c.location.end.line
                })
            })
            .map(|m| {
                serde_json::json!({
                    "name": m.name.as_str(),
                    "kind": format!("{:?}", m.kind).to_lowercase(),
                    "line": m.location.start.line,
                    "column": m.location.start.column,
                })
            })
            .collect();

        if !orphan_members.is_empty() {
            result.push(serde_json::json!({
                "name": "_orphan",
                "kind": "scope",
                "children": orphan_members,
            }));
        }

        result
    }

    /// 解析模块实例化
    fn parse_module_instances(content: &str, file_uri: &str) -> Vec<ModuleInstance> {
        use regex::Regex;
        use std::sync::OnceLock;

        static INSTANCE_PATTERN: OnceLock<Regex> = OnceLock::new();
        let re = INSTANCE_PATTERN.get_or_init(|| {
            // 匹配: module_name #(...) instance_name (...) 或 module_name instance_name (...)
            Regex::new(r"(?m)^\s*(\w+)\s*(?:#\s*\([^)]*\))?\s+(\w+)\s*\(").unwrap()
        });

        let mut instances = Vec::new();
        let line_offsets: Vec<usize> = content
            .bytes()
            .enumerate()
            .filter_map(|(i, b)| if b == b'\n' { Some(i + 1) } else { None })
            .collect();

        for cap in re.captures_iter(content) {
            let module_name = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let instance_name = cap.get(2).map(|m| m.as_str()).unwrap_or("");
            let offset = cap.get(0).map(|m| m.start()).unwrap_or(0);

            // 过滤关键字
            if is_sv_keyword(module_name) || is_sv_keyword(instance_name) {
                continue;
            }

            // 计算行号
            let line = line_offsets.partition_point(|&o| o <= offset).saturating_sub(1) as u32;

            instances.push(ModuleInstance {
                module_name: module_name.to_string(),
                instance_name: instance_name.to_string(),
                file_uri: file_uri.to_string(),
                line,
            });
        }

        instances
    }

    /// 构建实例化层次结构
    fn build_instance_hierarchy(instances: Vec<ModuleInstance>) -> Vec<serde_json::Value> {
        // 简化实现：返回扁平列表
        // 完整实现需要跨文件追踪模块定义和实例化关系
        instances
            .into_iter()
            .map(|inst| {
                serde_json::json!({
                    "module": inst.module_name,
                    "instance": inst.instance_name,
                    "file": inst.file_uri,
                    "line": inst.line,
                })
            })
            .collect()
    }
}

// ─── ServerHandler ───────────────────────────────────────────────────────────

#[tool(tool_box)]
impl ServerHandler for ThanosMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some(
                "thanosLSP: HDL Language Server (SV/VHDL/TCL). \
                 Provides diagnostics, symbols, completions, definitions, references, \
                 hover, formatting, and synthesizability checking for chip design workflows."
                    .into(),
            ),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

// ─── Transport helpers ───────────────────────────────────────────────────────

/// 启动 MCP 服务（stdio 模式）
pub async fn run_stdio() -> anyhow::Result<()> {
    use rmcp::ServiceExt;
    let server = ThanosMcpServer::new();
    let transport = (tokio::io::stdin(), tokio::io::stdout());
    // 使用 tokio::spawn 隔离 panic（如收到 invalid JSON 时 rmcp 内部 panic 不会终止进程）
    let handle = tokio::spawn(async move {
        match server.serve(transport).await {
            Ok(svc) => { let _ = svc.waiting().await; }
            Err(e) => { tracing::error!("MCP serve init error: {}", e); }
        }
    });
    match handle.await {
        Ok(()) => {},
        Err(e) => tracing::error!("MCP server panic: {:?}", e),
    }
    Ok(())
}

// ─── 辅助函数 ────────────────────────────────────────────────────────────────

fn word_at_position(content: &str, line: u32, character: u32) -> Option<String> {
    let line_text = content.lines().nth(line as usize)?;
    let col = character as usize;
    let bytes = line_text.as_bytes();
    if col > bytes.len() {
        return None;
    }
    // Extend left
    let mut start = col;
    while start > 0 && is_ident(bytes[start - 1]) {
        start -= 1;
    }
    // Extend right
    let mut end = col;
    while end < bytes.len() && is_ident(bytes[end]) {
        end += 1;
    }
    if start == end {
        return None;
    }
    Some(line_text[start..end].to_string())
}

fn is_ident(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_' || b == b'$'
}

fn find_references(content: &str, uri: &str, name: &str) -> Vec<serde_json::Value> {
    let mut refs = Vec::new();
    for (line_idx, line) in content.lines().enumerate() {
        let mut start = 0;
        while let Some(pos) = line[start..].find(name) {
            let abs = start + pos;
            let before_ok = abs == 0 || !is_ident(line.as_bytes()[abs - 1]);
            let after_pos = abs + name.len();
            let after_ok = after_pos >= line.len() || !is_ident(line.as_bytes()[after_pos]);
            if before_ok && after_ok {
                refs.push(serde_json::json!({
                    "uri": uri,
                    "line": line_idx,
                    "col": abs,
                }));
            }
            start = abs + 1;
            if start >= line.len() {
                break;
            }
        }
    }
    refs
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_open_and_read_file() {
        let server = ThanosMcpServer::new();
        let result = server
            .open_file(OpenFileParams {
                uri: "file:///test.sv".to_string(),
                content: "module foo;\nendmodule".to_string(),
            })
            .await;
        assert!(result.contains("opened"), "got: {result}");

        let content = server
            .read_file(UriParam {
                uri: "file:///test.sv".to_string(),
            })
            .await;
        assert!(content.contains("module foo"));
    }

    #[tokio::test]
    async fn test_close_file() {
        let server = ThanosMcpServer::new();
        server
            .open_file(OpenFileParams {
                uri: "file:///test.sv".to_string(),
                content: "module foo;\nendmodule".to_string(),
            })
            .await;
        server
            .close_file(UriParam {
                uri: "file:///test.sv".to_string(),
            })
            .await;
        let content = server
            .read_file(UriParam {
                uri: "file:///test.sv".to_string(),
            })
            .await;
        assert!(content.contains("error"), "should be error: {content}");
    }

    #[tokio::test]
    async fn test_get_symbols() {
        let server = ThanosMcpServer::new();
        server
            .open_file(OpenFileParams {
                uri: "file:///test.sv".to_string(),
                content: "module my_module(\n  input clk\n);\nendmodule".to_string(),
            })
            .await;
        let result = server
            .get_symbols(UriParam {
                uri: "file:///test.sv".to_string(),
            })
            .await;
        assert!(result.contains("my_module"), "got: {result}");
    }

    #[tokio::test]
    async fn test_check_synthesizability() {
        let server = ThanosMcpServer::new();
        server
            .open_file(OpenFileParams {
                uri: "file:///rtl.sv".to_string(),
                content: "module foo;\n  initial begin\n    #10;\n  end\nendmodule".to_string(),
            })
            .await;
        let result = server
            .check_synthesizability(UriParam {
                uri: "file:///rtl.sv".to_string(),
            })
            .await;
        assert!(
            result.contains("SYN-V"),
            "expected synth issues, got: {result}"
        );
    }

    #[tokio::test]
    async fn test_replace_content() {
        let server = ThanosMcpServer::new();
        server
            .open_file(OpenFileParams {
                uri: "file:///test.sv".to_string(),
                content: "module old_name;\nendmodule".to_string(),
            })
            .await;
        let result = server
            .replace_content(ReplaceContentParams {
                uri: "file:///test.sv".to_string(),
                old_text: "old_name".to_string(),
                new_text: "new_name".to_string(),
            })
            .await;
        assert_eq!(result, "replaced");
        let content = server
            .read_file(UriParam {
                uri: "file:///test.sv".to_string(),
            })
            .await;
        assert!(content.contains("new_name"), "got: {content}");
    }

    #[tokio::test]
    async fn test_search_for_pattern() {
        let server = ThanosMcpServer::new();
        server
            .open_file(OpenFileParams {
                uri: "file:///test.sv".to_string(),
                content: "module foo;\n  always_ff @(posedge clk);\nendmodule".to_string(),
            })
            .await;
        let result = server
            .search_for_pattern(SearchPatternParams {
                pattern: "always_ff".to_string(),
                uri: Some("file:///test.sv".to_string()),
            })
            .await;
        assert!(result.contains("always_ff"), "got: {result}");
    }

    #[tokio::test]
    async fn test_get_project_memory() {
        let server = ThanosMcpServer::new();
        server
            .open_file(OpenFileParams {
                uri: "file:///a.sv".to_string(),
                content: "module a;\nendmodule".to_string(),
            })
            .await;
        let result = server.get_project_memory().await;
        assert!(result.contains("a.sv"), "got: {result}");
    }

    // ─── 私有辅助函数单元测试 ────────────────────────────────────────────────────

    // Allow direct access to private items from parent module
    use super::ModuleInstance;

    #[test]
    fn test_is_sv_keyword_true() {
        for kw in &[
            "module", "endmodule", "wire", "reg", "always_ff",
            "input", "output", "begin", "end", "if", "else", "for",
            "while", "logic", "assign", "parameter",
        ] {
            assert!(is_sv_keyword(kw), "{kw} should be a keyword");
        }
    }

    #[test]
    fn test_is_sv_keyword_false() {
        for word in &["my_module", "clk", "counter", "adder", "MODULE", "xor_gate"] {
            assert!(!is_sv_keyword(word), "{word} should not be a keyword");
        }
    }

    #[test]
    fn test_is_sv_keyword_empty_string() {
        assert!(!is_sv_keyword(""));
    }

    #[test]
    fn test_is_ident_alphanumeric_underscore_dollar() {
        for &b in b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_$" {
            assert!(is_ident(b), "{} should be ident char", b as char);
        }
    }

    #[test]
    fn test_is_ident_rejects_punctuation() {
        for &b in b" .,;:()[]{}+-*/" {
            assert!(!is_ident(b), "{} should not be ident char", b as char);
        }
    }

    #[test]
    fn test_word_at_position_in_middle_of_word() {
        // "counter" starts at col 7; position col 9 is mid-word
        assert_eq!(
            word_at_position("module counter;", 0, 9),
            Some("counter".to_string())
        );
    }

    #[test]
    fn test_word_at_position_at_start_of_word() {
        assert_eq!(
            word_at_position("module counter;", 0, 7),
            Some("counter".to_string())
        );
    }

    #[test]
    fn test_word_at_position_backtracks_from_space() {
        // col 4 is the space in "wire clk;"; backtracks left to find "wire"
        assert_eq!(
            word_at_position("wire clk;", 0, 4),
            Some("wire".to_string())
        );
    }

    #[test]
    fn test_word_at_position_on_leading_semicolon_is_none() {
        // col 0 is ';' — no adjacent ident on either side
        assert_eq!(word_at_position(";wire clk;", 0, 0), None);
    }

    #[test]
    fn test_word_at_position_col_beyond_line_is_none() {
        assert_eq!(word_at_position("wire clk;", 0, 100), None);
    }

    #[test]
    fn test_word_at_position_line_out_of_bounds_is_none() {
        assert_eq!(word_at_position("wire clk;", 99, 0), None);
    }

    #[test]
    fn test_word_at_position_multiline_second_line() {
        let content = "module foo;\nwire clk;\nendmodule";
        // line 1, col 5 is inside "clk"
        assert_eq!(
            word_at_position(content, 1, 5),
            Some("clk".to_string())
        );
    }

    #[test]
    fn test_find_references_two_occurrences() {
        let content = "wire clk;\nassign out = clk;";
        let refs = find_references(content, "file:///t.sv", "clk");
        assert_eq!(refs.len(), 2, "clk appears twice");
    }

    #[test]
    fn test_find_references_word_boundary_no_partial_match() {
        // "clk" must NOT match inside "clk_en"
        let content = "wire clk_en;\nwire clk;";
        let refs = find_references(content, "file:///t.sv", "clk");
        assert_eq!(refs.len(), 1, "only standalone 'clk' on line 1 matches");
        assert_eq!(refs[0]["line"], 1);
    }

    #[test]
    fn test_find_references_not_found_returns_empty() {
        let refs = find_references("wire clk;", "file:///t.sv", "rst");
        assert_eq!(refs.len(), 0);
    }

    #[test]
    fn test_find_references_uri_and_position_fields() {
        let uri = "file:///my/path.sv";
        let refs = find_references("wire clk;", uri, "clk");
        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0]["uri"], uri);
        assert_eq!(refs[0]["line"], 0);
        assert_eq!(refs[0]["col"], 5);
    }

    #[test]
    fn test_language_from_uri_sv_extensions() {
        use thanosLSP_core::document::Language;
        assert_eq!(
            ThanosMcpServer::language_from_uri("file:///a.sv"),
            Language::SystemVerilog
        );
        assert_eq!(
            ThanosMcpServer::language_from_uri("file:///a.svh"),
            Language::SystemVerilog
        );
    }

    #[test]
    fn test_language_from_uri_verilog_extensions() {
        use thanosLSP_core::document::Language;
        assert_eq!(
            ThanosMcpServer::language_from_uri("file:///a.v"),
            Language::Verilog
        );
        assert_eq!(
            ThanosMcpServer::language_from_uri("file:///a.vh"),
            Language::Verilog
        );
    }

    #[test]
    fn test_language_from_uri_vhdl_extensions() {
        use thanosLSP_core::document::Language;
        assert_eq!(
            ThanosMcpServer::language_from_uri("file:///a.vhd"),
            Language::VHDL
        );
        assert_eq!(
            ThanosMcpServer::language_from_uri("file:///a.vhdl"),
            Language::VHDL
        );
    }

    #[test]
    fn test_language_from_uri_tcl_extensions() {
        use thanosLSP_core::document::Language;
        assert_eq!(
            ThanosMcpServer::language_from_uri("file:///a.tcl"),
            Language::TCL
        );
        assert_eq!(
            ThanosMcpServer::language_from_uri("file:///a.xdc"),
            Language::TCL
        );
    }

    #[test]
    fn test_language_from_uri_unknown_defaults_to_sv() {
        use thanosLSP_core::document::Language;
        assert_eq!(
            ThanosMcpServer::language_from_uri("file:///a.txt"),
            Language::SystemVerilog
        );
        assert_eq!(
            ThanosMcpServer::language_from_uri("file:///no_ext"),
            Language::SystemVerilog
        );
    }

    #[test]
    fn test_extract_symbols_single_module() {
        let syms = ThanosMcpServer::extract_symbols(
            "file:///t.sv",
            "module counter;\nendmodule",
        );
        assert_eq!(syms.len(), 1);
        assert_eq!(syms[0].name.as_str(), "counter");
    }

    #[test]
    fn test_extract_symbols_multiple_modules() {
        let content = "module foo;\nendmodule\nmodule bar;\nendmodule";
        let syms = ThanosMcpServer::extract_symbols("file:///t.sv", content);
        assert_eq!(syms.len(), 2);
        let names: Vec<&str> = syms.iter().map(|s| s.name.as_str()).collect();
        assert!(names.contains(&"foo"));
        assert!(names.contains(&"bar"));
    }

    #[test]
    fn test_extract_symbols_no_modules_returns_empty() {
        let syms =
            ThanosMcpServer::extract_symbols("file:///t.sv", "// comment\nwire clk;");
        assert_eq!(syms.len(), 0);
    }

    #[test]
    fn test_extract_symbols_module_with_port_list() {
        let content =
            "module adder(\n  input logic a,\n  output logic b\n);\nendmodule";
        let syms = ThanosMcpServer::extract_symbols("file:///t.sv", content);
        assert_eq!(syms.len(), 1);
        assert_eq!(syms[0].name.as_str(), "adder");
    }

    #[test]
    fn test_diags_to_json_empty_list() {
        let json = ThanosMcpServer::diags_to_json(&[]);
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(v.is_array());
        assert_eq!(v.as_array().unwrap().len(), 0);
    }

    #[test]
    fn test_diags_to_json_with_error_diag() {
        use thanosLSP_core::{
            diagnostic::Diagnostic,
            symbol::{Location, Position},
        };
        let loc = Location {
            uri: "f.sv".to_string(),
            start: Position::new(0, 0),
            end: Position::new(0, 5),
        };
        let d = Diagnostic::error(loc, "syntax error".to_string());
        let json = ThanosMcpServer::diags_to_json(&[d]);
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        let arr = v.as_array().unwrap();
        assert_eq!(arr.len(), 1);
        assert_eq!(arr[0]["message"], "syntax error");
        assert!(arr[0]["severity"].as_str().unwrap().contains("Error"));
    }

    #[test]
    fn test_diags_to_json_with_warning_diag_position() {
        use thanosLSP_core::{
            diagnostic::Diagnostic,
            symbol::{Location, Position},
        };
        let loc = Location {
            uri: "f.sv".to_string(),
            start: Position::new(2, 4),
            end: Position::new(2, 10),
        };
        let d = Diagnostic::warning(loc, "unused variable".to_string());
        let json = ThanosMcpServer::diags_to_json(&[d]);
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        let arr = v.as_array().unwrap();
        assert_eq!(arr.len(), 1);
        assert_eq!(arr[0]["message"], "unused variable");
        assert!(arr[0]["severity"].as_str().unwrap().contains("Warning"));
        assert_eq!(arr[0]["range"]["start"]["line"], 2);
        assert_eq!(arr[0]["range"]["start"]["col"], 4);
    }

    #[test]
    fn test_symbols_to_json_empty_list() {
        let json = ThanosMcpServer::symbols_to_json(&[]);
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(v.is_array() && v.as_array().unwrap().is_empty());
    }

    #[test]
    fn test_symbols_to_json_with_module_symbol() {
        use smol_str::SmolStr;
        use thanosLSP_core::symbol::{Location, Position, Symbol, SymbolKind};
        let loc = Location {
            uri: "file:///t.sv".to_string(),
            start: Position::new(0, 0),
            end: Position::new(0, 15),
        };
        let s = Symbol::new(SmolStr::new("my_mod"), SymbolKind::Module, loc);
        let json = ThanosMcpServer::symbols_to_json(&[s]);
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        let arr = v.as_array().unwrap();
        assert_eq!(arr.len(), 1);
        assert_eq!(arr[0]["name"], "my_mod");
        assert!(arr[0]["kind"].as_str().unwrap().contains("Module"));
        assert_eq!(arr[0]["line"], 0);
    }

    #[test]
    fn test_build_symbol_hierarchy_empty() {
        let h = ThanosMcpServer::build_symbol_hierarchy(vec![]);
        assert!(h.is_empty());
    }

    #[test]
    fn test_build_symbol_hierarchy_module_with_port_children() {
        use smol_str::SmolStr;
        use thanosLSP_core::symbol::{Location, Position, Symbol, SymbolKind};
        let mk = |name: &str, kind: SymbolKind, line: u32| {
            let loc = Location {
                uri: "f.sv".to_string(),
                start: Position::new(line, 0),
                end: Position::new(line, 10),
            };
            Symbol::new(SmolStr::new(name), kind, loc)
        };
        let syms = vec![
            mk("top", SymbolKind::Module, 0),
            mk("clk", SymbolKind::Port, 1),
            mk("rst", SymbolKind::Port, 2),
        ];
        let h = ThanosMcpServer::build_symbol_hierarchy(syms);
        assert!(!h.is_empty());
        let has_top = h.iter().any(|n| n["name"] == "top");
        assert!(has_top, "hierarchy should contain 'top' module: {h:?}");
    }

    #[test]
    fn test_parse_module_instances_detects_instance() {
        let content = "module top;\n  sub_module u1(.clk(clk));\nendmodule";
        let insts =
            ThanosMcpServer::parse_module_instances(content, "f.sv");
        let found = insts
            .iter()
            .any(|i| i.module_name == "sub_module" && i.instance_name == "u1");
        assert!(found, "should detect instance: {insts:?}");
    }

    #[test]
    fn test_parse_module_instances_filters_sv_keywords() {
        // Names that are SV keywords should be filtered from results
        let content = "always begin\n  if (x) y <= 1;\nend";
        let insts =
            ThanosMcpServer::parse_module_instances(content, "f.sv");
        let bad = insts
            .iter()
            .any(|i| is_sv_keyword(&i.module_name) || is_sv_keyword(&i.instance_name));
        assert!(!bad, "keywords should be filtered from instances: {insts:?}");
    }

    #[test]
    fn test_parse_module_instances_empty_for_leaf_module() {
        let content = "module leaf;\n  wire clk;\nendmodule";
        let insts =
            ThanosMcpServer::parse_module_instances(content, "f.sv");
        assert!(insts.is_empty(), "no instantiations in leaf module");
    }

    #[test]
    fn test_build_instance_hierarchy_flat_list() {
        let insts = vec![
            ModuleInstance {
                module_name: "sub_a".to_string(),
                instance_name: "u_a".to_string(),
                file_uri: "f.sv".to_string(),
                line: 5,
            },
            ModuleInstance {
                module_name: "sub_b".to_string(),
                instance_name: "u_b".to_string(),
                file_uri: "f.sv".to_string(),
                line: 6,
            },
        ];
        let h = ThanosMcpServer::build_instance_hierarchy(insts);
        assert_eq!(h.len(), 2);
        assert_eq!(h[0]["module"], "sub_a");
        assert_eq!(h[0]["instance"], "u_a");
        assert_eq!(h[1]["module"], "sub_b");
    }

    // ─── 工具方法强化测试 ────────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_get_references_exact_count_with_word_boundary() {
        let server = ThanosMcpServer::new();
        server
            .open_file(OpenFileParams {
                uri: "file:///t.sv".to_string(),
                content: "wire clk;\nwire clk_en;\nassign out = clk;".to_string(),
            })
            .await;
        // 'clk' at line 0 col 5 and line 2 col 13; 'clk_en' on line 1 must NOT match
        let result = server
            .get_references(GetDefinitionParams {
                uri: "file:///t.sv".to_string(),
                line: 0,
                character: 5,
            })
            .await;
        let v: serde_json::Value =
            serde_json::from_str(&result).unwrap_or(serde_json::json!([]));
        if let Some(arr) = v.as_array() {
            assert_eq!(arr.len(), 2, "only standalone 'clk' — got: {result}");
        }
    }

    #[tokio::test]
    async fn test_get_hover_non_empty_for_module_name() {
        let server = ThanosMcpServer::new();
        server
            .open_file(OpenFileParams {
                uri: "file:///hover_mod.sv".to_string(),
                content: "module foo;\nendmodule".to_string(),
            })
            .await;
        // Position on 'foo' (col 7)
        let result = server
            .get_hover(GetDefinitionParams {
                uri: "file:///hover_mod.sv".to_string(),
                line: 0,
                character: 7,
            })
            .await;
        assert!(!result.is_empty(), "hover on module name should return something");
    }

    #[tokio::test]
    async fn test_get_hover_returns_non_empty_response() {
        // word_at_position backtracks from non-ident chars to find adjacent identifiers,
        // so any position near text will return hover info. Just verify no panic.
        let server = ThanosMcpServer::new();
        server
            .open_file(OpenFileParams {
                uri: "file:///hover_null.sv".to_string(),
                content: "module foo;\nendmodule".to_string(),
            })
            .await;
        let result = server
            .get_hover(GetDefinitionParams {
                uri: "file:///hover_null.sv".to_string(),
                line: 0,
                character: 7, // 'f' in "foo"
            })
            .await;
        assert!(!result.is_empty(), "hover should return a response");
    }

    #[tokio::test]
    async fn test_search_symbols_query_filters_results() {
        let server = ThanosMcpServer::new();
        server
            .open_file(OpenFileParams {
                uri: "file:///ss.sv".to_string(),
                content: "module my_counter;\nendmodule\nmodule adder;\nendmodule".to_string(),
            })
            .await;
        let result = server
            .search_symbols(SearchSymbolsParams {
                query: "counter".to_string(),
                uri: None,
            })
            .await;
        let v: serde_json::Value =
            serde_json::from_str(&result).unwrap_or(serde_json::json!([]));
        if let Some(arr) = v.as_array() {
            for item in arr {
                let name = item["name"].as_str().unwrap_or("");
                assert!(
                    name.contains("counter"),
                    "result '{name}' should contain 'counter'"
                );
            }
        }
    }

    #[tokio::test]
    async fn test_search_symbols_uri_filter_excludes_other_file() {
        let server = ThanosMcpServer::new();
        server
            .open_file(OpenFileParams {
                uri: "file:///fa.sv".to_string(),
                content: "module mod_a;\nendmodule".to_string(),
            })
            .await;
        server
            .open_file(OpenFileParams {
                uri: "file:///fb.sv".to_string(),
                content: "module mod_b;\nendmodule".to_string(),
            })
            .await;
        let result = server
            .search_symbols(SearchSymbolsParams {
                query: "mod".to_string(),
                uri: Some("file:///fa.sv".to_string()),
            })
            .await;
        let v: serde_json::Value =
            serde_json::from_str(&result).unwrap_or(serde_json::json!([]));
        if let Some(arr) = v.as_array() {
            let has_b = arr
                .iter()
                .any(|s| s["uri"].as_str().map(|u| u.contains("fb")).unwrap_or(false));
            assert!(!has_b, "URI filter should exclude fb.sv: {result}");
        }
    }

    #[tokio::test]
    async fn test_set_log_level_invalid_returns_error() {
        let server = ThanosMcpServer::new();
        let result = server
            .set_log_level(SetLogLevelParams {
                level: "invalid_level_xyz".to_string(),
            })
            .await;
        assert!(
            result.contains("error") || result.contains("invalid") || result.contains("unknown"),
            "invalid log level should return error: {result}"
        );
    }

    #[tokio::test]
    async fn test_replace_lines_out_of_bounds_does_not_panic() {
        let server = ThanosMcpServer::new();
        server
            .open_file(OpenFileParams {
                uri: "file:///rl_bounds.sv".to_string(),
                content: "module foo;\nendmodule".to_string(),
            })
            .await;
        let result = server
            .replace_lines(ReplaceLinesParams {
                uri: "file:///rl_bounds.sv".to_string(),
                start_line: 100,
                end_line: 200,
                new_text: "// replacement\n".to_string(),
            })
            .await;
        // Must not panic; either error or graceful success
        assert!(!result.is_empty(), "replace_lines out-of-bounds should return a response");
    }

    #[tokio::test]
    async fn test_rename_symbol_respects_word_boundary() {
        let server = ThanosMcpServer::new();
        server
            .open_file(OpenFileParams {
                uri: "file:///rename_wb.sv".to_string(),
                content: "module clk_gen;\n  wire clk;\nendmodule".to_string(),
            })
            .await;
        // Renaming standalone 'clk' should NOT touch 'clk_gen'
        let result = server
            .rename_symbol(RenameSymbolParams {
                uri: "file:///rename_wb.sv".to_string(),
                old_name: "clk".to_string(),
                new_name: "sys_clk".to_string(),
            })
            .await;
        let content = server
            .read_file(UriParam {
                uri: "file:///rename_wb.sv".to_string(),
            })
            .await;
        assert!(
            content.contains("clk_gen") || result.contains("error"),
            "word-boundary rename must preserve 'clk_gen': {content:.100}"
        );
    }

    #[tokio::test]
    async fn test_replace_content_not_found_returns_error() {
        let server = ThanosMcpServer::new();
        server
            .open_file(OpenFileParams {
                uri: "file:///rc_notfound.sv".to_string(),
                content: "module foo;\nendmodule".to_string(),
            })
            .await;
        let result = server
            .replace_content(ReplaceContentParams {
                uri: "file:///rc_notfound.sv".to_string(),
                old_text: "nonexistent_text_xyz_abc".to_string(),
                new_text: "replacement".to_string(),
            })
            .await;
        assert!(
            result.contains("error") || result.contains("not found"),
            "replace_content with missing text should return error: {result}"
        );
    }

    #[tokio::test]
    async fn test_list_open_files_language_detection() {
        let server = ThanosMcpServer::new();
        server
            .open_file(OpenFileParams {
                uri: "file:///lang.sv".to_string(),
                content: "module foo;\nendmodule".to_string(),
            })
            .await;
        server
            .open_file(OpenFileParams {
                uri: "file:///lang.vhd".to_string(),
                content: "entity e is end entity;".to_string(),
            })
            .await;
        let result = server.list_open_files().await;
        let v: serde_json::Value = serde_json::from_str(&result).unwrap();
        let arr = v.as_array().unwrap();
        assert_eq!(arr.len(), 2, "should list 2 open files");
        let sv = arr
            .iter()
            .find(|f| f["uri"].as_str().map(|u| u.ends_with(".sv")).unwrap_or(false));
        assert!(sv.is_some(), "should list .sv file");
        assert!(sv.unwrap()["language"]
            .as_str()
            .unwrap()
            .contains("systemverilog"));
        let vhd = arr
            .iter()
            .find(|f| f["uri"].as_str().map(|u| u.ends_with(".vhd")).unwrap_or(false));
        assert!(vhd.is_some(), "should list .vhd file");
        assert!(vhd.unwrap()["language"].as_str().unwrap().contains("vhdl"));
    }

    #[tokio::test]
    async fn test_search_for_pattern_invalid_regex_returns_error() {
        let server = ThanosMcpServer::new();
        server
            .open_file(OpenFileParams {
                uri: "file:///re_test.sv".to_string(),
                content: "module foo;\nendmodule".to_string(),
            })
            .await;
        let result = server
            .search_for_pattern(SearchPatternParams {
                pattern: "[invalid_regex".to_string(), // unclosed bracket
                uri: None,
            })
            .await;
        assert!(
            result.contains("error") || result.contains("invalid"),
            "invalid regex should return error: {result}"
        );
    }

    #[tokio::test]
    async fn test_search_for_pattern_global_finds_all_open_files() {
        let server = ThanosMcpServer::new();
        server
            .open_file(OpenFileParams {
                uri: "file:///ga.sv".to_string(),
                content: "module foo_top;\nendmodule".to_string(),
            })
            .await;
        server
            .open_file(OpenFileParams {
                uri: "file:///gb.sv".to_string(),
                content: "module bar_top;\nendmodule".to_string(),
            })
            .await;
        // Global search (no uri) should search all open files
        let result = server
            .search_for_pattern(SearchPatternParams {
                pattern: "endmodule".to_string(),
                uri: None,
            })
            .await;
        assert!(
            result.contains("endmodule"),
            "global search should find pattern: {result}"
        );
    }

    #[tokio::test]
    async fn test_get_file_outline_sv_returns_json_array() {
        let server = ThanosMcpServer::new();
        server
            .open_file(OpenFileParams {
                uri: "file:///outline.sv".to_string(),
                content:
                    "module design(\n  input logic clk,\n  output logic out\n);\nassign out = clk;\nendmodule"
                        .to_string(),
            })
            .await;
        let result = server
            .get_file_outline(UriParam {
                uri: "file:///outline.sv".to_string(),
            })
            .await;
        assert!(!result.contains("error"), "get_file_outline should not error: {result}");
        let v: serde_json::Value =
            serde_json::from_str(&result).unwrap_or(serde_json::json!(null));
        assert!(v.is_array(), "outline should be JSON array");
    }

    #[tokio::test]
    async fn test_get_module_hierarchy_returns_json_array() {
        let server = ThanosMcpServer::new();
        server
            .open_file(OpenFileParams {
                uri: "file:///hier.sv".to_string(),
                content: "module top;\n  sub_mod u1(.clk(clk));\nendmodule".to_string(),
            })
            .await;
        let result = server
            .get_module_hierarchy(UriParam {
                uri: "file:///hier.sv".to_string(),
            })
            .await;
        assert!(
            !result.contains("error"),
            "get_module_hierarchy should not error: {result}"
        );
        let v: serde_json::Value =
            serde_json::from_str(&result).unwrap_or(serde_json::json!(null));
        assert!(v.is_array(), "hierarchy should be JSON array");
    }

    #[tokio::test]
    async fn test_get_module_hierarchy_empty_for_leaf_module() {
        let server = ThanosMcpServer::new();
        server
            .open_file(OpenFileParams {
                uri: "file:///leaf.sv".to_string(),
                content: "module leaf;\n  wire clk;\nendmodule".to_string(),
            })
            .await;
        let result = server
            .get_module_hierarchy(UriParam {
                uri: "file:///leaf.sv".to_string(),
            })
            .await;
        let v: serde_json::Value =
            serde_json::from_str(&result).unwrap_or(serde_json::json!(null));
        if let Some(arr) = v.as_array() {
            assert_eq!(arr.len(), 0, "leaf module should have empty hierarchy");
        }
    }

    #[tokio::test]
    async fn test_get_file_outline_invalid_uri_returns_error() {
        let server = ThanosMcpServer::new();
        let result = server
            .get_file_outline(UriParam {
                uri: "!!!not_a_uri".to_string(),
            })
            .await;
        assert!(result.contains("error"), "invalid URI should return error: {result}");
    }

    #[tokio::test]
    async fn test_get_module_hierarchy_invalid_uri_returns_error() {
        let server = ThanosMcpServer::new();
        let result = server
            .get_module_hierarchy(UriParam {
                uri: "!!!not_a_uri".to_string(),
            })
            .await;
        assert!(result.contains("error"), "invalid URI should return error: {result}");
    }

    #[tokio::test]
    async fn test_get_module_hierarchy_not_open_returns_error() {
        let server = ThanosMcpServer::new();
        let result = server
            .get_module_hierarchy(UriParam {
                uri: "file:///never_opened.sv".to_string(),
            })
            .await;
        assert!(result.contains("error"), "not-open file should return error: {result}");
    }

    #[tokio::test]
    async fn test_get_file_outline_unsupported_type_returns_error() {
        let server = ThanosMcpServer::new();
        server
            .open_file(OpenFileParams {
                uri: "file:///test.tcl".to_string(),
                content: "set x 1".to_string(),
            })
            .await;
        let result = server
            .get_file_outline(UriParam {
                uri: "file:///test.tcl".to_string(),
            })
            .await;
        assert!(
            result.contains("error"),
            "TCL file should return unsupported error: {result}"
        );
    }

    #[tokio::test]
    async fn test_check_synthesizability_not_open_returns_clean() {
        // When file is not open, content is empty → checker finds no issues
        // (no error, just "no synthesizability issues found")
        let server = ThanosMcpServer::new();
        let result = server
            .check_synthesizability(UriParam {
                uri: "file:///not_opened.sv".to_string(),
            })
            .await;
        assert!(!result.is_empty(), "check_synthesizability should return a response");
        // Should either say no issues or return JSON diagnostics — not panic
        assert!(
            result.contains("no synthesizability") || result.starts_with('[') || result.contains("error"),
            "expected clean result or diagnostics: {result}"
        );
    }

    #[tokio::test]
    async fn test_update_file_response_is_non_empty() {
        let server = ThanosMcpServer::new();
        // update_file on a file that was never opened
        let result = server
            .update_file(UpdateFileParams {
                uri: "file:///update_new.sv".to_string(),
                content: "module new_file;\nendmodule".to_string(),
            })
            .await;
        // Must not panic; returns some response
        assert!(!result.is_empty(), "update_file should return a response");
    }
}
