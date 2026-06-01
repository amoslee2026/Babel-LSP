//! 文档状态管理
//!
//! 维护文档版本、语言类型、内容缓冲区等状态

use ropey::Rope;
use std::time::Instant;
use url::Url;

/// 语言类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    SystemVerilog,
    Verilog,
    VHDL,
    TCL,
}

/// 文件三分类
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FileClass {
    /// RTL 设计文件
    RTL,
    /// 测试平台文件
    Testbench,
    /// 网表文件
    Netlist,
}

/// 文档状态
#[derive(Debug, Clone)]
pub struct DocumentState {
    /// 文件 URI
    pub uri: Url,
    /// 版本号（单调递增）
    pub version: u32,
    /// 语言类型
    pub language: Language,
    /// 文件分类
    pub file_class: FileClass,
    /// 内容缓冲区
    pub content: Rope,
    /// 符号列表（可选）
    pub symbols: Option<Vec<crate::symbol::Symbol>>,
    /// 最后修改时间
    pub last_modified: Instant,
    /// 内容哈希
    pub content_hash: [u8; 32],
}

impl DocumentState {
    /// 创建新文档状态
    pub fn new(uri: Url, language: Language, content: String) -> Self {
        Self {
            uri,
            version: 0,
            language,
            file_class: FileClass::RTL,
            content: Rope::from_str(&content),
            symbols: None,
            last_modified: Instant::now(),
            content_hash: Self::compute_hash(&content),
        }
    }

    /// 更新文档内容
    pub fn update(&mut self, content: String, version: u32) {
        self.content = Rope::from_str(&content);
        self.version = version;
        self.last_modified = Instant::now();
        self.content_hash = Self::compute_hash(&content);
        self.symbols = None; // 清除符号缓存
    }

    /// 计算内容哈希
    fn compute_hash(content: &str) -> [u8; 32] {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        let hash = hasher.finish();
        // 简化的哈希（实际应使用 SHA256）
        let mut result = [0u8; 32];
        result[..8].copy_from_slice(&hash.to_le_bytes());
        result
    }

    /// 获取内容字符串
    pub fn content_string(&self) -> String {
        self.content.to_string()
    }

    /// 获取行数
    pub fn line_count(&self) -> usize {
        self.content.len_lines()
    }
}

impl Language {
    /// 从文件路径推断语言类型
    pub fn from_path(path: &str) -> Option<Self> {
        let ext = path.rsplit('.').next()?;
        match ext {
            "sv" | "svh" => Some(Language::SystemVerilog),
            "v" | "vh" => Some(Language::Verilog),
            "vhd" | "vhdl" => Some(Language::VHDL),
            "tcl" => Some(Language::TCL),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_from_path() {
        assert_eq!(
            Language::from_path("test.sv"),
            Some(Language::SystemVerilog)
        );
        assert_eq!(Language::from_path("test.v"), Some(Language::Verilog));
        assert_eq!(Language::from_path("test.vhd"), Some(Language::VHDL));
        assert_eq!(Language::from_path("test.tcl"), Some(Language::TCL));
        assert_eq!(Language::from_path("test.txt"), None);
    }

    #[test]
    fn test_document_state_creation() {
        let uri = Url::parse("file:///test.sv").unwrap();
        let doc = DocumentState::new(
            uri,
            Language::SystemVerilog,
            "module test(); endmodule".to_string(),
        );
        assert_eq!(doc.version, 0);
        assert_eq!(doc.language, Language::SystemVerilog);
    }
}
