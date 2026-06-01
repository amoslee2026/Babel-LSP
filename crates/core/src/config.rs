//! 项目配置解析
//!
//! 解析 thanosLSP.json 配置文件

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// 项目根配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub project_root: PathBuf,
    pub hdl: HdlConfig,
    pub vhdl: VhdlConfig,
    pub tcl: TclConfig,
    pub classification: ClassificationConfig,
    pub server: ServerConfig,
    pub synth: SynthConfig,
    pub logging: LoggingConfig,
    pub memory: MemoryConfig,
}

/// HDL 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HdlConfig {
    pub filelists: Vec<PathBuf>,
    pub include_paths: Vec<PathBuf>,
    pub defines: Vec<(String, Option<String>)>,
}

/// VHDL 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VhdlConfig {
    pub libraries: Vec<String>,
    pub standard: VhdlStandard,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VhdlStandard {
    #[serde(rename = "93")]
    Vhdl93,
    #[serde(rename = "2002")]
    Vhdl2002,
    #[serde(rename = "2008")]
    Vhdl2008,
}

/// TCL 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TclConfig {
    pub source_paths: Vec<PathBuf>,
    pub eda_tools: Vec<EdaTool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EdaTool {
    Vivado,
    Quartus,
    SynopsysDC,
    SynopsysPT,
    CadenceGenus,
    CadenceInnovus,
}

/// 文件分类配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassificationConfig {
    pub rtl_patterns: Vec<String>,
    pub tb_patterns: Vec<String>,
    pub netlist_patterns: Vec<String>,
}

/// 服务器配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub lsp_port: Option<u16>,
    pub mcp_port: u16,
}

/// 可综合性检查配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynthConfig {
    pub enabled: bool,
    pub rules: Vec<String>,
}

/// 日志配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub file: Option<PathBuf>,
}

/// 记忆配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    pub scan_interval_secs: u64,
    pub persist_path: PathBuf,
}

impl ProjectConfig {
    /// 从文件加载配置
    pub fn from_file(path: &PathBuf) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Self = serde_json::from_str(&content)?;
        Ok(config)
    }

    /// 从项目根目录加载默认配置
    pub fn load_from_root(root: &Path) -> anyhow::Result<Self> {
        let config_path = root.join("thanosLSP.json");
        if config_path.exists() {
            Self::from_file(&config_path)
        } else {
            Self::default_for_root(root)
        }
    }

    /// 为项目根目录创建默认配置
    pub fn default_for_root(root: &Path) -> anyhow::Result<Self> {
        Ok(Self {
            project_root: root.to_path_buf(),
            hdl: HdlConfig {
                filelists: vec![],
                include_paths: vec![],
                defines: vec![],
            },
            vhdl: VhdlConfig {
                libraries: vec!["work".to_string()],
                standard: VhdlStandard::Vhdl2008,
            },
            tcl: TclConfig {
                source_paths: vec![],
                eda_tools: vec![EdaTool::Vivado],
            },
            classification: ClassificationConfig {
                rtl_patterns: vec!["**/*.v".to_string(), "**/*.sv".to_string()],
                tb_patterns: vec!["**/*_tb.*".to_string(), "**/*_test.*".to_string()],
                netlist_patterns: vec!["**/*_netlist.*".to_string()],
            },
            server: ServerConfig {
                lsp_port: Some(6030),
                mcp_port: 3000,
            },
            synth: SynthConfig {
                enabled: true,
                rules: vec!["all".to_string()],
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                file: None,
            },
            memory: MemoryConfig {
                scan_interval_secs: 300,
                persist_path: root.join(".thanos"),
            },
        })
    }
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self::default_for_root(&PathBuf::from(".")).expect("default config should always work")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ProjectConfig::default();
        assert_eq!(config.server.mcp_port, 3000);
        assert!(config.synth.enabled);
    }
}
