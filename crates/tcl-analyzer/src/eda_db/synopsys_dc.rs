//! Synopsys Design Compiler 命令数据库（~25 条关键命令）

use crate::eda_db::{CommandInfo, CommandParam, ParamKind};

pub fn synopsys_dc_commands() -> Vec<CommandInfo> {
    vec![
        cmd(
            "read_hdl",
            "读取 HDL 源文件",
            "Read",
            vec![
                param("files", ParamKind::String, true, "HDL 文件列表"),
                flag("verilog", false, "指定 Verilog 格式"),
                flag("vhdl", false, "指定 VHDL 格式"),
                flag("sv", false, "指定 SystemVerilog 格式"),
                flag("define", false, "Verilog 宏定义"),
            ],
        ),
        cmd(
            "analyze",
            "分析 HDL 文件",
            "Read",
            vec![
                flag("format", true, "格式（verilog/vhdl/sverilog）"),
                param("files", ParamKind::String, true, "文件列表"),
                flag("define", false, "宏定义"),
                flag("work", false, "工作库名称"),
            ],
        ),
        cmd(
            "elaborate",
            "展开并链接设计",
            "Read",
            vec![
                param("design", ParamKind::String, true, "顶层设计名"),
                flag("parameters", false, "参数覆盖列表"),
                flag("lib", false, "单元库"),
            ],
        ),
        cmd(
            "compile",
            "执行综合编译",
            "Synthesis",
            vec![
                flag("map_effort", false, "映射努力级别（low/medium/high）"),
                flag("incremental_mapping", false, "启用增量映射"),
                flag("no_design_rule", false, "不应用设计规则"),
                flag("exact_map", false, "精确映射"),
            ],
        ),
        cmd(
            "compile_ultra",
            "高努力综合编译",
            "Synthesis",
            vec![
                flag("no_autoungroup", false, "禁止自动解组"),
                flag("no_boundary_optimization", false, "禁止边界优化"),
                flag("retime", false, "启用 retiming"),
                flag("incremental", false, "增量模式"),
                flag("timing", false, "时序驱动"),
            ],
        ),
        cmd(
            "write",
            "写出综合结果网表",
            "Write",
            vec![
                flag("format", true, "格式（verilog/ddc/db 等）"),
                flag("output", true, "输出文件路径"),
                flag("hierarchy", false, "保留层次"),
            ],
        ),
        cmd(
            "write_sdc",
            "写出时序约束文件",
            "Write",
            vec![
                param("file", ParamKind::File, true, "输出 .sdc 文件"),
                flag("version", false, "SDC 版本"),
            ],
        ),
        cmd(
            "write_ddc",
            "写出 DDC 格式设计数据",
            "Write",
            vec![param("file", ParamKind::File, true, "输出 .ddc 文件")],
        ),
        cmd(
            "read_sdc",
            "读取 SDC 约束文件",
            "Read",
            vec![param("file", ParamKind::File, true, ".sdc 文件路径")],
        ),
        cmd(
            "create_clock",
            "创建时钟约束",
            "Timing",
            vec![
                flag("period", true, "时钟周期（ns）"),
                flag("name", false, "时钟名称"),
                flag("waveform", false, "波形 {上升 下降}"),
                param("source", ParamKind::String, false, "时钟源端口"),
            ],
        ),
        cmd(
            "set_dont_touch",
            "禁止对对象进行优化",
            "Constraint",
            vec![param("objects", ParamKind::String, true, "目标对象")],
        ),
        cmd(
            "set_dont_use",
            "禁止使用特定单元",
            "Constraint",
            vec![param("cells", ParamKind::String, true, "单元列表")],
        ),
        cmd(
            "set_max_area",
            "设置面积约束",
            "Constraint",
            vec![param("area", ParamKind::Int, true, "最大面积（等效门数）")],
        ),
        cmd(
            "set_max_fanout",
            "设置最大扇出约束",
            "Constraint",
            vec![
                param("fanout", ParamKind::Int, true, "最大扇出"),
                param("objects", ParamKind::String, true, "目标对象"),
            ],
        ),
        cmd(
            "set_input_delay",
            "设置输入延迟",
            "Timing",
            vec![
                param("delay", ParamKind::String, true, "延迟值（ns）"),
                flag("clock", true, "参考时钟"),
                param("ports", ParamKind::String, true, "目标输入端口"),
            ],
        ),
        cmd(
            "set_output_delay",
            "设置输出延迟",
            "Timing",
            vec![
                param("delay", ParamKind::String, true, "延迟值（ns）"),
                flag("clock", true, "参考时钟"),
                param("ports", ParamKind::String, true, "目标输出端口"),
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
            ],
        ),
        cmd(
            "report_area",
            "生成面积报告",
            "Report",
            vec![
                flag("hierarchy", false, "按层次报告"),
                flag("output", false, "输出文件"),
            ],
        ),
        cmd(
            "report_timing",
            "生成时序报告",
            "Report",
            vec![
                flag("max_paths", false, "最大路径数"),
                flag("delay_type", false, "延迟类型（max/min）"),
                flag("path_type", false, "路径类型（full/short）"),
            ],
        ),
        cmd(
            "report_constraints",
            "报告约束违例",
            "Report",
            vec![flag("all_violators", false, "只报告违例")],
        ),
        cmd("report_qor", "生成质量报告 QoR", "Report", vec![]),
        cmd("check_timing", "检查时序约束完整性", "Report", vec![]),
        cmd(
            "get_cells",
            "获取设计中的 cell",
            "Query",
            vec![
                param("pattern", ParamKind::String, false, "名称模式"),
                flag("hierarchical", false, "递归"),
                flag("filter", false, "过滤条件"),
            ],
        ),
        cmd(
            "link",
            "链接当前设计",
            "Read",
            vec![flag("lib", false, "单元库列表")],
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
