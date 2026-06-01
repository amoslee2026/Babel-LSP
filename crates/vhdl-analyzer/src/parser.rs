//! VHDL 基于正则的解析器
//!
//! VHDL 是大小写不敏感的语言，所有正则匹配均使用 case-insensitive 模式。

/// 端口方向
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PortDirection {
    In,
    Out,
    InOut,
    Buffer,
}

impl PortDirection {
    #[allow(dead_code)]
    fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "in" => PortDirection::In,
            "out" => PortDirection::Out,
            "inout" => PortDirection::InOut,
            "buffer" => PortDirection::Buffer,
            _ => PortDirection::In,
        }
    }
}

/// VHDL 端口
#[derive(Debug, Clone)]
pub struct VhdlPort {
    pub name: String,
    pub direction: PortDirection,
    pub data_type: String,
    pub line: u32,
}

/// VHDL 泛型参数
#[derive(Debug, Clone)]
pub struct VhdlGeneric {
    pub name: String,
    pub data_type: String,
    pub default_value: Option<String>,
    pub line: u32,
}

/// VHDL 实体
#[derive(Debug, Clone)]
pub struct VhdlEntity {
    pub name: String,
    pub ports: Vec<VhdlPort>,
    pub generics: Vec<VhdlGeneric>,
    pub start_line: u32,
    pub end_line: u32,
    /// end 语句中的名称（用于检测名称不一致）
    pub end_name: Option<String>,
}

/// VHDL 信号
#[derive(Debug, Clone)]
pub struct VhdlSignal {
    pub name: String,
    pub data_type: String,
    pub line: u32,
}

/// VHDL 组件实例化
#[derive(Debug, Clone)]
pub struct VhdlComponent {
    pub name: String,
    pub start_line: u32,
}

/// VHDL 进程
#[derive(Debug, Clone)]
pub struct VhdlProcess {
    pub label: Option<String>,
    pub sensitivity_list: Vec<String>,
    pub start_line: u32,
    pub end_line: u32,
}

/// VHDL 架构
#[derive(Debug, Clone)]
pub struct VhdlArchitecture {
    pub name: String,
    pub entity_name: String,
    pub signals: Vec<VhdlSignal>,
    pub components: Vec<VhdlComponent>,
    pub processes: Vec<VhdlProcess>,
    pub start_line: u32,
    pub end_line: u32,
}

/// VHDL 包
#[derive(Debug, Clone)]
pub struct VhdlPackage {
    pub name: String,
    pub types: Vec<String>,
    pub functions: Vec<String>,
    pub constants: Vec<String>,
    pub start_line: u32,
    pub end_line: u32,
}

/// 解析结果
#[derive(Debug, Clone, Default)]
pub struct VhdlParseResult {
    pub entities: Vec<VhdlEntity>,
    pub architectures: Vec<VhdlArchitecture>,
    pub packages: Vec<VhdlPackage>,
    pub use_clauses: Vec<String>,
}

// ── 辅助：去除行内注释 ────────────────────────────────────────────────────────

fn strip_comment(line: &str) -> &str {
    if let Some(pos) = line.find("--") {
        &line[..pos]
    } else {
        line
    }
}

// ── 解析器 ────────────────────────────────────────────────────────────────────

pub struct VhdlParser;

impl VhdlParser {
    pub fn new() -> Self {
        Self
    }

    /// 解析 VHDL 文本，返回结构化结果
    pub fn parse(&self, content: &str) -> VhdlParseResult {
        let lines: Vec<&str> = content.lines().collect();
        let mut result = VhdlParseResult {
            use_clauses: self.parse_use_clauses(&lines),
            ..Default::default()
        };

        // 解析顶层结构
        let mut i = 0;
        while i < lines.len() {
            let stripped = strip_comment(lines[i]).trim().to_lowercase();

            if stripped.starts_with("entity ") {
                if let Some((entity, next)) = self.parse_entity(&lines, i) {
                    result.entities.push(entity);
                    i = next;
                    continue;
                }
            } else if stripped.starts_with("architecture ") {
                if let Some((arch, next)) = self.parse_architecture(&lines, i) {
                    result.architectures.push(arch);
                    i = next;
                    continue;
                }
            } else if stripped.starts_with("package body ") {
                // 跳过 package body（仅处理 package）
            } else if stripped.starts_with("package ") {
                if let Some((pkg, next)) = self.parse_package(&lines, i) {
                    result.packages.push(pkg);
                    i = next;
                    continue;
                }
            }

            i += 1;
        }

        result
    }

    // ── use 子句 ──────────────────────────────────────────────────────────────

    fn parse_use_clauses(&self, lines: &[&str]) -> Vec<String> {
        let mut clauses = Vec::new();
        for line in lines {
            let s = strip_comment(line).trim().to_lowercase();
            if s.starts_with("use ") {
                // 去掉末尾分号
                let clause = s.trim_end_matches(';').trim().to_string();
                clauses.push(clause);
            }
        }
        clauses
    }

    // ── entity ───────────────────────────────────────────────────────────────

    fn parse_entity(&self, lines: &[&str], start: usize) -> Option<(VhdlEntity, usize)> {
        // entity <name> is
        let first = strip_comment(lines[start]).trim();
        let name = self.extract_keyword_name(first, "entity")?;

        // 找 "end [entity] [name];"
        let (end_idx, end_name) = self.find_end_entity(lines, start, &name)?;

        // 在 start..end_idx 范围内提取 generic 和 port 块
        let segment: Vec<&str> = lines[start..=end_idx].to_vec();

        let generics = self.parse_generic_block(&segment, start as u32);
        let ports = self.parse_port_block(&segment, start as u32);

        Some((
            VhdlEntity {
                name,
                ports,
                generics,
                start_line: start as u32,
                end_line: end_idx as u32,
                end_name,
            },
            end_idx + 1,
        ))
    }

    /// 找 entity 的 end 行，返回 (line_index, end_name)
    fn find_end_entity(
        &self,
        lines: &[&str],
        start: usize,
        _entity_name: &str,
    ) -> Option<(usize, Option<String>)> {
        let mut depth = 0usize;
        for (i, &line) in lines.iter().enumerate().skip(start) {
            let s = strip_comment(line).trim().to_lowercase();
            if s.starts_with("entity ") && s.contains(" is") {
                depth += 1;
            }
            if s.starts_with("end") {
                if depth <= 1 {
                    // 提取 end 后面的名称
                    let end_name = self.extract_end_name(&s);
                    return Some((i, end_name));
                }
                depth = depth.saturating_sub(1);
            }
        }
        None
    }

    /// 从 "end [entity] [name] ;" 中提取名称
    fn extract_end_name(&self, s: &str) -> Option<String> {
        // s 已经是 lowercase
        let after_end = s.trim_start_matches("end").trim();
        // 去掉可选的 "entity"
        let after_kw = if let Some(stripped) = after_end.strip_prefix("entity") {
            stripped.trim()
        } else if let Some(stripped) = after_end.strip_prefix("architecture") {
            stripped.trim()
        } else if let Some(stripped) = after_end.strip_prefix("package") {
            stripped.trim()
        } else {
            after_end
        };
        let name = after_kw.trim_end_matches(';').trim();
        if name.is_empty() {
            None
        } else {
            Some(name.to_string())
        }
    }

    // ── generic block ─────────────────────────────────────────────────────────

    fn parse_generic_block(&self, segment: &[&str], base_line: u32) -> Vec<VhdlGeneric> {
        // 找到包含 "generic" 关键字的行
        let generic_line_idx = segment.iter().position(|l| {
            strip_comment(l)
                .trim()
                .to_lowercase()
                .starts_with("generic")
        });
        let generic_line_idx = match generic_line_idx {
            Some(i) => i,
            None => return vec![],
        };

        // 提取 generic (...) 括号内容
        let (content, content_base_line) = match self.extract_paren_block(segment, generic_line_idx)
        {
            Some(v) => v,
            None => return vec![],
        };

        let mut generics = Vec::new();
        for (rel_line, decl) in self.split_declarations_with_lines(&content) {
            let decl = decl.trim();
            if decl.is_empty() {
                continue;
            }
            if let Some(g) = self.parse_one_generic(decl, base_line + content_base_line + rel_line)
            {
                generics.push(g);
            }
        }
        generics
    }

    fn parse_one_generic(&self, decl: &str, line: u32) -> Option<VhdlGeneric> {
        // name : type [:= default]
        let colon_pos = decl.find(':')?;
        let name = decl[..colon_pos].trim().to_string();
        if name.is_empty() {
            return None;
        }
        let rest = decl[colon_pos + 1..].trim();
        let (data_type, default_value) = if let Some(eq_pos) = rest.find(":=") {
            let t = rest[..eq_pos].trim().to_string();
            let v = rest[eq_pos + 2..]
                .trim()
                .trim_end_matches(';')
                .trim()
                .to_string();
            (t, Some(v))
        } else {
            (rest.trim_end_matches(';').trim().to_string(), None)
        };
        Some(VhdlGeneric {
            name,
            data_type,
            default_value,
            line,
        })
    }

    // ── port block ────────────────────────────────────────────────────────────

    fn parse_port_block(&self, segment: &[&str], base_line: u32) -> Vec<VhdlPort> {
        // 找到包含 "port" 关键字的行（忽略 "port map" 等）
        let port_line_idx = segment.iter().position(|l| {
            let s = strip_comment(l).trim().to_lowercase();
            s.starts_with("port") && !s.starts_with("port map")
        });
        let port_line_idx = match port_line_idx {
            Some(i) => i,
            None => return vec![],
        };

        // 提取 port (...) 括号内容
        let (content, content_base_line) = match self.extract_paren_block(segment, port_line_idx) {
            Some(v) => v,
            None => return vec![],
        };

        let mut ports = Vec::new();
        for (rel_line, decl) in self.split_declarations_with_lines(&content) {
            let decl = decl.trim().to_string();
            if decl.is_empty() {
                continue;
            }
            for p in self.parse_one_port_decl(&decl, base_line + content_base_line + rel_line) {
                ports.push(p);
            }
        }
        ports
    }

    /// 从 segment[start_idx] 开始找第一个 '(' 并提取匹配括号内部的内容
    /// 返回 (各行内容 Vec, 括号内容起始相对于 segment 的行偏移)
    fn extract_paren_block(
        &self,
        segment: &[&str],
        start_idx: usize,
    ) -> Option<(Vec<String>, u32)> {
        // 先把从 start_idx 开始的行拼接找到第一个 '('
        let mut paren_depth = 0i32;
        let mut inside = false;
        let mut result_lines: Vec<String> = Vec::new();
        let mut current_line = String::new();
        let mut first_line_offset = 0u32;
        let mut found_open = false;

        for (rel_i, line) in segment[start_idx..].iter().enumerate() {
            let stripped = strip_comment(line);
            for ch in stripped.chars() {
                if ch == '(' {
                    paren_depth += 1;
                    if paren_depth == 1 {
                        inside = true;
                        found_open = true;
                        first_line_offset = rel_i as u32;
                        continue;
                    }
                }
                if ch == ')' {
                    paren_depth -= 1;
                    if paren_depth == 0 && inside {
                        // 关闭
                        result_lines.push(current_line.clone());
                        return Some((result_lines, first_line_offset));
                    }
                }
                if inside && paren_depth > 0 {
                    current_line.push(ch);
                }
            }
            if inside {
                result_lines.push(current_line.clone());
                current_line.clear();
            }
        }
        if found_open {
            Some((result_lines, first_line_offset))
        } else {
            None
        }
    }

    /// 将括号内容（已按行分割）按分号拆分为声明，并记录每条声明首行的相对行号
    fn split_declarations_with_lines(&self, lines: &[String]) -> Vec<(u32, String)> {
        let mut result = Vec::new();
        let mut current = String::new();
        let mut current_start_line = 0u32;
        let mut first = true;

        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            // 每个分号结束一个声明
            let parts: Vec<&str> = trimmed.split(';').collect();
            for (j, part) in parts.iter().enumerate() {
                let part = part.trim();
                if !part.is_empty() {
                    if first {
                        current_start_line = i as u32;
                        first = false;
                    }
                    if !current.is_empty() {
                        current.push(' ');
                    }
                    current.push_str(part);
                }
                // 遇到分号（除了最后一个 part）时提交
                if j < parts.len() - 1 && !current.trim().is_empty() {
                    result.push((current_start_line, current.trim().to_string()));
                    current = String::new();
                    current_start_line = i as u32;
                    first = true;
                }
            }
        }
        if !current.trim().is_empty() {
            result.push((current_start_line, current.trim().to_string()));
        }
        result
    }

    /// 解析单条端口声明（可能有多个名称："a, b : in std_logic"）
    fn parse_one_port_decl(&self, decl: &str, line: u32) -> Vec<VhdlPort> {
        let mut result = Vec::new();
        let colon_pos = match decl.find(':') {
            Some(p) => p,
            None => return result,
        };
        let names_str = &decl[..colon_pos];
        let rest = decl[colon_pos + 1..].trim();

        // 分离方向和类型
        let lower_rest = rest.to_lowercase();
        let (direction, type_str) = if lower_rest.starts_with("inout") {
            (
                PortDirection::InOut,
                rest["inout".len()..].trim().to_string(),
            )
        } else if lower_rest.starts_with("buffer") {
            (
                PortDirection::Buffer,
                rest["buffer".len()..].trim().to_string(),
            )
        } else if lower_rest.starts_with("out") {
            (PortDirection::Out, rest["out".len()..].trim().to_string())
        } else if lower_rest.starts_with("in") {
            (PortDirection::In, rest["in".len()..].trim().to_string())
        } else {
            // 默认 in
            (PortDirection::In, rest.to_string())
        };

        let data_type = type_str.trim_end_matches(';').trim().to_string();

        for name in names_str.split(',') {
            let n = name.trim().to_string();
            if !n.is_empty() {
                result.push(VhdlPort {
                    name: n,
                    direction: direction.clone(),
                    data_type: data_type.clone(),
                    line,
                });
            }
        }
        result
    }

    // ── architecture ──────────────────────────────────────────────────────────

    fn parse_architecture(
        &self,
        lines: &[&str],
        start: usize,
    ) -> Option<(VhdlArchitecture, usize)> {
        // architecture <name> of <entity> is
        let first = strip_comment(lines[start]).trim();
        let lower_first = first.to_lowercase();

        // 提取架构名和实体名
        // 格式: architecture <arch_name> of <entity_name> is
        let after_arch = lower_first.trim_start_matches("architecture").trim();
        let of_pos = after_arch.to_lowercase().find(" of ")?;
        let arch_name = after_arch[..of_pos].trim().to_string();
        let after_of = after_arch[of_pos + 4..].trim();
        let entity_name = after_of
            .split_whitespace()
            .next()
            .unwrap_or("")
            .trim_end_matches(';')
            .to_string();

        // 找 begin 和 end
        let begin_idx = self.find_architecture_begin(lines, start)?;
        let end_idx = self.find_architecture_end(lines, start)?;

        // 信号在 start..begin_idx（声明区）
        let decl_segment: Vec<&str> = lines[start..=begin_idx].to_vec();
        let signals = self.parse_signals(&decl_segment, start as u32);
        let components = self.parse_components(&decl_segment, start as u32);

        // 进程在 begin_idx..end_idx（主体区）
        let body_segment: Vec<&str> = lines[begin_idx..=end_idx].to_vec();
        let processes = self.parse_processes(&body_segment, begin_idx as u32);

        Some((
            VhdlArchitecture {
                name: arch_name,
                entity_name,
                signals,
                components,
                processes,
                start_line: start as u32,
                end_line: end_idx as u32,
            },
            end_idx + 1,
        ))
    }

    fn find_architecture_begin(&self, lines: &[&str], start: usize) -> Option<usize> {
        for (i, line) in lines.iter().enumerate().skip(start) {
            let s = strip_comment(line).trim().to_lowercase();
            if s == "begin" || s.ends_with(" begin") || s == "begin;" {
                return Some(i);
            }
        }
        None
    }

    fn find_architecture_end(&self, lines: &[&str], start: usize) -> Option<usize> {
        let mut depth = 0usize;
        for (i, line) in lines.iter().enumerate().skip(start) {
            let s = strip_comment(line).trim().to_lowercase();
            if s.starts_with("architecture ") && s.contains(" is") {
                depth += 1;
            }
            // architecture body 的 end 是独立的 "end [architecture] [name];"
            if s.starts_with("end") && depth <= 1 {
                return Some(i);
            } else if s.starts_with("end") {
                depth = depth.saturating_sub(1);
            }
        }
        None
    }

    // ── signals ───────────────────────────────────────────────────────────────

    fn parse_signals(&self, segment: &[&str], base_line: u32) -> Vec<VhdlSignal> {
        let mut signals = Vec::new();
        for (i, line) in segment.iter().enumerate() {
            let s = strip_comment(line).trim();
            let lower = s.to_lowercase();
            if lower.starts_with("signal ") {
                // signal <name> : <type> [:= init];
                let after = s["signal".len()..].trim();
                if let Some(colon_pos) = after.find(':') {
                    let names_str = &after[..colon_pos];
                    let type_str = after[colon_pos + 1..].trim();
                    // 去掉 := 初始值
                    let data_type = if let Some(eq) = type_str.find(":=") {
                        type_str[..eq].trim().to_string()
                    } else {
                        type_str.trim_end_matches(';').trim().to_string()
                    };
                    for name in names_str.split(',') {
                        let n = name.trim().to_string();
                        if !n.is_empty() {
                            signals.push(VhdlSignal {
                                name: n,
                                data_type: data_type.clone(),
                                line: base_line + i as u32,
                            });
                        }
                    }
                }
            }
        }
        signals
    }

    // ── components ────────────────────────────────────────────────────────────

    fn parse_components(&self, segment: &[&str], base_line: u32) -> Vec<VhdlComponent> {
        let mut components = Vec::new();
        for (i, line) in segment.iter().enumerate() {
            let s = strip_comment(line).trim();
            let lower = s.to_lowercase();
            if lower.starts_with("component ") {
                let name = s["component".len()..]
                    .split_whitespace()
                    .next()
                    .unwrap_or("")
                    .trim_end_matches(';')
                    .to_string();
                if !name.is_empty() {
                    components.push(VhdlComponent {
                        name,
                        start_line: base_line + i as u32,
                    });
                }
            }
        }
        components
    }

    // ── processes ─────────────────────────────────────────────────────────────

    fn parse_processes(&self, segment: &[&str], base_line: u32) -> Vec<VhdlProcess> {
        let mut processes = Vec::new();
        let mut i = 0;
        while i < segment.len() {
            let s = strip_comment(segment[i]).trim();
            let lower = s.to_lowercase();

            // 检测 [label :] process [(sensitivity_list)]
            let (label, proc_lower) = if let Some(colon) = lower.find(':') {
                let lbl = lower[..colon].trim().to_string();
                let rest = lower[colon + 1..].trim().to_string();
                if rest.starts_with("process") {
                    (Some(lbl), rest)
                } else {
                    i += 1;
                    continue;
                }
            } else if lower.trim_start().starts_with("process") {
                (None, lower.trim_start().to_string())
            } else {
                i += 1;
                continue;
            };

            let start_line = base_line + i as u32;
            let sensitivity_list = self.extract_sensitivity_list(&proc_lower);

            // 找 end process
            let end_idx = self.find_end_process(segment, i);
            let end_line = base_line + end_idx as u32;

            processes.push(VhdlProcess {
                label,
                sensitivity_list,
                start_line,
                end_line,
            });

            i = end_idx + 1;
        }
        processes
    }

    fn extract_sensitivity_list(&self, proc_line: &str) -> Vec<String> {
        // process [(clk, rst, ...)]
        let after = proc_line.trim_start_matches("process").trim();
        if !after.starts_with('(') {
            return Vec::new();
        }
        if let Some(end) = after.find(')') {
            let content = &after[1..end];
            content
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        } else {
            Vec::new()
        }
    }

    fn find_end_process(&self, segment: &[&str], start: usize) -> usize {
        for (i, line) in segment.iter().enumerate().skip(start + 1) {
            let s = strip_comment(line).trim().to_lowercase();
            if s.starts_with("end process") || s == "end process;" {
                return i;
            }
        }
        // fallback
        segment.len().saturating_sub(1)
    }

    // ── package ───────────────────────────────────────────────────────────────

    fn parse_package(&self, lines: &[&str], start: usize) -> Option<(VhdlPackage, usize)> {
        let first = strip_comment(lines[start]).trim();
        let name = self.extract_keyword_name(first, "package")?;

        // 找 end [package] [name] ;
        let end_idx = self.find_end_block(lines, start, "package")?;

        let segment: Vec<&str> = lines[start..=end_idx].to_vec();
        let types = self.collect_identifiers_after_keyword(&segment, "type");
        let functions = self.collect_identifiers_after_keyword(&segment, "function");
        let constants = self.collect_identifiers_after_keyword(&segment, "constant");

        Some((
            VhdlPackage {
                name,
                types,
                functions,
                constants,
                start_line: start as u32,
                end_line: end_idx as u32,
            },
            end_idx + 1,
        ))
    }

    fn collect_identifiers_after_keyword(&self, segment: &[&str], keyword: &str) -> Vec<String> {
        let mut result = Vec::new();
        for line in segment {
            let s = strip_comment(line).trim();
            let lower = s.to_lowercase();
            let kw_with_space = format!("{} ", keyword);
            if lower.starts_with(&kw_with_space) {
                let name = s[keyword.len()..]
                    .split_whitespace()
                    .next()
                    .unwrap_or("")
                    .trim_end_matches(';')
                    .to_string();
                if !name.is_empty() {
                    result.push(name);
                }
            }
        }
        result
    }

    fn find_end_block(&self, lines: &[&str], start: usize, _keyword: &str) -> Option<usize> {
        let mut depth = 0usize;
        for (i, line) in lines.iter().enumerate().skip(start) {
            let s = strip_comment(line).trim().to_lowercase();
            // 计数嵌套的 begin/end（粗略）
            if s.starts_with("package ") && !s.starts_with("package body") {
                depth += 1;
            }
            if s.starts_with("end") && depth <= 1 {
                return Some(i);
            } else if s.starts_with("end") {
                depth = depth.saturating_sub(1);
            }
        }
        None
    }

    // ── 工具方法 ──────────────────────────────────────────────────────────────

    /// 从 "<keyword> <name> ..." 中提取名称
    fn extract_keyword_name(&self, line: &str, keyword: &str) -> Option<String> {
        let lower = line.to_lowercase();
        if !lower.starts_with(keyword) {
            return None;
        }
        let after = line[keyword.len()..].trim();
        let name = after
            .split_whitespace()
            .next()?
            .trim_end_matches(';')
            .to_string();
        if name.is_empty() {
            None
        } else {
            Some(name.to_lowercase())
        }
    }

    /// 找匹配的右括号，返回其在 s 中的字节索引（供未来扩展使用）
    #[allow(dead_code)]
    fn find_matching_paren(&self, s: &str, open_pos: usize) -> Option<usize> {
        let bytes = s.as_bytes();
        if open_pos >= bytes.len() || bytes[open_pos] != b'(' {
            return None;
        }
        let mut depth = 0usize;
        for (i, &byte) in bytes.iter().enumerate().skip(open_pos) {
            match byte {
                b'(' => depth += 1,
                b')' => {
                    depth -= 1;
                    if depth == 0 {
                        return Some(i);
                    }
                },
                _ => {},
            }
        }
        None
    }

    /// 按分号分割端口/泛型声明（供未来扩展使用）
    #[allow(dead_code)]
    fn split_port_declarations<'a>(&self, content: &'a str) -> Vec<&'a str> {
        // 按分号分割（VHDL 端口声明用分号分隔）
        content
            .split(';')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect()
    }
}

impl Default for VhdlParser {
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

    const SIMPLE_ENTITY: &str = r#"
library ieee;
use ieee.std_logic_1164.all;

entity counter is
    port (
        clk : in  std_logic;
        rst : in  std_logic;
        q   : out std_logic_vector(7 downto 0)
    );
end entity counter;
"#;

    const ENTITY_WITH_GENERICS: &str = r#"
entity fifo is
    generic (
        DEPTH : integer := 16;
        WIDTH : integer := 8
    );
    port (
        clk   : in  std_logic;
        din   : in  std_logic_vector(WIDTH-1 downto 0);
        dout  : out std_logic_vector(WIDTH-1 downto 0)
    );
end entity fifo;
"#;

    const ARCH_WITH_SIGNALS: &str = r#"
architecture rtl of counter is
    signal count : std_logic_vector(7 downto 0);
    signal enable : std_logic;
begin
    process(clk, rst)
    begin
        if rst = '1' then
            count <= (others => '0');
        elsif rising_edge(clk) then
            count <= count + 1;
        end if;
    end process;
end architecture rtl;
"#;

    const PACKAGE_EXAMPLE: &str = r#"
package my_pkg is
    type my_state is (IDLE, BUSY, DONE);
    constant MAX_VAL : integer := 255;
    function to_slv(val : integer) return std_logic_vector;
end package my_pkg;
"#;

    #[test]
    fn test_parse_simple_entity() {
        let parser = VhdlParser::new();
        let result = parser.parse(SIMPLE_ENTITY);

        assert_eq!(result.entities.len(), 1);
        let entity = &result.entities[0];
        assert_eq!(entity.name, "counter");
        assert_eq!(entity.ports.len(), 3);

        let clk = &entity.ports[0];
        assert_eq!(clk.name, "clk");
        assert_eq!(clk.direction, PortDirection::In);
        assert!(clk.data_type.contains("std_logic"));
    }

    #[test]
    fn test_parse_entity_with_generics() {
        let parser = VhdlParser::new();
        let result = parser.parse(ENTITY_WITH_GENERICS);

        assert_eq!(result.entities.len(), 1);
        let entity = &result.entities[0];
        assert_eq!(entity.name, "fifo");
        assert_eq!(entity.generics.len(), 2);

        let depth = &entity.generics[0];
        assert_eq!(depth.name, "DEPTH");
        assert_eq!(depth.data_type.to_lowercase(), "integer");
        assert_eq!(depth.default_value, Some("16".to_string()));
    }

    #[test]
    fn test_parse_ports_directions() {
        let vhdl = r#"
entity all_dirs is
    port (
        a : in    std_logic;
        b : out   std_logic;
        c : inout std_logic;
        d : buffer std_logic_vector(3 downto 0)
    );
end entity all_dirs;
"#;
        let parser = VhdlParser::new();
        let result = parser.parse(vhdl);
        let entity = &result.entities[0];
        assert_eq!(entity.ports.len(), 4);

        let dirs: Vec<&PortDirection> = entity.ports.iter().map(|p| &p.direction).collect();
        assert_eq!(dirs[0], &PortDirection::In);
        assert_eq!(dirs[1], &PortDirection::Out);
        assert_eq!(dirs[2], &PortDirection::InOut);
        assert_eq!(dirs[3], &PortDirection::Buffer);
    }

    #[test]
    fn test_parse_architecture_with_signals() {
        let parser = VhdlParser::new();
        let result = parser.parse(ARCH_WITH_SIGNALS);

        assert_eq!(result.architectures.len(), 1);
        let arch = &result.architectures[0];
        assert_eq!(arch.name, "rtl");
        assert_eq!(arch.entity_name, "counter");
        assert_eq!(arch.signals.len(), 2);

        let sig = &arch.signals[0];
        assert_eq!(sig.name, "count");
        assert!(sig.data_type.contains("std_logic_vector"));
    }

    #[test]
    fn test_parse_process() {
        let parser = VhdlParser::new();
        let result = parser.parse(ARCH_WITH_SIGNALS);

        let arch = &result.architectures[0];
        assert_eq!(arch.processes.len(), 1);

        let proc = &arch.processes[0];
        assert_eq!(proc.sensitivity_list, vec!["clk", "rst"]);
        assert!(proc.start_line > 0);
        assert!(proc.end_line >= proc.start_line);
    }

    #[test]
    fn test_parse_package() {
        let parser = VhdlParser::new();
        let result = parser.parse(PACKAGE_EXAMPLE);

        assert_eq!(result.packages.len(), 1);
        let pkg = &result.packages[0];
        assert_eq!(pkg.name, "my_pkg");
        assert!(!pkg.types.is_empty(), "should have type declarations");
        assert!(
            !pkg.constants.is_empty(),
            "should have constant declarations"
        );
        assert!(
            !pkg.functions.is_empty(),
            "should have function declarations"
        );
    }

    #[test]
    fn test_case_insensitive() {
        let vhdl = r#"
ENTITY upper_case IS
    PORT (
        CLK : IN STD_LOGIC;
        Q   : OUT STD_LOGIC
    );
END ENTITY upper_case;
"#;
        let parser = VhdlParser::new();
        let result = parser.parse(vhdl);
        assert_eq!(result.entities.len(), 1);
        assert_eq!(result.entities[0].name, "upper_case");
        assert_eq!(result.entities[0].ports.len(), 2);
        assert_eq!(result.entities[0].ports[0].direction, PortDirection::In);
        assert_eq!(result.entities[0].ports[1].direction, PortDirection::Out);
    }

    #[test]
    fn test_use_clauses() {
        let parser = VhdlParser::new();
        let result = parser.parse(SIMPLE_ENTITY);
        assert!(!result.use_clauses.is_empty());
        assert!(result.use_clauses[0].contains("ieee"));
    }

    #[test]
    fn test_strip_comment_with_comment() {
        let line = "signal a : std_logic; -- this is a comment";
        let stripped = strip_comment(line);
        assert_eq!(stripped, "signal a : std_logic; ");
    }

    #[test]
    fn test_strip_comment_no_comment() {
        let line = "signal a : std_logic;";
        let stripped = strip_comment(line);
        assert_eq!(stripped, line);
    }

    #[test]
    fn test_entity_end_with_architecture_keyword() {
        // 测试 end architecture 名称提取
        let vhdl = r#"
architecture rtl of test_entity is
begin
end architecture rtl;
"#;
        let parser = VhdlParser::new();
        let result = parser.parse(vhdl);
        assert_eq!(result.architectures.len(), 1);
        assert_eq!(result.architectures[0].name, "rtl");
    }

    #[test]
    fn test_entity_end_with_package_keyword() {
        // 测试 end package 名称提取
        let vhdl = r#"
package test_pkg is
    constant VAL : integer := 5;
end package test_pkg;
"#;
        let parser = VhdlParser::new();
        let result = parser.parse(vhdl);
        assert_eq!(result.packages.len(), 1);
        assert_eq!(result.packages[0].name, "test_pkg");
    }

    #[test]
    fn test_entity_with_inline_comments() {
        // 测试行内有注释的 entity
        let vhdl = r#"
entity commented is -- entity comment
    port ( -- port comment
        clk : in std_logic -- clock input
    );
end entity commented;
"#;
        let parser = VhdlParser::new();
        let result = parser.parse(vhdl);
        assert_eq!(result.entities.len(), 1);
        assert_eq!(result.entities[0].name, "commented");
        assert_eq!(result.entities[0].ports.len(), 1);
    }

    #[test]
    fn test_empty_entity_name() {
        // 测试空输入
        let parser = VhdlParser::new();
        let result = parser.parse("");
        assert!(result.entities.is_empty());
        assert!(result.architectures.is_empty());
        assert!(result.packages.is_empty());
    }

    #[test]
    fn test_generic_without_default() {
        // 测试无默认值的 generic
        let vhdl = r#"
entity gen_no_default is
    generic (
        WIDTH : integer
    );
    port (
        data : out std_logic_vector(WIDTH-1 downto 0)
    );
end entity gen_no_default;
"#;
        let parser = VhdlParser::new();
        let result = parser.parse(vhdl);
        assert_eq!(result.entities.len(), 1);
        assert_eq!(result.entities[0].generics.len(), 1);
        assert_eq!(result.entities[0].generics[0].name, "WIDTH");
        assert_eq!(result.entities[0].generics[0].default_value, None);
    }

    #[test]
    fn test_multiple_architectures() {
        // 测试多个 architecture
        let vhdl = r#"
architecture rtl of multi_arch is
begin
end architecture rtl;

architecture behavior of multi_arch is
begin
end architecture behavior;
"#;
        let parser = VhdlParser::new();
        let result = parser.parse(vhdl);
        assert_eq!(result.architectures.len(), 2);
        assert_eq!(result.architectures[0].name, "rtl");
        assert_eq!(result.architectures[1].name, "behavior");
    }
}
