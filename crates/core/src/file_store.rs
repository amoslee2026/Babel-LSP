//! 文件存储
//!
//! 维护文件内容缓冲区、版本管理

use crate::document::DocumentState;
use dashmap::DashMap;
use std::sync::Arc;
use url::Url;

/// 文件存储
pub struct FileStore {
    /// 文档映射表
    documents: DashMap<Url, DocumentState>,
}

impl FileStore {
    /// 创建空文件存储
    pub fn new() -> Self {
        Self {
            documents: DashMap::new(),
        }
    }

    /// 添加文件
    pub fn insert(&self, uri: Url, doc: DocumentState) {
        self.documents.insert(uri, doc);
    }

    /// 获取文件
    pub fn get(&self, uri: &Url) -> Option<Arc<DocumentState>> {
        self.documents.get(uri).map(|r| Arc::new(r.value().clone()))
    }

    /// 更新文件内容
    pub fn update(&self, uri: &Url, content: String, version: u32) -> bool {
        if let Some(mut doc) = self.documents.get_mut(uri) {
            doc.update(content, version);
            true
        } else {
            false
        }
    }

    /// 移除文件
    pub fn remove(&self, uri: &Url) -> Option<DocumentState> {
        self.documents.remove(uri).map(|(_, v)| v)
    }

    /// 获取所有文件 URI
    pub fn uris(&self) -> Vec<Url> {
        self.documents.iter().map(|r| r.key().clone()).collect()
    }

    /// 获取文件数量
    pub fn count(&self) -> usize {
        self.documents.len()
    }
}

impl Default for FileStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::Language;

    #[test]
    fn test_file_store_basic() {
        let store = FileStore::new();
        let uri = Url::parse("file:///test.sv").unwrap();
        let doc = DocumentState::new(
            uri.clone(),
            Language::SystemVerilog,
            "module test(); endmodule".to_string(),
        );

        store.insert(uri.clone(), doc);
        assert_eq!(store.count(), 1);

        let retrieved = store.get(&uri);
        assert!(retrieved.is_some());
    }

    #[test]
    fn test_file_store_update() {
        let store = FileStore::new();
        let uri = Url::parse("file:///test.sv").unwrap();
        let doc = DocumentState::new(
            uri.clone(),
            Language::SystemVerilog,
            "module test(); endmodule".to_string(),
        );

        store.insert(uri.clone(), doc);
        let updated = store.update(&uri, "module test2(); endmodule".to_string(), 1);
        assert!(updated);

        let retrieved = store.get(&uri).unwrap();
        assert_eq!(retrieved.version, 1);
    }
}
