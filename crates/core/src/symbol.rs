//! 符号类型定义
//!
//! 定义符号、符号类型、位置等核心类型

use lasso::Rodeo;
use smol_str::SmolStr;
use std::sync::Arc;

/// 符号位置
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Location {
    /// 文件 URI
    pub uri: String,
    /// 起始位置
    pub start: Position,
    /// 结束位置
    pub end: Position,
}

/// 位置（行、列）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub line: u32,
    pub column: u32,
}

impl Position {
    pub fn new(line: u32, column: u32) -> Self {
        Self { line, column }
    }
}

/// 符号类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SymbolKind {
    // HDL 符号
    Module,
    Port,
    Signal,
    Parameter,
    Typedef,
    Macro,
    Function,
    Task,
    Interface,
    Package,
    Class,
    // TCL 符号
    Proc,
    Variable,
    Namespace,
    // 单元库符号
    Cell,
}

/// 符号
#[derive(Debug, Clone)]
pub struct Symbol {
    /// 符号名称
    pub name: SmolStr,
    /// 符号类型
    pub kind: SymbolKind,
    /// 位置
    pub location: Location,
    /// 文档注释
    pub doc_comment: Option<String>,
    /// 详细信息
    pub detail: Option<String>,
    /// 子符号
    pub children: Vec<Symbol>,
}

impl Symbol {
    /// 创建新符号
    pub fn new(name: SmolStr, kind: SymbolKind, location: Location) -> Self {
        Self {
            name,
            kind,
            location,
            doc_comment: None,
            detail: None,
            children: vec![],
        }
    }

    /// 添加子符号
    pub fn add_child(&mut self, child: Symbol) {
        self.children.push(child);
    }

    /// 检查是否包含位置
    pub fn contains_position(&self, pos: Position) -> bool {
        let start = self.location.start;
        let end = self.location.end;
        pos.line >= start.line
            && pos.line <= end.line
            && (pos.line != start.line || pos.column >= start.column)
            && (pos.line != end.line || pos.column <= end.column)
    }
}

/// 符号表（使用字符串 intern 优化）
pub struct SymbolTable {
    /// 字符串 intern 池
    #[allow(dead_code)]
    strings: Arc<Rodeo>,
    /// 符号列表
    symbols: Vec<Symbol>,
}

impl SymbolTable {
    /// 创建空符号表
    pub fn new() -> Self {
        Self {
            strings: Arc::new(Rodeo::new()),
            symbols: vec![],
        }
    }

    /// 添加符号
    pub fn add(&mut self, symbol: Symbol) {
        self.symbols.push(symbol);
    }

    /// 获取所有符号
    pub fn symbols(&self) -> &[Symbol] {
        &self.symbols
    }

    /// 查找符号（按名称）
    pub fn find_by_name(&self, name: &str) -> Option<&Symbol> {
        self.symbols.iter().find(|s| s.name == name)
    }

    /// 查找符号（按位置）
    pub fn find_at_position(&self, uri: &str, pos: Position) -> Option<&Symbol> {
        self.symbols
            .iter()
            .find(|s| s.location.uri == uri && s.contains_position(pos))
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_creation() {
        let pos = Position::new(10, 5);
        assert_eq!(pos.line, 10);
        assert_eq!(pos.column, 5);
    }

    #[test]
    fn test_position_equality() {
        let pos1 = Position::new(5, 10);
        let pos2 = Position::new(5, 10);
        let pos3 = Position::new(5, 11);
        assert_eq!(pos1, pos2);
        assert_ne!(pos1, pos3);
    }

    #[test]
    fn test_position_copy() {
        let pos1 = Position::new(10, 20);
        let pos2 = pos1;
        assert_eq!(pos1, pos2);
    }

    #[test]
    fn test_location_creation() {
        let loc = Location {
            uri: "file:///test.sv".to_string(),
            start: Position::new(0, 0),
            end: Position::new(10, 20),
        };
        assert_eq!(loc.uri, "file:///test.sv");
        assert_eq!(loc.start.line, 0);
        assert_eq!(loc.end.line, 10);
    }

    #[test]
    fn test_location_clone() {
        let loc = Location {
            uri: "file:///test.sv".to_string(),
            start: Position::new(0, 0),
            end: Position::new(10, 20),
        };
        let cloned = loc.clone();
        assert_eq!(loc.uri, cloned.uri);
        assert_eq!(loc.start, cloned.start);
    }

    #[test]
    fn test_symbol_creation() {
        let loc = Location {
            uri: "file:///test.sv".to_string(),
            start: Position::new(0, 0),
            end: Position::new(10, 0),
        };
        let symbol = Symbol::new(
            SmolStr::from("test_module"),
            SymbolKind::Module,
            loc.clone(),
        );
        assert_eq!(symbol.name, "test_module");
        assert_eq!(symbol.kind, SymbolKind::Module);
        assert!(symbol.doc_comment.is_none());
        assert!(symbol.detail.is_none());
        assert!(symbol.children.is_empty());
    }

    #[test]
    fn test_symbol_with_all_fields() {
        let loc = Location {
            uri: "file:///test.sv".to_string(),
            start: Position::new(0, 0),
            end: Position::new(10, 0),
        };
        let mut symbol = Symbol::new(SmolStr::from("test_func"), SymbolKind::Function, loc);
        symbol.doc_comment = Some("Test function".to_string());
        symbol.detail = Some("function void test_func()".to_string());
        assert_eq!(symbol.doc_comment, Some("Test function".to_string()));
        assert_eq!(symbol.detail, Some("function void test_func()".to_string()));
    }

    #[test]
    fn test_symbol_add_child() {
        let parent_loc = Location {
            uri: "file:///test.sv".to_string(),
            start: Position::new(0, 0),
            end: Position::new(20, 0),
        };
        let mut parent = Symbol::new(SmolStr::from("parent"), SymbolKind::Module, parent_loc);

        let child_loc = Location {
            uri: "file:///test.sv".to_string(),
            start: Position::new(5, 0),
            end: Position::new(10, 0),
        };
        let child = Symbol::new(SmolStr::from("child"), SymbolKind::Signal, child_loc);

        parent.add_child(child);
        assert_eq!(parent.children.len(), 1);
        assert_eq!(parent.children[0].name, "child");
    }

    #[test]
    fn test_symbol_contains_position() {
        let loc = Location {
            uri: "file:///test.sv".to_string(),
            start: Position::new(0, 0),
            end: Position::new(10, 20),
        };
        let symbol = Symbol::new(SmolStr::from("test"), SymbolKind::Module, loc);
        assert!(symbol.contains_position(Position::new(5, 10)));
        assert!(!symbol.contains_position(Position::new(15, 0)));
    }

    #[test]
    fn test_symbol_contains_position_edge_cases() {
        let loc = Location {
            uri: "file:///test.sv".to_string(),
            start: Position::new(5, 10),
            end: Position::new(10, 20),
        };
        let symbol = Symbol::new(SmolStr::from("test"), SymbolKind::Module, loc);

        // Start position
        assert!(symbol.contains_position(Position::new(5, 10)));
        // Before start column
        assert!(!symbol.contains_position(Position::new(5, 5)));
        // End position
        assert!(symbol.contains_position(Position::new(10, 20)));
        // After end column
        assert!(!symbol.contains_position(Position::new(10, 25)));
        // Same line within range
        assert!(symbol.contains_position(Position::new(7, 0)));
    }

    #[test]
    fn test_symbol_kind_variants() {
        let kinds = [
            SymbolKind::Module,
            SymbolKind::Port,
            SymbolKind::Signal,
            SymbolKind::Parameter,
            SymbolKind::Typedef,
            SymbolKind::Macro,
            SymbolKind::Function,
            SymbolKind::Task,
            SymbolKind::Interface,
            SymbolKind::Package,
            SymbolKind::Class,
            SymbolKind::Proc,
            SymbolKind::Variable,
            SymbolKind::Namespace,
            SymbolKind::Cell,
        ];
        for kind in &kinds {
            let loc = Location {
                uri: "file:///test.sv".to_string(),
                start: Position::new(0, 0),
                end: Position::new(10, 0),
            };
            let symbol = Symbol::new(SmolStr::from("s"), *kind, loc);
            assert_eq!(symbol.kind, *kind);
        }
    }

    #[test]
    fn test_symbol_kind_equality() {
        assert_eq!(SymbolKind::Module, SymbolKind::Module);
        assert_ne!(SymbolKind::Module, SymbolKind::Function);
    }

    #[test]
    fn test_symbol_table_new() {
        let table = SymbolTable::new();
        assert!(table.symbols().is_empty());
    }

    #[test]
    fn test_symbol_table_default() {
        let table = SymbolTable::default();
        assert!(table.symbols().is_empty());
    }

    #[test]
    fn test_symbol_table_add() {
        let mut table = SymbolTable::new();
        let loc = Location {
            uri: "file:///test.sv".to_string(),
            start: Position::new(0, 0),
            end: Position::new(10, 0),
        };
        let symbol = Symbol::new(SmolStr::from("mod1"), SymbolKind::Module, loc);
        table.add(symbol);
        assert_eq!(table.symbols().len(), 1);
    }

    #[test]
    fn test_symbol_table_find_by_name() {
        let mut table = SymbolTable::new();
        let loc = Location {
            uri: "file:///test.sv".to_string(),
            start: Position::new(0, 0),
            end: Position::new(10, 0),
        };
        let symbol = Symbol::new(SmolStr::from("target"), SymbolKind::Module, loc);
        table.add(symbol);

        let found = table.find_by_name("target");
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "target");

        let not_found = table.find_by_name("nonexistent");
        assert!(not_found.is_none());
    }

    #[test]
    fn test_symbol_table_find_at_position() {
        let mut table = SymbolTable::new();
        let loc = Location {
            uri: "file:///test.sv".to_string(),
            start: Position::new(5, 0),
            end: Position::new(15, 0),
        };
        let symbol = Symbol::new(SmolStr::from("mod1"), SymbolKind::Module, loc);
        table.add(symbol);

        // Position inside symbol
        let found = table.find_at_position("file:///test.sv", Position::new(10, 5));
        assert!(found.is_some());

        // Position outside symbol
        let not_found = table.find_at_position("file:///test.sv", Position::new(20, 0));
        assert!(not_found.is_none());

        // Different URI
        let not_found_uri = table.find_at_position("file:///other.sv", Position::new(10, 5));
        assert!(not_found_uri.is_none());
    }

    #[test]
    fn test_symbol_table_multiple_symbols() {
        let mut table = SymbolTable::new();
        let loc1 = Location {
            uri: "file:///a.sv".to_string(),
            start: Position::new(0, 0),
            end: Position::new(10, 0),
        };
        let loc2 = Location {
            uri: "file:///b.sv".to_string(),
            start: Position::new(5, 0),
            end: Position::new(15, 0),
        };
        table.add(Symbol::new(
            SmolStr::from("mod_a"),
            SymbolKind::Module,
            loc1,
        ));
        table.add(Symbol::new(
            SmolStr::from("mod_b"),
            SymbolKind::Module,
            loc2,
        ));

        assert_eq!(table.symbols().len(), 2);
        assert!(table.find_by_name("mod_a").is_some());
        assert!(table.find_by_name("mod_b").is_some());
    }
}

#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        /// 不变式: start 位置本身始终在 symbol 范围内
        #[test]
        fn prop_start_position_always_contained(
            start_line in 0u32..1000u32,
            start_col in 0u32..200u32,
            end_line_offset in 0u32..100u32,
            end_col in 0u32..200u32,
        ) {
            let end_line = start_line + end_line_offset;
            let loc = Location {
                uri: "file:///test.sv".to_string(),
                start: Position::new(start_line, start_col),
                end: Position::new(end_line, end_col),
            };
            let symbol = Symbol::new(SmolStr::from("s"), SymbolKind::Module, loc);
            // start position is always contained (when end_line > start_line)
            if end_line > start_line {
                prop_assert!(symbol.contains_position(Position::new(start_line, start_col)));
            }
        }

        /// 不变式: end_line 之后的行不在 symbol 范围内
        #[test]
        fn prop_beyond_end_not_contained(
            start_line in 0u32..500u32,
            end_line in 1u32..500u32,
            beyond_offset in 1u32..100u32,
        ) {
            let end_line = start_line + end_line;
            let beyond_line = end_line + beyond_offset;
            let loc = Location {
                uri: "file:///test.sv".to_string(),
                start: Position::new(start_line, 0),
                end: Position::new(end_line, 0),
            };
            let symbol = Symbol::new(SmolStr::from("s"), SymbolKind::Module, loc);
            prop_assert!(!symbol.contains_position(Position::new(beyond_line, 0)));
        }
    }
}
