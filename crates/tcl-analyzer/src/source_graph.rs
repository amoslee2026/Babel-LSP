//! TCL source/include 依赖图

use std::collections::{HashMap, HashSet, VecDeque};

/// source 依赖图（有向图）
#[derive(Debug)]
pub struct SourceGraph {
    /// file -> 它 source 了哪些文件
    deps: HashMap<String, Vec<String>>,
}

impl SourceGraph {
    pub fn new() -> Self {
        Self {
            deps: HashMap::new(),
        }
    }

    /// 添加文件及其直接依赖（source 的文件列表）
    pub fn add_file(&mut self, path: &str, sources: Vec<String>) {
        self.deps.insert(path.to_string(), sources);
    }

    /// 获取某文件的直接依赖（它 source 的文件）
    pub fn get_dependencies(&self, path: &str) -> Vec<String> {
        self.deps.get(path).cloned().unwrap_or_default()
    }

    /// 获取依赖某文件的文件（谁 source 了它）
    pub fn get_dependents(&self, path: &str) -> Vec<String> {
        self.deps
            .iter()
            .filter_map(|(file, deps)| {
                if deps.contains(&path.to_string()) {
                    Some(file.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    /// 检测是否有循环依赖
    pub fn has_cycle(&self) -> bool {
        // DFS + 染色（白=0, gray=1, black=2）
        let mut color: HashMap<&str, u8> = HashMap::new();

        for start in self.deps.keys() {
            if !color.contains_key(start.as_str()) && self.dfs_has_cycle(start, &mut color) {
                return true;
            }
        }
        false
    }

    fn dfs_has_cycle<'a>(&'a self, node: &'a str, color: &mut HashMap<&'a str, u8>) -> bool {
        color.insert(node, 1); // gray = in progress
        if let Some(deps) = self.deps.get(node) {
            for dep in deps {
                let dep_str = dep.as_str();
                match color.get(dep_str).copied() {
                    Some(1) => return true, // back edge → cycle
                    Some(2) => continue,    // already fully processed
                    _ => {
                        if self.dfs_has_cycle(dep_str, color) {
                            return true;
                        }
                    },
                }
            }
        }
        color.insert(node, 2); // black = done
        false
    }

    /// 拓扑排序（Kahn 算法），返回依赖优先顺序（依赖先出现，使用者后出现）。
    /// 若存在环则返回 None。
    pub fn topological_order(&self) -> Option<Vec<String>> {
        if self.has_cycle() {
            return None;
        }

        // 收集所有节点
        let mut all_nodes: HashSet<String> = HashSet::new();
        for (file, deps) in &self.deps {
            all_nodes.insert(file.clone());
            for d in deps {
                all_nodes.insert(d.clone());
            }
        }

        // 使用逆图：edge  file→dep  变为 dep→file
        // 这样拥有 0 个依赖的节点（叶子）最先弹出
        // in_degree[node] = 该节点依赖的文件数（即 deps list 的长度）
        let mut in_degree: HashMap<String, usize> =
            all_nodes.iter().map(|n| (n.clone(), 0usize)).collect();
        // 逆邻接表：dep → [files that depend on dep]
        let mut reverse_adj: HashMap<String, Vec<String>> =
            all_nodes.iter().map(|n| (n.clone(), Vec::new())).collect();

        for (file, deps) in &self.deps {
            *in_degree.get_mut(file).unwrap() = deps.len();
            for dep in deps {
                reverse_adj
                    .entry(dep.clone())
                    .or_default()
                    .push(file.clone());
            }
        }

        // 把入度为 0 的节点加入队列（它们没有未满足的依赖）
        let mut queue_vec: Vec<String> = in_degree
            .iter()
            .filter(|(_, &deg)| deg == 0)
            .map(|(n, _)| n.clone())
            .collect();
        queue_vec.sort();
        let mut queue: VecDeque<String> = queue_vec.into();

        let mut order = Vec::new();
        while let Some(node) = queue.pop_front() {
            order.push(node.clone());
            // 所有依赖 node 的 files，in_degree 减 1
            if let Some(dependents) = reverse_adj.get(&node) {
                let mut next_batch: Vec<String> = Vec::new();
                for dependent in dependents {
                    let deg = in_degree.get_mut(dependent).unwrap();
                    *deg -= 1;
                    if *deg == 0 {
                        next_batch.push(dependent.clone());
                    }
                }
                next_batch.sort();
                for n in next_batch {
                    queue.push_back(n);
                }
            }
        }

        if order.len() == all_nodes.len() {
            Some(order)
        } else {
            None
        }
    }

    /// 获取图中所有文件路径
    pub fn all_files(&self) -> Vec<String> {
        self.deps.keys().cloned().collect()
    }
}

impl Default for SourceGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_simple_graph() -> SourceGraph {
        let mut g = SourceGraph::new();
        g.add_file(
            "main.tcl",
            vec!["utils.tcl".to_string(), "config.tcl".to_string()],
        );
        g.add_file("utils.tcl", vec!["helpers.tcl".to_string()]);
        g.add_file("config.tcl", vec![]);
        g.add_file("helpers.tcl", vec![]);
        g
    }

    #[test]
    fn test_add_file() {
        let g = build_simple_graph();
        assert!(g.all_files().contains(&"main.tcl".to_string()));
        assert!(g.all_files().contains(&"utils.tcl".to_string()));
    }

    #[test]
    fn test_get_dependencies() {
        let g = build_simple_graph();
        let deps = g.get_dependencies("main.tcl");
        assert!(deps.contains(&"utils.tcl".to_string()));
        assert!(deps.contains(&"config.tcl".to_string()));
    }

    #[test]
    fn test_get_dependencies_empty() {
        let g = build_simple_graph();
        let deps = g.get_dependencies("helpers.tcl");
        assert!(deps.is_empty());
    }

    #[test]
    fn test_get_dependents() {
        let g = build_simple_graph();
        let dependents = g.get_dependents("utils.tcl");
        assert!(dependents.contains(&"main.tcl".to_string()));
    }

    #[test]
    fn test_no_cycle() {
        let g = build_simple_graph();
        assert!(!g.has_cycle());
    }

    #[test]
    fn test_cycle_detection() {
        let mut g = SourceGraph::new();
        g.add_file("a.tcl", vec!["b.tcl".to_string()]);
        g.add_file("b.tcl", vec!["c.tcl".to_string()]);
        g.add_file("c.tcl", vec!["a.tcl".to_string()]); // cycle
        assert!(g.has_cycle());
    }

    #[test]
    fn test_self_cycle() {
        let mut g = SourceGraph::new();
        g.add_file("a.tcl", vec!["a.tcl".to_string()]);
        assert!(g.has_cycle());
    }

    #[test]
    fn test_topological_order_no_cycle() {
        let g = build_simple_graph();
        let order = g.topological_order();
        assert!(
            order.is_some(),
            "acyclic graph should have topological order"
        );
        let order = order.unwrap();
        // helpers.tcl and config.tcl must come before utils.tcl and main.tcl respectively
        let pos = |name: &str| order.iter().position(|s| s == name).unwrap();
        assert!(
            pos("helpers.tcl") < pos("utils.tcl"),
            "helpers before utils"
        );
        assert!(pos("utils.tcl") < pos("main.tcl"), "utils before main");
        assert!(pos("config.tcl") < pos("main.tcl"), "config before main");
    }

    #[test]
    fn test_topological_order_with_cycle() {
        let mut g = SourceGraph::new();
        g.add_file("a.tcl", vec!["b.tcl".to_string()]);
        g.add_file("b.tcl", vec!["a.tcl".to_string()]);
        assert!(g.topological_order().is_none());
    }

    #[test]
    fn test_unknown_file_dependencies() {
        let g = build_simple_graph();
        let deps = g.get_dependencies("nonexistent.tcl");
        assert!(deps.is_empty());
    }

    #[test]
    fn test_unknown_file_dependents() {
        let g = build_simple_graph();
        let deps = g.get_dependents("nonexistent.tcl");
        assert!(deps.is_empty());
    }
}
