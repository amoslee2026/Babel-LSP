//! Cadence Innovus 物理设计工具命令数据库（~20 条关键命令）

use crate::eda_db::{CommandInfo, CommandParam, ParamKind};

pub fn cadence_innovus_commands() -> Vec<CommandInfo> {
    vec![
        cmd(
            "read_design",
            "读取设计（网表 + 约束）",
            "Read",
            vec![
                param("netlist", ParamKind::File, true, "网表文件路径"),
                flag("lef", false, "LEF 文件列表"),
                flag("sdc", false, "SDC 约束文件"),
            ],
        ),
        cmd(
            "read_lef",
            "读取 LEF 物理库文件",
            "Read",
            vec![param("file", ParamKind::File, true, ".lef 文件路径")],
        ),
        cmd(
            "read_def",
            "读取 DEF 布局文件",
            "Read",
            vec![param("file", ParamKind::File, true, ".def 文件路径")],
        ),
        cmd(
            "read_sdc",
            "读取 SDC 时序约束",
            "Read",
            vec![param("file", ParamKind::File, true, ".sdc 文件路径")],
        ),
        cmd(
            "init_design",
            "初始化设计（加载 LEF/DEF/SDC）",
            "Setup",
            vec![
                flag("top_cell", true, "顶层 cell 名"),
                flag("lef_file", false, "LEF 文件列表"),
                flag("def_file", false, "DEF 文件"),
            ],
        ),
        cmd(
            "floorplan_design",
            "执行 Floorplan 设计",
            "Floorplan",
            vec![
                flag("core_utilization", false, "核心利用率（0~1）"),
                flag("core_aspect_ratio", false, "宽高比"),
                flag("core_to_boundary_distance", false, "核心到边界距离"),
            ],
        ),
        cmd(
            "add_power_rings",
            "添加电源环",
            "PowerPlan",
            vec![
                flag("nets", true, "电源/地线网络名列表"),
                flag("width", false, "金属宽度（um）"),
            ],
        ),
        cmd(
            "add_power_stripes",
            "添加电源条",
            "PowerPlan",
            vec![
                flag("nets", true, "电源/地线网络名"),
                flag("width", false, "条宽（um）"),
                flag("pitch", false, "间距（um）"),
            ],
        ),
        cmd(
            "place_design",
            "执行布局",
            "Place",
            vec![flag("concurrent_macros", false, "同时布局宏单元")],
        ),
        cmd(
            "opt_design",
            "物理优化",
            "Optimize",
            vec![
                flag("pre_cts", false, "CTS 前优化"),
                flag("post_cts", false, "CTS 后优化"),
                flag("post_route", false, "布线后优化"),
            ],
        ),
        cmd(
            "ccopt_design",
            "时钟树综合及优化（CCOpt）",
            "ClockTree",
            vec![flag("cts_effort", false, "努力级别（low/medium/high）")],
        ),
        cmd(
            "route_design",
            "执行布线",
            "Route",
            vec![flag(
                "concurrent_minimize_via_count_effort",
                false,
                "最小化过孔努力",
            )],
        ),
        cmd(
            "report_timing",
            "生成时序报告",
            "Report",
            vec![
                flag("max_paths", false, "最大路径数"),
                flag("late", false, "晚到路径（setup）"),
                flag("early", false, "早到路径（hold）"),
                flag("file", false, "输出文件"),
            ],
        ),
        cmd(
            "report_area",
            "生成面积报告",
            "Report",
            vec![flag("file", false, "输出文件")],
        ),
        cmd(
            "report_power",
            "生成功耗报告",
            "Report",
            vec![flag("file", false, "输出文件")],
        ),
        cmd(
            "write_db",
            "保存 Innovus 工程数据库",
            "Write",
            vec![param("file", ParamKind::File, true, "输出 .enc 数据库路径")],
        ),
        cmd(
            "write_netlist",
            "写出最终网表",
            "Write",
            vec![
                param("file", ParamKind::File, true, "输出网表文件路径"),
                flag("top_module_only", false, "仅写出顶层模块"),
            ],
        ),
        cmd(
            "write_def",
            "写出 DEF 布局文件",
            "Write",
            vec![param("file", ParamKind::File, true, "输出 .def 文件路径")],
        ),
        cmd(
            "write_gds",
            "写出 GDS II 文件（流片）",
            "Write",
            vec![
                param("file", ParamKind::File, true, "输出 .gds 文件路径"),
                flag("map", false, "层映射文件"),
            ],
        ),
        cmd(
            "verify_drc",
            "执行 DRC 物理验证",
            "Verify",
            vec![flag("report", false, "DRC 报告输出路径")],
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
