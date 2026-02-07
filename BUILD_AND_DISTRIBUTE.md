# Quick Build & Distribution Guide

This is the TL;DR for building and distributing Octaskly with professional installers.

---

## For Developers: How to Build

### One-Command Build (All Platforms)

```bash
bash scripts/build-all.sh
```

This:
1. ✅ Builds release binary with Rust
2. ✅ Creates native installer for your OS
3. ✅ Outputs to `dist/` folder
4. ✅ Displays download link

### Platform-Specific

**Windows** (requires Inno Setup):
```bash
# 1. Download Inno Setup: https://jrsoftware.com/isdl.php
# 2. Run:
iscc installer/octaskly-installer.iss
# Output: dist/windows/octaskly-setup-1.0.0.exe
```

**macOS**:
```bash
bash scripts/build-macos-pkg.sh
# Output: dist/macos/octaskly-1.0.0.pkg
```

**Linux**:
```bash
# Debian/Ubuntu
bash scripts/build-linux-deb.sh amd64
# Output: dist/linux/octaskly_1.0.0_amd64.deb

# RedHat/Fedora
bash scripts/build-linux-rpm.sh x86_64
# Output: dist/linux/octaskly-1.0.0-1.x86_64.rpm

# Or self-extracting (works anywhere)
bash scripts/build-linux-run.sh
# Output: dist/linux/octaskly-installer-1.0.0.run
```

---

## For End Users: How to Install

### Windows
```
1. Download octaskly-setup-1.0.0.exe
2. Double-click
3. Click "Install"
4. Done
```

### macOS
```
1. Download octaskly-1.0.0.pkg
2. Double-click
3. Click "Install"
4. Enter password (once)
5. Done
```

### Linux (Debian/Ubuntu)
```bash
sudo dpkg -i octaskly_1.0.0_amd64.deb
```

Or if you prefer apt:
```bash
sudo apt install ./octaskly_1.0.0_amd64.deb
```

### Linux (RedHat/Fedora)
```bash
sudo rpm -i octaskly-1.0.0-1.x86_64.rpm
```

Or if you prefer dnf:
```bash
sudo dnf install ./octaskly-1.0.0-1.x86_64.rpm
```

### Linux (Self-Extracting)
```bash
sudo ./octaskly-installer-1.0.0.run
```

---

## For Distributors: Upload & Share

### File Organization

Create this structure on your website:

```
octaskly.io/downloads/

├── Windows
│   └── octaskly-setup-1.0.0.exe (15 MB)
│
├── macOS
│   └── octaskly-1.0.0.pkg (12 MB)
│
└── Linux
    ├── octaskly_1.0.0_amd64.deb (12 MB)
    ├── octaskly-1.0.0-1.x86_64.rpm (12 MB)
    └── octaskly-installer-1.0.0.run (12 MB)
```

### Example Download Page

```markdown
# Download Octaskly

Choose your operating system:

## Windows
**octaskly-setup-1.0.0.exe** | 15 MB
[Download] - Just double-click and follow the wizard

## macOS
**octaskly-1.0.0.pkg** | 12 MB
[Download] - Double-click to install

## Linux
- **Debian/Ubuntu**: octaskly_1.0.0_amd64.deb (12 MB)
  `sudo dpkg -i octaskly_1.0.0_amd64.deb`
  
- **RedHat/Fedora**: octaskly-1.0.0-1.x86_64.rpm (12 MB)
  `sudo rpm -i octaskly-1.0.0-1.x86_64.rpm`
  
- **Universal**: octaskly-installer-1.0.0.run (12 MB)
  `sudo ./octaskly-installer-1.0.0.run`
```

---

## Professional Touches (Optional)

### Code Signing

**Windows**:
```powershell
signtool sign /f cert.pfx /p password /t http://timestamp.server \
  dist/windows/octaskly-setup-1.0.0.exe
```

**macOS**:
```bash
productsign --sign "Developer ID Installer: Your Name" \
  dist/macos/octaskly-unsigned.pkg \
  dist/macos/octaskly-1.0.0.pkg
```

### Checksums

```bash
cd dist && find . -type f -exec sha256sum {} \; > SHA256SUMS
# Users can verify: sha256sum -c SHA256SUMS
```

### Version Numbering

Update in Rust (Cargo.toml):
```toml
[package]
version = "1.0.0"
```

All installers automatically get the version number from there.

---

## CI/CD Integration

### GitHub Actions Example

```yaml
name: Build Installers

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    
    steps:
      - uses: actions/checkout@v2
      - uses: rust-lang/rust-toolchain@v1
      
      - name: Build
        run: cargo build --release
      
      - name: Create Installer
        run: |
          if [ "$RUNNER_OS" == "Windows" ]; then
            # iscc installer/octaskly-installer.iss
          elif [ "$RUNNER_OS" == "macOS" ]; then
            bash scripts/build-macos-pkg.sh
          else
            bash scripts/build-linux-deb.sh amd64
          fi
      
      - name: Upload
        uses: actions/upload-release-asset@v1
```

---

## Troubleshooting

### Build Fails: "Binary not found"
```bash
# Make sure you built release first
cargo build --release
```

### Windows: "iscc not found"
```bash
# Install Inno Setup
# https://jrsoftware.com/isdl.php
```

### macOS: "pkgbuild not found"
```bash
# Install Xcode Command Line Tools
xcode-select --install
```

### Linux: "dpkg-deb not found"
```bash
# Install build tools
sudo apt-get install build-essential
```

---

## File Locations

After building, find installers here:

```
dist/
├── windows/
│   └── octaskly-setup-1.0.0.exe
├── macos/
│   └── octaskly-1.0.0.pkg
└── linux/
    ├── octaskly_1.0.0_amd64.deb
    ├── octaskly-1.0.0-1.x86_64.rpm
    └── octaskly-installer-1.0.0.run
```

---

## Next Steps

1. **Build**: `bash scripts/build-all.sh` (on each OS)
2. **Test**: Install and run `octaskly --version`
3. **Sign** (optional): Code sign for trust
4. **Upload**: To your website
5. **Document**: Update download page
6. **Release**: Announce to users

---

## That's It!

Users now experience professional installation on their OS of choice. No confusion. No cargo. No silly bash scripts. Just download and run.

This is how Docker, Node.js, Git, and Python do it. Octaskly is now at that level.
