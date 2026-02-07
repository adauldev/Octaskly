#!/bin/bash
set -e
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$SCRIPT_DIR/.."
ARCH="${1:-amd64}"
VERSION="1.0.0"
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'
echo -e "${BLUE}╔════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║    Octaskly .deb Package Builder       ║${NC}"
echo -e "${BLUE}║    Architecture: $ARCH                    ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════╝${NC}"
echo ""
if ! command -v dpkg-deb &> /dev/null; then
    echo -e "${RED}✗ dpkg-deb not found. Install build-essential:${NC}"
    echo "  sudo apt-get install build-essential"
    exit 1
fi
BINARY="$PROJECT_ROOT/target/release/octaskly"
DEB_ROOT="/tmp/octaskly-deb-root"
DEB_OUTPUT="$PROJECT_ROOT/dist/linux"
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
echo -e "${YELLOW}Creating .deb package structure...${NC}"
mkdir -p "$DEB_ROOT/DEBIAN"
mkdir -p "$DEB_ROOT/usr/bin"
mkdir -p "$DEB_ROOT/usr/share/doc/octaskly"
cp "$BINARY" "$DEB_ROOT/usr/bin/octaskly"
chmod 755 "$DEB_ROOT/usr/bin/octaskly"
echo -e "${GREEN}✓ Binary installed to $DEB_ROOT/usr/bin/octaskly${NC}"
cat > "$DEB_ROOT/DEBIAN/control" << EOF
Package: octaskly
Version: $VERSION
Architecture: $ARCH
Maintainer: Octaskly Team <team@octaskly.io>
Homepage: https://octaskly.io
Depends: libc6 (>= 2.2.1)
Priority: optional
Section: utils
Description: Octaskly - Distributed Task Orchestration with P2P Sharing
 Octaskly is a distributed task orchestration system with peer-to-peer resource
 sharing for seamless compute distribution across local networks.
 .
 Features:
  - Automatic peer discovery with mDNS
  - Intelligent task distribution
  - Hybrid centralized/P2P architecture
  - Cross-platform support
  - AES-256-GCM encryption and JWT authentication
EOF
echo -e "${GREEN}✓ Control file created${NC}"
cat > "$DEB_ROOT/DEBIAN/postinst" << 'EOF'
#!/bin/bash
set -e
if [ "$1" = "configure" ]; then
    chmod +x /usr/bin/octaskly
    echo "Octaskly installed successfully!"
    echo "Run 'octaskly --help' to get started"
fi
exit 0
EOF
chmod 755 "$DEB_ROOT/DEBIAN/postinst"
cat > "$DEB_ROOT/DEBIAN/postrm" << 'EOF'
#!/bin/bash
set -e
if [ "$1" = "remove" ]; then
    echo "Octaskly removed"
fi
exit 0
EOF
chmod 755 "$DEB_ROOT/DEBIAN/postrm"
mkdir -p "$DEB_OUTPUT"
echo -e "${YELLOW}Building .deb package...${NC}"
PACKAGE_NAME="octaskly_${VERSION}_${ARCH}.deb"
dpkg-deb --build "$DEB_ROOT" "$DEB_OUTPUT/$PACKAGE_NAME"
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ Package created: $DEB_OUTPUT/$PACKAGE_NAME${NC}"
else
    echo -e "${RED}✗ Package creation failed${NC}"
    exit 1
fi
rm -rf "$DEB_ROOT"
echo ""
echo -e "${YELLOW}Package Details:${NC}"
dpkg-deb -I "$DEB_OUTPUT/$PACKAGE_NAME"
echo ""
echo -e "${GREEN}╔════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║    .deb Package Creation Complete! ✓   ║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════╝${NC}"
echo ""
echo -e "${BLUE}Location:${NC} $DEB_OUTPUT/$PACKAGE_NAME"
echo -e "${BLUE}Size:${NC} $(du -h "$DEB_OUTPUT/$PACKAGE_NAME" | cut -f1)"
echo ""
echo -e "${YELLOW}Installation:${NC}"
echo "  sudo dpkg -i $PACKAGE_NAME"
echo ""
echo -e "${YELLOW}Uninstallation:${NC}"
echo "  sudo apt remove octaskly"
echo ""
