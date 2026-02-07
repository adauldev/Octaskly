# Octaskly Installation Guide for Users

**Version**: 1.0.0  
**Date**: February 7, 2026

Welcome! This quick guide helps you install Octaskly on your computer. Choose your operating system below to get started.

---

## Choose Your Installation Method

### Option 1: Download Pre-Built Installers (Easiest)

| OS | Download | Installation Time |
|---|---|---|
| **🪟 Windows** | `.exe` installer | 2 minutes |
| **🍎 macOS** | `.pkg` installer | 2 minutes |
| **🐧 Linux** | `.deb` / `.rpm` / `.run` | 3 minutes |

👇 **See platform-specific guides below**

---

### Option 2: Install from GitHub (No Web Domain)

**Don't have download links?** Clone and build directly from GitHub repository:

- **Windows**: Clone and build (see [Windows Build from GitHub](USER_INSTALLATION_WINDOWS.md#option-3-build-from-github-advanced))
- **macOS**: Clone and build (see [macOS GitHub section](USER_INSTALLATION_MACOS.md))
- **Linux**: Clone and build (see [Linux GitHub section](USER_INSTALLATION_LINUX.md))
- **Termux/Android**: Clone and build (see [Termux Clone Option](USER_INSTALLATION_TERMUX.md#option-2-clone-and-install-from-github))

---

## Quick Overview

Octaskly is a task scheduler and worker coordination tool. 

- **Easy installation** - Installers available for all platforms
- **No coding required** for pre-built versions - Just download and install
- **GitHub source available** - Build from source if needed
- **Multiple installation methods** - Choose what works for you

---

## For Windows Users

🪟 **[View Windows Installation Guide](USER_INSTALLATION_WINDOWS.md)**

- Installer (recommended) or manual installation
- Step-by-step instructions
- Troubleshooting tips

---

## For macOS Users

🍎 **[View macOS Installation Guide](USER_INSTALLATION_MACOS.md)**

- Installer (recommended) or manual installation
- Apple Silicon (M1/M2) compatibility
- Troubleshooting tips

---

## For Linux Users

🐧 **[View Linux Installation Guide](USER_INSTALLATION_LINUX.md)**

- Debian/Ubuntu (.deb)
- Fedora/RHEL (.rpm)
- Universal installer (.run)
- Manual installation
- Troubleshooting for all distros

---

## What You'll Need

✅ A computer running Windows, macOS, or Linux  
✅ Administrator access (for system-wide installation)  
✅ 30-50 MB of disk space  
✅ No other dependencies needed

---

## Installation Methods Available

### Professional Installers (Easier)
- **Windows**: `.exe` installer with graphical wizard
- **macOS**: `.pkg` installer with wizard
- **Linux**: `.deb`, `.rpm`, or `.run` installers

### Manual Installation (Advanced)
All platforms support manual installation if you prefer more control.

---

## After Installation

Once installed, verify it works:

```bash
octaskly --version
```

Then:
1. Check available commands: `octaskly --help`
2. See the main [README.md](README.md) for usage
3. Check `examples/` folder for sample usage

---

## Getting Help

Each platform-specific guide includes a **Troubleshooting** section. 

**Common issues**:
- Command not found after installation
- Permission denied errors
- Download or installation problems

All covered in your platform's guide!

---

## Platform-Specific Guides

### 🪟 Windows Installation

**Methods available**:
- `.exe` installer (easiest) 
- Package managers: `winget` or `chocolatey`
- Manual installation
- GitHub clone and build
- Termux build on Android

**Features**:
- Detailed troubleshooting guide
- System PATH configuration help
- Multiple installation options

[👉 Full Windows Guide](USER_INSTALLATION_WINDOWS.md)

---

### 🍎 macOS Installation

**Methods available**:
- `.pkg` installer (easiest)
- Manual installation
- Homebrew (when available)
- GitHub clone and build

**Features**:
- Apple Silicon (M1/M2) support
- Detailed troubleshooting guide
- Multiple installation options

[👉 Full macOS Guide](USER_INSTALLATION_MACOS.md)

---

### 🐧 Linux Installation

**Methods available**:
- Package manager (Debian/Ubuntu .deb)
- Package manager (Fedora/RHEL .rpm)
- Universal installer (.run)
- Manual installation
- GitHub clone and build

**Features**:
- Distribution detection help
- Multiple installation methods per distro
- Detailed troubleshooting for all distros
- Architecture detection (32-bit, 64-bit, ARM)

[👉 Full Linux Guide](USER_INSTALLATION_LINUX.md)

---

### 🤖 Termux (Android)

**Build and run on Android devices**:

**Methods available**:
- GitHub one-line install
- Clone and build
- Build with Rust in Termux
- Transfer binary to Windows/Mac

**Features**:
- Complete Termux setup guide
- Network coordination between devices
- Storage and permission help
- Background running instructions

[👉 Full Termux Installation Guide](USER_INSTALLATION_TERMUX.md)

---

## FAQ

**Q: Do I need to install Rust or Cargo?**  
A: No for pre-built installers! Only if building from GitHub source code.

**Q: Can I install without administrator access?**  
A: Yes, all platform guides show user-level installation options.

**Q: Which version should I download?**  
A: Check your OS:
- Windows: `octaskly-setup-1.0.0.exe` (or use winget/choco)
- macOS: `octaskly-1.0.0.pkg`
- Linux: Choose based on your distribution (.deb, .rpm, or .run)

**Q: Where do I download the installers?**  
A: See [README.md](README.md) for download links. Or use [GitHub Installation](USER_INSTALLATION_GITHUB.md).

**Q: How do I install from GitHub without a web domain?**  
A: Clone the repository and build from source (instructions in each platform guide under "Build from GitHub/Source").

**Q: Can I use Termux/Android?**  
A: Yes! See [Termux Installation Guide](USER_INSTALLATION_TERMUX.md).

**Q: What's the difference between winget and chocolatey?**  
A: Both work on Windows. `winget` is newer (Windows 11+), `choco` broader support. Try winget first.

**Q: Can I build on Android and use on Windows?**  
A: Yes! Build in Termux, transfer binary to Windows - see [Termux Guide](USER_INSTALLATION_TERMUX.md).

**Q: How do I uninstall?**  
A: Check the uninstallation section in your platform's guide.

**Q: How much disk space do I need?**  
A: ~30-50 MB for binary + 200MB+ if building from source.

---

## Tech Specs

- **Installer sizes**: ~12-15 MB (depending on platform)
- **Disk space needed**: 30-50 MB
- **Dependencies**: Minimal (most included)
- **Platforms**: Windows 10+, macOS 10.13+, All Linux distros

---

**Start installing**: Choose your OS above!
