//! slang AST JSON 节点类型及解析

use anyhow::{Context, Result};
use serde::Deserialize;

/// AST 节点（对应 slang --ast-json 输出格式）
#[derive(Debug, Clone, Deserialize)]
pub struct AstNode {
    /// 节点类型（如 "CompilationUnit"、"Module"、"Port" 等）
    pub kind: String,
    /// 节点名称（可选，如模块名、端口名等）
    pub name: Option<String>,
    /// 起始行（1-based，来自 slang）
    #[serde(default)]
    pub start_line: u32,
    /// 起始列（1-based）
    #[serde(default)]
    pub start_col: u32,
    /// 结束行（1-based）
    #[serde(default)]
    pub end_line: u32,
    /// 结束列（1-based）
    #[serde(default)]
    pub end_col: u32,
    /// 子节点
    #[serde(default)]
    pub children: Vec<AstNode>,
}

impl AstNode {
    /// 按 kind 查找所有直接子节点
    pub fn find_children_by_kind(&self, kind: &str) -> Vec<&AstNode> {
        self.children.iter().filter(|c| c.kind == kind).collect()
    }

    /// 深度优先搜索，找到第一个匹配 kind 的节点
    pub fn find_first_by_kind(&self, kind: &str) -> Option<&AstNode> {
        if self.kind == kind {
            return Some(self);
        }
        for child in &self.children {
            if let Some(found) = child.find_first_by_kind(kind) {
                return Some(found);
            }
        }
        None
    }

    /// 深度优先遍历所有节点（包括自身）
    pub fn walk<F>(&self, visitor: &mut F)
    where
        F: FnMut(&AstNode),
    {
        visitor(self);
        for child in &self.children {
            child.walk(visitor);
        }
    }

    /// 查找所有匹配 kind 的节点（深度优���）
    pub fn find_all_by_kind(&self, kind: &str) -> Vec<&AstNode> {
        let mut result = Vec::new();
        self.walk(&mut |node| {
            if node.kind == kind {
                // 安全：walk 借用的是 &self，result 收集引用
                // 这里需要绕过借用检查，用原始指针技巧
            }
            let _ = node;
        });
        // 使用递归版本避免借用问题
        self.collect_by_kind(kind, &mut result);
        result
    }

    fn collect_by_kind<'a>(&'a self, kind: &str, out: &mut Vec<&'a AstNode>) {
        if self.kind == kind {
            out.push(self);
        }
        for child in &self.children {
            child.collect_by_kind(kind, out);
        }
    }

    /// 获取节点的显示名称
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or(&self.kind)
    }
}

/// AST JSON 解析器
pub struct AstJsonParser;

impl AstJsonParser {
    pub fn new() -> Self {
        Self
    }

    /// 从 JSON 字符串解析 AST
    pub fn parse(&self, json: &str) -> Result<AstNode> {
        serde_json::from_str(json).context("解析 AST JSON 失败")
    }
}

impl Default for AstJsonParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_json() -> &'static str {
        r#"{
            "kind": "CompilationUnit",
            "name": null,
            "start_line": 1,
            "start_col": 1,
            "end_line": 20,
            "end_col": 1,
            "children": [
                {
                    "kind": "Module",
                    "name": "top",
                    "start_line": 1,
                    "start_col": 1,
                    "end_line": 10,
                    "end_col": 10,
                    "children": [
                        {
                            "kind": "Port",
                            "name": "clk",
                            "start_line": 2,
                            "start_col": 5,
                            "end_line": 2,
                            "end_col": 8,
                            "children": []
                        },
                        {
                            "kind": "Port",
                            "name": "rst",
                            "start_line": 3,
                            "start_col": 5,
                            "end_line": 3,
                            "end_col": 8,
                            "children": []
                        }
                    ]
                }
            ]
        }"#
    }

    #[test]
    fn test_parse_valid_json() {
        let parser = AstJsonParser::new();
        let node = parser.parse(make_test_json()).unwrap();
        assert_eq!(node.kind, "CompilationUnit");
        assert_eq!(node.children.len(), 1);
        assert_eq!(node.children[0].kind, "Module");
        assert_eq!(node.children[0].name.as_deref(), Some("top"));
    }

    #[test]
    fn test_find_children_by_kind() {
        let parser = AstJsonParser::new();
        let root = parser.parse(make_test_json()).unwrap();
        let modules = root.find_children_by_kind("Module");
        assert_eq!(modules.len(), 1);
        let ports = modules[0].find_children_by_kind("Port");
        assert_eq!(ports.len(), 2);
    }

    #[test]
    fn test_find_first_by_kind() {
        let parser = AstJsonParser::new();
        let root = parser.parse(make_test_json()).unwrap();
        let port = root.find_first_by_kind("Port");
        assert!(port.is_some());
        assert_eq!(port.unwrap().name.as_deref(), Some("clk"));
    }

    #[test]
    fn test_walk_visits_all() {
        let parser = AstJsonParser::new();
        let root = parser.parse(make_test_json()).unwrap();
        let mut count = 0;
        root.walk(&mut |_| count += 1);
        // CompilationUnit + Module + Port*2 = 4
        assert_eq!(count, 4);
    }

    #[test]
    fn test_find_all_by_kind() {
        let parser = AstJsonParser::new();
        let root = parser.parse(make_test_json()).unwrap();
        let ports = root.find_all_by_kind("Port");
        assert_eq!(ports.len(), 2);
        assert_eq!(ports[0].name.as_deref(), Some("clk"));
        assert_eq!(ports[1].name.as_deref(), Some("rst"));
    }

    #[test]
    fn test_parse_invalid_json() {
        let parser = AstJsonParser::new();
        assert!(parser.parse("{invalid json}").is_err());
    }

    #[test]
    fn test_find_nonexistent_kind() {
        let parser = AstJsonParser::new();
        let root = parser.parse(make_test_json()).unwrap();
        assert!(root.find_first_by_kind("NonExistent").is_none());
        assert!(root.find_children_by_kind("NonExistent").is_empty());
    }
}
