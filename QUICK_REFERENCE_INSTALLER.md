# Octaskly Installer Quick Reference Card

## For Developers: Build Command

```bash
# One command to build all installers for your OS
bash scripts/build-all.sh

# Platform-specific (if one-command doesn't work)
# Windows: iscc installer/octaskly-installer.iss
# macOS: bash scripts/build-macos-pkg.sh
# Linux: bash scripts/build-linux-deb.sh amd64
```

## For Users: Installation by OS

### Windows
```
Download: octaskly-setup-1.0.0.exe
Install: Double-click → Next → Install
Done: Binary in PATH
```

### macOS
```
Download: octaskly-1.0.0.pkg
Install: Double-click → Enter password
Done: Binary in PATH
```

### Linux (Debian/Ubuntu)
```
Download: octaskly_1.0.0_amd64.deb
Install: sudo dpkg -i octaskly_1.0.0_amd64.deb
Done: Binary in PATH
```

### Linux (RedHat/Fedora)
```
Download: octaskly-1.0.0-1.x86_64.rpm
Install: sudo rpm -i octaskly-1.0.0-1.x86_64.rpm
Done: Binary in PATH
```

### Linux (Any Distribution)
```
Download: octaskly-installer-1.0.0.run
Install: sudo ./octaskly-installer-1.0.0.run
Done: Binary in PATH
```

## Verify Installation

```bash
octaskly --version
```

## Files & Locations

| Purpose | File |
|---------|------|
| Build guide | [BUILD_AND_DISTRIBUTE.md](BUILD_AND_DISTRIBUTE.md) |
| Reference | [INSTALLATION_REFERENCE.md](INSTALLATION_REFERENCE.md) |
| Details | [NATIVE_INSTALLERS.md](NATIVE_INSTALLERS.md) |
| Completion | [PHASE3_COMPLETION.md](PHASE3_COMPLETION.md) |
| Tech setup | [SESSION_SUMMARY.md](SESSION_SUMMARY.md) |

## OS Detection

```bash
# Windows
iscc installer/octaskly-installer.iss

# macOS
bash scripts/build-macos-pkg.sh

# Linux (auto-detects and builds deb/rpm)
bash scripts/build-linux-deb.sh amd64 && bash scripts/build-linux-rpm.sh x86_64

# Or use universal build
bash scripts/build-all.sh
```

## Output Locations

```
dist/
├── windows/octaskly-setup-1.0.0.exe
├── macos/octaskly-1.0.0.pkg
└── linux/
    ├── octaskly_1.0.0_amd64.deb
    ├── octaskly-1.0.0-1.x86_64.rpm
    └── octaskly-installer-1.0.0.run
```

## Installer Requirements

| OS | Required | Command |
|----|----------|---------|
| Windows | Inno Setup | Download from https://jrsoftware.com/isdl.php |
| macOS | Xcode tools | `xcode-select --install` |
| Linux .deb | dpkg-deb | `sudo apt-get install build-essential` |
| Linux .rpm | rpmbuild | `sudo dnf install rpm-build` |
| All | Rust | Already in `Cargo.toml` |

## Troubleshooting (TL;DR)

| Issue | Solution |
|-------|----------|
| iscc not found | Install Inno Setup |
| pkgbuild fails | Install Xcode Command Line Tools |
| dpkg not found | Install build-essential |
| rpmbuild fails | Install rpm-build |
| Binary not found | Run: `cargo build --release` first |

## Version Info

Current version: **1.0.0** (from Cargo.toml)

All installers automatically use this version in filenames:
- `octaskly-setup-1.0.0.exe`
- `octaskly-1.0.0.pkg`
- `octaskly_1.0.0_amd64.deb`
- etc.

To update version: Edit `Cargo.toml`, then rebuild.

## Features by Installer

| Feature | Windows | macOS | Linux |
|---------|---------|-------|-------|
| GUI installer | ✅ | ✅ | .deb/.rpm only |
| Auto PATH | ✅ | ✅ | ✅ |
| Uninstaller | ✅ | ✅ | ✅ |
| Code signing | ✅ | ✅ | ✅ |
| Admin check | ✅ | ✅ | ✅ |

## Download Page Example

```
Octaskly Downloads

Windows
  octaskly-setup-1.0.0.exe | 15 MB
  [Download] Click and run

macOS
  octaskly-1.0.0.pkg | 12 MB
  [Download] Click and run

Linux
  octaskly_1.0.0_amd64.deb | 12 MB
  octaskly-1.0.0-1.x86_64.rpm | 12 MB
  octaskly-installer-1.0.0.run | 12 MB
  [Download each] Follow install instructions
```

## Next Steps

1. **Build**: `bash scripts/build-all.sh` on each OS
2. **Test**: Install and run `octaskly --version`
3. **Sign** (optional): Code sign for enterprise
4. **Upload**: To website/GitHub Releases
5. **Distribute**: Share download link with users

## Enterprise Checklist

- [ ] Build all installers on each platform
- [ ] Test installation on each platform
- [ ] Verify binary works: `octaskly --version`
- [ ] Generate SHA256 checksums: `cd dist && find . -type f -exec sha256sum {} \;`
- [ ] Create GitHub Releases with installers
- [ ] Update download page
- [ ] Optional: Code sign installers
- [ ] Optional: Set up CI/CD pipeline

## That's It!

Users are now 2 minutes away from using Octaskly:
1. Download installer from website
2. Run it (double-click or `sudo` command)
3. Use `octaskly --version` to verify
4. Continue with normal usage

No cargo, no scripts, no terminal complexity.
This is professional software installation.

---

**See full details in:**
- [BUILD_AND_DISTRIBUTE.md](BUILD_AND_DISTRIBUTE.md) - Build guide
- [INSTALLATION_REFERENCE.md](INSTALLATION_REFERENCE.md) - Master reference
- [README.md](README.md) - Main documentation
