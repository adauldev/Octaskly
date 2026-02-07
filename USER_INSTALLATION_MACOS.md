# Octaskly Installation Guide - macOS

**Version**: 1.0.0  
**Date**: February 7, 2026  
**Platform**: macOS 10.13+

---

## Overview

This guide shows you how to install Octaskly on macOS. Choose your preferred installation method.

---

## Option 1: Installer (Recommended ⭐)

The easiest way to install Octaskly on macOS.

### Step 1: Download

- Go to [GitHub Releases](https://github.com/adauldev/octaskly/releases/latest)
- Download `octaskly-1.0.0.pkg`
- Save to your Downloads folder

### Step 2: Run the Installer

1. Open **Finder**
2. Navigate to Downloads folder
3. Double-click `octaskly-1.0.0.pkg`
4. A window will appear asking to confirm

### Step 3: Follow the Installer

1. Click "Continue" in the introduction screen
2. Read the license agreement
3. Click "Agree" to accept the license
4. Choose installation location
   - Default is `/usr/local/bin/` (recommended)
   - Click "Install"
5. Enter your macOS password when prompted
   - This is required for system-wide installation
6. Wait for installation to complete
7. Click "Close"

### Step 4: Verify Installation

1. Open **Terminal**
   - Press `Cmd + Space`, type `terminal`, press Enter
   - Or open Applications → Utilities → Terminal

2. Type the verification command:
   ```bash
   octaskly --version
   ```

3. You should see:
   ```
   octaskly 1.0.0
   ```

✅ **Success!** Octaskly is installed and ready to use.

---

## Option 2: Manual Installation (Advanced)

If you prefer more control or the installer doesn't work:

### Step 1: Download the Binary

- Download the `octaskly` binary file (not `.pkg`)
- Save to your Downloads folder

### Step 2: Move to System Location

Open Terminal and run:

```bash
# Copy the binary to system location
sudo cp ~/Downloads/octaskly /usr/local/bin/

# Make it executable
sudo chmod +x /usr/local/bin/octaskly
```

When prompted, enter your macOS password.

### Step 3: Verify

```bash
octaskly --version
```

You should see:
```
octaskly 1.0.0
```

✅ **Success!** Octaskly is installed.

---

## Troubleshooting

### "octaskly: command not found"

**Cause**: Command not found in PATH

**Solutions**:

1. **Close and reopen Terminal**:
   - Close Terminal window
   - Open a new one
   - Try `octaskly --version` again

2. **Check if installed correctly**:
   ```bash
   which octaskly
   ```
   Should show `/usr/local/bin/octaskly`

3. **OR check with full path**:
   ```bash
   /usr/local/bin/octaskly --version
   ```

4. **If not helpful, try refresh shell**:
   ```bash
   source ~/.bash_profile
   # or for zsh users:
   source ~/.zshrc
   ```
   Then try again

4. **Try full path**:
   ```bash
   /usr/local/bin/octaskly --version
   ```

### "Permission denied" during installation

**Cause**: Don't have write permissions to `/usr/local/bin/`

**Solutions**:

1. **Use `sudo` with installer**:
   - Right-click the `.pkg` file
   - Select "Open"
   - Enter your macOS password when prompted

2. **Install to home directory instead**:
   ```bash
   mkdir -p ~/bin
   cp octaskly ~/bin/
   chmod +x ~/bin/octaskly
   ```
   Then add to PATH in `~/.bash_profile`:
   ```bash
   export PATH="$HOME/bin:$PATH"
   ```
   Reload: `source ~/.bash_profile`

### "damaged application" error on macOS

**Cause**: macOS blocking unsigned applications

**Solutions**:

1. **Open with right-click**:
   - Find the `.pkg` file in Finder
   - Right-click it
   - Select "Open"
   - Click "Open" in the security dialog

2. **Allow unsigned apps** (temporary):
   - System Preferences → Security & Privacy → General
   - Click "Open anyway"

3. **Remove quarantine attribute**:
   ```bash
   xattr -d com.apple.quarantine ~/Downloads/octaskly-1.0.0.pkg
   ```
   Then try opening again

### "octaskly-1.0.0.pkg can't be opened"

**Cause**: File corruption or incomplete download

**Solutions**:

1. **Delete and re-download**:
   ```bash
   rm ~/Downloads/octaskly-1.0.0.pkg
   ```
   Then download again from the downloads page

2. **Check file integrity**:
   ```bash
   ls -lh ~/Downloads/octaskly-1.0.0.pkg
   ```
   Size should match what's shown on downloads page

3. **Try manual installation** (see Option 2 above)

### M1/M2 Mac (Apple Silicon) compatibility

**Q**: Will Octaskly work on my M1/M2 Mac?

**A**: Yes, if the binary is built for Apple Silicon (ARM64). Check the downloads page for `arm64` version, or it may auto-detect your architecture.

**If you get architecture errors**:
```bash
# Check your architecture
arch
```

- Shows `arm64` = Apple Silicon (M1/M2)
- Shows `i386` = Intel Mac

Make sure you download the correct version for your Mac.

---

## Uninstallation

### Option 1: Command Line (Easiest)

```bash
sudo rm /usr/local/bin/octaskly
```

When prompted, enter your macOS password.

### Option 2: Manual

1. Open **Finder**
2. Press `Cmd + Shift + G` (Go to folder)
3. Type: `/usr/local/bin/`
4. Find `octaskly`
5. Drag to Trash

---

## Next Steps

After successful installation:

1. **View all commands**:
   ```bash
   octaskly --help
   ```

2. **Start using Octaskly**:
   - See the main [README.md](README.md) for usage instructions
   - Check examples in `examples/` folder

3. **Getting help**:
   - Run `octaskly <command> --help` for command-specific help
   - Visit project documentation

---

**Back to**: [User Installation Guide](USER_INSTALLATION.md)
