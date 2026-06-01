//! Cell 索引构建测试
//!
//! 测试覆盖：
//! - Cell 索引构建
//! - 名称查询
//! - 批量添加
//! - Cell 数量统计

use std::path::PathBuf;
use thanosLSP_cell::cell_index::CellIndex;
use thanosLSP_cell::parser::{Cell, Port, PortDirection};

/// 创建测试 Cell
fn make_cell(name: &str, ports: Vec<(&str, PortDirection, usize)>) -> Cell {
    Cell {
        name: name.to_string(),
        ports: ports
            .into_iter()
            .map(|(n, d, w)| Port {
                name: n.to_string(),
                direction: d,
                width: w,
            })
            .collect(),
        description: None,
        source_file: PathBuf::from("test.v"),
        line: 1,
    }
}

/// 基础索引构建测试
#[test]
fn build_basic_index() {
    let mut index = CellIndex::new();
    let cell = make_cell(
        "and2",
        vec![
            ("X", PortDirection::Output, 1),
            ("A", PortDirection::Input, 1),
            ("B", PortDirection::Input, 1),
        ],
    );

    index.add(cell);
    assert_eq!(index.len(), 1);
}

/// Cell 查询测试
#[test]
fn lookup_cell_by_name() {
    let mut index = CellIndex::new();
    let cell = make_cell(
        "or2",
        vec![
            ("Y", PortDirection::Output, 1),
            ("A", PortDirection::Input, 1),
            ("B", PortDirection::Input, 1),
        ],
    );

    index.add(cell);

    let found = index.get("or2");
    assert!(found.is_some());
    assert_eq!(found.unwrap().name, "or2");

    let not_found = index.get("nonexistent");
    assert!(not_found.is_none());
}

/// 批量添加测试
#[test]
fn add_multiple_cells() {
    let mut index = CellIndex::new();

    for name in ["and2", "or2", "xor2", "inv"] {
        let cell = make_cell(
            name,
            vec![
                ("Y", PortDirection::Output, 1),
                ("A", PortDirection::Input, 1),
            ],
        );
        index.add(cell);
    }

    assert_eq!(index.len(), 4);
    assert!(index.get("and2").is_some());
    assert!(index.get("or2").is_some());
    assert!(index.get("xor2").is_some());
    assert!(index.get("inv").is_some());
}

/// Cell 重复处理测试（同名 Cell 应覆盖）
#[test]
fn handle_duplicate_names() {
    let mut index = CellIndex::new();

    let cell1 = make_cell(
        "dup",
        vec![
            ("X", PortDirection::Output, 1),
            ("A", PortDirection::Input, 1),
        ],
    );

    let cell2 = make_cell(
        "dup",
        vec![
            ("Y", PortDirection::Output, 1),
            ("A", PortDirection::Input, 1),
            ("B", PortDirection::Input, 1),
        ],
    );

    index.add(cell1);
    index.add(cell2);

    // 后添加的应覆盖
    assert_eq!(index.len(), 1);
    let found = index.get("dup").unwrap();
    assert_eq!(found.ports.len(), 3);
}

/// 端口迭代测试
#[test]
fn iterate_cells() {
    let mut index = CellIndex::new();

    for name in ["a", "b", "c"] {
        index.add(make_cell(name, vec![]));
    }

    let count = index.cells().count();
    assert_eq!(count, 3);
}

/// 从 Cell 列表构建索引测试
#[test]
fn build_from_cells() {
    let cells = vec![
        make_cell("c1", vec![]),
        make_cell("c2", vec![]),
        make_cell("c3", vec![]),
    ];

    let index = CellIndex::from_cells(cells);
    assert_eq!(index.len(), 3);
}

/// 清空索引测试
#[test]
fn clear_index() {
    let mut index = CellIndex::new();
    index.add(make_cell("test", vec![]));
    assert_eq!(index.len(), 1);

    index.clear();
    assert_eq!(index.len(), 0);
}
