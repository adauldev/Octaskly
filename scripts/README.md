# Octaskly Installation Scripts

## Overview
Octaskly provides separate installers optimized for different platforms to ensure the best user experience on each OS.

## Installation Methods

### Linux, macOS, WSL, and Termux
**Script:** `install.sh`

Supports:
- Linux (Debian, Ubuntu, RHEL, Arch, Alpine, etc.)
- macOS (Intel and Apple Silicon)
- Windows Subsystem for Linux (WSL)
- **Termux (Android)** - Automatically uses `$PREFIX/bin` path

**Installation:**
```bash
bash install.sh install
```

Or with curl (ideal for Termux):
```bash
curl -fsSL https://octaskly.io/install.sh | sh
```

**Build only (without installing):**
```bash
bash install.sh build
```

**Termux-Specific:**
- Path: `$PREFIX/bin` (usually `/data/data/com.termux/files/usr/bin`)
- No manual `chmod +x` needed - script handles it automatically
- Automatic shell profile updates (`.bashrc`, `.zshrc`)

---

### Windows (Native PowerShell)
**Script:** `install.ps1`

Requires:
- Windows 10/11
- PowerShell 5.1 or later
- **Administrator privileges** (required for system PATH modification)

**Installation:**
```powershell
powershell -ExecutionPolicy Bypass -File install.ps1 install
```

**Build only (without installing):**
```powershell
powershell -ExecutionPolicy Bypass -File install.ps1 build
```

**Step-by-step for Windows:**
1. Open PowerShell as Administrator (right-click → Run as administrator)
2. Navigate to the `scripts` folder:
   ```powershell
   cd C:\path\to\Octaskly\scripts
   ```
3. Run the installer:
   ```powershell
   powershell -ExecutionPolicy Bypass -File install.ps1
   ```

---

## Why Separate Installers?

### Professional Separation (Option 1 - Selected)
- **install.sh** - Unix-like systems (bash/shell native)
- **install.ps1** - Windows native (PowerShell native)

**Advantages:**
- ✅ Each script use the native tooling for its OS
- ✅ Clear, maintainable code without cross-platform compatibility hacks
- ✅ Follows industry standard (Docker, Node.js, Rustup)
- ✅ Best user experience on each platform
- ✅ Professional and clean

### Why Not Auto-Detect?
- PowerShell execution policies block automatic elevation
- Windows Defender may block dynamic spawning
- PATH modifications require admin on Windows, not on Unix
- Better UX when users explicitly choose the right installer

---

## Troubleshooting

### Linux / macOS

**Binary not found:**
```bash
bash install.sh build
```

**Permission denied:**
Installer will use `sudo` automatically for `/usr/local/bin` when needed.

**PATH not updated:**
```bash
source ~/.bashrc  # or ~/.zshrc for macOS
```

### Termux (Android)

**Installation methods:**
```bash
# Direct installation
bash install.sh install

# Or recommended pattern (curl | sh):
curl -fsSL https://octaskly.io/install.sh | sh
```

**Binary not found:**
```bash
# Build from source
bash install.sh build
```

**PATH not updated:**
```bash
# Termux shell profile
source ~/.bashrc
```

**Recompile for Android (native):**
If you need native Android binary instead of standard Linux aarch64:
```bash
rustup target add aarch64-linux-android
cargo build --release --target aarch64-linux-android
```

### Windows

**PowerShell execution policy blocked:**
```powershell
powershell -ExecutionPolicy Bypass -File install.ps1
```

**Admin privileges required:**
Right-click PowerShell → "Run as administrator"

**Rust/Cargo not found:**
Install from: https://rustup.rs or
```powershell
winget install Rustlang.Rust.MSVC
```

**PATH changes not applied:**
Close and reopen PowerShell/CMD after installation

---

## Installation Locations

| OS | Location |
|----|----|
| Linux | `/usr/local/bin/octaskly` |
| macOS | `/usr/local/bin/octaskly` |
| WSL | `/usr/local/bin/octaskly` |
| Termux | `$PREFIX/bin/octaskly` (usually `/data/data/com.termux/files/usr/bin/octaskly`) |
| Windows | `C:\Program Files\octaskly\octaskly.exe` |

---

## Future Enhancements (Enterprise)

For maximum professionalism, Octaskly could eventually support:
- **Windows:** Native .msi/.exe installer (WiX / Inno Setup)
- **macOS:** .pkg installer
- **Linux:** .deb, .rpm distributions

This would be ideal for enterprise deployments but requires additional tooling and CI/CD infrastructure.

---

## Testing Your Installation

After installation, verify it works:

```bash
# All platforms
octaskly --version
octaskly --help

# Test dispatcher
octaskly dispatcher --port 7878 --ui

# Test worker
octaskly worker -n test-worker
```

---

## Support

For issues or questions:
1. Check the [main README](../README.md)
2. Review [INSTALLATION.md](../INSTALLATION.md)
3. See [HELP.md](../HELP.md) for command reference
