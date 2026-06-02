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
use crate::cross_lang::CrossLangIndex;
use crate::file_store::FileStore;
use crate::project_index::ProjectIndex;
use crate::project_memory::{MemoryConfig, ProjectMemory};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, info};

/// 核心引擎，管理所有核心子系统
pub struct CoreEngine {
    pub config: Arc<ProjectConfig>,
    pub file_store: Arc<FileStore>,
    pub project_index: Arc<Mutex<ProjectIndex>>,
    pub cross_lang_index: Arc<Mutex<CrossLangIndex>>,
    pub project_memory: Option<Arc<ProjectMemory>>,
    pub start_time: std::time::Instant,
}

impl CoreEngine {
    /// 创建并初始化 CoreEngine
    pub async fn initialize(config: ProjectConfig) -> anyhow::Result<Self> {
        let root = config.project_root.clone();
        info!("CoreEngine initializing with project root: {:?}", root);

        let engine = Self {
            config: Arc::new(config),
            file_store: Arc::new(FileStore::new()),
            project_index: Arc::new(Mutex::new(ProjectIndex::new(root)?)),
            cross_lang_index: Arc::new(Mutex::new(CrossLangIndex::new())),
            project_memory: None,
            start_time: std::time::Instant::now(),
        };

        debug!(
            "CoreEngine initialized in {}ms",
            engine.start_time.elapsed().as_millis()
        );
        Ok(engine)
    }

    /// 启用文件监听（project_memory）
    pub fn enable_project_memory(&mut self, mem_cfg: MemoryConfig) -> anyhow::Result<()> {
        // 注意: ProjectMemory 尚未公开初始化接口,保留此方法用于将来的文件监听功能
        let _ = mem_cfg;
        Ok(())
    }

    /// 获取项目索引引用（用于跨模块共享）
    pub fn project_index_arc(&self) -> Arc<Mutex<ProjectIndex>> {
        self.project_index.clone()
    }

    /// 获取跨语言索引引用
    pub fn cross_lang_index_arc(&self) -> Arc<Mutex<CrossLangIndex>> {
        self.cross_lang_index.clone()
    }

    /// 引擎运行时长（毫秒）
    pub fn uptime_ms(&self) -> u64 {
        self.start_time.elapsed().as_millis() as u64
    }
}
