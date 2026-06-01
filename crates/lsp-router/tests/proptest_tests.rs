//! SystemVerilog 解析器属性测试

use proptest::prelude::*;

// 生成有效的 SystemVerilog 标识符
fn valid_identifier() -> impl Strategy<Value = String> {
    "[a-zA-Z_][a-zA-Z0-9_]{0,20}".prop_map(String::from)
}

proptest! {
    #[test]
    fn test_sv_module_name_extracted(name in valid_identifier()) {
        let sv = format!("module {};\nendmodule", name);
        let uri = url::Url::parse("file:///test.sv").unwrap();
        let symbols = thanosLSP_lsp::backend::extract_symbols_basic(&uri, &sv);

        prop_assert_eq!(symbols.len(), 1);
        prop_assert_eq!(symbols[0].name.as_str(), name);
    }

    #[test]
    fn test_sv_multiple_modules(count in 1usize..10) {
        let modules: Vec<String> = (0..count)
            .map(|i| format!("module m{};\nendmodule", i))
            .collect();

        let sv = modules.join("\n");
        let uri = url::Url::parse("file:///test.sv").unwrap();
        let symbols = thanosLSP_lsp::backend::extract_symbols_basic(&uri, &sv);

        prop_assert_eq!(symbols.len(), count);
    }
}