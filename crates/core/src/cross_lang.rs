//! 跨语言符号映射
//!
//! HDL ↔ TCL 跨语言引用解析

use std::collections::HashMap;

/// 跨语言引用类型
#[derive(Debug, Clone)]
pub enum CrossLangRef {
    /// TCL 文件引用 HDL 文件
    TclToHdl {
        tcl_file: String,
        hdl_file: String,
        relation: String,
    },
    /// HDL 文件被 TCL 引用
    HdlToTcl {
        hdl_file: String,
        tcl_file: String,
        relation: String,
    },
}

/// 跨语言索引
pub struct CrossLangIndex {
    /// TCL → HDL 映射
    tcl_to_hdl: HashMap<String, Vec<String>>,
    /// HDL → TCL 映射
    hdl_to_tcl: HashMap<String, Vec<String>>,
    /// 引用详情
    references: Vec<CrossLangRef>,
}

impl CrossLangIndex {
    /// 创建空索引
    pub fn new() -> Self {
        Self {
            tcl_to_hdl: HashMap::new(),
            hdl_to_tcl: HashMap::new(),
            references: vec![],
        }
    }

    /// 添加引用
    pub fn add_reference(&mut self, reference: CrossLangRef) {
        match &reference {
            CrossLangRef::TclToHdl {
                tcl_file, hdl_file, ..
            } => {
                self.tcl_to_hdl
                    .entry(tcl_file.clone())
                    .or_default()
                    .push(hdl_file.clone());
                self.hdl_to_tcl
                    .entry(hdl_file.clone())
                    .or_default()
                    .push(tcl_file.clone());
            },
            CrossLangRef::HdlToTcl {
                hdl_file, tcl_file, ..
            } => {
                self.hdl_to_tcl
                    .entry(hdl_file.clone())
                    .or_default()
                    .push(tcl_file.clone());
                self.tcl_to_hdl
                    .entry(tcl_file.clone())
                    .or_default()
                    .push(hdl_file.clone());
            },
        }
        self.references.push(reference);
    }

    /// 获取 TCL 文件引用的 HDL 文件
    pub fn hdl_files_for_tcl(&self, tcl_file: &str) -> Option<&Vec<String>> {
        self.tcl_to_hdl.get(tcl_file)
    }

    /// 获取 HDL 文件被哪些 TCL 文件引用
    pub fn tcl_files_for_hdl(&self, hdl_file: &str) -> Option<&Vec<String>> {
        self.hdl_to_tcl.get(hdl_file)
    }

    /// 获取所有引用
    pub fn references(&self) -> &[CrossLangRef] {
        &self.references
    }

    /// 清除索引
    pub fn clear(&mut self) {
        self.tcl_to_hdl.clear();
        self.hdl_to_tcl.clear();
        self.references.clear();
    }
}

impl Default for CrossLangIndex {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cross_lang_index() {
        let mut index = CrossLangIndex::new();
        index.add_reference(CrossLangRef::TclToHdl {
            tcl_file: "run.tcl".to_string(),
            hdl_file: "design.sv".to_string(),
            relation: "read_verilog".to_string(),
        });

        assert!(index.hdl_files_for_tcl("run.tcl").is_some());
        assert!(index.tcl_files_for_hdl("design.sv").is_some());
    }

    #[test]
    fn test_bidirectional_mapping() {
        let mut index = CrossLangIndex::new();
        index.add_reference(CrossLangRef::TclToHdl {
            tcl_file: "synth.tcl".to_string(),
            hdl_file: "top.sv".to_string(),
            relation: "read_verilog".to_string(),
        });
        index.add_reference(CrossLangRef::TclToHdl {
            tcl_file: "synth.tcl".to_string(),
            hdl_file: "sub.sv".to_string(),
            relation: "read_verilog".to_string(),
        });

        let hdl_files = index.hdl_files_for_tcl("synth.tcl").unwrap();
        assert_eq!(hdl_files.len(), 2);
        assert!(hdl_files.contains(&"top.sv".to_string()));
        assert!(hdl_files.contains(&"sub.sv".to_string()));

        let tcl_files = index.tcl_files_for_hdl("top.sv").unwrap();
        assert_eq!(tcl_files.len(), 1);
        assert_eq!(tcl_files[0], "synth.tcl");
    }

    #[test]
    fn test_clear_index() {
        let mut index = CrossLangIndex::new();
        index.add_reference(CrossLangRef::TclToHdl {
            tcl_file: "run.tcl".to_string(),
            hdl_file: "design.sv".to_string(),
            relation: "read_verilog".to_string(),
        });
        assert_eq!(index.references().len(), 1);
        index.clear();
        assert_eq!(index.references().len(), 0);
        assert!(index.hdl_files_for_tcl("run.tcl").is_none());
    }

    #[test]
    fn test_hdl_to_tcl_variant() {
        let mut index = CrossLangIndex::new();
        index.add_reference(CrossLangRef::HdlToTcl {
            hdl_file: "design.sv".to_string(),
            tcl_file: "constraints.tcl".to_string(),
            relation: "constraint".to_string(),
        });
        assert!(index.tcl_files_for_hdl("design.sv").is_some());
        assert!(index.hdl_files_for_tcl("constraints.tcl").is_some());
    }
}
