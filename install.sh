#!/data/data/com.termux/files/usr/bin/bash
#
# farsi-shell installer for Termux
#
# Usage:
#   curl -sSL https://raw.githubusercontent.com/Argh94/farsi-shell/main/install.sh | bash
#

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

REPO_URL="https://github.com/Argh94/farsi-shell"
RELEASE_URL="${REPO_URL}/releases/latest/download"
INSTALL_DIR="${PREFIX}/bin"

if [ -z "$PREFIX" ]; then
    INSTALL_DIR="/data/data/com.termux/files/usr/bin"
fi

detect_arch() {
    local arch=$(uname -m)
    case "$arch" in
        aarch64|arm64) echo "aarch64" ;;
        armv7*|armv8*|arm) echo "arm" ;;
        x86_64) echo "x86_64" ;;
        i*86) echo "i686" ;;
        *) echo "unknown" ;;
    esac
}

print_banner() {
    echo -e "${BLUE}"
    echo "╔══════════════════════════════════════════════════════════╗"
    echo "║           farsi-shell Installer for Termux              ║"
    echo "╠══════════════════════════════════════════════════════════╣"
    echo "║  Persian/Arabic text display with correct reshaping     ║"
    echo "║  and BiDi support for Termux terminal                   ║"
    echo "╚══════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
}

check_termux() {
    if [ ! -d "/data/data/com.termux" ]; then
        echo -e "${YELLOW}Warning: This installer is designed for Termux.${NC}"
        read -p "Continue anyway? (y/N) " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            exit 1
        fi
    fi
}

check_deps() {
    echo -e "${BLUE}Checking dependencies...${NC}"
    local missing=()
    if ! command -v curl &> /dev/null; then
        missing+=("curl")
    fi
    if ! command -v git &> /dev/null; then
        missing+=("git")
    fi
    if [ ${#missing[@]} -gt 0 ]; then
        echo -e "${YELLOW}Installing missing dependencies: ${missing[*]}${NC}"
        pkg install -y ${missing[*]}
    fi
    echo -e "${GREEN}✓ All dependencies satisfied${NC}"
}

build_from_source() {
    echo -e "${BLUE}Building farsi-shell from source...${NC}"
    if ! command -v cargo &> /dev/null; then
        echo -e "${YELLOW}Installing Rust...${NC}"
        pkg install -y rust
    fi
    if [ ! -d "farsi-shell-src" ]; then
        git clone "${REPO_URL}.git" farsi-shell-src
    fi
    cd farsi-shell-src
    echo -e "${BLUE}Compiling...${NC}"
    cargo build --release
    if [ $? -ne 0 ]; then
        echo -e "${RED}Build failed!${NC}"
        return 1
    fi
    cp target/release/farsi-shell /tmp/farsi-shell
    chmod +x /tmp/farsi-shell
    echo -e "${GREEN}✓ Built successfully${NC}"
    return 0
}

download_binary() {
    local arch=$1
    echo -e "${BLUE}Downloading farsi-shell for ${arch}...${NC}"
    local binary_name="farsi-shell-${arch}-linux-android"
    local download_url="${RELEASE_URL}/${binary_name}"
    local temp_file="/tmp/farsi-shell"
    if curl -sSL -o "$temp_file" "$download_url" 2>/dev/null; then
        chmod +x "$temp_file"
        echo -e "${GREEN}✓ Downloaded successfully${NC}"
        return 0
    fi
    echo -e "${YELLOW}Binary not found. Building from source...${NC}"
    build_from_source
    return $?
}

install_binary() {
    echo -e "${BLUE}Installing farsi-shell to ${INSTALL_DIR}...${NC}"
    mkdir -p "$INSTALL_DIR"
    cp /tmp/farsi-shell "${INSTALL_DIR}/farsi-shell"
    chmod +x "${INSTALL_DIR}/farsi-shell"
    echo -e "${GREEN}✓ Installed to ${INSTALL_DIR}/farsi-shell${NC}"
}

configure_shell() {
    echo -e "${BLUE}Configuring shell...${NC}"
    local shell_rc="$HOME/.bashrc"
    local shell_name=$(basename "$SHELL")
    case "$shell_name" in
        bash) shell_rc="$HOME/.bashrc" ;;
        zsh) shell_rc="$HOME/.zshrc" ;;
        fish) shell_rc="$HOME/.config/fish/config.fish" ;;
    esac
    if grep -q "farsi-shell" "$shell_rc" 2>/dev/null; then
        echo -e "${YELLOW}farsi-shell is already configured in ${shell_rc}${NC}"
        return 0
    fi
    echo ""
    echo -e "${YELLOW}Do you want farsi-shell to start automatically?${NC}"
    read -p "Auto-start? (Y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Nn]$ ]]; then
        echo -e "${BLUE}Skipping auto-start. Run 'farsi-shell' manually.${NC}"
        return 0
    fi
    echo "" >> "$shell_rc"
    echo "# Persian/Arabic text display for Termux" >> "$shell_rc"
    echo 'if [ -z "$FARSI_SHELL" ] && [ -t 0 ] && command -v farsi-shell &> /dev/null; then' >> "$shell_rc"
    echo '    exec farsi-shell' >> "$shell_rc"
    echo 'fi' >> "$shell_rc"
    echo -e "${GREEN}✓ Added farsi-shell to ${shell_rc}${NC}"
}

cleanup() {
    rm -f /tmp/farsi-shell
    rm -rf farsi-shell-src 2>/dev/null
}

main() {
    print_banner
    check_termux
    check_deps
    local arch=$(detect_arch)
    echo -e "${BLUE}Detected architecture: ${arch}${NC}"
    if [ "$arch" = "unknown" ]; then
        echo -e "${RED}Error: Unsupported architecture${NC}"
        exit 1
    fi
    download_binary "$arch"
    install_binary
    configure_shell
    cleanup
    echo ""
    echo -e "${GREEN}╔══════════════════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║           Installation Complete! 🎉                      ║${NC}"
    echo -e "${GREEN}╠══════════════════════════════════════════════════════════╣${NC}"
    echo -e "${GREEN}║  To start: Restart Termux or run 'farsi-shell'          ║${NC}"
    echo -e "${GREEN}║  Persian text will now display correctly!                ║${NC}"
    echo -e "${GREEN}╚══════════════════════════════════════════════════════════╝${NC}"
}

main "$@"
