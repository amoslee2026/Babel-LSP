//! Synopsys PrimeTime 命令数据库（~20 条关键命令）

use crate::eda_db::{CommandInfo, CommandParam, ParamKind};

pub fn synopsys_pt_commands() -> Vec<CommandInfo> {
    vec![
        cmd(
            "read_verilog",
            "读取 Verilog 网表",
            "Read",
            vec![
                param("files", ParamKind::String, true, "Verilog 文件列表"),
                flag("work", false, "工作库"),
            ],
        ),
        cmd(
            "read_db",
            "读取 .db 格式单元库",
            "Read",
            vec![param("files", ParamKind::String, true, ".db 文件列表")],
        ),
        cmd(
            "link_design",
            "链接顶层设计",
            "Design",
            vec![param("design", ParamKind::String, true, "顶层设计名")],
        ),
        cmd(
            "current_design",
            "设置当前设计",
            "Design",
            vec![param("design", ParamKind::String, true, "设计名")],
        ),
        cmd(
            "read_sdc",
            "读取 SDC 约束文件",
            "Constraint",
            vec![param("file", ParamKind::File, true, ".sdc 文件路径")],
        ),
        cmd(
            "create_clock",
            "创建时钟约束",
            "Timing",
            vec![
                flag("period", true, "时钟周期（ns）"),
                flag("name", false, "时钟名称"),
                flag("waveform", false, "波形"),
                param("source", ParamKind::String, false, "时钟源"),
            ],
        ),
        cmd(
            "create_generated_clock",
            "创建派生时钟",
            "Timing",
            vec![
                flag("name", true, "时钟名"),
                flag("source", true, "源时钟"),
                flag("divide_by", false, "分频比"),
                flag("multiply_by", false, "倍频比"),
            ],
        ),
        cmd(
            "set_input_delay",
            "设置输入延迟",
            "Timing",
            vec![
                param("delay", ParamKind::String, true, "延迟值（ns）"),
                flag("clock", true, "参考时钟"),
                param("ports", ParamKind::String, true, "目标端口"),
            ],
        ),
        cmd(
            "set_output_delay",
            "设置输出延迟",
            "Timing",
            vec![
                param("delay", ParamKind::String, true, "延迟值（ns）"),
                flag("clock", true, "参考时钟"),
                param("ports", ParamKind::String, true, "目标端口"),
            ],
        ),
        cmd(
            "set_false_path",
            "设置虚假路径",
            "Timing",
            vec![
                flag("from", false, "起始"),
                flag("to", false, "结束"),
                flag("through", false, "途经"),
            ],
        ),
        cmd(
            "set_multicycle_path",
            "设置多周期路径",
            "Timing",
            vec![
                param("cycles", ParamKind::Int, true, "周期数"),
                flag("from", false, "起始"),
                flag("to", false, "结束"),
                flag("setup", false, "影响 setup"),
                flag("hold", false, "影响 hold"),
            ],
        ),
        cmd(
            "update_timing",
            "更新时序数据库",
            "Analysis",
            vec![flag("full", false, "完整更新")],
        ),
        cmd(
            "report_timing",
            "生成时序报告",
            "Report",
            vec![
                flag("max_paths", false, "最大路径数"),
                flag("delay_type", false, "延迟类型（max/min）"),
                flag("path_type", false, "路径类型（full/summary）"),
                flag("file", false, "输出文件"),
                flag("nosplit", false, "不分页"),
            ],
        ),
        cmd(
            "report_timing_requirements",
            "报告时序约束需求",
            "Report",
            vec![],
        ),
        cmd(
            "report_constraint",
            "报告约束违例",
            "Report",
            vec![
                flag("all_violators", false, "仅显示违例"),
                flag("file", false, "输出文件"),
            ],
        ),
        cmd(
            "report_power",
            "生成功耗报告",
            "Report",
            vec![
                flag("analysis_effort", false, "分析努力（low/medium/high）"),
                flag("hierarchy", false, "按层次报告"),
                flag("file", false, "输出文件"),
            ],
        ),
        cmd(
            "report_clock_timing",
            "报告时钟时序",
            "Report",
            vec![flag("type", false, "报告类型（skew/latency/transition）")],
        ),
        cmd("check_timing", "检查时序约束完整性", "Analysis", vec![]),
        cmd(
            "get_cells",
            "获取 cell 列表",
            "Query",
            vec![
                param("pattern", ParamKind::String, false, "名称模式"),
                flag("hierarchical", false, "递归"),
                flag("filter", false, "过滤"),
            ],
        ),
        cmd(
            "get_nets",
            "获取 net 列表",
            "Query",
            vec![
                param("pattern", ParamKind::String, false, "名称模式"),
                flag("hierarchical", false, "递归"),
            ],
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
