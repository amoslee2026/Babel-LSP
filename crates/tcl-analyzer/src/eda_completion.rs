//! EDA 工具命令补全（基于 EdaDatabase）

use crate::eda_db::{EdaDatabase, EdaTool};

/// 补全建议
#[derive(Debug, Clone)]
pub struct CompletionSuggestion {
    pub label: String,
    pub detail: String,
    pub documentation: Option<String>,
}

/// EDA 命令补全引擎
pub struct EdaCompletion {
    db: EdaDatabase,
}

impl EdaCompletion {
    pub fn new(tool: EdaTool) -> Self {
        Self {
            db: EdaDatabase::new(tool),
        }
    }

    /// 前缀补全，返回补全建议列表
    pub fn complete(&self, prefix: &str) -> Vec<CompletionSuggestion> {
        self.db
            .get_completions(prefix)
            .into_iter()
            .map(|cmd| {
                let required_params: Vec<String> = cmd
                    .params
                    .iter()
                    .filter(|p| p.required)
                    .map(|p| p.name.clone())
                    .collect();

                let detail = if required_params.is_empty() {
                    format!("[{}] {}", cmd.category, cmd.name)
                } else {
                    format!(
                        "[{}] {} {}",
                        cmd.category,
                        cmd.name,
                        required_params.join(" ")
                    )
                };

                CompletionSuggestion {
                    label: cmd.name.clone(),
                    detail,
                    documentation: Some(build_hover_markdown(cmd)),
                }
            })
            .collect()
    }

    /// 获取命令的 hover 文档（Markdown 格式）
    pub fn get_hover(&self, command: &str) -> Option<String> {
        self.db.lookup(command).map(build_hover_markdown)
    }

    /// 根据 TCL 文件内容检测所用的 EDA 工具类型
    pub fn detect_tool_from_context(content: &str) -> Option<EdaTool> {
        let tools = [
            EdaTool::Vivado,
            EdaTool::Quartus,
            EdaTool::SynopsysDc,
            EdaTool::SynopsysPt,
            EdaTool::CadenceGenus,
            EdaTool::CadenceInnovus,
        ];

        for tool in &tools {
            for hint in tool.package_hints() {
                // 匹配 `package require <hint>` 或 `load_package <hint>`
                if content.contains(&format!("package require {hint}"))
                    || content.contains(&format!("load_package {hint}"))
                {
                    return Some(*tool);
                }
            }
        }

        // 根据特征命令进行启发式检测
        if content.contains("synth_design") || content.contains("write_bitstream") {
            return Some(EdaTool::Vivado);
        }
        if content.contains("set_global_assignment") || content.contains("execute_module") {
            return Some(EdaTool::Quartus);
        }
        if content.contains("compile_ultra") || content.contains("write_ddc") {
            return Some(EdaTool::SynopsysDc);
        }
        if content.contains("update_timing") || content.contains("report_constraint") {
            return Some(EdaTool::SynopsysPt);
        }
        if content.contains("synthesize_to_mapped") || content.contains("synthesize_to_generic") {
            return Some(EdaTool::CadenceGenus);
        }
        if content.contains("ccopt_design") || content.contains("write_gds") {
            return Some(EdaTool::CadenceInnovus);
        }

        None
    }

    /// 返回数据库中的命令总数
    pub fn command_count(&self) -> usize {
        self.db.command_count()
    }
}

impl Default for EdaCompletion {
    fn default() -> Self {
        Self::new(EdaTool::Vivado)
    }
}

/// 为命令构建 Markdown hover 文档
fn build_hover_markdown(cmd: &crate::eda_db::CommandInfo) -> String {
    let mut md = String::new();
    md.push_str(&format!("## `{}`\n\n", cmd.name));
    md.push_str(&format!("**分类**: {}\n\n", cmd.category));
    md.push_str(&format!("{}\n\n", cmd.description));

    if !cmd.params.is_empty() {
        md.push_str("### 参数\n\n");
        md.push_str("| 参数 | 类型 | 必需 | 说明 |\n");
        md.push_str("|------|------|------|------|\n");
        for p in &cmd.params {
            let kind_str = match &p.kind {
                crate::eda_db::ParamKind::Flag => "flag".to_string(),
                crate::eda_db::ParamKind::String => "string".to_string(),
                crate::eda_db::ParamKind::Int => "int".to_string(),
                crate::eda_db::ParamKind::Enum(vals) => format!("enum({})", vals.join("|")),
                crate::eda_db::ParamKind::File => "file".to_string(),
            };
            let required = if p.required { "yes" } else { "no" };
            md.push_str(&format!(
                "| `{}` | {} | {} | {} |\n",
                p.name, kind_str, required, p.description
            ));
        }
    }

    md
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complete_prefix_vivado() {
        let comp = EdaCompletion::new(EdaTool::Vivado);
        let results = comp.complete("report_");
        assert!(
            results.len() >= 2,
            "should find multiple report_ completions, got {}",
            results.len()
        );
        for r in &results {
            assert!(r.label.starts_with("report_"));
            assert!(!r.detail.is_empty());
        }
    }

    #[test]
    fn test_complete_empty_prefix() {
        let comp = EdaCompletion::new(EdaTool::Vivado);
        let results = comp.complete("");
        assert_eq!(results.len(), comp.command_count());
    }

    #[test]
    fn test_complete_no_match() {
        let comp = EdaCompletion::new(EdaTool::Vivado);
        let results = comp.complete("zzz_no_such_cmd");
        assert!(results.is_empty());
    }

    #[test]
    fn test_get_hover_existing() {
        let comp = EdaCompletion::new(EdaTool::Vivado);
        let hover = comp.get_hover("synth_design");
        assert!(hover.is_some());
        let text = hover.unwrap();
        assert!(
            text.contains("synth_design"),
            "hover should mention command name"
        );
    }

    #[test]
    fn test_get_hover_nonexistent() {
        let comp = EdaCompletion::new(EdaTool::Vivado);
        assert!(comp.get_hover("nonexistent_cmd").is_none());
    }

    #[test]
    fn test_get_hover_contains_params() {
        let comp = EdaCompletion::new(EdaTool::Vivado);
        let hover = comp.get_hover("create_clock").unwrap();
        assert!(
            hover.contains("period") || hover.contains("参数"),
            "hover should list params"
        );
    }

    #[test]
    fn test_detect_tool_vivado_package() {
        let content = "package require Vivado 1.1\nsynth_design -top top_module\n";
        let tool = EdaCompletion::detect_tool_from_context(content);
        assert_eq!(tool, Some(EdaTool::Vivado));
    }

    #[test]
    fn test_detect_tool_quartus_package() {
        let content = "package require quartus::flow\nproject_open myproj\n";
        let tool = EdaCompletion::detect_tool_from_context(content);
        assert_eq!(tool, Some(EdaTool::Quartus));
    }

    #[test]
    fn test_detect_tool_dc_heuristic() {
        let content =
            "# DC script\ncompile_ultra -no_autoungroup\nwrite -format verilog -output out.v\n";
        let tool = EdaCompletion::detect_tool_from_context(content);
        assert_eq!(tool, Some(EdaTool::SynopsysDc));
    }

    #[test]
    fn test_detect_tool_innovus_heuristic() {
        let content =
            "init_design -top_cell my_chip\nplace_design\nccopt_design\nwrite_gds out.gds\n";
        let tool = EdaCompletion::detect_tool_from_context(content);
        assert_eq!(tool, Some(EdaTool::CadenceInnovus));
    }

    #[test]
    fn test_detect_tool_unknown() {
        let content = "puts hello\nset x 1\n";
        let tool = EdaCompletion::detect_tool_from_context(content);
        assert!(tool.is_none());
    }

    #[test]
    fn test_complete_suggestion_has_documentation() {
        let comp = EdaCompletion::new(EdaTool::SynopsysDc);
        let results = comp.complete("compile");
        assert!(!results.is_empty());
        for r in &results {
            assert!(
                r.documentation.is_some(),
                "each suggestion should have docs"
            );
        }
    }

    #[test]
    fn test_cadence_genus_db() {
        let comp = EdaCompletion::new(EdaTool::CadenceGenus);
        let results = comp.complete("synthesize");
        assert!(!results.is_empty());
        let hover = comp.get_hover("read_hdl");
        assert!(hover.is_some());
    }
}
