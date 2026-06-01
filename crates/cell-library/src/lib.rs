#![allow(non_snake_case)]
//! 标准单元库索引

pub mod cell_index;
pub mod parser;
pub mod redb_store;

pub use cell_index::CellIndex;
pub use parser::{Cell, CellParser, Port, PortDirection};
pub use redb_store::CellStore;

use std::path::{Path, PathBuf};

/// CellLibrary：管理多个标准单元库的加载、查询
pub struct CellLibrary {
    index: CellIndex,
    db_path: PathBuf,
    lib_path: PathBuf,
}

impl CellLibrary {
    /// 从 Verilog 文件初始化 CellLibrary
    pub fn from_file<P: AsRef<Path>, Q: AsRef<Path>>(
        lib_path: P,
        db_path: Q,
    ) -> anyhow::Result<Self> {
        let lib_path = lib_path.as_ref().to_path_buf();
        let db_path = db_path.as_ref().to_path_buf();

        // 尝试从缓存加载
        let store = CellStore::open(&db_path)?;
        let file_hash = compute_file_hash(&lib_path)?;

        if store.check_hash_valid(&file_hash) {
            // 缓存有效，直接加载
            let cells = store.load_all_cells()?;
            let index = CellIndex::from_cells(cells);
            return Ok(Self {
                index,
                db_path,
                lib_path,
            });
        }

        // 需要解析
        let content = std::fs::read_to_string(&lib_path)?;
        let parser = CellParser::new();
        let cells = parser.parse_with_path(&content, &lib_path)?;

        // 保存到缓存
        store.save_cells(&cells)?;
        store.save_hash(&file_hash)?;

        let index = CellIndex::from_cells(cells);
        Ok(Self {
            index,
            db_path,
            lib_path,
        })
    }

    /// 查询 Cell
    pub fn lookup(&self, name: &str) -> Option<&Cell> {
        self.index.get(name)
    }

    /// 获取端口信息
    pub fn get_port_info(&self, name: &str) -> Option<&[Port]> {
        self.lookup(name).map(|c| c.ports.as_slice())
    }

    /// Cell 数量
    pub fn cell_count(&self) -> usize {
        self.index.len()
    }

    /// Cell 名称列表
    pub fn cell_names(&self) -> Vec<String> {
        self.index.cells().map(|c| c.name.clone()).collect()
    }

    /// 格式化 Hover 信息
    pub fn format_hover(&self, name: &str) -> Option<String> {
        let cell = self.lookup(name)?;
        let port_info = cell
            .ports
            .iter()
            .map(|p| format!("{}: {}", p.name, p.direction_str()))
            .collect::<Vec<_>>()
            .join(", ");
        Some(format!("{} [{}]", cell.name, port_info))
    }

    /// 刷新库（重新解析）
    pub fn refresh(&mut self) -> anyhow::Result<()> {
        let content = std::fs::read_to_string(&self.lib_path)?;
        let parser = CellParser::new();
        let cells = parser.parse_with_path(&content, &self.lib_path)?;

        let file_hash = compute_file_hash(&self.lib_path)?;
        let store = CellStore::open(&self.db_path)?;
        store.save_cells(&cells)?;
        store.save_hash(&file_hash)?;

        self.index = CellIndex::from_cells(cells);
        Ok(())
    }
}

/// 计算文件 hash
fn compute_file_hash<P: AsRef<Path>>(path: P) -> anyhow::Result<String> {
    use sha2::{Digest, Sha256};
    let content = std::fs::read(path)?;
    let mut hasher = Sha256::new();
    hasher.update(&content);
    let hash = hasher.finalize();
    Ok(format!("{:x}", hash))
}
