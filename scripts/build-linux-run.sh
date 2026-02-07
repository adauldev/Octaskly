#!/bin/bash

# Octaskly Self-Extracting Installer for Linux
# This script creates an octaskly-installer.run file
# Usage: bash build-linux-run.sh
# User: sudo ./octaskly-installer.run

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$SCRIPT_DIR/.."

VERSION="1.0.0"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}╔════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  Octaskly Self-Extracting Installer    ║${NC}"
echo -e "${BLUE}║  (.run builder for Linux)              ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════╝${NC}"
echo ""

BINARY="$PROJECT_ROOT/target/release/octaskly"
INSTALLER_TEMP="/tmp/octaskly-installer-build"
OUTPUT_DIR="$PROJECT_ROOT/dist/linux"

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

# Create temporary directory
rm -rf "$INSTALLER_TEMP"
mkdir -p "$INSTALLER_TEMP"

# Copy binary
cp "$BINARY" "$INSTALLER_TEMP/octaskly"
chmod +x "$INSTALLER_TEMP/octaskly"

# Create the installer script header
INSTALLER_SCRIPT="$OUTPUT_DIR/octaskly-installer-$VERSION.run"
mkdir -p "$OUTPUT_DIR"

# Build self-extracting archive
echo -e "${YELLOW}Creating self-extracting installer...${NC}"

# Write the installer script
cat > "$INSTALLER_SCRIPT" << 'INSTALLER_EOF'
#!/bin/bash

# Octaskly Self-Extracting Installer
# Professional, no-fuss installation for Linux

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}╔════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║    Octaskly Linux Installer            ║${NC}"
echo -e "${BLUE}║    version: 1.0.0                      ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════╝${NC}"
echo ""

# Check if running as root
if [ "$EUID" -ne 0 ]; then
   echo -e "${RED}✗ This installer must be run as root (sudo)${NC}"
   exit 1
fi

# Check target directory is writable
if [ ! -w "/usr/local/bin" ]; then
    echo -e "${RED}✗ No write permission to /usr/local/bin${NC}"
    exit 1
fi

# Extract directory (last byte offset is appended by the builder)
OFFSET=$(tail -c 10 "$0" | head -c 8)
ARCHIVE_START=$(($(wc -c < "$0") - $OFFSET))

echo -e "${YELLOW}Extracting files...${NC}"
mkdir -p /tmp/octaskly-install-$$
tail -c +$ARCHIVE_START "$0" | tar xz -C /tmp/octaskly-install-$$

if [ ! -f "/tmp/octaskly-install-$$/octaskly" ]; then
    echo -e "${RED}✗ Failed to extract binary${NC}"
    rm -rf "/tmp/octaskly-install-$$"
    exit 1
fi

echo -e "${GREEN}✓ Files extracted${NC}"
echo ""

echo -e "${YELLOW}Installing...${NC}"
cp "/tmp/octaskly-install-$$/octaskly" "/usr/local/bin/octaskly"
chmod +x "/usr/local/bin/octaskly"

# Cleanup
rm -rf "/tmp/octaskly-install-$$"

echo -e "${GREEN}✓ Binary installed to /usr/local/bin/octaskly${NC}"
echo ""

echo -e "${GREEN}╔════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║    Installation Complete! ✓            ║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════╝${NC}"
echo ""

# Test installation
if command -v octaskly &> /dev/null; then
    echo -e "${GREEN}✓ octaskly is now available globally${NC}"
    echo ""
    echo -e "${BLUE}Version:${NC}"
    octaskly --version
else
    echo -e "${YELLOW}⚠ Restart your terminal to use octaskly${NC}"
fi

echo ""
echo -e "${YELLOW}Next steps:${NC}"
echo "  octaskly --help              Show help"
echo "  octaskly dispatcher --port 7878  Start dispatcher"
echo "  octaskly worker -n my-worker     Start worker"
echo ""

exit 0
INSTALLER_EOF

chmod +x "$INSTALLER_SCRIPT"

# Create the binary archive
echo -e "${YELLOW}Packaging binary...${NC}"
cd "$INSTALLER_TEMP"
tar czf /tmp/octaskly-archive.tar.gz octaskly
ARCHIVE_SIZE=$(stat -f%z /tmp/octaskly-archive.tar.gz 2>/dev/null || stat -c%s /tmp/octaskly-archive.tar.gz)

# Append archive to installer script
echo -e "${YELLOW}Appending archive to installer...${NC}"
cat /tmp/octaskly-archive.tar.gz >> "$INSTALLER_SCRIPT"

# Add offset info at the end (10 bytes: 8 for size + 2 for newline)
FINAL_SIZE=$(stat -f%z "$INSTALLER_SCRIPT" 2>/dev/null || stat -c%s "$INSTALLER_SCRIPT")
OFFSET_STR=$(printf "%08d\n" "$ARCHIVE_SIZE")
echo "$OFFSET_STR" >> "$INSTALLER_SCRIPT"

# Cleanup
rm -rf "$INSTALLER_TEMP"
rm /tmp/octaskly-archive.tar.gz

echo -e "${GREEN}✓ Installer created: $INSTALLER_SCRIPT${NC}"
echo ""

echo -e "${GREEN}╔════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║    .run Installer Creation Complete! ✓ ║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════╝${NC}"
echo ""
echo -e "${BLUE}Location:${NC} $INSTALLER_SCRIPT"
echo -e "${BLUE}Size:${NC} $(du -h "$INSTALLER_SCRIPT" | cut -f1)"
echo ""
echo -e "${YELLOW}Usage:${NC}"
echo "  sudo $INSTALLER_SCRIPT"
echo ""
echo -e "${YELLOW}Or make it executable and run:${NC}"
echo "  chmod +x octaskly-installer-$VERSION.run"
echo "  sudo ./octaskly-installer-$VERSION.run"
echo ""
