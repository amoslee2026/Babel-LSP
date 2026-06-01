//! VHDL 符号收集器
//!
//! 将 VhdlParseResult 转换为通用的 Symbol 列表

use smol_str::SmolStr;

use thanosLSP_core::symbol::{Location, Position, Symbol, SymbolKind};

use crate::parser::{VhdlArchitecture, VhdlEntity, VhdlParseResult};

pub struct VhdlSymbolCollector;

impl VhdlSymbolCollector {
    pub fn new() -> Self {
        Self
    }

    /// 将解析结果转换为符号列表
    ///
    /// - entity        -> SymbolKind::Module（包含端口和泛型作为子符号）
    /// - port          -> SymbolKind::Port
    /// - generic       -> SymbolKind::Parameter
    /// - architecture  -> SymbolKind::Namespace
    /// - signal        -> SymbolKind::Variable
    /// - package       -> SymbolKind::Package
    /// - process       -> SymbolKind::Function（以 label 或 "process_<line>" 为名）
    pub fn collect(&self, parse_result: &VhdlParseResult, file_uri: &str) -> Vec<Symbol> {
        let mut symbols = Vec::new();

        // 实体 -> Module
        for entity in &parse_result.entities {
            symbols.push(self.entity_to_symbol(entity, file_uri));
        }

        // 架构 -> Namespace
        for arch in &parse_result.architectures {
            symbols.push(self.architecture_to_symbol(arch, file_uri));
        }

        // 包 -> Package
        for pkg in &parse_result.packages {
            let loc = Location {
                uri: file_uri.to_string(),
                start: Position::new(pkg.start_line, 0),
                end: Position::new(pkg.end_line, 0),
            };
            let mut sym = Symbol::new(SmolStr::from(pkg.name.clone()), SymbolKind::Package, loc);
            sym.detail = Some(format!("package {}", pkg.name));
            symbols.push(sym);
        }

        symbols
    }

    fn entity_to_symbol(&self, entity: &VhdlEntity, file_uri: &str) -> Symbol {
        let loc = Location {
            uri: file_uri.to_string(),
            start: Position::new(entity.start_line, 0),
            end: Position::new(entity.end_line, 0),
        };
        let mut sym = Symbol::new(SmolStr::from(entity.name.clone()), SymbolKind::Module, loc);
        sym.detail = Some(format!("entity {}", entity.name));

        // 泛型 -> Parameter
        for generic in &entity.generics {
            let gloc = Location {
                uri: file_uri.to_string(),
                start: Position::new(generic.line, 0),
                end: Position::new(generic.line, 0),
            };
            let mut gsym = Symbol::new(
                SmolStr::from(generic.name.clone()),
                SymbolKind::Parameter,
                gloc,
            );
            gsym.detail = Some(format!(
                "{} : {}{}",
                generic.name,
                generic.data_type,
                generic
                    .default_value
                    .as_ref()
                    .map(|v| format!(" := {}", v))
                    .unwrap_or_default()
            ));
            sym.add_child(gsym);
        }

        // 端口 -> Port
        for port in &entity.ports {
            let ploc = Location {
                uri: file_uri.to_string(),
                start: Position::new(port.line, 0),
                end: Position::new(port.line, 0),
            };
            let mut psym = Symbol::new(SmolStr::from(port.name.clone()), SymbolKind::Port, ploc);
            psym.detail = Some(format!(
                "{} : {:?} {}",
                port.name, port.direction, port.data_type
            ));
            sym.add_child(psym);
        }

        sym
    }

    fn architecture_to_symbol(&self, arch: &VhdlArchitecture, file_uri: &str) -> Symbol {
        let loc = Location {
            uri: file_uri.to_string(),
            start: Position::new(arch.start_line, 0),
            end: Position::new(arch.end_line, 0),
        };
        let arch_display = format!("{} of {}", arch.name, arch.entity_name);
        let mut sym = Symbol::new(
            SmolStr::from(arch_display.clone()),
            SymbolKind::Namespace,
            loc,
        );
        sym.detail = Some(format!("architecture {}", arch_display));

        // 信号 -> Variable
        for signal in &arch.signals {
            let sloc = Location {
                uri: file_uri.to_string(),
                start: Position::new(signal.line, 0),
                end: Position::new(signal.line, 0),
            };
            let mut ssym = Symbol::new(
                SmolStr::from(signal.name.clone()),
                SymbolKind::Variable,
                sloc,
            );
            ssym.detail = Some(format!("signal {} : {}", signal.name, signal.data_type));
            sym.add_child(ssym);
        }

        // 进程 -> Function
        for proc in &arch.processes {
            let proc_name = proc
                .label
                .clone()
                .unwrap_or_else(|| format!("process_{}", proc.start_line));
            let ploc = Location {
                uri: file_uri.to_string(),
                start: Position::new(proc.start_line, 0),
                end: Position::new(proc.end_line, 0),
            };
            let mut psym =
                Symbol::new(SmolStr::from(proc_name.clone()), SymbolKind::Function, ploc);
            psym.detail = Some(if proc.sensitivity_list.is_empty() {
                "process".to_string()
            } else {
                format!("process({})", proc.sensitivity_list.join(", "))
            });
            sym.add_child(psym);
        }

        sym
    }
}

impl Default for VhdlSymbolCollector {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// 测试
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::VhdlParser;

    const FILE_URI: &str = "file:///test.vhd";

    const ENTITY_VHDL: &str = r#"
entity adder is
    generic (
        WIDTH : integer := 8
    );
    port (
        a   : in  std_logic_vector(7 downto 0);
        b   : in  std_logic_vector(7 downto 0);
        sum : out std_logic_vector(8 downto 0)
    );
end entity adder;
"#;

    const ARCH_VHDL: &str = r#"
architecture rtl of adder is
    signal carry : std_logic;
    signal tmp   : std_logic_vector(7 downto 0);
begin
    main_proc : process(a, b)
    begin
        sum <= ('0' & a) + ('0' & b);
    end process;
end architecture rtl;
"#;

    #[test]
    fn test_entity_to_symbol() {
        let parser = VhdlParser::new();
        let collector = VhdlSymbolCollector::new();

        let result = parser.parse(ENTITY_VHDL);
        let symbols = collector.collect(&result, FILE_URI);

        // 应有一个 Module 符号
        let entity_sym = symbols.iter().find(|s| s.kind == SymbolKind::Module);
        assert!(entity_sym.is_some(), "entity should become a Module symbol");
        let entity_sym = entity_sym.unwrap();
        assert_eq!(entity_sym.name, "adder");
    }

    #[test]
    fn test_ports_to_symbols() {
        let parser = VhdlParser::new();
        let collector = VhdlSymbolCollector::new();

        let result = parser.parse(ENTITY_VHDL);
        let symbols = collector.collect(&result, FILE_URI);

        let entity_sym = symbols
            .iter()
            .find(|s| s.kind == SymbolKind::Module)
            .unwrap();

        // 3 端口 + 1 泛型 = 4 子符号
        let port_children: Vec<_> = entity_sym
            .children
            .iter()
            .filter(|s| s.kind == SymbolKind::Port)
            .collect();
        assert_eq!(port_children.len(), 3, "should have 3 port children");
        let param_children: Vec<_> = entity_sym
            .children
            .iter()
            .filter(|s| s.kind == SymbolKind::Parameter)
            .collect();
        assert_eq!(
            param_children.len(),
            1,
            "should have 1 generic/parameter child"
        );
    }

    #[test]
    fn test_signals_to_symbols() {
        let parser = VhdlParser::new();
        let collector = VhdlSymbolCollector::new();

        let result = parser.parse(ARCH_VHDL);
        let symbols = collector.collect(&result, FILE_URI);

        let arch_sym = symbols.iter().find(|s| s.kind == SymbolKind::Namespace);
        assert!(
            arch_sym.is_some(),
            "architecture should become Namespace symbol"
        );
        let arch_sym = arch_sym.unwrap();

        let signal_children: Vec<_> = arch_sym
            .children
            .iter()
            .filter(|s| s.kind == SymbolKind::Variable)
            .collect();
        assert_eq!(signal_children.len(), 2, "should have 2 signal children");
        assert_eq!(signal_children[0].name, "carry");
    }

    #[test]
    fn test_process_to_function_symbol() {
        let parser = VhdlParser::new();
        let collector = VhdlSymbolCollector::new();

        let result = parser.parse(ARCH_VHDL);
        let symbols = collector.collect(&result, FILE_URI);

        let arch_sym = symbols
            .iter()
            .find(|s| s.kind == SymbolKind::Namespace)
            .unwrap();
        let proc_children: Vec<_> = arch_sym
            .children
            .iter()
            .filter(|s| s.kind == SymbolKind::Function)
            .collect();
        assert_eq!(
            proc_children.len(),
            1,
            "should have 1 process as Function symbol"
        );
        // 有标签时使用标签名
        assert_eq!(proc_children[0].name, "main_proc");
    }

    #[test]
    fn test_package_to_symbol() {
        let vhdl = r#"
package util_pkg is
    type state_t is (IDLE, DONE);
end package util_pkg;
"#;
        let parser = VhdlParser::new();
        let collector = VhdlSymbolCollector::new();

        let result = parser.parse(vhdl);
        let symbols = collector.collect(&result, FILE_URI);

        let pkg_sym = symbols.iter().find(|s| s.kind == SymbolKind::Package);
        assert!(pkg_sym.is_some(), "package should become Package symbol");
        assert_eq!(pkg_sym.unwrap().name, "util_pkg");
    }
}
