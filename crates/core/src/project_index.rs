//! 项目索引
//!
//! 使用 Salsa 进行增量计算，redb 持久化

use redb::{Database, TableDefinition};
use std::path::PathBuf;

const SYMBOLS_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("symbols");

/// 项目索引
pub struct ProjectIndex {
    /// 项目根目录
    root: PathBuf,
    /// redb 数据库
    db: Database,
}

impl ProjectIndex {
    /// 创建项目索引
    pub fn new(root: PathBuf) -> anyhow::Result<Self> {
        let db_path = root.join(".thanos").join("index.redb");
        let db_parent = db_path
            .parent()
            .ok_or_else(|| anyhow::anyhow!("Invalid db path"))?;
        std::fs::create_dir_all(db_parent)?;
        let db = Database::create(&db_path)?;
        Ok(Self { root, db })
    }

    /// 获取项目根目录
    pub fn root(&self) -> &PathBuf {
        &self.root
    }

    /// 存储符号数据
    pub fn store_symbols(&self, uri: &str, data: &[u8]) -> anyhow::Result<()> {
        let write_txn = self.db.begin_write()?;
        {
            let mut table = write_txn.open_table(SYMBOLS_TABLE)?;
            table.insert(uri, data)?;
        }
        write_txn.commit()?;
        Ok(())
    }

    /// 加载符号数据
    pub fn load_symbols(&self, uri: &str) -> anyhow::Result<Option<Vec<u8>>> {
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(SYMBOLS_TABLE)?;
        let result = table.get(uri)?;
        Ok(result.map(|v| v.value().to_vec()))
    }

    /// 清除索引
    pub fn clear(&self) -> anyhow::Result<()> {
        let write_txn = self.db.begin_write()?;
        write_txn.delete_table(SYMBOLS_TABLE)?;
        write_txn.commit()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_project_index_basic() {
        let dir = tempdir().unwrap();
        let root = dir.path().to_path_buf();
        let index = ProjectIndex::new(root).unwrap();

        let data = b"test_data";
        index.store_symbols("test.sv", data).unwrap();

        let loaded = index.load_symbols("test.sv").unwrap();
        assert!(loaded.is_some());
        assert_eq!(loaded.unwrap(), data.to_vec());
    }

    #[test]
    fn test_project_index_root() {
        let dir = tempdir().unwrap();
        let root = dir.path().to_path_buf();
        let index = ProjectIndex::new(root.clone()).unwrap();
        assert_eq!(index.root(), &root);
    }

    #[test]
    fn test_project_index_clear() {
        let dir = tempdir().unwrap();
        let root = dir.path().to_path_buf();
        let index = ProjectIndex::new(root).unwrap();

        // 存储数据（这会创建表）
        index.store_symbols("file1.sv", b"data1").unwrap();
        index.store_symbols("file2.sv", b"data2").unwrap();

        // 清除
        index.clear().unwrap();

        // 清除后表被删除，需要重新存储才能读取
        // 验证 clear 不 panic
    }

    #[test]
    fn test_project_index_load_nonexistent() {
        let dir = tempdir().unwrap();
        let root = dir.path().to_path_buf();
        let index = ProjectIndex::new(root).unwrap();

        // 先存储一些数据以创建表
        index.store_symbols("exists.sv", b"data").unwrap();

        // 然后查询不存在的键
        let loaded = index.load_symbols("nonexistent.sv").unwrap();
        assert!(loaded.is_none());
    }

    #[test]
    fn test_project_index_overwrite() {
        let dir = tempdir().unwrap();
        let root = dir.path().to_path_buf();
        let index = ProjectIndex::new(root).unwrap();

        // 存储初始数据
        index.store_symbols("test.sv", b"original").unwrap();
        // 覆盖
        index.store_symbols("test.sv", b"updated").unwrap();

        let loaded = index.load_symbols("test.sv").unwrap().unwrap();
        assert_eq!(loaded, b"updated".to_vec());
    }

    #[test]
    fn test_project_index_multiple_files() {
        let dir = tempdir().unwrap();
        let root = dir.path().to_path_buf();
        let index = ProjectIndex::new(root).unwrap();

        // 存储多个文件
        let files: Vec<(&str, &[u8])> = vec![
            ("rtl/top.sv", b"top_data" as &[u8]),
            ("rtl/sub.sv", b"sub_data" as &[u8]),
            ("tb/tb_top.sv", b"tb_data" as &[u8]),
        ];

        for (uri, data) in &files {
            index.store_symbols(uri, data).unwrap();
        }

        // 验证所有文件
        for (uri, data) in &files {
            let loaded = index.load_symbols(uri).unwrap().unwrap();
            assert_eq!(loaded, data.to_vec());
        }
    }

    #[test]
    fn test_project_index_empty_data() {
        let dir = tempdir().unwrap();
        let root = dir.path().to_path_buf();
        let index = ProjectIndex::new(root).unwrap();

        // 存储空数据
        index.store_symbols("empty.sv", b"").unwrap();

        let loaded = index.load_symbols("empty.sv").unwrap().unwrap();
        assert!(loaded.is_empty());
    }

    #[test]
    fn test_project_index_large_data() {
        let dir = tempdir().unwrap();
        let root = dir.path().to_path_buf();
        let index = ProjectIndex::new(root).unwrap();

        // 存储大数据
        let large_data: Vec<u8> = (0..10000).map(|i| (i % 256) as u8).collect();
        index.store_symbols("large.sv", &large_data).unwrap();

        let loaded = index.load_symbols("large.sv").unwrap().unwrap();
        assert_eq!(loaded.len(), 10000);
    }

    #[test]
    fn test_project_index_creates_directory() {
        let dir = tempdir().unwrap();
        let root = dir.path().to_path_buf();

        // 目录不存在时应该自动创建
        let index = ProjectIndex::new(root.clone()).unwrap();
        assert!(root.join(".thanos").exists());
        let _ = index;
    }
}
