//! 文档同步处理器
//!
//! 处理 didOpen/didChange/didClose 事件，更新 FileStore 并触发分析

use std::sync::Arc;
use thanosLSP_core::{
    document::{DocumentState, Language},
    file_store::FileStore,
};
use tracing::{debug, warn};
use url::Url;

/// 从 LSP URI 字符串创建文件 URI
pub fn parse_uri(uri: &str) -> Option<Url> {
    Url::parse(uri).ok()
}

/// 从文件扩展名推断语言
pub fn language_from_uri(uri: &Url) -> Language {
    let path = uri.path();
    if path.ends_with(".sv") || path.ends_with(".svh") {
        Language::SystemVerilog
    } else if path.ends_with(".v") || path.ends_with(".vh") {
        Language::Verilog
    } else if path.ends_with(".vhd") || path.ends_with(".vhdl") {
        Language::VHDL
    } else if path.ends_with(".tcl") || path.ends_with(".xdc") {
        Language::TCL
    } else {
        Language::SystemVerilog // default
    }
}

/// 处理 didOpen 事件
pub fn handle_did_open(file_store: &Arc<FileStore>, uri: &Url, content: String, version: i32) {
    let lang = language_from_uri(uri);
    let mut doc = DocumentState::new(uri.clone(), lang, content);
    doc.version = version as u32;
    file_store.insert(uri.clone(), doc);
    debug!("opened: {}", uri);
}

/// 处理 didChange 事件
pub fn handle_did_change(file_store: &Arc<FileStore>, uri: &Url, content: String, version: i32) {
    if !file_store.update(uri, content.clone(), version as u32) {
        // File wasn't tracked yet, add it
        let lang = language_from_uri(uri);
        let mut doc = DocumentState::new(uri.clone(), lang, content);
        doc.version = version as u32;
        file_store.insert(uri.clone(), doc);
    }
    debug!("changed: {}", uri);
}

/// 处理 didClose 事件
pub fn handle_did_close(file_store: &Arc<FileStore>, uri: &Url) {
    file_store.remove(uri);
    debug!("closed: {}", uri);
}

/// 处理 didSave 事件
pub fn handle_did_save(file_store: &Arc<FileStore>, uri: &Url) {
    if file_store.get(uri).is_none() {
        warn!("didSave for untracked file: {}", uri);
    }
    debug!("saved: {}", uri);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    fn make_store() -> Arc<FileStore> {
        Arc::new(FileStore::new())
    }

    fn test_uri() -> Url {
        Url::parse("file:///test.sv").unwrap()
    }

    #[test]
    fn test_handle_did_open() {
        let store = make_store();
        let uri = test_uri();
        handle_did_open(&store, &uri, "module foo;".to_string(), 1);
        assert!(store.get(&uri).is_some());
        let doc = store.get(&uri).unwrap();
        assert_eq!(doc.version, 1);
    }

    #[test]
    fn test_handle_did_change() {
        let store = make_store();
        let uri = test_uri();
        handle_did_open(&store, &uri, "module foo;".to_string(), 1);
        handle_did_change(&store, &uri, "module bar;".to_string(), 2);
        let doc = store.get(&uri).unwrap();
        assert_eq!(doc.version, 2);
        assert!(doc.content.to_string().contains("bar"));
    }

    #[test]
    fn test_handle_did_close() {
        let store = make_store();
        let uri = test_uri();
        handle_did_open(&store, &uri, "module foo;".to_string(), 1);
        handle_did_close(&store, &uri);
        assert!(store.get(&uri).is_none());
    }

    #[test]
    fn test_language_from_uri() {
        assert_eq!(
            language_from_uri(&Url::parse("file:///a.sv").unwrap()),
            Language::SystemVerilog
        );
        assert_eq!(
            language_from_uri(&Url::parse("file:///a.v").unwrap()),
            Language::Verilog
        );
        assert_eq!(
            language_from_uri(&Url::parse("file:///a.vhd").unwrap()),
            Language::VHDL
        );
        assert_eq!(
            language_from_uri(&Url::parse("file:///a.tcl").unwrap()),
            Language::TCL
        );
    }
}
