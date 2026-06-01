//! Intel Quartus Prime 命令数据库（~20 条关键命令）

use crate::eda_db::{CommandInfo, CommandParam, ParamKind};

pub fn quartus_commands() -> Vec<CommandInfo> {
    vec![
        cmd(
            "project_new",
            "创建新的 Quartus 工程",
            "Project",
            vec![
                param("name", ParamKind::String, true, "工程名称"),
                flag("revision", false, "Revision 名称"),
                flag("overwrite", false, "覆盖已有工程"),
            ],
        ),
        cmd(
            "project_open",
            "打开已有 Quartus 工程",
            "Project",
            vec![param("name", ParamKind::String, true, "工程名称或路径")],
        ),
        cmd("project_close", "关闭当前工程", "Project", vec![]),
        cmd(
            "set_global_assignment",
            "设置全局工程属性",
            "Settings",
            vec![
                flag("name", true, "属性名（如 FAMILY、DEVICE）"),
                flag("value", true, "属性值"),
                flag("section_id", false, "段 ID"),
            ],
        ),
        cmd(
            "set_instance_assignment",
            "设置实例属性",
            "Settings",
            vec![
                flag("name", true, "属性名"),
                flag("value", true, "属性值"),
                flag("to", false, "目标实例或端口"),
                flag("from", false, "起始端点（时序）"),
                flag("section_id", false, "段 ID"),
            ],
        ),
        cmd(
            "set_location_assignment",
            "设置 IO 位置约束",
            "Settings",
            vec![
                param(
                    "location",
                    ParamKind::String,
                    true,
                    "引脚位置（如 PIN_AA1）",
                ),
                flag("to", true, "目标端口名"),
            ],
        ),
        cmd(
            "execute_module",
            "执行 Quartus 编译流程模块",
            "Flow",
            vec![
                flag("module", true, "模块名（synthesis/fit/asm/sta 等）"),
                flag("args", false, "附加参数"),
            ],
        ),
        cmd(
            "execute_flow",
            "执行完整编译流程",
            "Flow",
            vec![param(
                "flow",
                ParamKind::Enum(vec!["compile".into(), "implement".into()]),
                true,
                "流程名称",
            )],
        ),
        cmd(
            "load_package",
            "加载 Quartus TCL 包",
            "Package",
            vec![
                param("pkg", ParamKind::String, true, "包名（如 flow、project）"),
                flag("version", false, "版本号"),
            ],
        ),
        cmd(
            "create_timing_netlist",
            "创建时序网表",
            "Timing",
            vec![flag("model", false, "时序模型（slow/fast）")],
        ),
        cmd(
            "read_sdc",
            "读取 SDC 约束文件",
            "Timing",
            vec![param("file", ParamKind::File, true, ".sdc 文件路径")],
        ),
        cmd(
            "write_sdc",
            "写出 SDC 约束文件",
            "Timing",
            vec![param("file", ParamKind::File, true, "输出 .sdc 文件")],
        ),
        cmd(
            "report_timing",
            "生成时序报告",
            "Report",
            vec![
                flag("from", false, "起始节点"),
                flag("to", false, "结束节点"),
                flag("nworst", false, "最差路径数"),
                flag("detail", false, "详细程度（summary/path_only/full_path）"),
                flag("file", false, "输出文件"),
            ],
        ),
        cmd(
            "report_fmax_summary",
            "报告最高工作频率摘要",
            "Report",
            vec![flag("file", false, "输出文件")],
        ),
        cmd("check_timing", "检查时序约束", "Timing", vec![]),
        cmd(
            "get_timing_paths",
            "获取时序路径",
            "Timing",
            vec![
                flag("from", false, "起始"),
                flag("to", false, "结束"),
                flag("nworst", false, "路径数量"),
            ],
        ),
        cmd(
            "get_nodes",
            "获取设计节点",
            "Query",
            vec![
                param("pattern", ParamKind::String, false, "名称模式"),
                flag("type", false, "节点类型（port/reg/cell）"),
            ],
        ),
        cmd(
            "get_ports",
            "获取顶层端口",
            "Query",
            vec![param("pattern", ParamKind::String, false, "名称模式")],
        ),
        cmd(
            "get_pins",
            "获取元件引脚",
            "Query",
            vec![param("pattern", ParamKind::String, false, "名称模式")],
        ),
        cmd(
            "get_clocks",
            "获取时钟列表",
            "Timing",
            vec![param("pattern", ParamKind::String, false, "时钟名模式")],
        ),
    ]
}

fn cmd(name: &str, description: &str, category: &str, params: Vec<CommandParam>) -> CommandInfo {
    CommandInfo {
        name: name.to_string(),
        description: description.to_string(),
        params,
        category: category.to_string(),
    }
}

fn param(name: &str, kind: ParamKind, required: bool, description: &str) -> CommandParam {
    CommandParam {
        name: name.to_string(),
        kind,
        required,
        description: description.to_string(),
    }
}

fn flag(name: &str, required: bool, description: &str) -> CommandParam {
    CommandParam {
        name: format!("-{name}"),
        kind: ParamKind::Flag,
        required,
        description: description.to_string(),
    }
}
