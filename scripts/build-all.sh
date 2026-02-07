#!/bin/bash
set -e
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$SCRIPT_DIR/.."
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'
echo -e "${BLUE}╔════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║      Octaskly Universal Installer Builder          ║${NC}"
echo -e "${BLUE}║      Builds native installers for all platforms    ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════╝${NC}"
echo ""
detect_os() {
    case "$OSTYPE" in
        linux-gnu*|linux*)
            echo "linux"
            ;;
        darwin*)
            echo "macos"
            ;;
        msys|cygwin)
            echo "windows"
            ;;
        *)
            echo "unknown"
            ;;
    esac
}
CURRENT_OS=$(detect_os)
echo -e "${YELLOW}Detected OS: $CURRENT_OS${NC}"
echo ""
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}✗ Cargo not found. Install Rust:${NC}"
    echo "  https://rustup.rs"
    exit 1
fi
echo -e "${YELLOW}Building release binary...${NC}"
cd "$PROJECT_ROOT"
cargo build --release
echo -e "${GREEN}✓ Binary ready${NC}"
echo ""
mkdir -p "$PROJECT_ROOT/dist/windows"
mkdir -p "$PROJECT_ROOT/dist/macos"
mkdir -p "$PROJECT_ROOT/dist/linux"
if [ "$CURRENT_OS" = "windows" ]; then
    echo -e "${YELLOW}Building Windows installer (Inno Setup)...${NC}"
    echo -e "${YELLOW}Note: Requires Inno Setup to be installed${NC}"
    echo "  Download from: https://jrsoftware.com/isdl.php"
    echo ""
    echo "Usage:"
    echo "  iscc installer/octaskly-installer.iss"
    echo ""
elif [ "$CURRENT_OS" = "macos" ]; then
    echo -e "${YELLOW}Building macOS installer (.pkg)...${NC}"
    bash "$SCRIPT_DIR/build-macos-pkg.sh"
elif [ "$CURRENT_OS" = "linux" ]; then
    echo -e "${YELLOW}Building Linux installers...${NC}"
    if command -v dpkg &> /dev/null; then
        echo ""
        echo -e "${YELLOW}Building .deb package...${NC}"
        bash "$SCRIPT_DIR/build-linux-deb.sh" amd64
    fi
    if command -v rpmbuild &> /dev/null; then
        echo ""
        echo -e "${YELLOW}Building .rpm package...${NC}"
        bash "$SCRIPT_DIR/build-linux-rpm.sh" x86_64
    fi
    if [ ! -f "$SCRIPT_DIR/build-linux-run.sh" ]; then
        echo ""
        echo -e "${YELLOW}Building .run self-extracting installer...${NC}"
        bash "$SCRIPT_DIR/build-linux-run.sh"
    fi
fi
echo ""
echo -e "${GREEN}╔════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║      Build Complete! ✓                            ║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${BLUE}Installers located in:${NC} $PROJECT_ROOT/dist"
echo ""
ls -lh "$PROJECT_ROOT/dist"/*/ 2>/dev/null || true
echo ""
echo -e "${YELLOW}Next steps:${NC}"
echo "  1. Sign the installers (optional but recommended)"
echo "  2. Upload to your website"
echo "  3. Users download and install"
echo ""
