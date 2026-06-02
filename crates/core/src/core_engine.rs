//! @requirement REQ-M01-F01, REQ-M01-F02 @auto:it.tdd
//! 核心引擎聚合结构
//!
//! CoreEngine 将所有核心子系统聚合为统一入口，供上层协议层（M02 LSP Router、
//! M03 MCP Server）共享使用。
//!
//! 初始化顺序（见 impl_spec/M01_core/MAS.md §3.1）：
//!   1. Config (M01a)           ─┐
//!   2. Symbol (M01c)            ├─ Phase 1 并行
//!   3. Diagnostic (M01d)        │
//!   4. FileClassifier (M01h)   ─┘
//!   5. Logging (M01j)          ─┐
//!   6. Document (M01b)          ├─ Phase 2 串行
//!   7. CrossLang (M01i)        ─┘
//!   8. FileStore (M01e)         ── Phase 3
//!   9. ProjectIndex (M01f)      ── Phase 4
//!  10. ProjectMemory (M01g)     ── Phase 5

use crate::config::ProjectConfig;
use crate::diagnostic::Diagnostic;
use crate::document::DocumentState;
use crate::file_store::FileStore;
use crate::project_index::ProjectIndex;
use crate::project_memory::{MemoryConfig, ProjectMemory};
use crate::symbol::Symbol;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, info};

/// 核心引擎，管理所有核心子系统
pub struct CoreEngine {
    pub config: Arc<ProjectConfig>,
    pub doc_index: Arc<Mutex<HashMap<String, DocumentState>>>,
    pub file_store: Arc<FileStore>,
    pub project_index: Arc<Mutex<ProjectIndex>>,
    pub project_memory: Option<Arc<ProjectMemory>>,
    pub start_time: std::time::Instant,
}

impl CoreEngine {
    /// 创建并初始化 CoreEngine
    pub async fn initialize(config: ProjectConfig) -> anyhow::Result<Self> {
        info!("CoreEngine initializing with project root: {:?}", config.project_root);

        let engine = Self {
            config: Arc::new(config),
            doc_index: Arc::new(Mutex::new(HashMap::new())),
            file_store: Arc::new(FileStore::new()),
            project_index: Arc::new(Mutex::new(ProjectIndex::default())),
            project_memory: None,
            start_time: std::time::Instant::now(),
        };

        debug!("CoreEngine initialized in {}ms", engine.start_time.elapsed().as_millis());
        Ok(engine)
    }

    /// 启用文件监听（project_memory）
    pub fn enable_project_memory(&mut self, mem_cfg: MemoryConfig) {
        self.project_memory = Some(Arc::new(ProjectMemory::new(mem_cfg)));
    }

    /// 获取文档状态
    pub async fn get_document(&self, uri: &str) -> Option<DocumentState> {
        self.doc_index.lock().await.get(uri).cloned()
    }

    /// 获取文件的符号列表（委托给 project_index）
    pub async fn get_symbols(&self, uri: &str) -> Vec<Symbol> {
        self.project_index.lock().await.get_symbols(uri).unwrap_or_default()
    }

    /// 搜索符号
    pub async fn search_symbols(&self, query: &str) -> Vec<Symbol> {
        self.project_index.lock().await.search(query).unwrap_or_default()
    }

    /// 获取文件的诊断列表
    pub async fn get_diagnostics(&self, uri: &str) -> Vec<Diagnostic> {
        self.project_index.lock().await.get_diagnostics(uri).unwrap_or_default()
    }

    /// 更新文件内容
    pub async fn update_file(&self, uri: &str, content: &str) -> anyhow::Result<()> {
        self.file_store.store(uri, content).await?;
        self.project_index.lock().await.index_file(uri, content).await?;
        Ok(())
    }

    /// 获取跨语言引用
    pub async fn get_cross_lang_refs(&self, symbol_name: &str) -> Vec<crate::cross_lang::CrossLangRef> {
        crate::cross_lang::lookup_cross_lang_refs(symbol_name)
    }

    /// 持久化索引到磁盘
    pub async fn flush(&self) -> anyhow::Result<()> {
        self.project_index.lock().await.flush().await?;
        Ok(())
    }

    /// 设置日志级别
    pub fn set_log_level(&self, level: &str) {
        crate::logging::set_log_level(level);
    }

    /// 引擎运行时长（毫秒）
    pub fn uptime_ms(&self) -> u64 {
        self.start_time.elapsed().as_millis() as u64
    }
}
