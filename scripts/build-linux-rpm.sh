#!/bin/bash
set -e
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$SCRIPT_DIR/.."
ARCH="${1:-x86_64}"
VERSION="1.0.0"
RELEASE="1"
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'
echo -e "${BLUE}╔════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║    Octaskly .rpm Package Builder       ║${NC}"
echo -e "${BLUE}║    Architecture: $ARCH                    ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════╝${NC}"
echo ""
if ! command -v rpmbuild &> /dev/null; then
    echo -e "${RED}✗ rpmbuild not found. Install rpm-build:${NC}"
    echo "  sudo dnf install rpm-build"
    exit 1
fi
BINARY="$PROJECT_ROOT/target/release/octaskly"
RPM_BUILD_DIR="/tmp/octaskly-rpm-build"
RPM_OUTPUT="$PROJECT_ROOT/dist/linux"
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
echo -e "${YELLOW}Creating RPM build structure...${NC}"
mkdir -p "$RPM_BUILD_DIR/SOURCES"
mkdir -p "$RPM_BUILD_DIR/SPECS"
mkdir -p "$RPM_BUILD_DIR/BUILD"
mkdir -p "$RPM_BUILD_DIR/RPMS"
mkdir -p "$RPM_BUILD_DIR/SOURCES/octaskly-$VERSION/usr/bin"
cp "$BINARY" "$RPM_BUILD_DIR/SOURCES/octaskly-$VERSION/usr/bin/octaskly"
chmod +x "$RPM_BUILD_DIR/SOURCES/octaskly-$VERSION/usr/bin/octaskly"
cd "$RPM_BUILD_DIR/SOURCES"
tar czf "octaskly-$VERSION.tar.gz" "octaskly-$VERSION"
rm -rf "octaskly-$VERSION"
echo -e "${GREEN}✓ Source tarball created${NC}"
cat > "$RPM_BUILD_DIR/SPECS/octaskly.spec" << EOF
Name:           octaskly
Version:        $VERSION
Release:        $RELEASE
Summary:        Distributed Task Orchestration with P2P Sharing
License:        MIT
URL:            https://octaskly.io
Source0:        %{name}-%{version}.tar.gz
Requires:       glibc
%description
Octaskly is a distributed task orchestration system with peer-to-peer resource
sharing for seamless compute distribution across local networks.
Features:
- Automatic peer discovery with mDNS
- Intelligent task distribution
- Hybrid centralized/P2P architecture
- Cross-platform support
- AES-256-GCM encryption and JWT authentication
%prep
%setup -q
%install
mkdir -p %{buildroot}/usr/bin
install -m 755 usr/bin/octaskly %{buildroot}/usr/bin/octaskly
%files
/usr/bin/octaskly
%post
echo "Octaskly installed successfully!"
echo "Run 'octaskly --help' to get started"
%postun
echo "Octaskly removed"
%changelog
* $(date '+%a %b %d %Y') Octaskly Team <team@octaskly.io> - $VERSION-$RELEASE
- Initial RPM release
EOF
echo -e "${GREEN}✓ .spec file created${NC}"
echo ""
echo -e "${YELLOW}Building .rpm package...${NC}"
rpmbuild -ba \
    --define "_topdir $RPM_BUILD_DIR" \
    --define "_arch $ARCH" \
    "$RPM_BUILD_DIR/SPECS/octaskly.spec"
if [ $? -eq 0 ]; then
    mkdir -p "$RPM_OUTPUT"
    cp "$RPM_BUILD_DIR/RPMS/$ARCH/octaskly-$VERSION-$RELEASE.$ARCH.rpm" "$RPM_OUTPUT/"
    echo -e "${GREEN}✓ Package created: $RPM_OUTPUT/octaskly-$VERSION-$RELEASE.$ARCH.rpm${NC}"
else
    echo -e "${RED}✗ Package creation failed${NC}"
    exit 1
fi
rm -rf "$RPM_BUILD_DIR"
PKG_PATH="$RPM_OUTPUT/octaskly-$VERSION-$RELEASE.$ARCH.rpm"
echo ""
echo -e "${YELLOW}Package Details:${NC}"
rpm -qi "$PKG_PATH" 2>/dev/null || rpm -qp "$PKG_PATH"
echo ""
echo -e "${GREEN}╔════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║    .rpm Package Creation Complete! ✓   ║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════╝${NC}"
echo ""
echo -e "${BLUE}Location:${NC} $PKG_PATH"
echo -e "${BLUE}Size:${NC} $(du -h "$PKG_PATH" | cut -f1)"
echo ""
echo -e "${YELLOW}Installation:${NC}"
echo "  sudo dnf install octaskly-$VERSION-$RELEASE.$ARCH.rpm"
echo "  # or"
echo "  sudo rpm -i octaskly-$VERSION-$RELEASE.$ARCH.rpm"
echo ""
echo -e "${YELLOW}Uninstallation:${NC}"
echo "  sudo dnf remove octaskly"
echo ""
