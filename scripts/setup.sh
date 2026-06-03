#!/usr/bin/env bash
set -euo pipefail

# Babel-LSP 依赖安装和配置脚本
# 安装系统依赖（slang）、编译项目、生成默认配置

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log_info()  { echo -e "${GREEN}[INFO]${NC} $*"; }
log_warn()  { echo -e "${YELLOW}[WARN]${NC} $*"; }
log_error() { echo -e "${RED}[ERROR]${NC} $*"; }

INSTALL_DIR="${HOME}/.local/bin"
SLANG_VERSION="${SLANG_VERSION:-latest}"
PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"

# --------------- 1. 系统依赖 ---------------
install_system_deps() {
    log_info "检查系统依赖..."

    local missing=()
    for cmd in curl tar gcc make; do
        if ! command -v "$cmd" &>/dev/null; then
            missing+=("$cmd")
        fi
    done

    if [ "${#missing[@]}" -gt 0 ]; then
        log_info "安装缺失系统包: ${missing[*]}"
        if command -v apt-get &>/dev/null; then
            sudo apt-get update -qq
            sudo apt-get install -y -qq "${missing[@]}"
        elif command -v dnf &>/dev/null; then
            sudo dnf install -y "${missing[@]}"
        elif command -v pacman &>/dev/null; then
            sudo pacman -S --noconfirm "${missing[@]}"
        else
            log_error "无法自动安装。请手动安装: ${missing[*]}"
            exit 1
        fi
    fi
}

# --------------- 2. Rust 工具链 ---------------
install_rust() {
    if command -v rustc &>/dev/null; then
        local version
        version=$(rustc --version | grep -oP '\d+\.\d+' | head -1)
        if [ "$(printf '%s\n' "1.80" "$version" | sort -V | head -1)" = "1.80" ]; then
            log_info "Rust ${version} >= 1.80, OK"
            return
        fi
        log_warn "Rust ${version} < 1.80, 需要升级"
    fi

    log_info "安装 Rust 工具链..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
    source "${HOME}/.cargo/env"
}

# --------------- 3. slang ---------------
install_slang() {
    if command -v slang &>/dev/null; then
        log_info "slang 已安装: $(slang --version 2>&1 | head -1)"
        return
    fi

    log_info "安装 slang (IEEE 1800-2023 SystemVerilog 解析器)..."

    local arch
    arch=$(uname -m)
    case "$arch" in
        x86_64)  arch="x64" ;;
        aarch64) arch="arm64" ;;
        *) log_error "不支持的架构: $arch"; exit 1 ;;
    esac

    local tmpdir
    tmpdir=$(mktemp -d)
    local url="https://github.com/MikePopoloski/slang/releases/${SLANG_VERSION}/download/slang-linux-${arch}.tar.gz"

    curl -fsSL "$url" -o "$tmpdir/slang.tar.gz"
    tar xzf "$tmpdir/slang.tar.gz" -C "$tmpdir"
    mkdir -p "$INSTALL_DIR"
    cp "$tmpdir"/bin/slang "$INSTALL_DIR/"
    rm -rf "$tmpdir"

    if command -v slang &>/dev/null; then
        log_info "slang 安装完成: $(slang --version 2>&1 | head -1)"
    else
        log_warn "slang 安装完成但不在 PATH 中，请将 ${INSTALL_DIR} 加入 PATH"
    fi
}

# --------------- 4. 可选工具 ---------------
install_optional() {
    # verible (SV formatter)
    if ! command -v verible-verilog-format &>/dev/null; then
        log_info "安装 verible (CHIPS Alliance formatter)..."
        local tmpdir
        tmpdir=$(mktemp -d)
        curl -fsSL "https://github.com/chipsalliance/verible/releases/latest/download/verible-bin-linux-static.tar.gz" \
            -o "$tmpdir/verible.tar.gz" 2>/dev/null && {
            tar xzf "$tmpdir/verible.tar.gz" -C "$tmpdir"
            cp "$tmpdir"/verible/bin/verible-verilog-* "$INSTALL_DIR/" 2>/dev/null || true
        } || log_warn "verible 安装失败（可选，不影响核心功能）"
        rm -rf "$tmpdir"
    fi

    # nagelfar (TCL linter)
    if ! command -v nagelfar &>/dev/null; then
        log_info "nagelfar 可在 https://nagelfar.sourceforge.net/ 下载（可选 TCL 深度诊断）"
    fi
}

# --------------- 5. 编译项目 ---------------
build_project() {
    log_info "编译 Babel-LSP..."
    cd "$PROJECT_ROOT"
    cargo build --release
    mkdir -p "$INSTALL_DIR"
    cp target/release/thanosLSP "$INSTALL_DIR/Babel-LSP"
    log_info "二进制已安装到: ${INSTALL_DIR}/Babel-LSP"
}

# --------------- 6. 生成默认配置 ---------------
generate_config() {
    local config_path="${1:-${PROJECT_ROOT}/Babel-LSP.json}"

    if [ -f "$config_path" ]; then
        log_warn "配置文件已存在: $config_path，跳过生成"
        return
    fi

    log_info "生成默认配置文件: $config_path"

    cat > "$config_path" <<'EOF'
{
  "project": {
    "name": ""
  },
  "hdl": {
    "source_dirs": ["src/**/*.sv", "src/**/*.v"],
    "defines": [],
    "include_dirs": ["src/include"],
    "filelists": [],
    "cell_libraries": []
  },
  "vhdl": {
    "source_dirs": ["src/**/*.vhd"],
    "libraries": {"work": "src"}
  },
  "tcl": {
    "source_dirs": ["scripts/**/*.tcl"],
    "eda_tools": ["synopsys_dc", "vivado"]
  },
  "classification": {
    "rtl_patterns": ["src/rtl/**"],
    "tb_patterns": ["tb/**", "sim/**"],
    "netlist_patterns": ["netlist/**"]
  },
  "server": {
    "lsp_port": 6030,
    "mcp_port": 3000,
    "mcp_token": null,
    "diagnostic_trigger": "on_change",
    "max_threads": 0
  },
  "synth": {
    "rules": "all",
    "rtl_patterns": ["src/rtl/**/*.sv", "src/rtl/**/*.vhd"],
    "tb_patterns": ["tb/**", "sim/**"],
    "netlist_patterns": ["netlist/**"]
  },
  "logging": {
    "level": "warn",
    "file": null,
    "rotation": "daily",
    "max_files": 7
  },
  "memory": {
    "scan_interval_minutes": 5
  }
}
EOF
}

# --------------- 7. PATH 配置 ---------------
configure_path() {
    if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
        log_info "将 ${INSTALL_DIR} 加入 PATH..."
        local rc_file=""
        case "$(basename "$SHELL")" in
            zsh)  rc_file="${HOME}/.zshrc" ;;
            bash) rc_file="${HOME}/.bashrc" ;;
            *)    rc_file="${HOME}/.profile" ;;
        esac

        if ! grep -q "$INSTALL_DIR" "$rc_file" 2>/dev/null; then
            echo "export PATH=\"${INSTALL_DIR}:\$PATH\"" >> "$rc_file"
        fi
        log_info "已写入 ${rc_file}，运行 source ${rc_file} 或重新打开终端使其生效"
    fi
}

# --------------- 8. 验证 ---------------
verify() {
    log_info "验证安装..."
    local ok=true

    if ! command -v Babel-LSP &>/dev/null; then
        export PATH="${INSTALL_DIR}:${PATH}"
    fi

    echo ""
    log_info "=== Babel-LSP 安装验证 ==="

    Babel-LSP --version 2>/dev/null && echo "  ✓ Babel-LSP" || { log_error "  ✗ Babel-LSP 未找到"; ok=false; }
    slang --version 2>/dev/null     && echo "  ✓ slang"       || log_warn  "  ! slang 未安装（SV 分析需要）"
    rustc --version 2>/dev/null     && echo "  ✓ rustc"       || log_warn  "  ! rustc"
    echo ""

    if $ok; then
        log_info "验证通过！"
    else
        log_error "部分组件缺失，请检查错误信息"
    fi
}

# --------------- main ---------------
main() {
    echo ""
    echo "╔══════════════════════════════════════╗"
    echo "║   Babel-LSP 安装配置脚本 v2.0       ║"
    echo "╚══════════════════════════════════════╝"
    echo ""

    install_system_deps
    install_rust
    install_slang
    install_optional
    build_project
    generate_config "$@"
    configure_path
    verify

    log_info "全部完成！"
    log_info "快速开始:"
    echo "  Babel-LSP --version"
    echo "  Babel-LSP daemon --config ./Babel-LSP.json"
    echo "  Babel-LSP status"
}

main "$@"
