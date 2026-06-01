//! EDA 命令数据库（统一入口）

pub mod cadence_genus;
pub mod cadence_innovus;
pub mod quartus;
pub mod synopsys_dc;
pub mod synopsys_pt;
pub mod vivado;

use std::collections::HashMap;

/// 参数类型
#[derive(Debug, Clone, PartialEq)]
pub enum ParamKind {
    /// 开关标志（如 -force）
    Flag,
    /// 字符串值
    String,
    /// 整数值
    Int,
    /// 枚举，携带合法值列表
    Enum(Vec<std::string::String>),
    /// 文件路径
    File,
}

/// 命令参数描述
#[derive(Debug, Clone)]
pub struct CommandParam {
    pub name: std::string::String,
    pub kind: ParamKind,
    pub required: bool,
    pub description: std::string::String,
}

/// 命令信息
#[derive(Debug, Clone)]
pub struct CommandInfo {
    pub name: std::string::String,
    pub description: std::string::String,
    pub params: Vec<CommandParam>,
    pub category: std::string::String,
}

/// 支持的 EDA 工具
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EdaTool {
    Vivado,
    Quartus,
    SynopsysDc,
    SynopsysPt,
    CadenceGenus,
    CadenceInnovus,
}

impl EdaTool {
    /// 返回用于在 TCL 中探测工具类型的典型 package 名
    pub fn package_hints(&self) -> &[&str] {
        match self {
            EdaTool::Vivado => &["Vivado"],
            EdaTool::Quartus => &["quartus", "quartus::project", "quartus::flow"],
            EdaTool::SynopsysDc => &["Design_Compiler", "dc"],
            EdaTool::SynopsysPt => &["PrimeTime", "pt"],
            EdaTool::CadenceGenus => &["genus"],
            EdaTool::CadenceInnovus => &["innovus"],
        }
    }
}

/// EDA 工具命令数据库
pub struct EdaDatabase {
    commands: HashMap<std::string::String, CommandInfo>,
    pub tool: EdaTool,
}

impl EdaDatabase {
    /// 根据工具类型加载内置命令数据
    pub fn new(tool: EdaTool) -> Self {
        let cmd_list = match tool {
            EdaTool::Vivado => vivado::vivado_commands(),
            EdaTool::Quartus => quartus::quartus_commands(),
            EdaTool::SynopsysDc => synopsys_dc::synopsys_dc_commands(),
            EdaTool::SynopsysPt => synopsys_pt::synopsys_pt_commands(),
            EdaTool::CadenceGenus => cadence_genus::cadence_genus_commands(),
            EdaTool::CadenceInnovus => cadence_innovus::cadence_innovus_commands(),
        };

        let commands = cmd_list.into_iter().map(|c| (c.name.clone(), c)).collect();

        Self { commands, tool }
    }

    /// 精确查找命令
    pub fn lookup(&self, name: &str) -> Option<&CommandInfo> {
        self.commands.get(name)
    }

    /// 前缀补全（返回所有以 prefix 开头的命令）
    pub fn get_completions(&self, prefix: &str) -> Vec<&CommandInfo> {
        let mut results: Vec<&CommandInfo> = self
            .commands
            .values()
            .filter(|c| c.name.starts_with(prefix))
            .collect();
        results.sort_by(|a, b| a.name.cmp(&b.name));
        results
    }

    /// 返回所有命令（按名称排序）
    pub fn all_commands(&self) -> Vec<&CommandInfo> {
        let mut cmds: Vec<&CommandInfo> = self.commands.values().collect();
        cmds.sort_by(|a, b| a.name.cmp(&b.name));
        cmds
    }

    /// 命令总数
    pub fn command_count(&self) -> usize {
        self.commands.len()
    }

    /// 按分类获取命令
    pub fn commands_by_category(&self, category: &str) -> Vec<&CommandInfo> {
        let mut cmds: Vec<&CommandInfo> = self
            .commands
            .values()
            .filter(|c| c.category == category)
            .collect();
        cmds.sort_by(|a, b| a.name.cmp(&b.name));
        cmds
    }
}

/// 兼容旧存根的别名
pub type EdaDb = EdaDatabase;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vivado_command_count() {
        let db = EdaDatabase::new(EdaTool::Vivado);
        assert!(
            db.command_count() >= 20,
            "Vivado should have ≥20 commands, got {}",
            db.command_count()
        );
    }

    #[test]
    fn test_synopsys_dc_command_count() {
        let db = EdaDatabase::new(EdaTool::SynopsysDc);
        assert!(db.command_count() >= 20);
    }

    #[test]
    fn test_synopsys_pt_command_count() {
        let db = EdaDatabase::new(EdaTool::SynopsysPt);
        assert!(db.command_count() >= 20);
    }

    #[test]
    fn test_quartus_command_count() {
        let db = EdaDatabase::new(EdaTool::Quartus);
        assert!(db.command_count() >= 20);
    }

    #[test]
    fn test_cadence_genus_command_count() {
        let db = EdaDatabase::new(EdaTool::CadenceGenus);
        assert!(db.command_count() >= 20);
    }

    #[test]
    fn test_cadence_innovus_command_count() {
        let db = EdaDatabase::new(EdaTool::CadenceInnovus);
        assert!(db.command_count() >= 20);
    }

    #[test]
    fn test_lookup_existing() {
        let db = EdaDatabase::new(EdaTool::Vivado);
        let cmd = db.lookup("synth_design");
        assert!(cmd.is_some(), "synth_design should exist in Vivado DB");
        assert_eq!(cmd.unwrap().name, "synth_design");
    }

    #[test]
    fn test_lookup_nonexistent() {
        let db = EdaDatabase::new(EdaTool::Vivado);
        assert!(db.lookup("nonexistent_cmd_xyz").is_none());
    }

    #[test]
    fn test_get_completions_prefix() {
        let db = EdaDatabase::new(EdaTool::Vivado);
        let results = db.get_completions("report_");
        assert!(
            results.len() >= 2,
            "should find multiple report_ commands, got {}",
            results.len()
        );
        for r in &results {
            assert!(r.name.starts_with("report_"));
        }
    }

    #[test]
    fn test_get_completions_empty_prefix() {
        let db = EdaDatabase::new(EdaTool::Vivado);
        let results = db.get_completions("");
        assert_eq!(results.len(), db.command_count());
    }

    #[test]
    fn test_all_commands_sorted() {
        let db = EdaDatabase::new(EdaTool::Vivado);
        let cmds = db.all_commands();
        for window in cmds.windows(2) {
            assert!(
                window[0].name <= window[1].name,
                "commands should be sorted"
            );
        }
    }

    #[test]
    fn test_command_has_description() {
        let db = EdaDatabase::new(EdaTool::SynopsysDc);
        for cmd in db.all_commands() {
            assert!(
                !cmd.description.is_empty(),
                "command {} should have a description",
                cmd.name
            );
        }
    }

    #[test]
    fn test_commands_by_category() {
        let db = EdaDatabase::new(EdaTool::Vivado);
        let timing_cmds = db.commands_by_category("Timing");
        assert!(
            !timing_cmds.is_empty(),
            "Vivado should have Timing category commands"
        );
    }
}
