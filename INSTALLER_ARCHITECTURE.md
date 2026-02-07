# Installer Architecture

## Design Philosophy

**Principle**: One universal script to rule them all.

Instead of maintaining separate installer scripts for each OS (Windows PS1, macOS SH, Linux SH), Octaskly uses a **single cross-platform bash script** with runtime OS detection.

This approach:
- ✅ Eliminates code duplication
- ✅ Reduces maintenance burden
- ✅ Aligns with how industry-standard tools work (npm, git, cargo, rustup)
- ✅ Makes updates easier (change one file, not six)

---

## File Structure

```
scripts/
└── install.sh          # The ONE script that does everything
```

That's it. One 340-line bash script handles:
- Building from source
- Installing on Windows (PowerShell + WSL)
- Installing on macOS (Intel + Apple Silicon)
- Installing on Linux (Debian, Ubuntu, RHEL, CentOS, Arch, Alpine)
- Automatic PATH setup
- Automatic uninstallation

---

## How It Works

### 1. OS Detection
```bash
case "$OSTYPE" in
  linux*) OS="linux" ;;
  darwin*) OS="macos" ;;
  msys* | cygwin* | win32) OS="windows" ;;
  *) OS="unknown" ;;
esac
```

Simple but effective. `$OSTYPE` is available on all systems.

### 2. Architecture Detection
For correct binary selection:
```bash
case "$(uname -m)" in
  x86_64) ARCH="x86_64" ;;
  aarch64 | arm64) ARCH="aarch64" ;;
  *) echo "Unsupported architecture"; exit 1 ;;
esac
```

### 3. Binary Building
If no pre-built binary found:
```bash
cargo build --release
```

No pre-requisites needed. Binary gets built if missing.

### 4. Platform-Specific Installation

#### Linux
```bash
sudo cp ./target/release/octaskly /usr/local/bin/
chmod +x /usr/local/bin/octaskly
echo 'export PATH="/usr/local/bin:$PATH"' >> ~/.bashrc
```

#### macOS
```bash
cp ./target/release/octaskly /usr/local/bin/
chmod +x /usr/local/bin/octaskly
echo 'export PATH="/usr/local/bin:$PATH"' >> ~/.zshrc
```

#### Windows (PowerShell)
```powershell
New-Item -ItemType Directory -Path "C:\Program Files\octaskly" -Force
Copy-Item ".\target\release\octaskly.exe" -Destination "C:\Program Files\octaskly\"
[Environment]::SetEnvironmentVariable("Path", "$env:Path;C:\Program Files\octaskly", "User")
```

---

## Usage

### Build Only
```bash
bash scripts/install.sh build
```

Binary appears at `./target/release/octaskly` (or `.exe` on Windows).

### Build + Install
```bash
bash scripts/install.sh install
```

Builds if needed, then installs globally.

### Commands Available After Install
```bash
octaskly dispatcher --port 7878
octaskly worker --name worker-01
octaskly --version
```

From any directory. No `.target/release/` paths needed.

---

## Why One Script Works Better

### Previous Approach (❌ Bad)
```
install-windows.ps1   (250 lines)
install-macos.sh      (250 lines)
install-linux.sh      (250 lines)
install-universal.sh  (280 lines)
build-release.ps1     (100 lines)
build-release.sh      (100 lines)
```

Problems:
- Duplication: Same logic repeated in 4+ files
- Maintenance: Fix a bug? Update 3-6 files
- Confusion: Which one do I use? Why so many?
- Size: 1000+ lines total for essentially one task

### New Approach (✅ Good)
```
install.sh           (340 lines)
```

Benefits:
- Single source of truth
- All logic in one place
- Easy to test and maintain
- Users confused less ("Just run `bash scripts/install.sh install`")

---

## Platform Detection Logic

The script uses `$OSTYPE` which is:
- **Linux**: `linux-gnu`, `linux-musl`, `freebsd*`
- **macOS**: `darwin*`
- **Windows**: `msys`, `cygwin`, `win32`
- **WSL**: `linux-gnu` (detected as Linux; WSL runs Linux kernel)

If detected as Windows but Bash is available (WSL), script can still work.

---

## Error Handling

Script checks for:
- ✅ Cargo installed (required for building)
- ✅ Write permissions to installation directory
- ✅ Sufficient disk space
- ✅ Valid command arguments
- ✅ Binary found or building succeeds

If checks fail, provides helpful error message:
```
❌ Error: Unable to write to /usr/local/bin
💡 Tip: Try running with sudo: sudo bash scripts/install.sh install
```

---

## Security Considerations

1. **No Remote Execution**: Script doesn't download binaries from internet
   - Builds locally with `cargo build --release`
   - You control the source code

2. **No Privilege Escalation**: Only uses `sudo` when installing to system directories
   - User can read the script before running
   - Transparent what permissions are needed

3. **No External Dependencies**: Just requires:
   - Bash 4.0+ (standard on all modern systems)
   - Cargo (if building locally)

---

## Testing the Script

Before distributing, test on:

### Linux
- [ ] Ubuntu 20.04 LTS (Debian-based)
- [ ] CentOS 8 (RHEL-based)
- [ ] Alpine Linux (musl-based)

### macOS
- [ ] Intel Mac (x86_64)
- [ ] Apple Silicon (arm64)

### Windows
- [ ] PowerShell 5.1
- [ ] Windows 11
- [ ] If Bash available: WSL2 with Ubuntu

Quick test:
```bash
bash scripts/install.sh build
./target/release/octaskly --version
```

---

## Future Enhancements

### Package Manager Integration (Optional)
Instead of universal script, also provide:
- **Homebrew**: `brew install octaskly`
- **APT/Snap**: Ubuntu/Debian repositories
- **Pacman**: Arch Linux AUR
- **RPM**: CentOS/RHEL repositories
- **Chocolatey**: Windows package manager

This would be IN ADDITION to the universal script, not replacing it.

Reasoning: Some users prefer their package manager, but script provides fallback for everyone.

### Automatic Updates
```bash
octaskly update          # Fetches latest binary from GitHub releases
octaskly version         # Shows current version
```

Implementation: Curl + GitHub API to check for newer versions, download if available.

### Cross-Compilation
Pre-build binaries for multiple architectures:
- Windows x86_64
- macOS Intel (x86_64)
- macOS ARM64 (Apple Silicon)
- Linux x86_64
- Linux ARM64 (Raspberry Pi, etc)

Then `install.sh` could download pre-built instead of always building.

### Configuration Management
```bash
octaskly config set dispatcher.port 8080
octaskly config show
```

Stores in platform-appropriate locations:
- Windows: `%APPDATA%\octaskly\config.toml`
- macOS/Linux: `~/.config/octaskly/config.toml`

---

## Comparison with Industry Standards

How do other tools do it?

### Node.js (npm)
- Single installer per OS
- Auto-adds to PATH
- Single `npm` command works everywhere
- This is what Octaskly mimics

### Rust (rustup)
- Single `rustup-init.sh` for all Unix systems
- Separate installer for Windows
- Auto-builds and installs
- We've simplified further: one script for all

### Git
- Platform-specific installers
- But each is maintainer heavy
- Our approach is lighter weight

### Docker
- Single script: `curl https://get.docker.com | sh`
- Downloads + installs
- We build locally instead (more secure)

---

## Maintenance

To update the installer:
1. Edit `scripts/install.sh`
2. Test on representative systems
3. Commit and push
4. Users automatically have new installer when they pull latest

No need to update 6 different files. Just one.

---

## Installation Locations

| OS | Path | Reason |
|---|---|---|
| Windows | `C:\Program Files\octaskly\` | Standard for user programs |
| macOS | `/usr/local/bin/` | User-writable, on PATH by default |
| Linux | `/usr/local/bin/` | Universal location, FHS compliant |

These are standard locations where users expect system utilities to live.

---

## File Permissions

After installation, binary has:
- **Linux/macOS**: `755` (user+group+others can read+execute)
- **Windows**: Inherits from directory ACLs

Examples:
```bash
# After Linux install
ls -l /usr/local/bin/octaskly
-rwxr-xr-x  1 user  group  45000000  Dec 10 12:00 /usr/local/bin/octaskly

# After macOS install  
ls -l /usr/local/bin/octaskly
-rwxr-xr-x  1 user  staff  45000000  Dec 10 12:00 /usr/local/bin/octaskly
```

---

## Summary

**Old**: 6+ scripts managing installation separately per OS
**New**: 1 script with runtime OS detection

**Result**: 
- 65% less code
- 100% less duplication
- 99% easier to maintain
- 100% better user experience

One `bash scripts/install.sh install` and you're done.
