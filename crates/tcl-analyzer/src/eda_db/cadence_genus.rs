//! Cadence Genus 综合工具命令数据库（~20 条关键命令）

use crate::eda_db::{CommandInfo, CommandParam, ParamKind};

pub fn cadence_genus_commands() -> Vec<CommandInfo> {
    vec![
        cmd(
            "read_hdl",
            "读取 HDL 源文件",
            "Read",
            vec![
                param("files", ParamKind::String, true, "HDL 文件列表"),
                flag("language", false, "语言（verilog/vhdl/sverilog）"),
                flag("define", false, "宏定义"),
                flag("include", false, "包含目录"),
            ],
        ),
        cmd(
            "read_libs",
            "读取单元库",
            "Read",
            vec![param(
                "files",
                ParamKind::String,
                true,
                ".lib / .db 文件列表",
            )],
        ),
        cmd(
            "elaborate",
            "展开设计",
            "Read",
            vec![
                param("design", ParamKind::String, true, "顶层设计名"),
                flag("parameters", false, "参数覆盖"),
            ],
        ),
        cmd(
            "init_design",
            "初始化设计环境",
            "Setup",
            vec![
                flag("top", true, "顶层模块名"),
                flag("auto_ungroup", false, "自动解组"),
            ],
        ),
        cmd(
            "set_db",
            "设置 Genus 数据库属性",
            "Setup",
            vec![
                param(
                    "attribute",
                    ParamKind::String,
                    true,
                    "属性名（如 syn_generic_effort）",
                ),
                param("value", ParamKind::String, true, "属性值"),
            ],
        ),
        cmd(
            "get_db",
            "获取 Genus 数据库属性",
            "Setup",
            vec![param("attribute", ParamKind::String, true, "属性名")],
        ),
        cmd(
            "synthesize_to_generic",
            "综合到通用门（逻辑综合第一阶段）",
            "Synthesis",
            vec![flag("effort", false, "努力级别（low/medium/high）")],
        ),
        cmd(
            "synthesize_to_mapped",
            "映射到工艺库（逻辑综合第二阶段）",
            "Synthesis",
            vec![
                flag("effort", false, "努力级别"),
                flag("incremental", false, "增量模式"),
            ],
        ),
        cmd(
            "synthesize",
            "执行完整综合流程",
            "Synthesis",
            vec![flag("effort", false, "努力级别（low/medium/high）")],
        ),
        cmd(
            "create_clock",
            "创建时钟约束",
            "Timing",
            vec![
                flag("period", true, "时钟周期（ns）"),
                flag("name", false, "时钟名称"),
                flag("waveform", false, "波形定义"),
                param("source", ParamKind::String, false, "时钟源"),
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
            "report_timing",
            "生成时序报告",
            "Report",
            vec![
                flag("max_paths", false, "最大路径数"),
                flag("slack_lesser_than", false, "显示 slack 小于该值的路径"),
                flag("output_format", false, "输出格式"),
            ],
        ),
        cmd(
            "report_area",
            "生成面积报告",
            "Report",
            vec![flag("hierarchy", false, "按层次报告")],
        ),
        cmd("report_qor", "生成 QoR 摘要报告", "Report", vec![]),
        cmd(
            "report_power",
            "生成功耗报告",
            "Report",
            vec![flag("hierarchy", false, "按层次报告")],
        ),
        cmd(
            "write_hdl",
            "写出网表（Verilog/VHDL）",
            "Write",
            vec![
                param("file", ParamKind::File, false, "输出文件路径"),
                flag("format", false, "格式（verilog/vhdl）"),
            ],
        ),
        cmd(
            "write_sdc",
            "写出 SDC 约束文件",
            "Write",
            vec![param("file", ParamKind::File, true, "输出 .sdc 文件")],
        ),
        cmd(
            "write_db",
            "保存 Genus 数据库",
            "Write",
            vec![
                param("file", ParamKind::File, true, "输出 .db 文件"),
                flag("all_root_cells", false, "包含所有根 cell"),
            ],
        ),
        cmd(
            "check_timing_intent",
            "检查时序约束意图",
            "Analysis",
            vec![],
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
