#!/bin/bash

# Octaskly Installer for Linux, macOS, WSL, and Termux (Android)
# Supports: Linux distributions, macOS (Intel/Apple Silicon), WSL, Termux
# NOT for native Windows - use install.ps1 with PowerShell

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

is_termux() {
    [ -n "$TERMUX_VERSION" ] || [ -d /data/data/com.termux ]
}

detect_os() {
    if is_termux; then
        OS="termux"
        DISTRO="termux"
        DISTRO_NAME="Termux (Android)"
    else
        case "$OSTYPE" in
            linux-gnu*|linux*)
                OS="linux"
                if [ -f /etc/os-release ]; then
                    . /etc/os-release
                    DISTRO="$ID"
                    DISTRO_NAME="$NAME"
                else
                    DISTRO="unknown"
                    DISTRO_NAME="Linux"
                fi
                ;;
            darwin*)
                OS="macos"
                ARCH=$(uname -m)
                case "$ARCH" in
                    arm64) ARCH_NAME="Apple Silicon" ;;
                    x86_64) ARCH_NAME="Intel" ;;
                    *) ARCH_NAME="Unknown" ;;
                esac
                ;;
            *)
                echo -e "${RED}Unsupported OS: $OSTYPE${NC}"
                echo -e "${RED}For Windows: use install.ps1 with PowerShell instead${NC}"
                exit 1
                ;;
        esac
    fi
}

build_release() {
    echo -e "${BLUE}Building Octaskly release binary...${NC}"
    
    if ! command -v cargo &> /dev/null; then
        echo -e "${RED}✗ Cargo not found. Install Rust: https://rustup.rs${NC}"
        exit 1
    fi
    
    echo -e "${BLUE}Running: cargo build --release${NC}"
    cargo build --release
    
    BINARY="./target/release/octaskly"
    
    if [ ! -f "$BINARY" ]; then
        echo -e "${RED}✗ Build failed${NC}"
        exit 1
    fi
    
    echo -e "${GREEN}✓ Build successful: $BINARY${NC}"
    echo "$BINARY"
}

install_on_linux() {
    local binary=$1
    local install_path="/usr/local/bin"
    
    echo -e "${BLUE}┌─ Installing for Linux ($DISTRO_NAME)${NC}"
    
    if [ ! -w "$install_path" ]; then
        echo -e "${YELLOW}⚠  Need sudo to write to $install_path${NC}"
        sudo cp "$binary" "$install_path/octaskly"
        sudo chmod +x "$install_path/octaskly"
    else
        cp "$binary" "$install_path/octaskly"
        chmod +x "$install_path/octaskly"
    fi
    
    if [ -f ~/.bashrc ]; then
        if ! grep -q "octaskly" ~/.bashrc; then
            echo "export PATH=\"$install_path:\$PATH\"" >> ~/.bashrc
        fi
    fi
    
    if [ -f ~/.zshrc ]; then
        if ! grep -q "octaskly" ~/.zshrc; then
            echo "export PATH=\"$install_path:\$PATH\"" >> ~/.zshrc
        fi
    fi
    
    echo -e "${BLUE}└─${NC}"
}

install_on_macos() {
    local binary=$1
    local install_path="/usr/local/bin"
    
    echo -e "${BLUE}┌─ Installing for macOS ($ARCH_NAME)${NC}"
    
    if [ ! -w "$install_path" ]; then
        echo -e "${YELLOW}⚠  Need sudo to write to $install_path${NC}"
        sudo cp "$binary" "$install_path/octaskly"
        sudo chmod +x "$install_path/octaskly"
    else
        cp "$binary" "$install_path/octaskly"
        chmod +x "$install_path/octaskly"
    fi
    
    for config in ~/.zshrc ~/.bashrc ~/.bash_profile; do
        if [ -f "$config" ]; then
            if ! grep -q "octaskly" "$config"; then
                echo "export PATH=\"$install_path:\$PATH\"" >> "$config"
            fi
        fi
    done
    
    echo -e "${BLUE}└─${NC}"
}

install_on_termux() {
    local binary=$1
    local install_path="${PREFIX:-/data/data/com.termux/files/usr}/bin"
    
    echo -e "${BLUE}┌─ Installing for Termux (Android)${NC}"
    
    if [ ! -w "$install_path" ]; then
        echo -e "${RED}✗ No write permission to $install_path${NC}"
        exit 1
    fi
    
    cp "$binary" "$install_path/octaskly"
    chmod +x "$install_path/octaskly"
    
    echo -e "${BLUE}└─${NC}"
}

main() {
    case "$1" in
        build)
            build_release
            ;;
        install)
            echo ""
            echo -e "${BLUE}╔════════════════════════════════════════╗${NC}"
            echo -e "${BLUE}║    OCTASKLY Installer (Linux/macOS)    ║${NC}"
            echo -e "${BLUE}║         Distributed Task System        ║${NC}"
            echo -e "${BLUE}╚════════════════════════════════════════╝${NC}"
            echo ""
            
            detect_os
            echo -e "${YELLOW}Detected: $DISTRO_NAME ($OS)${NC}"
            echo ""
            
            local binary=""
            
            if [ -f "./target/release/octaskly" ]; then
                binary="./target/release/octaskly"
            else
                echo -e "${YELLOW}Binary not found, building...${NC}"
                binary=$(build_release)
            fi
            
            echo -e "${GREEN}✓ Found binary: $binary${NC}"
            echo ""
            
            case "$OS" in
                linux)
                    install_on_linux "$binary"
                    ;;
                macos)
                    install_on_macos "$binary"
                    ;;
                termux)
                    install_on_termux "$binary"
                    ;;
                *)
                    echo -e "${RED}✗ Unsupported OS: $OS${NC}"
                    exit 1
                    ;;
            esac
            
            echo ""
            echo -e "${GREEN}╔════════════════════════════════════════╗${NC}"
            echo -e "${GREEN}║    Installation Complete! ✓            ║${NC}"
            echo -e "${GREEN}║    Run: octaskly --help                ║${NC}"
            echo -e "${GREEN}╚════════════════════════════════════════╝${NC}"
            echo ""
            
            if command -v octaskly &> /dev/null; then
                echo -e "${GREEN}✓ octaskly is now globally accessible!${NC}"
                echo ""
                octaskly --version || true
            else
                echo -e "${YELLOW}⚠  Restart your shell or run: source ~/.bashrc${NC}"
            fi
            ;;
        *)
            echo -e "${BLUE}Octaskly Installer for Linux, macOS, WSL, and Termux${NC}"
            echo ""
            echo "Usage: bash $0 <command>"
            echo ""
            echo "Commands:"
            echo "  build    - Build release binary (cargo build --release)"
            echo "  install  - Install binary (auto-detects OS and path)"
            echo ""
            echo "Supported platforms:"
            echo "  • Linux (Debian, Ubuntu, RHEL, Arch, Alpine, etc)"
            echo "  • macOS (Intel and Apple Silicon)"
            echo "  • Windows Subsystem for Linux (WSL)"
            echo "  • Termux (Android - \$PREFIX/bin)"
            echo ""
            echo "For Windows native PowerShell: use install.ps1 instead"
            echo ""
            echo "Usage patterns:"
            echo "  curl -fsSL https://octaskly.io/install.sh | sh"
            echo "  bash $0 build"
            echo "  bash $0 install"
            echo ""
            ;;
    esac
}

main "$@"
