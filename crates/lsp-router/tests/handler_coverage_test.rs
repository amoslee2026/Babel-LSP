//! lsp-router handler coverage 补充测试

use smol_str::SmolStr;
use std::sync::Arc;
use babel_lsp_core::file_store::FileStore;
use babel_lsp_core::symbol::{Location, Position, Symbol, SymbolKind};
use babel_lsp_lsp::handlers::{
    completion::handle_completion,
    definition::handle_all_definitions,
    formatting::{content_to_edits, handle_formatting},
    references::{find_references_in_content, handle_references},
    sync::{handle_did_change, handle_did_open, handle_did_save, language_from_uri, parse_uri},
};
use url::Url;

fn make_store() -> Arc<FileStore> {
    Arc::new(FileStore::new())
}

fn make_symbol(name: &str, line: u32) -> Symbol {
    Symbol::new(
        SmolStr::from(name),
        SymbolKind::Module,
        Location {
            uri: "file:///test.sv".to_string(),
            start: Position::new(line, 0),
            end: Position::new(line + 1, 0),
        },
    )
}

// ============================================================
// sync.rs coverage
// ============================================================

#[test]
fn test_parse_uri_valid() {
    let result = parse_uri("file:///test.sv");
    assert!(result.is_some());
}

#[test]
fn test_parse_uri_invalid() {
    let result = parse_uri(":::not_a_uri");
    assert!(result.is_none());
}

#[test]
fn test_language_from_uri_svh() {
    use babel_lsp_core::document::Language;
    let lang = language_from_uri(&Url::parse("file:///a.svh").unwrap());
    assert_eq!(lang, Language::SystemVerilog);
}

#[test]
fn test_language_from_uri_vh() {
    use babel_lsp_core::document::Language;
    let lang = language_from_uri(&Url::parse("file:///a.vh").unwrap());
    assert_eq!(lang, Language::Verilog);
}

#[test]
fn test_language_from_uri_vhdl() {
    use babel_lsp_core::document::Language;
    let lang = language_from_uri(&Url::parse("file:///a.vhdl").unwrap());
    assert_eq!(lang, Language::VHDL);
}

#[test]
fn test_language_from_uri_xdc() {
    use babel_lsp_core::document::Language;
    let lang = language_from_uri(&Url::parse("file:///constraints.xdc").unwrap());
    assert_eq!(lang, Language::TCL);
}

#[test]
fn test_language_from_uri_unknown_defaults_to_sv() {
    use babel_lsp_core::document::Language;
    let lang = language_from_uri(&Url::parse("file:///a.unknown").unwrap());
    assert_eq!(lang, Language::SystemVerilog);
}

#[test]
fn test_handle_did_change_untracked_file() {
    let store = make_store();
    let uri = Url::parse("file:///untracked.sv").unwrap();
    handle_did_change(&store, &uri, "module new_mod;".to_string(), 1);
    assert!(store.get(&uri).is_some());
}

#[test]
fn test_handle_did_save_untracked_no_panic() {
    let store = make_store();
    let uri = Url::parse("file:///untracked_save.sv").unwrap();
    handle_did_save(&store, &uri);
}

#[test]
fn test_handle_did_save_tracked() {
    let store = make_store();
    let uri = Url::parse("file:///tracked.sv").unwrap();
    handle_did_open(&store, &uri, "module tracked;".to_string(), 1);
    handle_did_save(&store, &uri);
    assert!(store.get(&uri).is_some());
}

// ============================================================
// references.rs coverage
// ============================================================

#[test]
fn test_find_references_invalid_uri_returns_empty() {
    let refs = find_references_in_content("assign foo = 1;", ":::not-a-uri", "foo");
    assert!(refs.is_empty());
}

#[test]
fn test_handle_references_matching() {
    let symbols = vec![
        make_symbol("my_module", 0),
        make_symbol("other_mod", 5),
        make_symbol("my_module", 10),
    ];
    let refs = handle_references(&symbols, "my_module");
    assert_eq!(refs.len(), 2);
}

#[test]
fn test_handle_references_no_match() {
    let symbols = vec![make_symbol("adder", 0)];
    let refs = handle_references(&symbols, "counter");
    assert!(refs.is_empty());
}

#[test]
fn test_handle_references_invalid_symbol_uri_skipped() {
    let bad_sym = Symbol::new(
        SmolStr::from("bad"),
        SymbolKind::Module,
        Location {
            uri: ":::invalid".to_string(),
            start: Position::new(0, 0),
            end: Position::new(1, 0),
        },
    );
    let refs = handle_references(&[bad_sym], "bad");
    let _ = refs; // no panic
}

// ============================================================
// formatting.rs coverage
// ============================================================

#[test]
fn test_handle_formatting_no_verible_no_panic() {
    let edits = handle_formatting("module foo; endmodule");
    let _ = edits;
}

#[test]
fn test_content_to_edits_multiline() {
    let original = "a\nb\nc";
    let formatted = "a\n  b\nc";
    let edits = content_to_edits(original, formatted);
    assert_eq!(edits.len(), 1);
    assert_eq!(edits[0].range.start.line, 0);
}

#[test]
fn test_content_to_edits_empty_unchanged() {
    let edits = content_to_edits("", "");
    assert!(edits.is_empty());
}

// ============================================================
// completion.rs coverage - edge cases
// ============================================================

#[test]
fn test_completion_empty_prefix_matches_all() {
    let symbols = vec![make_symbol("adder", 0), make_symbol("counter", 5)];
    let completions = handle_completion(&symbols, "", false);
    assert!(!completions.is_empty());
}

#[test]
fn test_completion_no_match_prefix() {
    let symbols = vec![make_symbol("adder", 0)];
    let completions = handle_completion(&symbols, "zzz_no_match", false);
    assert!(completions.is_empty());
}

#[test]
fn test_completion_with_keywords_enabled() {
    let symbols: Vec<Symbol> = vec![];
    let completions = handle_completion(&symbols, "mod", true);
    let _ = completions;
}

// ============================================================
// definition.rs coverage
// ============================================================

#[test]
fn test_definition_by_name_found() {
    let symbols = vec![make_symbol("adder", 0), make_symbol("counter", 5)];
    let defs = handle_all_definitions(&symbols, "adder");
    assert!(!defs.is_empty());
}

#[test]
fn test_definition_by_name_not_found() {
    let symbols = vec![make_symbol("adder", 0)];
    let defs = handle_all_definitions(&symbols, "nonexistent");
    assert!(defs.is_empty());
}

#[test]
fn test_definition_empty_symbols() {
    let defs = handle_all_definitions(&[], "anything");
    assert!(defs.is_empty());
}
