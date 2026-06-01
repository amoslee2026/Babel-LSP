//! 项目记忆
//!
//! 文件监听 + 定时扫描 + 增量更新

use crate::project_index::ProjectIndex;
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;

/// 项目记忆配置
pub struct MemoryConfig {
    /// 扫描间隔（秒）
    pub scan_interval: Duration,
    /// 监听目录
    pub watch_dirs: Vec<PathBuf>,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            scan_interval: Duration::from_secs(300),
            watch_dirs: vec![],
        }
    }
}

/// 项目记忆管理器
pub struct ProjectMemory {
    /// 项目索引
    index: Arc<ProjectIndex>,
    /// 文件监听器
    watcher: RecommendedWatcher,
    /// 事件接收通道
    #[allow(dead_code)]
    event_rx: mpsc::Receiver<Event>,
}

impl ProjectMemory {
    /// 创建项目记忆管理器
    pub fn new(index: Arc<ProjectIndex>, _config: MemoryConfig) -> anyhow::Result<Self> {
        let (event_tx, event_rx) = mpsc::channel(100);

        // 创建文件监听器
        let watcher = RecommendedWatcher::new(
            move |res: Result<Event, notify::Error>| {
                if let Ok(event) = res {
                    let _ = event_tx.blocking_send(event);
                }
            },
            notify::Config::default(),
        )?;

        Ok(Self {
            index,
            watcher,
            event_rx,
        })
    }

    /// 启动监听
    pub fn start_watch(&mut self, path: &Path) -> anyhow::Result<()> {
        self.watcher.watch(path, RecursiveMode::Recursive)?;
        Ok(())
    }

    /// 获取项目索引
    pub fn index(&self) -> &Arc<ProjectIndex> {
        &self.index
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_memory_config_default() {
        let config = MemoryConfig::default();
        assert_eq!(config.scan_interval, Duration::from_secs(300));
        assert!(config.watch_dirs.is_empty());
    }

    #[test]
    fn test_memory_config_custom() {
        let config = MemoryConfig {
            scan_interval: Duration::from_secs(60),
            watch_dirs: vec![PathBuf::from("/tmp/test")],
        };
        assert_eq!(config.scan_interval, Duration::from_secs(60));
        assert_eq!(config.watch_dirs.len(), 1);
        assert_eq!(config.watch_dirs[0], PathBuf::from("/tmp/test"));
    }

    #[test]
    fn test_memory_config_with_multiple_dirs() {
        let config = MemoryConfig {
            scan_interval: Duration::from_secs(120),
            watch_dirs: vec![
                PathBuf::from("/proj/rtl"),
                PathBuf::from("/proj/tb"),
                PathBuf::from("/proj/scripts"),
            ],
        };
        assert_eq!(config.watch_dirs.len(), 3);
    }

    fn create_test_index() -> Arc<ProjectIndex> {
        let temp_dir = tempdir().unwrap();
        Arc::new(ProjectIndex::new(temp_dir.path().to_path_buf()).unwrap())
    }

    #[test]
    fn test_project_memory_creation() {
        let index = create_test_index();
        let config = MemoryConfig::default();
        let result = ProjectMemory::new(index.clone(), config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_project_memory_index_access() {
        let index = create_test_index();
        let config = MemoryConfig::default();
        let memory = ProjectMemory::new(index.clone(), config).unwrap();
        let retrieved_index = memory.index();
        assert!(Arc::ptr_eq(&index, retrieved_index));
    }

    #[test]
    fn test_memory_config_scan_intervals() {
        // Test various scan intervals
        for secs in [60u64, 120, 300, 600, 1800] {
            let config = MemoryConfig {
                scan_interval: Duration::from_secs(secs),
                watch_dirs: vec![],
            };
            assert_eq!(config.scan_interval, Duration::from_secs(secs));
        }
    }

    #[test]
    fn test_memory_config_zero_interval() {
        // Edge case: zero interval
        let config = MemoryConfig {
            scan_interval: Duration::from_secs(0),
            watch_dirs: vec![],
        };
        assert_eq!(config.scan_interval, Duration::from_secs(0));
    }

    #[test]
    fn test_project_memory_with_empty_watch_dirs() {
        let index = create_test_index();
        let config = MemoryConfig {
            scan_interval: Duration::from_secs(60),
            watch_dirs: vec![],
        };
        let result = ProjectMemory::new(index, config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_project_memory_start_watch() {
        let index = create_test_index();
        let config = MemoryConfig::default();
        let mut memory = ProjectMemory::new(index, config).unwrap();
        let temp_dir = tempdir().unwrap();
        let result = memory.start_watch(temp_dir.path());
        assert!(result.is_ok());
    }

    #[test]
    fn test_project_memory_with_custom_config() {
        let index = create_test_index();
        let temp_dir = tempdir().unwrap();
        let config = MemoryConfig {
            scan_interval: Duration::from_secs(600),
            watch_dirs: vec![temp_dir.path().to_path_buf()],
        };
        let result = ProjectMemory::new(index, config);
        assert!(result.is_ok());
    }
}
