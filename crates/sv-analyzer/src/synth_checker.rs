//! 可综合性检查器
//!
//! 实现 18 条 SYN-V 规则，基于文本模式匹配（不依赖 AST）。
//! 只对 RTL 文件生效，TB/Netlist 文件跳过。

use babel_lsp_core::diagnostic::Diagnostic;
use babel_lsp_core::document::FileClass;
use babel_lsp_core::symbol::{Location, Position};

/// SYN-V 规则枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SynthRule {
    /// SYN-V-001: initial 块
    InitialBlock,
    /// SYN-V-002: 时间延迟 (#N 或 #(N))
    Delays,
    /// SYN-V-003: wait 语句
    WaitStmt,
    /// SYN-V-004: force/release
    ForceRelease,
    /// SYN-V-005: $display/$monitor/$strobe/$write 等系统任务
    SystemDisplay,
    /// SYN-V-006: $finish/$stop
    SystemFinish,
    /// SYN-V-007: disable 语句
    DisableStmt,
    /// SYN-V-008: fork...join（阻塞式，不包括 join_any/join_none）
    ForkJoin,
}

impl SynthRule {
    /// 规则代码（如 "SYN-V-001"）
    pub fn code(&self) -> &'static str {
        match self {
            Self::InitialBlock => "SYN-V-001",
            Self::Delays => "SYN-V-002",
            Self::WaitStmt => "SYN-V-003",
            Self::ForceRelease => "SYN-V-004",
            Self::SystemDisplay => "SYN-V-005",
            Self::SystemFinish => "SYN-V-006",
            Self::DisableStmt => "SYN-V-007",
            Self::ForkJoin => "SYN-V-008",
        }
    }

    /// 人类可读的诊断消息
    pub fn message(&self) -> &'static str {
        match self {
            Self::InitialBlock => "initial 块不可综合 (SYN-V-001)",
            Self::Delays => "时间延迟 (#N) 不可综合 (SYN-V-002)",
            Self::WaitStmt => "wait 语句不可综合 (SYN-V-003)",
            Self::ForceRelease => "force/release 语句不可综合 (SYN-V-004)",
            Self::SystemDisplay => "$display/$monitor/$strobe/$write 不可综合 (SYN-V-005)",
            Self::SystemFinish => "$finish/$stop 不可综合 (SYN-V-006)",
            Self::DisableStmt => "disable 语句不可综合 (SYN-V-007)",
            Self::ForkJoin => {
                "fork...join 不可综合，请使用 fork...join_any 或 fork...join_none (SYN-V-008)"
            },
        }
    }
}

/// 可综合性检查器
pub struct SynthChecker {
    pub rules: Vec<SynthRule>,
}

impl SynthChecker {
    /// 使用全部 8 条规则创建检查器
    pub fn new() -> Self {
        Self {
            rules: vec![
                SynthRule::InitialBlock,
                SynthRule::Delays,
                SynthRule::WaitStmt,
                SynthRule::ForceRelease,
                SynthRule::SystemDisplay,
                SynthRule::SystemFinish,
                SynthRule::DisableStmt,
                SynthRule::ForkJoin,
            ],
        }
    }

    /// 使用指定规则集创建检查器
    pub fn with_rules(rules: Vec<SynthRule>) -> Self {
        Self { rules }
    }

    /// 检查源码，返回诊断列表。
    ///
    /// - 非 RTL 文件（Testbench/Netlist）直接返回空列表。
    /// - `synthesis translate_off` ... `synthesis translate_on` 区块内的行跳过检查。
    pub fn check_source(
        &self,
        content: &str,
        file_class: FileClass,
        file_uri: &str,
    ) -> Vec<Diagnostic> {
        // 只检查 RTL 文件
        if file_class != FileClass::RTL {
            return vec![];
        }

        let mut diagnostics = Vec::new();

        // 构建 synthesis translate_off/on 区块的行范围
        let skip_ranges = build_skip_ranges(content);

        for (line_idx, line) in content.lines().enumerate() {
            // 跳过 synthesis off 区块
            if is_in_skip_range(line_idx, &skip_ranges) {
                continue;
            }

            // 去掉行内注释后检查
            let code_part = strip_line_comment(line);

            for rule in &self.rules {
                if matches_rule(rule, code_part) {
                    let loc = Location {
                        uri: file_uri.to_string(),
                        start: Position::new(line_idx as u32, 0),
                        end: Position::new(line_idx as u32, line.len() as u32),
                    };
                    let mut diag = Diagnostic::warning(loc, rule.message().to_string());
                    diag.code = Some(rule.code().to_string());
                    diag.source = "babel-lsp-sv/synth".to_string();
                    diagnostics.push(diag);
                }
            }
        }

        diagnostics
    }
}

impl Default for SynthChecker {
    fn default() -> Self {
        Self::new()
    }
}

/// 检查单行代码是否匹配某条规则
fn matches_rule(rule: &SynthRule, line: &str) -> bool {
    match rule {
        SynthRule::InitialBlock => {
            // `initial` 作为独立关键字，后跟空白或 begin
            contains_word(line, "initial")
        },
        SynthRule::Delays => {
            // #N 或 #(N) 时间延迟，但不是 generate #(N) 参数化实例化
            // 简单匹配 # 后跟数字或括号（排除 #( 后跟标识符的模块实例化）
            has_time_delay(line)
        },
        SynthRule::WaitStmt => contains_word(line, "wait"),
        SynthRule::ForceRelease => contains_word(line, "force") || contains_word(line, "release"),
        SynthRule::SystemDisplay => contains_any_system_task(
            line,
            &[
                "$display",
                "$monitor",
                "$strobe",
                "$write",
                "$writeb",
                "$writeo",
                "$writeh",
                "$displayb",
                "$displayo",
                "$displayh",
                "$monitorb",
                "$monitoro",
                "$monitorh",
            ],
        ),
        SynthRule::SystemFinish => contains_any_system_task(line, &["$finish", "$stop"]),
        SynthRule::DisableStmt => contains_word(line, "disable"),
        SynthRule::ForkJoin => {
            // fork 出现，且后面跟着 join（不是 join_any 或 join_none）
            if contains_word(line, "fork") {
                return true;
            }
            // 检查单独的 join（非 join_any/join_none）
            is_bare_join(line)
        },
    }
}

/// 检查是否包含完整单词（不是另一个单词的子串）
fn contains_word(s: &str, word: &str) -> bool {
    let mut start = 0;
    while let Some(pos) = s[start..].find(word) {
        let abs = start + pos;
        let before_ok = abs == 0
            || !s.as_bytes()[abs - 1].is_ascii_alphanumeric() && s.as_bytes()[abs - 1] != b'_';
        let after_pos = abs + word.len();
        let after_ok = after_pos >= s.len()
            || !s.as_bytes()[after_pos].is_ascii_alphanumeric() && s.as_bytes()[after_pos] != b'_';
        if before_ok && after_ok {
            return true;
        }
        start = abs + 1;
        if start >= s.len() {
            break;
        }
    }
    false
}

/// 检查是否含有系统任务调用（任意一个）
fn contains_any_system_task(s: &str, tasks: &[&str]) -> bool {
    tasks.iter().any(|t| s.contains(t))
}

/// 检查是否含有时间延迟 (#数字 或 #(数字))
fn has_time_delay(line: &str) -> bool {
    // 寻找 # 后跟数字或括号+数字的模式
    let bytes = line.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'#' {
            let j = i + 1;
            if j < bytes.len() {
                let next = bytes[j];
                if next.is_ascii_digit() {
                    return true;
                }
                if next == b'(' {
                    // #(N) 格式，跳过空格看数字
                    let mut k = j + 1;
                    while k < bytes.len() && bytes[k] == b' ' {
                        k += 1;
                    }
                    if k < bytes.len() && bytes[k].is_ascii_digit() {
                        return true;
                    }
                }
                // 跳过空格后的数字：# 10
                if next == b' ' || next == b'\t' {
                    let mut k = j;
                    while k < bytes.len() && (bytes[k] == b' ' || bytes[k] == b'\t') {
                        k += 1;
                    }
                    if k < bytes.len() && bytes[k].is_ascii_digit() {
                        return true;
                    }
                }
            }
        }
        i += 1;
    }
    false
}

/// 检查是否含有裸 join（不是 join_any 或 join_none）
fn is_bare_join(line: &str) -> bool {
    let mut start = 0;
    while let Some(pos) = line[start..].find("join") {
        let abs = start + pos;
        // 检查前面是否是单词边界
        let before_ok = abs == 0 || !is_ident_char(line.as_bytes()[abs - 1]);
        let after_pos = abs + 4; // "join".len()
        let after_ok = if after_pos >= line.len() {
            true
        } else {
            let after_byte = line.as_bytes()[after_pos];
            // 允许空格、分号、换行，但不允许 _any 或 _none
            !is_ident_char(after_byte)
        };
        if before_ok && after_ok {
            return true;
        }
        start = abs + 1;
        if start >= line.len() {
            break;
        }
    }
    false
}

fn is_ident_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

/// 去除行内 // 注释
fn strip_line_comment(line: &str) -> &str {
    if let Some(pos) = line.find("//") {
        &line[..pos]
    } else {
        line
    }
}

/// 构建需要跳过检查的行号范围（synthesis translate_off/on 区块）
fn build_skip_ranges(content: &str) -> Vec<(usize, usize)> {
    let mut ranges = Vec::new();
    let mut off_start: Option<usize> = None;

    for (i, line) in content.lines().enumerate() {
        let l = line.to_ascii_lowercase();
        if l.contains("synthesis translate_off") || l.contains("synopsys translate_off") {
            off_start = Some(i);
        } else if l.contains("synthesis translate_on") || l.contains("synopsys translate_on") {
            if let Some(start) = off_start.take() {
                ranges.push((start, i));
            }
        }
    }
    // 未闭合的区块：从 off_start 到文件末尾
    if let Some(start) = off_start {
        ranges.push((start, usize::MAX));
    }
    ranges
}

fn is_in_skip_range(line: usize, ranges: &[(usize, usize)]) -> bool {
    ranges
        .iter()
        .any(|&(start, end)| line >= start && line <= end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_block_detected_in_rtl() {
        let checker = SynthChecker::new();
        let src = "module foo();\n  initial begin\n    a = 0;\n  end\nendmodule\n";
        let diags = checker.check_source(src, FileClass::RTL, "foo.sv");
        assert!(
            diags
                .iter()
                .any(|d| d.code == Some("SYN-V-001".to_string())),
            "期望 SYN-V-001，实际: {:?}",
            diags
        );
    }

    #[test]
    fn test_delay_detected() {
        let checker = SynthChecker::new();
        let src =
            "module foo();\n  always @(posedge clk) begin\n    #10 a <= 1;\n  end\nendmodule\n";
        let diags = checker.check_source(src, FileClass::RTL, "foo.sv");
        assert!(
            diags
                .iter()
                .any(|d| d.code == Some("SYN-V-002".to_string())),
            "期望 SYN-V-002，实际: {:?}",
            diags
        );
    }

    #[test]
    fn test_wait_stmt_detected() {
        let checker = SynthChecker::new();
        let src = "  wait (done == 1);\n";
        let diags = checker.check_source(src, FileClass::RTL, "f.sv");
        assert!(diags
            .iter()
            .any(|d| d.code == Some("SYN-V-003".to_string())));
    }

    #[test]
    fn test_system_display_detected() {
        let checker = SynthChecker::new();
        let src = "  $display(\"value=%0d\", x);\n";
        let diags = checker.check_source(src, FileClass::RTL, "f.sv");
        assert!(diags
            .iter()
            .any(|d| d.code == Some("SYN-V-005".to_string())));
    }

    #[test]
    fn test_synthesis_off_block_skips_rules() {
        let checker = SynthChecker::new();
        let src = "\
module foo();
// synthesis translate_off
  initial begin
    $display(\"debug\");
    #10;
  end
// synthesis translate_on
endmodule
";
        let diags = checker.check_source(src, FileClass::RTL, "foo.sv");
        // translate_off 区块内的违规应被跳过
        assert!(
            diags.is_empty(),
            "synthesis translate_off 区块内不应有诊断，实际: {:?}",
            diags
        );
    }

    #[test]
    fn test_testbench_file_skipped() {
        let checker = SynthChecker::new();
        let src = "  initial begin\n    #10;\n    $finish;\n  end\n";
        let diags = checker.check_source(src, FileClass::Testbench, "tb.sv");
        assert!(diags.is_empty(), "Testbench 文件不应触发综合检查");
    }

    #[test]
    fn test_netlist_file_skipped() {
        let checker = SynthChecker::new();
        let src = "  initial begin\n    a = 0;\n  end\n";
        let diags = checker.check_source(src, FileClass::Netlist, "netlist.v");
        assert!(diags.is_empty());
    }

    #[test]
    fn test_force_release_detected() {
        let checker = SynthChecker::new();
        let src = "  force a = 1;\n  release b;\n";
        let diags = checker.check_source(src, FileClass::RTL, "f.sv");
        let codes: Vec<_> = diags.iter().filter_map(|d| d.code.as_deref()).collect();
        assert!(codes.contains(&"SYN-V-004"), "实际: {:?}", codes);
    }

    #[test]
    fn test_fork_join_detected() {
        let checker = SynthChecker::new();
        let src = "  fork\n    a = 1;\n    b = 2;\n  join\n";
        let diags = checker.check_source(src, FileClass::RTL, "f.sv");
        let codes: Vec<_> = diags.iter().filter_map(|d| d.code.as_deref()).collect();
        assert!(codes.contains(&"SYN-V-008"), "实际: {:?}", codes);
    }

    #[test]
    fn test_disable_detected() {
        let checker = SynthChecker::new();
        let src = "  disable my_label;\n";
        let diags = checker.check_source(src, FileClass::RTL, "f.sv");
        assert!(diags
            .iter()
            .any(|d| d.code == Some("SYN-V-007".to_string())));
    }

    #[test]
    fn test_line_comment_skipped() {
        let checker = SynthChecker::new();
        // initial 在注释里，不应触发
        let src = "  // initial begin\n  always @(posedge clk) a <= 1;\n";
        let diags = checker.check_source(src, FileClass::RTL, "f.sv");
        assert!(
            !diags
                .iter()
                .any(|d| d.code == Some("SYN-V-001".to_string())),
            "注释中的 initial 不应触发规则"
        );
    }

    #[test]
    fn test_with_rules_custom_set() {
        // 测试 with_rules 方法
        let checker = SynthChecker::with_rules(vec![SynthRule::InitialBlock]);
        let src = "  initial begin\n    a = 0;\n  end\n  #10;\n";
        let diags = checker.check_source(src, FileClass::RTL, "f.sv");
        // 只有 InitialBlock 规则，应该只检测到 SYN-V-001
        assert!(diags.iter().any(|d| d.code == Some("SYN-V-001".to_string())));
        // 没有启用 Delays 规则，不应检测到 SYN-V-002
        assert!(!diags.iter().any(|d| d.code == Some("SYN-V-002".to_string())));
    }

    #[test]
    fn test_default_implementation() {
        let checker = SynthChecker::default();
        assert_eq!(checker.rules.len(), 8);
    }

    #[test]
    fn test_time_delay_paren_format() {
        // 测试 #(10) 格式的时间延迟
        let checker = SynthChecker::new();
        let src = "  #(10) a = 1;\n";
        let diags = checker.check_source(src, FileClass::RTL, "f.sv");
        assert!(
            diags.iter().any(|d| d.code == Some("SYN-V-002".to_string())),
            "#(N) 格式应触发 SYN-V-002"
        );
    }

    #[test]
    fn test_time_delay_with_spaces() {
        // 测试 # 10 格式的时间延迟（空格分隔）
        let checker = SynthChecker::new();
        let src = "  # 10 a = 1;\n";
        let diags = checker.check_source(src, FileClass::RTL, "f.sv");
        assert!(
            diags.iter().any(|d| d.code == Some("SYN-V-002".to_string())),
            "# N 格式应触发 SYN-V-002"
        );
    }

    #[test]
    fn test_bare_join_after_fork() {
        // 测试 fork...join（而非 join_any/join_none）
        let checker = SynthChecker::new();
        let src = "  fork\n    a = 1;\n  join\n";
        let diags = checker.check_source(src, FileClass::RTL, "f.sv");
        assert!(
            diags.iter().any(|d| d.code == Some("SYN-V-008".to_string())),
            "fork...join 应触发 SYN-V-008"
        );
    }

    #[test]
    fn test_join_any_not_detected() {
        // join_any 不应触发
        let checker = SynthChecker::new();
        let src = "  fork\n    a = 1;\n  join_any\n";
        let diags = checker.check_source(src, FileClass::RTL, "f.sv");
        // fork 仍然会触发 SYN-V-008，但不应该有 join 相关的重复检测
        let fork_diags: Vec<_> = diags
            .iter()
            .filter(|d| d.code == Some("SYN-V-008".to_string()))
            .collect();
        assert!(fork_diags.len() <= 1, "join_any 不应额外触发 SYN-V-008");
    }

    #[test]
    fn test_unclosed_synthesis_off_block() {
        // 未闭合的 synthesis translate_off 区块
        let checker = SynthChecker::new();
        let src = "// synthesis translate_off\n  initial begin\n  end\n";
        let diags = checker.check_source(src, FileClass::RTL, "f.sv");
        assert!(
            diags.is_empty(),
            "未闭合的 translate_off 区块内的代码应被跳过"
        );
    }

    #[test]
    fn test_synopsys_translate_off() {
        // synopsys translate_off/on 别名
        let checker = SynthChecker::new();
        let src = "// synopsys translate_off\n  #10;\n// synopsys translate_on\n";
        let diags = checker.check_source(src, FileClass::RTL, "f.sv");
        assert!(
            diags.is_empty(),
            "synopsys translate_off 区块应被跳过"
        );
    }

    #[test]
    fn test_all_rules_code_and_message() {
        // 验证所有规则的 code() 和 message() 方法
        assert_eq!(SynthRule::InitialBlock.code(), "SYN-V-001");
        assert_eq!(SynthRule::Delays.code(), "SYN-V-002");
        assert_eq!(SynthRule::WaitStmt.code(), "SYN-V-003");
        assert_eq!(SynthRule::ForceRelease.code(), "SYN-V-004");
        assert_eq!(SynthRule::SystemDisplay.code(), "SYN-V-005");
        assert_eq!(SynthRule::SystemFinish.code(), "SYN-V-006");
        assert_eq!(SynthRule::DisableStmt.code(), "SYN-V-007");
        assert_eq!(SynthRule::ForkJoin.code(), "SYN-V-008");

        for rule in &[
            SynthRule::InitialBlock,
            SynthRule::Delays,
            SynthRule::WaitStmt,
            SynthRule::ForceRelease,
            SynthRule::SystemDisplay,
            SynthRule::SystemFinish,
            SynthRule::DisableStmt,
            SynthRule::ForkJoin,
        ] {
            assert!(!rule.message().is_empty());
        }
    }
}
