//! redb 持久化存储测试
//!
//! 测试覆盖：
//! - 存储 open/create
//! - Cell 保存/加载
//! - Hash 存储/验证
//! - 增量更新检测
//! - 事务一致性

use std::path::PathBuf;
use tempfile::tempdir;
use babel_lsp_cell::parser::{Cell, Port, PortDirection};
use babel_lsp_cell::redb_store::CellStore;

/// 创建测试 Cell
fn make_cell(name: &str) -> Cell {
    Cell {
        name: name.to_string(),
        ports: vec![
            Port {
                name: "X".to_string(),
                direction: PortDirection::Output,
                width: 1,
            },
            Port {
                name: "A".to_string(),
                direction: PortDirection::Input,
                width: 1,
            },
        ],
        description: Some("test cell".to_string()),
        source_file: PathBuf::from("test.v"),
        line: 1,
    }
}

/// 基础存储创建测试
#[test]
fn create_store() {
    let dir = tempdir().expect("tempdir should work");
    let db_path = dir.path().join("test.redb");

    let store = CellStore::open(&db_path).expect("store should open");
    // 基本验证：能成功打开
    let _ = store;
}

/// Cell 保存测试
#[test]
fn save_single_cell() {
    let dir = tempdir().expect("tempdir should work");
    let db_path = dir.path().join("cells.redb");

    let store = CellStore::open(&db_path).expect("store should open");
    let cell = make_cell("test_cell");

    store.save_cell(&cell).expect("save should succeed");
}

/// Cell 加载测试
#[test]
fn load_saved_cell() {
    let dir = tempdir().expect("tempdir should work");
    let db_path = dir.path().join("cells.redb");

    let store = CellStore::open(&db_path).expect("store should open");
    let cell = make_cell("loadable_cell");

    store.save_cell(&cell).expect("save should succeed");

    let loaded = store
        .load_cell("loadable_cell")
        .expect("load should succeed");
    assert!(loaded.is_some());

    let loaded_cell = loaded.unwrap();
    assert_eq!(loaded_cell.name, "loadable_cell");
    assert_eq!(loaded_cell.ports.len(), 2);
}

/// 批量保存测试
#[test]
fn save_multiple_cells() {
    let dir = tempdir().expect("tempdir should work");
    let db_path = dir.path().join("batch.redb");

    let store = CellStore::open(&db_path).expect("store should open");

    let cells: Vec<Cell> = ["c1", "c2", "c3", "c4", "c5"]
        .into_iter()
        .map(make_cell)
        .collect();

    store.save_cells(&cells).expect("batch save should succeed");

    // 验证全部可加载
    for name in ["c1", "c2", "c3", "c4", "c5"] {
        let loaded = store.load_cell(name).expect("load should succeed");
        assert!(loaded.is_some());
    }
}

/// Hash 存储/验证测试
#[test]
fn store_and_verify_hash() {
    let dir = tempdir().expect("tempdir should work");
    let db_path = dir.path().join("hash.redb");

    let store = CellStore::open(&db_path).expect("store should open");

    let hash = "abc123def456".to_string();
    store.save_hash(&hash).expect("hash save should succeed");

    let stored_hash = store.load_hash().expect("hash load should succeed");
    assert_eq!(stored_hash, Some(hash));
}

/// Hash 不匹配检测测试
#[test]
fn detect_hash_mismatch() {
    let dir = tempdir().expect("tempdir should work");
    let db_path = dir.path().join("mismatch.redb");

    let store = CellStore::open(&db_path).expect("store should open");
    store.save_hash("old_hash").expect("save should work");

    let current_hash = "new_hash".to_string();
    let is_valid = store.check_hash_valid(&current_hash);
    assert!(!is_valid);
}

/// Hash 匹配验证测试
#[test]
fn verify_matching_hash() {
    let dir = tempdir().expect("tempdir should work");
    let db_path = dir.path().join("match.redb");

    let store = CellStore::open(&db_path).expect("store should open");
    let hash = "correct_hash".to_string();
    store.save_hash(&hash).expect("save should work");

    let is_valid = store.check_hash_valid(&hash);
    assert!(is_valid);
}

/// 索引全量保存/加载测试
#[test]
fn save_and_load_full_index() {
    let dir = tempdir().expect("tempdir should work");
    let db_path = dir.path().join("full.redb");

    let store = CellStore::open(&db_path).expect("store should open");

    let cells: Vec<Cell> = ["inv", "and2", "or2", "xor2"]
        .into_iter()
        .map(make_cell)
        .collect();

    store.save_cells(&cells).expect("save should work");
    store.save_hash("test_hash").expect("hash should work");

    let loaded_cells = store.load_all_cells().expect("load all should work");
    assert_eq!(loaded_cells.len(), 4);
}

/// 空 Cell 查询测试
#[test]
fn query_nonexistent_cell() {
    let dir = tempdir().expect("tempdir should work");
    let db_path = dir.path().join("empty.redb");

    let store = CellStore::open(&db_path).expect("store should open");

    let loaded = store.load_cell("nonexistent").expect("load should work");
    assert!(loaded.is_none());
}

/// Cell 数量统计测试
#[test]
fn count_cells_in_store() {
    let dir = tempdir().expect("tempdir should work");
    let db_path = dir.path().join("count.redb");

    let store = CellStore::open(&db_path).expect("store should open");

    assert_eq!(store.cell_count(), 0);

    for name in ["a", "b", "c"] {
        store.save_cell(&make_cell(name)).expect("save should work");
    }

    assert_eq!(store.cell_count(), 3);
}

/// 删除 Cell 测试
#[test]
fn delete_cell() {
    let dir = tempdir().expect("tempdir should work");
    let db_path = dir.path().join("delete.redb");

    let store = CellStore::open(&db_path).expect("store should open");
    store
        .save_cell(&make_cell("to_delete"))
        .expect("save should work");

    assert!(store.load_cell("to_delete").expect("load").is_some());

    store.delete_cell("to_delete").expect("delete should work");
    assert!(store.load_cell("to_delete").expect("load").is_none());
}

/// 清空存储测试
#[test]
fn clear_store() {
    let dir = tempdir().expect("tempdir should work");
    let db_path = dir.path().join("clear.redb");

    let store = CellStore::open(&db_path).expect("store should open");

    for name in ["x", "y", "z"] {
        store.save_cell(&make_cell(name)).expect("save");
    }

    assert_eq!(store.cell_count(), 3);

    store.clear().expect("clear should work");
    assert_eq!(store.cell_count(), 0);
}
