//! CellLibrary 集成测试
//!
//! 测试覆盖：
//! - 库初始化
//! - Cell 查询
//! - 多库支持
//! - 端口信息获取

use std::path::PathBuf;
use tempfile::tempdir;
use thanosLSP_cell::CellLibrary;

/// 创建测试 Verilog 文件
fn create_test_lib(dir: &std::path::Path) -> PathBuf {
    let lib_file = dir.join("test_lib.v");
    let content = r#"
// Test library cells
module inv (Y, A);
  output Y;
  input A;
  not (Y, A);
endmodule

module and2 (X, A, B);
  output X;
  input A, B;
  and (X, A, B);
endmodule

module or2 (Y, A, B);
  output Y;
  input A, B;
  or (Y, A, B);
endmodule
"#;
    std::fs::write(&lib_file, content).expect("write should work");
    lib_file
}

/// 库初始化测试
#[test]
fn initialize_library() {
    let dir = tempdir().expect("tempdir should work");
    let lib_file = create_test_lib(dir.path());
    let db_path = dir.path().join("lib.redb");

    let lib = CellLibrary::from_file(&lib_file, &db_path).expect("library should initialize");

    assert!(lib.cell_count() > 0);
}

/// Cell 查询测试
#[test]
fn lookup_cell_in_library() {
    let dir = tempdir().expect("tempdir should work");
    let lib_file = create_test_lib(dir.path());
    let db_path = dir.path().join("lib.redb");

    let lib = CellLibrary::from_file(&lib_file, &db_path).expect("library should initialize");

    let inv = lib.lookup("inv").expect("inv should exist");
    assert_eq!(inv.name, "inv");
    assert_eq!(inv.ports.len(), 2);

    let and2 = lib.lookup("and2").expect("and2 should exist");
    assert_eq!(and2.ports.len(), 3);
}

/// 不存在 Cell 查询测试
#[test]
fn lookup_nonexistent_cell() {
    let dir = tempdir().expect("tempdir should work");
    let lib_file = create_test_lib(dir.path());
    let db_path = dir.path().join("lib.redb");

    let lib = CellLibrary::from_file(&lib_file, &db_path).expect("library should initialize");

    let result = lib.lookup("nonexistent");
    assert!(result.is_none());
}

/// 端口信息获取测试
#[test]
fn get_port_info() {
    let dir = tempdir().expect("tempdir should work");
    let lib_file = create_test_lib(dir.path());
    let db_path = dir.path().join("lib.redb");

    let lib = CellLibrary::from_file(&lib_file, &db_path).expect("library should initialize");

    let ports = lib.get_port_info("and2").expect("should get ports");
    assert_eq!(ports.len(), 3);

    // 验证端口格式
    assert!(ports.iter().any(|p| p.name == "X"));
    assert!(ports.iter().any(|p| p.name == "A"));
    assert!(ports.iter().any(|p| p.name == "B"));
}

/// 缓存加载测试（二次启动应使用缓存）
#[test]
fn load_from_cache() {
    let dir = tempdir().expect("tempdir should work");
    let lib_file = create_test_lib(dir.path());
    let db_path = dir.path().join("cache.redb");

    // 第一次初始化
    let lib1 = CellLibrary::from_file(&lib_file, &db_path).expect("first init should work");
    let count1 = lib1.cell_count();

    // 第二次初始化（应使用缓存）
    let lib2 = CellLibrary::from_file(&lib_file, &db_path).expect("second init should work");
    let count2 = lib2.cell_count();

    assert_eq!(count1, count2);
}

/// Cell 数量统计测试
#[test]
fn count_library_cells() {
    let dir = tempdir().expect("tempdir should work");
    let lib_file = create_test_lib(dir.path());
    let db_path = dir.path().join("lib.redb");

    let lib = CellLibrary::from_file(&lib_file, &db_path).expect("library should initialize");

    assert_eq!(lib.cell_count(), 3); // inv, and2, or2
}

/// Cell 列表测试
#[test]
fn list_all_cells() {
    let dir = tempdir().expect("tempdir should work");
    let lib_file = create_test_lib(dir.path());
    let db_path = dir.path().join("lib.redb");

    let lib = CellLibrary::from_file(&lib_file, &db_path).expect("library should initialize");

    let names = lib.cell_names();
    assert!(names.contains(&"inv".to_string()));
    assert!(names.contains(&"and2".to_string()));
    assert!(names.contains(&"or2".to_string()));
}

/// Hover 格式测试
#[test]
fn format_hover_info() {
    let dir = tempdir().expect("tempdir should work");
    let lib_file = create_test_lib(dir.path());
    let db_path = dir.path().join("lib.redb");

    let lib = CellLibrary::from_file(&lib_file, &db_path).expect("library should initialize");

    let hover = lib.format_hover("and2").expect("should format");
    assert!(hover.contains("X"));
    assert!(hover.contains("output"));
    assert!(hover.contains("A"));
    assert!(hover.contains("input"));
}
