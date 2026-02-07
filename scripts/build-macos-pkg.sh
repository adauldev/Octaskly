#!/bin/bash
set -e
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$SCRIPT_DIR/.."
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'
echo -e "${BLUE}╔════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║    Octaskly macOS Package Builder       ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════╝${NC}"
echo ""
if ! [[ "$OSTYPE" == "darwin"* ]]; then
    echo -e "${RED}✗ This script requires macOS${NC}"
    exit 1
fi
if ! command -v pkgbuild &> /dev/null; then
    echo -e "${RED}✗ pkgbuild not found. Install Xcode Command Line Tools:${NC}"
    echo "  xcode-select --install"
    exit 1
fi
BINARY="$PROJECT_ROOT/target/release/octaskly"
PKG_ROOT="/tmp/octaskly-pkg-root"
PKG_OUTPUT="$PROJECT_ROOT/dist/macos"
echo -e "${YELLOW}Building binary...${NC}"
if [ ! -f "$BINARY" ]; then
    echo -e "${YELLOW}Binary not found at $BINARY, building...${NC}"
    cd "$PROJECT_ROOT"
    cargo build --release
fi
if [ ! -f "$BINARY" ]; then
    echo -e "${RED}✗ Build failed${NC}"
    exit 1
fi
echo -e "${GREEN}✓ Binary ready: $BINARY${NC}"
echo ""
echo -e "${YELLOW}Creating package structure...${NC}"
mkdir -p "$PKG_ROOT/usr/local/bin"
cp "$BINARY" "$PKG_ROOT/usr/local/bin/octaskly"
chmod +x "$PKG_ROOT/usr/local/bin/octaskly"
echo -e "${GREEN}✓ Binary installed to $PKG_ROOT/usr/local/bin/octaskly${NC}"
echo ""
mkdir -p "$PKG_OUTPUT"
echo -e "${YELLOW}Building .pkg installer...${NC}"
pkgbuild \
    --root "$PKG_ROOT" \
    --identifier "com.octaskly.cli" \
    --version "1.0.0" \
    --install-location "/" \
    --scripts="$SCRIPT_DIR/macos-scripts" \
    "$PKG_OUTPUT/octaskly-1.0.0.pkg"
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ Package created: $PKG_OUTPUT/octaskly-1.0.0.pkg${NC}"
else
    echo -e "${RED}✗ Package creation failed${NC}"
    exit 1
fi
if security find-certificate -c "Developer ID Installer" > /dev/null 2>&1; then
    echo -e "${YELLOW}Signing package...${NC}"
    productsign --sign "Developer ID Installer" \
        "$PKG_OUTPUT/octaskly-1.0.0.pkg" \
        "$PKG_OUTPUT/octaskly-1.0.0-signed.pkg"
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓ Package signed${NC}"
        mv "$PKG_OUTPUT/octaskly-1.0.0-signed.pkg" "$PKG_OUTPUT/octaskly-1.0.0.pkg"
    fi
fi
rm -rf "$PKG_ROOT"
echo ""
echo -e "${GREEN}╔════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║    Package Creation Complete! ✓        ║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════╝${NC}"
echo ""
echo -e "${BLUE}Location:${NC} $PKG_OUTPUT/octaskly-1.0.0.pkg"
echo -e "${BLUE}Size:${NC} $(du -h "$PKG_OUTPUT/octaskly-1.0.0.pkg" | cut -f1)"
echo ""
echo -e "${YELLOW}To distribute:${NC}"
echo "  1. Upload to website"
echo "  2. Users double-click to install"
echo "  3. No cargo, bash, or PATH config needed"
echo ""
