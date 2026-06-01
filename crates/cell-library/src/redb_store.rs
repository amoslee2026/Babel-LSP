//! redb 持久化存储

use redb::{Database, ReadableTable, TableDefinition};
use std::path::Path;

use super::parser::Cell;

const CELLS_TABLE: TableDefinition<&str, &str> = TableDefinition::new("cells");
const META_TABLE: TableDefinition<&str, &str> = TableDefinition::new("meta");

/// Cell 存储
#[derive(Debug)]
pub struct CellStore {
    db: Database,
}

impl CellStore {
    /// 打开或创建存储
    pub fn open<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let db = Database::create(path)?;
        // 确保 table 存在
        let write_txn = db.begin_write()?;
        {
            let _ = write_txn.open_table(CELLS_TABLE)?;
            let _ = write_txn.open_table(META_TABLE)?;
        }
        write_txn.commit()?;
        Ok(Self { db })
    }

    /// 保存单个 Cell
    pub fn save_cell(&self, cell: &Cell) -> anyhow::Result<()> {
        let write_txn = self.db.begin_write()?;
        {
            let mut table = write_txn.open_table(CELLS_TABLE)?;
            let json = serde_json::to_string(cell)?;
            table.insert(&cell.name.as_str(), &json.as_str())?;
        }
        write_txn.commit()?;
        Ok(())
    }

    /// 批量保存 Cell
    pub fn save_cells(&self, cells: &[Cell]) -> anyhow::Result<()> {
        let write_txn = self.db.begin_write()?;
        {
            let mut table = write_txn.open_table(CELLS_TABLE)?;
            for cell in cells {
                let json = serde_json::to_string(cell)?;
                table.insert(&cell.name.as_str(), &json.as_str())?;
            }
        }
        write_txn.commit()?;
        Ok(())
    }

    /// 加载单个 Cell
    pub fn load_cell(&self, name: &str) -> anyhow::Result<Option<Cell>> {
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(CELLS_TABLE)?;
        let result = table.get(name)?;
        match result {
            Some(value) => {
                let cell: Cell = serde_json::from_str(value.value())?;
                Ok(Some(cell))
            },
            None => Ok(None),
        }
    }

    /// 加载所有 Cell
    pub fn load_all_cells(&self) -> anyhow::Result<Vec<Cell>> {
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(CELLS_TABLE)?;
        let mut cells = Vec::new();
        for entry_result in table.iter()? {
            let entry = entry_result?;
            let (_, value) = entry;
            let cell: Cell = serde_json::from_str(value.value())?;
            cells.push(cell);
        }
        Ok(cells)
    }

    /// 保存文件 hash
    pub fn save_hash(&self, hash: &str) -> anyhow::Result<()> {
        let write_txn = self.db.begin_write()?;
        {
            let mut table = write_txn.open_table(META_TABLE)?;
            table.insert("hash", hash)?;
        }
        write_txn.commit()?;
        Ok(())
    }

    /// 加载文件 hash
    pub fn load_hash(&self) -> anyhow::Result<Option<String>> {
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(META_TABLE)?;
        let result = table.get("hash")?;
        match result {
            Some(value) => Ok(Some(value.value().to_string())),
            None => Ok(None),
        }
    }

    /// 检查 hash 是否匹配
    pub fn check_hash_valid(&self, current_hash: &str) -> bool {
        match self.load_hash() {
            Ok(Some(stored_hash)) => stored_hash == current_hash,
            _ => false,
        }
    }

    /// Cell 数量
    pub fn cell_count(&self) -> usize {
        match self.load_all_cells() {
            Ok(cells) => cells.len(),
            Err(_) => 0,
        }
    }

    /// 删除 Cell
    pub fn delete_cell(&self, name: &str) -> anyhow::Result<()> {
        let write_txn = self.db.begin_write()?;
        {
            let mut table = write_txn.open_table(CELLS_TABLE)?;
            table.remove(name)?;
        }
        write_txn.commit()?;
        Ok(())
    }

    /// 清空存储
    pub fn clear(&self) -> anyhow::Result<()> {
        let write_txn = self.db.begin_write()?;
        {
            let mut table = write_txn.open_table(CELLS_TABLE)?;
            // 收集所有 keys
            let keys: Vec<String> = table
                .iter()?
                .map(|e| e.map(|(k, _)| k.value().to_string()))
                .collect::<Result<Vec<_>, _>>()?;
            // 删除所有 entries
            for key in keys {
                table.remove(&key.as_str())?;
            }
        }
        write_txn.commit()?;
        Ok(())
    }

    /// 保存元信息
    pub fn save_meta(&self, key: &str, value: &str) -> anyhow::Result<()> {
        let write_txn = self.db.begin_write()?;
        {
            let mut table = write_txn.open_table(META_TABLE)?;
            table.insert(key, value)?;
        }
        write_txn.commit()?;
        Ok(())
    }

    /// 加载元信息
    pub fn load_meta(&self, key: &str) -> anyhow::Result<Option<String>> {
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(META_TABLE)?;
        let result = table.get(key)?;
        match result {
            Some(value) => Ok(Some(value.value().to_string())),
            None => Ok(None),
        }
    }
}
