//! Vivado TCL 命令数据库（~30 条关键命令）

use crate::eda_db::{CommandInfo, CommandParam, ParamKind};

pub fn vivado_commands() -> Vec<CommandInfo> {
    vec![
        cmd(
            "create_project",
            "创建新的 Vivado 工程",
            "Project",
            vec![
                param("name", ParamKind::String, true, "工程名称"),
                param("dir", ParamKind::String, true, "工程目录"),
                flag("part", false, "目标器件（如 xc7a35tcpg236-1）"),
                flag("force", false, "若工程已存在则覆盖"),
            ],
        ),
        cmd(
            "open_project",
            "打开已有 Vivado 工程",
            "Project",
            vec![param("file", ParamKind::File, true, ".xpr 文件路径")],
        ),
        cmd("close_project", "关闭当前工程", "Project", vec![]),
        cmd(
            "add_files",
            "向工程添加源文件或约束文件",
            "Project",
            vec![
                param("files", ParamKind::String, true, "文件列表（支持通配符）"),
                flag("norecurse", false, "不递归搜索目录"),
                flag("fileset", false, "指定 fileset 名称"),
            ],
        ),
        cmd(
            "set_property",
            "设置对象属性",
            "Project",
            vec![
                param("name", ParamKind::String, true, "属性名"),
                param("value", ParamKind::String, true, "属性值"),
                param("objects", ParamKind::String, true, "目标对象"),
            ],
        ),
        cmd(
            "get_property",
            "获取对象属性",
            "Project",
            vec![
                param("name", ParamKind::String, true, "属性名"),
                param("objects", ParamKind::String, true, "目标对象"),
            ],
        ),
        cmd(
            "synth_design",
            "运行综合",
            "Synthesis",
            vec![
                flag("top", true, "顶层模块名"),
                flag("part", false, "目标器件"),
                flag(
                    "directive",
                    false,
                    "综合策略（Default/AreaOptimized_high 等）",
                ),
                flag("flatten_hierarchy", false, "展平层次（none/full/rebuilt）"),
                flag("include_dirs", false, "包含目录列表"),
            ],
        ),
        cmd(
            "opt_design",
            "运行综合后优化",
            "Implementation",
            vec![
                flag("directive", false, "优化策略"),
                flag("retarget", false, "启用 retarget 优化"),
            ],
        ),
        cmd(
            "place_design",
            "运行布局",
            "Implementation",
            vec![flag(
                "directive",
                false,
                "布局策略（Default/Extra_Effort_High 等）",
            )],
        ),
        cmd(
            "phys_opt_design",
            "物理优化",
            "Implementation",
            vec![flag("directive", false, "物理优化策略")],
        ),
        cmd(
            "route_design",
            "运行布线",
            "Implementation",
            vec![flag("directive", false, "布线策略")],
        ),
        cmd(
            "write_bitstream",
            "生成比特流文件",
            "Bitstream",
            vec![
                param("file", ParamKind::File, true, "输出 .bit 文件路径"),
                flag("force", false, "强制覆盖已有文件"),
            ],
        ),
        cmd(
            "get_cells",
            "获取设计中的 cell 对象列表",
            "Query",
            vec![
                param(
                    "pattern",
                    ParamKind::String,
                    false,
                    "名称模式（支持通配符）",
                ),
                flag("hierarchical", false, "递归搜索层次"),
                flag("filter", false, "属性过滤表达式"),
            ],
        ),
        cmd(
            "get_nets",
            "获取 net 对象列表",
            "Query",
            vec![
                param("pattern", ParamKind::String, false, "名称模式"),
                flag("hierarchical", false, "递归搜索"),
                flag("filter", false, "属性过滤"),
            ],
        ),
        cmd(
            "get_ports",
            "获取顶层端口列表",
            "Query",
            vec![
                param("pattern", ParamKind::String, false, "名称模式"),
                flag("filter", false, "属性过滤"),
            ],
        ),
        cmd(
            "get_pins",
            "获取 pin 对象列表",
            "Query",
            vec![
                param("pattern", ParamKind::String, false, "名称模式（如 inst/D）"),
                flag("hierarchical", false, "递归搜索"),
            ],
        ),
        cmd(
            "get_clocks",
            "获取时钟对象列表",
            "Timing",
            vec![param("pattern", ParamKind::String, false, "时钟名模式")],
        ),
        cmd(
            "create_clock",
            "创建时钟约束",
            "Timing",
            vec![
                flag("period", true, "时钟周期（ns）"),
                flag("name", false, "时钟名称"),
                flag("waveform", false, "波形定义 {上升沿 下降沿}"),
                param("source", ParamKind::String, false, "时钟源端口/pin"),
            ],
        ),
        cmd(
            "create_generated_clock",
            "创建派生时钟约束",
            "Timing",
            vec![
                flag("name", true, "时钟名称"),
                flag("source", true, "源时钟"),
                flag("divide_by", false, "分频比"),
                flag("multiply_by", false, "倍频比"),
            ],
        ),
        cmd(
            "set_false_path",
            "设置虚假路径",
            "Timing",
            vec![
                flag("from", false, "起始端点"),
                flag("to", false, "终止端点"),
                flag("through", false, "途经节点"),
            ],
        ),
        cmd(
            "set_multicycle_path",
            "设置多周期路径",
            "Timing",
            vec![
                param("cycles", ParamKind::Int, true, "周期数"),
                flag("from", false, "起始端点"),
                flag("to", false, "终止端点"),
                flag("setup", false, "影响 setup 分析"),
                flag("hold", false, "影响 hold 分析"),
            ],
        ),
        cmd(
            "set_input_delay",
            "设置输入延迟约束",
            "Timing",
            vec![
                flag("clock", true, "参考时钟"),
                param("delay", ParamKind::String, true, "延迟值（ns）"),
                param("port", ParamKind::String, true, "目标端口"),
            ],
        ),
        cmd(
            "set_output_delay",
            "设置输出延迟约束",
            "Timing",
            vec![
                flag("clock", true, "参考时钟"),
                param("delay", ParamKind::String, true, "延迟值（ns）"),
                param("port", ParamKind::String, true, "目标端口"),
            ],
        ),
        cmd(
            "report_timing",
            "生成时序报告",
            "Report",
            vec![
                flag("max_paths", false, "最大报告路径数"),
                flag("nworst", false, "最差路径数"),
                flag("delay_type", false, "延迟类型（min/max）"),
                flag("file", false, "输出文件路径"),
            ],
        ),
        cmd(
            "report_timing_summary",
            "生成时序摘要报告",
            "Report",
            vec![
                flag("max_paths", false, "最大路径数"),
                flag("file", false, "输出文件"),
                flag("warn_on_violation", false, "违例时警告"),
            ],
        ),
        cmd(
            "report_utilization",
            "生成资源利用率报告",
            "Report",
            vec![
                flag("hierarchical", false, "按层次显示"),
                flag("file", false, "输出文件路径"),
            ],
        ),
        cmd(
            "report_power",
            "生成功耗报告",
            "Report",
            vec![
                flag("file", false, "输出文件路径"),
                flag("format", false, "格式（xml/text）"),
            ],
        ),
        cmd(
            "write_xdc",
            "导出 XDC 约束文件",
            "Constraints",
            vec![
                param("file", ParamKind::File, true, "输出 .xdc 文件路径"),
                flag("force", false, "强制覆盖"),
            ],
        ),
        cmd(
            "read_xdc",
            "读取 XDC 约束文件",
            "Constraints",
            vec![param("file", ParamKind::File, true, "XDC 文件路径")],
        ),
        cmd(
            "write_checkpoint",
            "保存设计检查点",
            "Design",
            vec![
                param("file", ParamKind::File, true, "输出 .dcp 文件路径"),
                flag("force", false, "强制覆盖"),
            ],
        ),
        cmd(
            "read_checkpoint",
            "读取设计检查点",
            "Design",
            vec![param("file", ParamKind::File, true, ".dcp 文件路径")],
        ),
        cmd(
            "launch_runs",
            "启动 Implementation 或 Synthesis 运行",
            "Flow",
            vec![
                param("runs", ParamKind::String, true, "运行名称（如 impl_1）"),
                flag("jobs", false, "并行任务数"),
                flag("wait_on_run", false, "等待运行完成"),
            ],
        ),
        cmd(
            "wait_on_run",
            "等待指定运行完成",
            "Flow",
            vec![param("run", ParamKind::String, true, "运行名称")],
        ),
    ]
}

// ── 构建工具函数 ──────────────────────────────────────────

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
