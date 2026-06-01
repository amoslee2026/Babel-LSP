//! TCL 解析器属性测试

use proptest::prelude::*;

// 生成有效的 TCL 标识符
fn valid_identifier() -> impl Strategy<Value = String> {
    "[a-zA-Z_][a-zA-Z0-9_]{0,20}".prop_map(String::from)
}

proptest! {
    #[test]
    fn test_tcl_proc_name_preserved(name in valid_identifier()) {
        let tcl = format!("proc {} {{}} {{\n}}", name);
        let parser = thanosLSP_tcl::parser::TclParser::new();
        let result = parser.parse(&tcl);

        prop_assert_eq!(result.procs.len(), 1);
        prop_assert_eq!(&result.procs[0].name, &name);
    }

    #[test]
    fn test_tcl_variable_name_preserved(name in valid_identifier()) {
        let tcl = format!("set {} 42", name);
        let parser = thanosLSP_tcl::parser::TclParser::new();
        let result = parser.parse(&tcl);

        prop_assert!(!result.variables.is_empty());
        prop_assert!(result.variables.iter().any(|v| v.name == name));
    }

    #[test]
    fn test_tcl_proc_args_count(args in 0usize..10) {
        let arg_names: Vec<String> = (0..args)
            .map(|i| format!("arg{}", i))
            .collect();

        let tcl = format!(
            "proc myproc {{{}}} {{\n}}",
            arg_names.join(" ")
        );

        let parser = thanosLSP_tcl::parser::TclParser::new();
        let result = parser.parse(&tcl);

        prop_assert_eq!(result.procs.len(), 1);
        prop_assert_eq!(result.procs[0].args.len(), args);
    }
}