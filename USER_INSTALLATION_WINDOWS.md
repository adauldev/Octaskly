# Octaskly Installation Guide - Windows

**Version**: 1.0.0  
**Date**: February 7, 2026  
**Platform**: Windows 10+

---

## Overview

This guide shows you how to install Octaskly on Windows. Choose your preferred installation method.

---

## Option 1: Installer (Recommended ⭐)

The easiest way to install Octaskly on Windows.

### Step 1: Download

- Go to [GitHub Releases](https://github.com/adauldev/octaskly/releases/latest)
- Download `octaskly-setup-1.0.0.exe`
- Save to your Downloads folder

### Step 2: Run the Installer

- Double-click `octaskly-setup-1.0.0.exe`
- A wizard window will appear
- Click "Next" to proceed

### Step 3: Choose Installation Location

- The installer suggests `C:\Program Files\octaskly\`
- Click "Next" to accept or choose a different location
- Click "Install"

### Step 4: Complete Installation

- Wait for installation to complete
- Check the box "Create desktop shortcut" (optional)
- Click "Finish"

### Step 5: Verify Installation

1. Open a new **Command Prompt** or **PowerShell**
   - Press `Win + R`, type `cmd` or `powershell`, press Enter
   - Or search for "Command Prompt" in Start menu

2. Type the verification command:
   ```powershell
   octaskly --version
   ```

3. You should see:
   ```
   octaskly 1.0.0
   ```

✅ **Success!** Octaskly is installed and ready to use.

---

## Option 2: Windows Package Managers (Advanced)

If you prefer using package managers:

### Using winget

```powershell
winget install octaskly
```

**Requires**: Windows 10+ with winget installed (usually pre-installed)

**Result**: Automatically adds to PATH

---

### Using Chocolatey

First, install Chocolatey if you don't have it:

```powershell
# Run as Administrator
Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))
```

Then install Octaskly:

```powershell
choco install octaskly
```

---

## Option 3: Build from GitHub (Advanced)

**No pre-built installers available?** Clone from GitHub and build yourself:

### Prerequisites

- Git for Windows: https://git-scm.com/download/win
- Rust/Cargo: https://rustup.rs/ (one-time setup)

### Step 1: Clone Repository

```powershell
git clone https://github.com/your-username/octaskly.git
cd octaskly
```

Replace `your-username` with actual GitHub username and resolve branch names.

### Step 2: Build Release Binary

```powershell
cargo build --release
```

This will take 5-10 minutes on first build. Binary will be at:
```
target\release\octaskly.exe
```

### Step 3: Install to System

```powershell
mkdir "C:\Program Files\octaskly" -Force
Copy-Item "target\release\octaskly.exe" -Destination "C:\Program Files\octaskly\"
```

### Step 4: Add to PATH

Add `C:\Program Files\octaskly\` to System PATH (see [Option 5](#option-5-manual-installation-advanced) for PATH setup instructions).

### Step 5: Verify

```powershell
octaskly --version
```

✅ **Success!** Octaskly is built and installed.

---

## Option 4: Build on Termux (Android)

**Advanced**: Build on Android using Termux terminal emulator, then transfer to Windows:

### Step 1: Set up Termux

1. Install "Termux" from Google Play or F-Droid
2. Open Termux and update:
   ```bash
   pkg update && pkg upgrade
   ```

3. Install Rust:
   ```bash
   pkg install rust
   ```

### Step 2: Clone and Build

```bash
# In Termux
git clone https://github.com/your-username/octaskly.git
cd octaskly
cargo build --release
```

Binary will be at: `/data/data/com.termux/files/home/octaskly/target/release/octaskly`

### Step 3: Transfer to Windows

1. **Set up SSH in Termux**:
   ```bash
   pkg install openssh
   sshd  # Start SSH server
   ```

2. **From Windows, copy binary via SSH**:
   ```powershell
   # Get Termux IP
   # From Termux: ip addr

   # Copy file
   scp -P 8022 u0_a123@192.168.1.xxx:/data/data/com.termux/files/home/octaskly/target/release/octaskly C:\octaskly\
   ```

3. **Or use file transfer app** like Syncthing, Nextcloud, or AirDroid

See [Termux Installation Guide](USER_INSTALLATION_TERMUX.md) for details.

---

## Option 5: Manual Installation (Advanced)

If you prefer more control or the installer doesn't work:

### Step 1: Extract the Binary

1. Download `octaskly.exe` to your computer
2. Create a folder: `C:\octaskly\`
3. Move `octaskly.exe` into `C:\octaskly\`

### Step 2: Add to System PATH

This allows you to use the `octaskly` command from anywhere.

**Method A: Using GUI (Easiest)**

1. Press `Win + X` and select "System"
2. Click "Advanced system settings" (or search "Environment Variables")
3. Click "Environment Variables" button
4. Under "User variables", select the row with `Path` and click "Edit"
5. Click "New"
6. Type: `C:\octaskly`
7. Click "OK" three times
8. Close any open Command Prompt/PowerShell and open a new one

**Method B: Using Command Line (PowerShell as Admin)**

```powershell
# Run PowerShell as Administrator
[Environment]::SetEnvironmentVariable("PATH", "$env:PATH;C:\octaskly", "User")
```

Then close and reopen PowerShell.

### Step 3: Verify

1. Open a new **Command Prompt** or **PowerShell**
2. Type:
   ```powershell
   octaskly --version
   ```
3. You should see:
   ```
   octaskly 1.0.0
   ```

✅ **Success!** Octaskly is installed.

---

## Troubleshooting

### "octaskly: command not found" or "'octaskly' is not recognized"

**Cause**: Command not found in PATH

**Solutions**:
1. **Close and reopen your terminal** - new PATH takes effect after restart
2. **Verify PATH was set correctly**:
   ```powershell
   echo $env:PATH
   ```
   Look for `C:\octaskly` (or wherever you installed it)

3. **Manually verify the file exists**:
   ```powershell
   Get-Item C:\octaskly\octaskly.exe
   ```
   If this returns an error, the file isn't in that location

4. **Try full path**:
   ```powershell
   C:\octaskly\octaskly.exe --version
   ```

### Installation fails with "Permission denied"

**Cause**: Don't have admin rights to install system-wide

**Solutions**:
1. **Run as Administrator**:
   - Right-click installer → "Run as administrator"
   - Or use PowerShell as admin for manual install

2. **Install to user folder instead**:
   - Use a location like `C:\Users\YourUsername\AppData\Local\octaskly\`
   - Add that path to your PATH instead

### "Setup has detected incompatible architecture"

**Cause**: Wrong processor version downloaded

**Solutions**:
1. Check if you're on 64-bit or 32-bit Windows:
   - Press `Win + R`, type `wmic os get osarchitecture`, press Enter
   - Shows either `64-bit` or `32-bit`

2. Download the correct version from downloads page

3. If only 64-bit is available and you're on 32-bit, contact support

### Installer won't run

**Cause**: Windows SmartScreen or antivirus blocking

**Solutions**:
1. **Windows SmartScreen**:
   - Click "More info" in the dialog
   - Click "Run anyway"

2. **Antivirus**:
   - Temporarily disable antivirus
   - Or add exception for the installer
   - Run installer again

3. **Try manual installation** (see Option 2 above)

---

## Uninstallation

### Using Control Panel (Easiest)

1. Press `Win + X` and select "Apps and features"
   - Or: Settings → Apps → Apps & features

2. Search for "octaskly" or "Octaskly"

3. Select it and click "Uninstall"

4. Confirm the uninstallation

### Manual Uninstallation

1. Delete the installation folder:
   ```powershell
   Remove-Item "C:\Program Files\octaskly" -Recurse
   ```

2. Remove from PATH (if you added it manually):
   - Follow "Add to System PATH" section above
   - Edit the PATH variable
   - Remove the octaskly entry

---

## Next Steps

After successful installation:

1. **View all commands**:
   ```powershell
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
