# Babel-LSP

HDL Language Server 提供 VHDL/Verilog/SystemVerilog/TCL 语言支持的 LSP + MCP 服务，专为 ASIC/FPGA 芯片设计工作流打造。

[![Rust](https://img.shields.io/badge/rust-1.80+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## 概述

Babel-LSP 是一个基于 Rust 的单进程双协议语言服务器，同时提供 **标准 LSP 接口**（可供 VSCode/Neovim/Zed 等编辑器使用）和 **MCP SSE 接口**（供 Claude Code AI Agent 调用）。两套接口共享同一内存符号表，覆盖从 AI 驱动开发到传统 IDE 编辑的全场景工作流。

### 核心能力

- **HDL 语义分析**：实时诊断、跳转定义、查找引用、智能补全、悬停信息、格式化
- **Filelist 解析**：支持 Synopsys `.f` 和 Cadence filelist 格式
- **标准单元库索引**：解析 Verilog gate-level model，提供 cell pin/function/timing 信息
- **文件三分类**：自动识别 RTL / Testbench / Netlist 文件类型
- **跨语言引用**：HDL ↔ TCL 符号跳转与补全
- **Synopsys DC/PT TCL API**：EDA 工具脚本的专用命令补全
- **代码编辑**：通过 MCP 接口实现符号级代码修改（重命名、替换、插入等）
- **项目持久记忆**：基于 redb 的增量分析缓存，项目重开秒级恢复

### 性能目标

| 指标 | 目标 |
|------|------|
| 单文件诊断延迟 | < 200ms（CLI）/ < 50ms（FFI） |
| 项目初始化（1000 文件） | < 5s |
| 重启恢复（缓存命中） | < 1s |
| 内存占用（10000 文件） | < 500MB |

## 架构

```
crates/
├── core/              # 共享层：配置、文档状态、符号类型、诊断、文件存储、项目索引
├── sv-analyzer/       # SystemVerilog/Verilog 分析器（slang 驱动）
├── vhdl-analyzer/     # VHDL 分析器（vhdl_lang 驱动）
├── tcl-analyzer/      # TCL 分析器（tree-sitter + EDA 命令补全）
├── filelist-parser/   # Synopsys .f / Cadence filelist 解析器
├── cell-library/      # 标准单元库 gate-level model 解析索引
├── lsp-router/        # LSP 协议处理层（tower-lsp stdio）
├── mcp-server/        # MCP 协议处理层（rmcp SSE localhost）
└── babel-lsp/         # 可执行入口，组装运行
```

## 快速开始

### 依赖

- Rust 1.80+
- [slang](https://github.com/MikePopoloski/slang)（SV 解析引擎）
- Linux x86_64 / aarch64

### 构建

```bash
cargo build --release
```

### LSP 模式

在编辑器中配置 LSP 客户端连接：

```json
{
  "lsp": {
    "babel-lsp": {
      "command": "babel-lsp",
      "args": ["--mode", "lsp"]
    }
  }
}
```

### MCP 模式（Claude Code）

```json
{
  "mcpServers": {
    "babel-lsp": {
      "command": "babel-lsp",
      "args": ["--mode", "mcp"]
    }
  }
}
```

### 配置

项目根目录放置 `Babel-LSP.json`：

```json
{
  "filelist": ["flist.f", "filelist.f"],
  "cell_libraries": ["/path/to/standard_cells.v"],
  "include_dirs": ["src/", "inc/"]
}
```

## 参考项目

| 项目 | 用途 |
|------|------|
| [slang](https://github.com/MikePopoloski/slang) | SystemVerilog 解析前端 |
| [verible](https://github.com/chipsalliance/verible) | CHIPS Alliance lint + format |
| [verilator](https://github.com/verilator/verilator) | 编译 + lint |
| [vhdl_lang](https://github.com/VHDL/vhdl_lang) | VHDL 解析 |
| [iverilog](https://github.com/steveicarus/iverilog) | Icarus Verilog |

## 开发

```bash
cargo test
cargo fmt --all -- --check
cargo clippy -- -D warnings
```

### 技术栈

| 决策 | 选择 | 原因 |
|------|------|------|
| 语言 | Rust + Tokio async | 性能、类型安全、生态系统 |
| SV 解析 | slang | IEEE 1800-2023 完整支持 |
| LSP 协议 | tower-lsp | 单客户端 stdio |
| 持久化 | redb | 嵌入式 KV 存储 |
| 增量计算 | Salsa | 响应式增量重分析 |
| MCP 传输 | rmcp SSE | localhost 单用户 |

## 许可证

MIT
