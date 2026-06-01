//! Cell 名称 + 端口索引

use std::collections::HashMap;

use super::parser::Cell;

/// Cell 索引
#[derive(Debug)]
pub struct CellIndex {
    cells: HashMap<String, Cell>,
}

impl CellIndex {
    /// 创建空索引
    pub fn new() -> Self {
        Self {
            cells: HashMap::new(),
        }
    }

    /// 从 Cell 列表构建索引
    pub fn from_cells(cells: Vec<Cell>) -> Self {
        let mut index = HashMap::new();
        for cell in cells {
            index.insert(cell.name.clone(), cell);
        }
        Self { cells: index }
    }

    /// 添加 Cell
    pub fn add(&mut self, cell: Cell) {
        self.cells.insert(cell.name.clone(), cell);
    }

    /// 查询 Cell
    pub fn get(&self, name: &str) -> Option<&Cell> {
        self.cells.get(name)
    }

    /// Cell 数量
    pub fn len(&self) -> usize {
        self.cells.len()
    }

    /// 是否为空
    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    /// 迭代所有 Cell
    pub fn cells(&self) -> impl Iterator<Item = &Cell> {
        self.cells.values()
    }

    /// Cell 名称迭代器
    pub fn names(&self) -> impl Iterator<Item = &String> {
        self.cells.keys()
    }

    /// 清空索引
    pub fn clear(&mut self) {
        self.cells.clear();
    }

    /// 移除 Cell
    pub fn remove(&mut self, name: &str) -> Option<Cell> {
        self.cells.remove(name)
    }
}

impl Default for CellIndex {
    fn default() -> Self {
        Self::new()
    }
}
