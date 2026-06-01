//! gate-level Verilog 解析

use std::path::{Path, PathBuf};

/// Cell 解析器
pub struct CellParser;

impl CellParser {
    pub fn new() -> Self {
        Self
    }

    /// 解析 Verilog 内容
    pub fn parse(&self, content: &str) -> anyhow::Result<Vec<Cell>> {
        self.parse_with_path(content, PathBuf::new())
    }

    /// 解析 Verilog 内容并记录源文件路径
    pub fn parse_with_path<P: AsRef<Path>>(
        &self,
        content: &str,
        path: P,
    ) -> anyhow::Result<Vec<Cell>> {
        let path = path.as_ref().to_path_buf();
        let mut cells = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        let mut i = 0;
        while i < lines.len() {
            let line = lines[i].trim();

            // 检查 module 定义
            if line.starts_with("module ") || line.starts_with("primitive ") {
                let is_primitive = line.starts_with("primitive ");
                let (cell, end_line) = self.parse_module(&lines, i, is_primitive, &path)?;
                cells.push(cell);
                i = end_line + 1;
            } else {
                i += 1;
            }
        }

        Ok(cells)
    }

    /// 解析单个 module 或 primitive
    fn parse_module(
        &self,
        lines: &[&str],
        start: usize,
        is_primitive: bool,
        path: &Path,
    ) -> anyhow::Result<(Cell, usize)> {
        let first_line = lines[start].trim();

        // 查找 module 前的注释作为 description
        let description = self.find_description(lines, start);

        // 检测是否是 ANSI-style（端口声明在 header 中）
        let mut ports: Vec<Port> = Vec::new();
        let mut end_line = start;

        // 收集完整的 header（可能跨多行）
        let mut header_lines: Vec<&str> = vec![first_line];
        let mut header_end_line = start;

        // 检查第一行是否已包含完整的 header（以 ; 结尾）
        if !first_line.trim().ends_with(';') {
            // 继续收集直到找到 );
            for (j, &raw_line) in lines.iter().enumerate().skip(start + 1) {
                let line = raw_line.trim();
                header_lines.push(line);
                header_end_line = j;
                if line.ends_with(';') || line == ");" {
                    break;
                }
            }
        }

        // 合并 header 行
        let full_header = header_lines.join(" ");

        // 解析 header 中的端口
        if full_header.contains('(') {
            let header_content = self.extract_header_content(&full_header)?;
            ports = self.parse_header_ports(&header_content)?;
        }

        let end_keyword = if is_primitive {
            "endprimitive"
        } else {
            "endmodule"
        };
        let mut in_specify = false;
        let mut in_table = false;

        // 从 header 结束后的下一行开始继续解析
        let body_start = header_end_line + 1;

        for (i, &raw_line) in lines.iter().enumerate().skip(body_start) {
            let line = raw_line.trim();

            // 跳过空行和注释
            if line.is_empty() || line.starts_with("//") {
                continue;
            }

            // 检测 specify block
            if line == "specify" {
                in_specify = true;
                continue;
            }
            if line == "endspecify" {
                in_specify = false;
                continue;
            }

            // 检测 table block（UDP）
            if line == "table" {
                in_table = true;
                continue;
            }
            if line == "endtable" {
                in_table = false;
                continue;
            }

            // 跳过 specify 和 table 内容
            if in_specify || in_table {
                continue;
            }

            // 检查结束
            if line == end_keyword {
                end_line = i;
                break;
            }

            // 解析端口声明（非 ANSI-style）
            self.parse_port_declaration(line, &mut ports)?;
        }

        // 计算实际定义行号（跳过注释和空行）
        let actual_line = self.find_actual_module_line(lines, start);

        let cell = Cell {
            name: self.extract_module_name(&full_header)?,
            ports,
            description,
            source_file: path.to_path_buf(),
            line: actual_line + 1, // 1-based 行号
        };

        Ok((cell, end_line))
    }

    /// 找到 module 的实际定义行（跳过前面的注释）
    fn find_actual_module_line(&self, lines: &[&str], start: usize) -> usize {
        // 向上查找，找到第一个非注释行
        for i in (0..=start).rev() {
            let line = lines[i].trim();
            if line.is_empty() || line.starts_with("//") {
                continue;
            }
            return i;
        }
        start
    }

    /// 提取 header 内容（括号内的部分）
    fn extract_header_content(&self, header: &str) -> anyhow::Result<String> {
        let start_pos = header.find('(').unwrap_or(0);
        let end_pos = header.rfind(';').unwrap_or(header.len());
        let end_pos = header[..end_pos].rfind(')').unwrap_or(end_pos);

        Ok(header[start_pos + 1..end_pos].trim().to_string())
    }

    /// 解析 header 中的端口声明（ANSI-style）
    fn parse_header_ports(&self, content: &str) -> anyhow::Result<Vec<Port>> {
        let mut ports = Vec::new();

        // 分割每个端口声明
        for part in content.split(',') {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }

            // 尝试解析 ANSI-style: output [width] name
            let port = self.parse_single_port_declaration(part);
            ports.push(port);
        }

        Ok(ports)
    }

    /// 解析单个端口声明
    fn parse_single_port_declaration(&self, decl: &str) -> Port {
        let decl = decl.trim();

        // 解析方向关键字
        let (direction, rest) = if let Some(stripped) = decl.strip_prefix("input ") {
            (PortDirection::Input, stripped)
        } else if let Some(stripped) = decl.strip_prefix("output ") {
            (PortDirection::Output, stripped)
        } else if let Some(stripped) = decl.strip_prefix("inout ") {
            (PortDirection::Inout, stripped)
        } else {
            // 无方向声明，默认为 input
            (PortDirection::Input, decl)
        };

        let rest = rest.trim();

        // 解析位宽 [n:m]
        let (width, name) = if rest.starts_with('[') {
            let bracket_end = rest.find(']').unwrap_or(0);
            if bracket_end > 0 {
                let range = &rest[1..bracket_end];
                let width = self.parse_width_inline(range);
                let name_str = &rest[bracket_end + 1..];
                (width, name_str.trim())
            } else {
                (1, rest)
            }
        } else {
            (1, rest)
        };

        Port {
            name: name.to_string(),
            direction,
            width,
        }
    }

    /// 从完整 header 提取 module 名称
    fn extract_module_name(&self, header: &str) -> anyhow::Result<String> {
        let header = header.trim();
        let rest = if let Some(stripped) = header.strip_prefix("module ") {
            stripped
        } else if let Some(stripped) = header.strip_prefix("primitive ") {
            stripped
        } else {
            return Ok(String::new());
        };

        // 名称在第一个空格或括号之前
        let end_pos = rest.find([' ', '(', ';']).unwrap_or(rest.len());

        Ok(rest[..end_pos].trim().to_string())
    }

    /// 查找 module 前的注释作为 description
    fn find_description(&self, lines: &[&str], start: usize) -> Option<String> {
        // 向上查找注释
        for i in (0..start).rev() {
            let line = lines[i].trim();
            if line.is_empty() {
                continue;
            }
            if let Some(stripped) = line.strip_prefix("//") {
                return Some(stripped.trim().to_string());
            }
            // 非注释行，停止
            break;
        }
        None
    }

    /// 解析端口声明：output X, input A, inout B, output [7:0] OUT
    fn parse_port_declaration(&self, line: &str, ports: &mut Vec<Port>) -> anyhow::Result<()> {
        let line = line.trim();

        // 解析方向关键字
        let (direction, rest) = if let Some(stripped) = line.strip_prefix("input ") {
            (PortDirection::Input, stripped)
        } else if let Some(stripped) = line.strip_prefix("output ") {
            (PortDirection::Output, stripped)
        } else if let Some(stripped) = line.strip_prefix("inout ") {
            (PortDirection::Inout, stripped)
        } else {
            // 不是端口声明
            return Ok(());
        };

        // 去除末尾分号
        let rest = rest.trim();
        let rest = if let Some(stripped) = rest.strip_suffix(';') {
            stripped
        } else {
            rest
        };

        // 解析位宽 [n:m]
        let (width, names_str) = if rest.starts_with('[') {
            let bracket_end = rest.find(']').unwrap_or(0);
            if bracket_end > 0 {
                let range = &rest[1..bracket_end];
                let width = self.parse_width_inline(range);
                let names = &rest[bracket_end + 1..];
                (width, names.trim())
            } else {
                (1, rest.trim())
            }
        } else {
            (1, rest.trim())
        };

        // 解析端口名称列表
        for name in names_str.split(',') {
            let name = name.trim().to_string();
            if name.is_empty() {
                continue;
            }
            // 更新对应端口的方向和位宽（如果已存在）
            for port in ports.iter_mut() {
                if port.name == name {
                    port.direction = direction;
                    port.width = width;
                }
            }
            // 如果不存在，添加新端口
            if !ports.iter().any(|p| p.name == name) {
                ports.push(Port {
                    name,
                    direction,
                    width,
                });
            }
        }

        Ok(())
    }

    /// 解析位宽（内联版本）
    fn parse_width_inline(&self, range: &str) -> usize {
        let parts: Vec<&str> = range.split(':').collect();
        if parts.len() == 2 {
            let high: usize = parts[0].trim().parse().unwrap_or(0);
            let low: usize = parts[1].trim().parse().unwrap_or(0);
            high - low + 1
        } else if parts.len() == 1 {
            parts[0].trim().parse().unwrap_or(1)
        } else {
            1
        }
    }
}

impl Default for CellParser {
    fn default() -> Self {
        Self::new()
    }
}

/// 标准单元
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Cell {
    /// Cell 名称
    pub name: String,
    /// 端口列表
    pub ports: Vec<Port>,
    /// 功能描述（注释）
    pub description: Option<String>,
    /// 来源文件路径
    pub source_file: PathBuf,
    /// 定义起始行（1-based）
    pub line: usize,
}

impl Cell {
    /// 查找指定名称的端口
    pub fn find_port(&self, name: &str) -> Option<&Port> {
        self.ports.iter().find(|p| p.name == name)
    }
}

/// 端口定义
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Port {
    /// 端口名称
    pub name: String,
    /// 方向
    pub direction: PortDirection,
    /// 位宽
    pub width: usize,
}

impl Port {
    /// 创建新端口（默认 input，位宽 1）
    pub fn new(name: String) -> Self {
        Self {
            name,
            direction: PortDirection::Input,
            width: 1,
        }
    }

    /// 方向字符串表示
    pub fn direction_str(&self) -> &'static str {
        match self.direction {
            PortDirection::Input => "input",
            PortDirection::Output => "output",
            PortDirection::Inout => "inout",
        }
    }
}

/// 端口方向
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PortDirection {
    Input,
    Output,
    Inout,
}
