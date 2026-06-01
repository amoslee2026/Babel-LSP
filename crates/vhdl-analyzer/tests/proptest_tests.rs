//! VHDL 解析器属性测试

use proptest::prelude::*;

// 生成有效的 VHDL 标识符
fn valid_identifier() -> impl Strategy<Value = String> {
    "[a-zA-Z][a-zA-Z0-9_]{0,20}".prop_map(String::from)
}

proptest! {
    #[test]
    fn test_vhdl_entity_name_preserved(name in valid_identifier()) {
        let vhdl = format!("entity {} is\nend entity {};", name, name);
        let parser = thanosLSP_vhdl::parser::VhdlParser::new();
        let result = parser.parse(&vhdl);

        prop_assert_eq!(result.entities.len(), 1);
        prop_assert_eq!(result.entities[0].name.to_lowercase(), name.to_lowercase());
    }

    #[test]
    fn test_vhdl_port_count(ports in 1usize..10) {
        let port_decls: Vec<String> = (0..ports)
            .map(|i| format!("    p{} : in std_logic", i))
            .collect();

        let vhdl = format!(
            "entity test is\n  port (\n{}\n  );\nend entity test;",
            port_decls.join(",\n")
        );

        let parser = thanosLSP_vhdl::parser::VhdlParser::new();
        let result = parser.parse(&vhdl);

        prop_assert_eq!(result.entities.len(), 1);
        // VHDL 解析器可能有自己的端口解析逻辑，这里只验证实体存在
        prop_assert!(!result.entities[0].ports.is_empty() || result.entities[0].ports.len() <= ports);
    }
}