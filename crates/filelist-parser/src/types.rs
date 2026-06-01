//! Filelist 解析器数据类型定义

use std::collections::HashSet;
use std::path::PathBuf;

/// 解析选项
#[derive(Debug, Clone)]
pub struct ParseOptions {
    /// 基础路径（相对路径解析基准）
    pub base_path: PathBuf,
    /// 最大递归深度
    pub max_depth: u32,
    /// 是否启用 Cadence 扩展
    pub enable_cadence: bool,
    /// 已访问文件集合（循环检测）
    pub visited: HashSet<PathBuf>,
}

impl Default for ParseOptions {
    fn default() -> Self {
        Self {
            base_path: PathBuf::from("."),
            max_depth: 50,
            enable_cadence: false,
            visited: HashSet::new(),
        }
    }
}

impl ParseOptions {
    /// 创建新的解析选项
    pub fn new(base_path: PathBuf) -> Self {
        Self {
            base_path,
            max_depth: 50,
            enable_cadence: false,
            visited: HashSet::new(),
        }
    }

    /// 启用 Cadence 扩展
    pub fn with_cadence(mut self) -> Self {
        self.enable_cadence = true;
        self
    }

    /// 设置最大递归深度
    pub fn with_max_depth(mut self, depth: u32) -> Self {
        self.max_depth = depth;
        self
    }
}

/// 解析结果
#[derive(Debug, Default, Clone)]
pub struct FilelistResult {
    /// 源文件列表（含 library 归属）
    pub source_files: Vec<SourceFileInfo>,
    /// Include 搜索路径
    pub include_dirs: Vec<PathBuf>,
    /// 宏定义
    pub macro_defines: Vec<MacroDefine>,
    /// 库文件（-v）
    pub library_files: Vec<PathBuf>,
    /// 库目录（-y）
    pub library_dirs: Vec<PathBuf>,
    /// 库文件扩展名（+libext+）
    pub library_extensions: Vec<String>,
    /// 解析过程中的 Warning
    pub warnings: Vec<ParseWarning>,
}

impl FilelistResult {
    /// 合并另一个解析结果
    pub fn merge(&mut self, other: FilelistResult) {
        self.source_files.extend(other.source_files);
        self.include_dirs.extend(other.include_dirs);
        self.macro_defines.extend(other.macro_defines);
        self.library_files.extend(other.library_files);
        self.library_dirs.extend(other.library_dirs);
        self.library_extensions.extend(other.library_extensions);
        self.warnings.extend(other.warnings);
    }

    /// 添加源文件
    pub fn add_source_file(&mut self, path: PathBuf, library: Option<String>) {
        self.source_files.push(SourceFileInfo { path, library });
    }

    /// 添加 include 目录
    pub fn add_include_dir(&mut self, path: PathBuf) {
        self.include_dirs.push(path);
    }

    /// 添加宏定义
    pub fn add_macro_define(&mut self, name: String, value: Option<String>) {
        self.macro_defines.push(MacroDefine { name, value });
    }

    /// 添加库文件
    pub fn add_library_file(&mut self, path: PathBuf) {
        self.library_files.push(path);
    }

    /// 添加库目录
    pub fn add_library_dir(&mut self, path: PathBuf) {
        self.library_dirs.push(path);
    }

    /// 添加库扩展名
    pub fn add_library_extensions(&mut self, exts: Vec<String>) {
        self.library_extensions.extend(exts);
    }

    /// 添加警告
    pub fn add_warning(&mut self, warning: ParseWarning) {
        self.warnings.push(warning);
    }
}

/// 源文件信息
#[derive(Debug, Clone)]
pub struct SourceFileInfo {
    /// 文件路径
    pub path: PathBuf,
    /// Cadence -makelib 归属
    pub library: Option<String>,
}

/// 宏定义
#[derive(Debug, Clone)]
pub struct MacroDefine {
    /// 宏名称
    pub name: String,
    /// 宏值（可选）
    pub value: Option<String>,
}

/// 解析警告
#[derive(Debug, Clone)]
pub enum ParseWarning {
    /// 环境变量未定义
    EnvUndefined { var_name: String, line: u32 },
    /// 文件路径不存在
    PathNotFound { path: PathBuf, line: u32 },
    /// 格式错误的行
    MalformedLine { content: String, line: u32 },
}

/// 解析行结果
#[derive(Debug, Clone)]
pub enum ParsedLine {
    /// 空行
    Empty,
    /// 源文件路径
    SourceFile(PathBuf),
    /// 嵌套 filelist（-f）
    NestedFilelist(PathBuf),
    /// 库文件（-v）
    LibraryFile(PathBuf),
    /// 库目录（-y）
    LibraryDir(PathBuf),
    /// Include 目录（+incdir+）
    IncludeDir(PathBuf),
    /// 宏定义（+define+）
    MacroDefine(MacroDefine),
    /// 库扩展名（+libext+）
    LibExtensions(Vec<String>),
    /// Cadence 库开始（-makelib）
    MakeLib(String),
    /// Cadence 库结束（-endlib）
    EndLib,
    /// 未知选项（作为源文件处理）
    UnknownOption(String),
}
