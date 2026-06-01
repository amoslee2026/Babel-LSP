//! L6 Oracle 验证测试
//!
//! 1. LSP 3.17 spec 合规 Oracle
//! 2. 变形测试 (Metamorphic Relations MR-1..MR-4)

use smol_str::SmolStr;
use thanosLSP_core::symbol::{Location, Position, Symbol, SymbolKind};
use thanosLSP_lsp::handlers::{
    completion::handle_completion, definition::handle_all_definitions, hover::handle_hover,
    lifecycle::build_server_capabilities, symbols::handle_document_symbols,
};

fn make_symbol(name: &str, start_line: u32, end_line: u32) -> Symbol {
    Symbol::new(
        SmolStr::from(name),
        SymbolKind::Module,
        Location {
            uri: "file:///oracle.sv".to_string(),
            start: Position::new(start_line, 0),
            end: Position::new(end_line, 0),
        },
    )
}

// ============================================================
// Oracle 1: LSP 3.17 spec 合规
// ============================================================

/// LSP 3.17: 服务器必须声明 completion_provider 才能响应 textDocument/completion
#[test]
fn oracle_lsp_completion_capability_declared() {
    let caps = build_server_capabilities();
    assert!(
        caps.completion_provider.is_some(),
        "LSP 3.17: completion_provider must be declared"
    );
}

/// LSP 3.17: definition_provider 必须声明
#[test]
fn oracle_lsp_definition_capability_declared() {
    let caps = build_server_capabilities();
    assert!(
        caps.definition_provider.is_some(),
        "LSP 3.17: definition_provider must be declared"
    );
}

/// LSP 3.17: text_document_sync 必须声明，且支持 open/close
#[test]
fn oracle_lsp_text_document_sync_open_close() {
    use tower_lsp::lsp_types::TextDocumentSyncCapability;
    let caps = build_server_capabilities();
    match caps.text_document_sync {
        Some(TextDocumentSyncCapability::Options(opts)) => {
            assert_eq!(
                opts.open_close,
                Some(true),
                "LSP 3.17: open_close must be true"
            );
        },
        _ => panic!("text_document_sync must use Options variant"),
    }
}

/// LSP 3.17: hover_provider 必须声明
#[test]
fn oracle_lsp_hover_capability_declared() {
    let caps = build_server_capabilities();
    assert!(
        caps.hover_provider.is_some(),
        "LSP 3.17: hover_provider must be declared"
    );
}

// ============================================================
// Oracle 2: 变形测试 Metamorphic Relations
// ============================================================

/// MR-1: 添加空行不改变 module 数量
/// 对同一 SV 内容，在模块间加入空行后，document symbols 数量不变
#[test]
fn mr1_blank_lines_do_not_change_symbol_count() {
    let syms = vec![make_symbol("mod_a", 0, 5), make_symbol("mod_b", 6, 10)];
    let doc_syms_before = handle_document_symbols(&syms);

    // Simulate adding blank lines by shifting positions
    let syms_with_blanks = vec![make_symbol("mod_a", 0, 5), make_symbol("mod_b", 8, 12)]; // gap grew
    let doc_syms_after = handle_document_symbols(&syms_with_blanks);

    assert_eq!(
        doc_syms_before.len(),
        doc_syms_after.len(),
        "MR-1: blank lines should not change symbol count"
    );
}

/// MR-2: 符号重命名后，定义查找返回新名称
#[test]
fn mr2_rename_updates_definition_lookup() {
    let original = vec![make_symbol("old_name", 0, 5)];
    let renamed = vec![make_symbol("new_name", 0, 5)];

    assert!(handle_hover(&original, "old_name").is_some());
    assert!(handle_hover(&original, "new_name").is_none());
    assert!(handle_hover(&renamed, "new_name").is_some());
    assert!(handle_hover(&renamed, "old_name").is_none());
}

/// MR-3: 定义查找满足单调性 — 多个同名符号返回 >= 单个符号查找
#[test]
fn mr3_all_definitions_monotonic() {
    let syms = vec![
        make_symbol("clk", 0, 0),
        make_symbol("clk", 5, 5),
        make_symbol("data", 10, 10),
    ];

    let all_clk = handle_all_definitions(&syms, "clk");
    let all_data = handle_all_definitions(&syms, "data");

    assert_eq!(all_clk.len(), 2, "MR-3: should find both clk definitions");
    assert_eq!(all_data.len(), 1, "MR-3: should find one data definition");
    assert!(
        all_clk.len() >= all_data.len(),
        "MR-3: more symbols → more or equal results"
    );
}

/// MR-4: 空符号表的 hover 始终返回 None（null oracle）
#[test]
fn mr4_empty_symbol_table_hover_always_none() {
    let empty: Vec<Symbol> = vec![];
    for name in &["clk", "rst_n", "data_valid", "any_signal"] {
        assert!(
            handle_hover(&empty, name).is_none(),
            "MR-4: empty symbol table must return None for {}",
            name
        );
    }
}

// ============================================================
// Oracle 3: completion 一致性
// ============================================================

/// completion 对空符号表始终不 panic，返回空或关键字结果
#[test]
fn oracle_completion_empty_symbols_no_panic() {
    let empty: Vec<Symbol> = vec![];
    let result = handle_completion(&empty, "", true);
    // No panic is the requirement; keywords should be returned
    let _ = result;
}

/// completion 的触发字符 '.' 应出现在 trigger_characters 中
#[test]
fn oracle_completion_dot_trigger_registered() {
    let caps = build_server_capabilities();
    let comp = caps.completion_provider.unwrap();
    let triggers = comp.trigger_characters.unwrap_or_default();
    assert!(
        triggers.contains(&".".to_string()),
        "LSP 3.17: '.' must be a completion trigger character"
    );
}
