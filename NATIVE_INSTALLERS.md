# Octaskly Native Installers - Professional Distribution

**Status**: Production Ready  
**Version**: 1.0.0  
**Date**: February 7, 2026

---

## Overview

Octaskly has moved from bash/PowerShell scripts to **professional native installers** for each platform. This is the enterprise-grade approach used by Docker, Node.js, Git, Python, and other major projects.

### User Experience (The Goal)

| Before | After |
|--------|-------|
| Download bash script → run in terminal | Download installer → double-click → done |
| Manage chmod manually | Automatic |
| Configure PATH yourself | Automatic |
| Requires cargo/rust | Not needed |
| Platform confusion | Clear per-platform installer |

### Why Native Installers?

✅ **Professional**: Users expect this from real software  
✅ **Automatic**: No manual chmod, PATH, or cargo  
✅ **Signed**: Can be code-signed for security  
✅ **Clean uninstall**: Proper package management  
✅ **Enterprise**: Works in corporate environments  
✅ **Familiar**: Windows/Mac/Linux users know how these work  

---

## Platform-Specific Installers

### 🪟 Windows: Inno Setup (.exe)

**File**: `installer/octaskly-installer.iss`  
**Output**: `dist/windows/octaskly-setup-1.0.0.exe`

**What it does**:
- Double-click → wizard opens
- Selects installation location
- Adds to system PATH automatically
- Creates desktop icon (optional)
- Fully uninstallable via Add/Remove Programs

**Build it**:
```bash
# Requires Inno Setup (free download)
# https://jrsoftware.com/isdl.php

iscc installer/octaskly-installer.iss
```

**User experience**:
```
1. Download octaskly-setup-1.0.0.exe
2. Double-click
3. Click "Install"
4. Done → octaskly available in CMD/PowerShell
```

**Key features** (from .iss file):
- Admin privilege check
- System PATH registration
- Optional desktop icon
- Uninstaller support
- Modern UI

---

### 🍎 macOS: .pkg Installer

**File**: `scripts/build-macos-pkg.sh`  
**Output**: `dist/macos/octaskly-1.0.0.pkg`

**What it does**:
- Double-click → installer wizard
- Installs to `/usr/local/bin/octaskly`
- Handles permissions automatically
- Optional code signing with Developer ID
- Can be wrapped in .dmg for distribution

**Build it**:
```bash
bash scripts/build-macos-pkg.sh
```

**User experience**:
```
1. Download octaskly-1.0.0.pkg
2. Double-click
3. Click "Install"
4. Enter password (1x for /usr/local/bin access)
5. Done → octaskly available in terminal
```

**Key features**:
- Professional .pkg format (Apple standard)
- Automatic chmod +x handling
- Optional code signing (Developer ID)
- postinstall/preinstall scripts
- Clean installs to Apple-standard path

---

### 🐧 Linux: .deb + .rpm + .run

#### Option 1: .deb (Debian/Ubuntu)

**File**: `scripts/build-linux-deb.sh`  
**Output**: `dist/linux/octaskly_1.0.0_amd64.deb`

**Build it**:
```bash
bash scripts/build-linux-deb.sh amd64
```

**User experience**:
```bash
sudo dpkg -i octaskly_1.0.0_amd64.deb
# or
sudo apt install ./octaskly_1.0.0_amd64.deb
```

**Key features**:
- Standard Debian package format
- Automatic postinstall/postrm scripts
- System-wide availability
- Easy uninstall: `sudo apt remove octaskly`
- Depends on libc6 only

---

#### Option 2: .rpm (RHEL/Fedora/CentOS)

**File**: `scripts/build-linux-rpm.sh`  
**Output**: `dist/linux/octaskly-1.0.0-1.x86_64.rpm`

**Build it**:
```bash
bash scripts/build-linux-rpm.sh x86_64
```

**User experience**:
```bash
sudo rpm -i octaskly-1.0.0-1.x86_64.rpm
# or
sudo dnf install ./octaskly-1.0.0-1.x86_64.rpm
```

**Key features**:
- Standard RPM format for RedHat-based distros
- Automatic postinstall/postuninstall
- System-wide availability
- Easy uninstall: `sudo dnf remove octaskly`

---

#### Option 3: .run (Self-Extracting - Most Like Windows)

**File**: `scripts/build-linux-run.sh`  
**Output**: `dist/linux/octaskly-installer-1.0.0.run`

**Build it**:
```bash
bash scripts/build-linux-run.sh
```

**User experience**:
```bash
sudo ./octaskly-installer-1.0.0.run
```

**Key features**:
- Self-contained, single file
- Extracts and installs automatically
- No package manager needed
- Most similar to Windows Inno Setup UX
- Used by NVIDIA, Oracle, VMware

---

## Distribution Strategy

### Recommended File Structure

```
octaskly.io/downloads/

├── Windows/
│   └── octaskly-setup-1.0.0.exe
│
├── macOS/
│   └── octaskly-1.0.0.pkg
│   └── octaskly-1.0.0.dmg  (optional wrapper)
│
└── Linux/
    ├── octaskly_1.0.0_amd64.deb
    ├── octaskly-1.0.0-1.x86_64.rpm
    └── octaskly-installer-1.0.0.run
```

### User Download Page

```
Download Octaskly v1.0.0

Windows
  octaskly-setup-1.0.0.exe (15 MB)
  [Download] - Double-click to install

macOS
  octaskly-1.0.0.pkg (12 MB)
  [Download] - Double-click to install

Linux
  Debian/Ubuntu: octaskly_1.0.0_amd64.deb (12 MB)
  RedHat/Fedora: octaskly-1.0.0-1.x86_64.rpm (12 MB)
  Universal: octaskly-installer-1.0.0.run (12 MB)
```

---

## Building All Installers

### Quick Start

```bash
bash scripts/build-all.sh
```

This script:
1. Detects current OS
2. Builds release binary (Rust)
3. Creates native installer for that OS
4. Outputs to `dist/` directory

### Platform-Specific Manual Builds

**Windows (requires Inno Setup)**:
```bash
iscc installer/octaskly-installer.iss
```

**macOS**:
```bash
bash scripts/build-macos-pkg.sh
```

**Linux**:
```bash
# Debian/Ubuntu
bash scripts/build-linux-deb.sh amd64

# RedHat/Fedora
bash scripts/build-linux-rpm.sh x86_64

# Self-extracting
bash scripts/build-linux-run.sh
```

---

## Code Signing (Optional But Recommended)

### Windows Code Signing

```bash
# Sign .exe with certificate
signtool sign /f certificate.pfx /p password \
  /t http://timestamp.server.com \
  dist/windows/octaskly-setup-1.0.0.exe
```

### macOS Code Signing

```bash
# Sign .pkg with Developer ID
productsign --sign "Developer ID Installer: Your Name" \
  dist/macos/octaskly-unsigned.pkg \
  dist/macos/octaskly-1.0.0.pkg
```

---

## Comparison: Old vs New

| Aspect | Bash Script | Native Installer |
|--------|-------------|------------------|
| **User opens** | Terminal → bash command | Installer file → double-click |
| **chmod +x** | User must run | Automatic |
| **PATH setup** | Manual shell config | Automatic |
| **Cargo/Rust** | Required | Not needed |
| **Uninstall** | Manual deletion | Package manager |
| **Code signing** | Not possible | Yes (Pro) |
| **Corporate approval** | Low | High |
| **Professional feeling** | Hackish | Enterprise |

---

## Technical Implementation Details

### Windows Inno Setup (.iss)

```pascal
[Setup]
AppId={{GUID}}
DefaultDirName={autopf}\Octaskly
ArchitecturesInstallIn64BitMode=x64
PrivilegesRequired=admin

[Files]
Source: "..\target\release\octaskly.exe"; DestDir: "{app}"

[Code]
procedure RegisterPathEntry;
// Adds to system PATH automatically
```

### macOS pkgbuild

```bash
pkgbuild \
  --root pkgroot \
  --identifier com.octaskly.cli \
  --version 1.0.0 \
  octaskly.pkg
```

### Linux .deb control

```
Package: octaskly
Version: 1.0.0
Architecture: amd64
Depends: libc6

[postinstall]
chmod +x /usr/bin/octaskly
```

### Linux .run Self-Extracting

- Binary tar.gz appended to bash script
- User runs: `sudo ./octaskly-installer.run`
- Script extracts and copies to `/usr/local/bin`
- Automatic chmod handling

---

## Testing Installers

### Windows
```powershell
# Run .exe
.\dist\windows\octaskly-setup-1.0.0.exe

# Verify
octaskly --version
```

### macOS
```bash
# Install
sudo installer -pkg dist/macos/octaskly-1.0.0.pkg -target /

# Verify
octaskly --version
```

### Linux (Debian)
```bash
# Install
sudo dpkg -i dist/linux/octaskly_1.0.0_amd64.deb

# Verify
octaskly --version

# Uninstall
sudo apt remove octaskly
```

### Linux (.run)
```bash
# Install
sudo ./dist/linux/octaskly-installer-1.0.0.run

# Verify
octaskly --version
```

---

## Troubleshooting

### Windows
- **Error: admin required** → Right-click .exe → Run as Administrator
- **PATH not updated** → Restart Windows or open new CMD window
- **Uninstall issues** → Use Control Panel → Programs → Add/Remove Programs

### macOS
- **Error: package can't be opened** → System Preferences → Security → Allow
- **Permission denied** → Password prompt is normal (for /usr/local/bin)
- **Not in PATH** → Restart terminal or run: `source ~/.zshrc`

### Linux (.deb)
- **Dependencies missing** → `sudo apt install -f`
- **Uninstall** → `sudo apt remove octaskly`
- **Version conflict** → `sudo apt-get install --reinstall octaskly`

### Linux (.run)
- **Permission denied** → Run as root: `sudo ./octaskly-installer.run`
- **Extraction fails** → Check disk space: `df -h`

---

## Future Enhancements

### Short-term
- ✅ Code signing for all platforms
- ✅ Build automation in CI/CD (GitHub Actions)
- ✅ Checksum verification

### Medium-term
- macOS .dmg disk image wrapper
- Windows MSIX package (Microsoft Store)
- Automatic update mechanism

### Long-term
- Homebrew formula (macOS)
- Linux distro repositories integration
- Enterprise MSI installer (Windows)

---

## Migration from Bash Installers

### Users with Old Bash Script

Old way (still works):
```bash
bash scripts/install.sh install
```

New way (recommended):
```bash
# Download and run installer for your OS
chmod +x octaskly-installer-1.0.0.run
sudo ./octaskly-installer-1.0.0.run
```

### For Developers

Update your documentation to point to:
```
downloads/
├── Windows: octaskly-setup-1.0.0.exe
├── macOS: octaskly-1.0.0.pkg
└── Linux: octaskly_1.0.0_amd64.deb (or .rpm or .run)
```

---

## Files in This Implementation

| File | Purpose |
|------|---------|
| `installer/octaskly-installer.iss` | Windows Inno Setup config |
| `scripts/build-macos-pkg.sh` | macOS .pkg builder |
| `scripts/build-linux-deb.sh` | Linux .deb builder |
| `scripts/build-linux-rpm.sh` | Linux .rpm builder |
| `scripts/build-linux-run.sh` | Linux .run builder |
| `scripts/build-all.sh` | Universal builder for all platforms |

---

## Summary

✅ **Windows**: Professional Inno Setup installer (.exe)  
✅ **macOS**: Apple-standard .pkg installer  
✅ **Linux**: .deb + .rpm + .run options  
✅ **Cross-platform**: All follow industry standards  
✅ **Professional**: Code signing ready  
✅ **Automated**: Build scripts for CI/CD  

**User experience**: Download single file → double-click → done. No setup. No confusion. No cargo. Just works.

This is how enterprise software is distributed. Octaskly is now at that level.
