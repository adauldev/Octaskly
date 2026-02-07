# Octaskly Installation Guide - Linux

**Version**: 1.0.0  
**Date**: February 7, 2026  
**Platform**: All major Linux distributions

---

## Overview

This guide shows you how to install Octaskly on Linux. Choose your distribution type and installation method.

---

## Which Distribution Do You Have?

Not sure? Run this command:

```bash
cat /etc/os-release
```

Or:

```bash
lsb_release -d
```

Check the result:
- **Ubuntu, Debian, Linux Mint, Elementary OS** → Use [.deb Section](#option-1-debian--ubuntu-deb)
- **Fedora, RHEL, CentOS, Rocky Linux, AlmaLinux** → Use [.rpm Section](#option-2-fedora--rhel-rpm)
- **Other/Unsure** → Use [Universal .run Section](#option-3-universal-installer-run)

---

## Option 1: Debian / Ubuntu (.deb)

For Ubuntu, Debian, Linux Mint, Elementary OS, and similar.

### Step 1: Download

- Download `octaskly_1.0.0_amd64.deb` from the downloads page
- Save to your `Downloads` folder (or anywhere)

### Step 2: Install

**Method A: Using GUI (Easiest)**

1. Open file manager
2. Navigate to `Downloads` folder
3. Right-click `octaskly_1.0.0_amd64.deb`
4. Select "Open with" → "Software Center" (or "Software Install")
5. Click "Install"
6. Enter your password when prompted
7. Wait for installation to complete

**Method B: Using Command Line**

Open Terminal and run:

```bash
sudo apt install ./Downloads/octaskly_1.0.0_amd64.deb
```

Or if file is in current directory:

```bash
sudo apt install ./octaskly_1.0.0_amd64.deb
```

When prompted, enter your password.

**Method C: Using dpkg (Direct)**

```bash
sudo dpkg -i ~/Downloads/octaskly_1.0.0_amd64.deb
```

### Step 3: Verify Installation

```bash
octaskly --version
```

You should see:
```
octaskly 1.0.0
```

✅ **Success!** Octaskly is installed.

### Uninstall

```bash
sudo apt remove octaskly
```

---

## Option 2: Fedora / RHEL (.rpm)

For Fedora, RHEL, CentOS, Rocky Linux, AlmaLinux, and similar.

### Step 1: Download

- Download `octaskly-1.0.0-1.x86_64.rpm` from the downloads page
- Save to your `Downloads` folder (or anywhere)

### Step 2: Install

**Method A: Using GUI (Easiest)**

1. Open file manager
2. Navigate to `Downloads` folder
3. Right-click `octaskly-1.0.0-1.x86_64.rpm`
4. Select "Open with" → "Software Center" (or "Software Install")
5. Click "Install"
6. Enter your password when prompted
7. Wait for installation to complete

**Method B: Using dnf (Recommended)**

```bash
sudo dnf install ~/Downloads/octaskly-1.0.0-1.x86_64.rpm
```

**Method C: Using rpm (Direct)**

```bash
sudo rpm -i ~/Downloads/octaskly-1.0.0-1.x86_64.rpm
```

### Step 3: Verify Installation

```bash
octaskly --version
```

You should see:
```
octaskly 1.0.0
```

✅ **Success!** Octaskly is installed.

### Uninstall

```bash
sudo dnf remove octaskly
```

Or with rpm:
```bash
sudo rpm -e octaskly
```

---

## Option 3: Universal Installer (.run)

For any Linux distribution. Use this if your distro isn't listed above.

### Step 1: Download

- Download `octaskly-installer-1.0.0.run` from the downloads page
- Save to your `Downloads` folder (or anywhere)

### Step 2: Make Executable

```bash
chmod +x ~/Downloads/octaskly-installer-1.0.0.run
```

### Step 3: Run Installer

```bash
sudo ~/Downloads/octaskly-installer-1.0.0.run
```

The installer will:
- Prompt for your password
- Extract files to `/opt/octaskly/`
- Set up system-wide availability
- Create an uninstaller

### Step 4: Verify Installation

```bash
octaskly --version
```

You should see:
```
octaskly 1.0.0
```

✅ **Success!** Octaskly is installed.

### Uninstall

```bash
sudo /opt/octaskly/uninstall.sh
```

---

## Option 4: Manual Installation (Advanced)

If none of the above work for you:

### Step 1: Download Binary

- Download the `octaskly` binary file (not `.deb` or `.rpm`)
- Save to your computer (e.g., Downloads folder)

### Step 2: Install

```bash
# Copy to system location
sudo cp ~/Downloads/octaskly /usr/local/bin/

# Make it executable
sudo chmod +x /usr/local/bin/octaskly
```

### Step 3: Verify

```bash
octaskly --version
```

You should see:
```
octaskly 1.0.0
```

✅ **Success!** Octaskly is installed.

### Uninstall

```bash
sudo rm /usr/local/bin/octaskly
```

---

## Troubleshooting

### "sudo: command not found" or "not in sudoers file"

**Cause**: Your user account doesn't have sudo privileges

**Solutions**:

1. **Ask your system administrator** to install Octaskly for you

2. **If you're the admin**, add your user to sudoers:
   ```bash
   su -
   usermod -aG sudo yourusername
   logout
   # Log back in
   ```

### "octaskly: command not found"

**Cause**: Command not in PATH after installation

**Solutions**:

1. **Reload your shell**:
   ```bash
   source ~/.bashrc
   ```
   Then try again

2. **Check if installed**:
   ```bash
   which octaskly
   ```
   Should show `/usr/local/bin/octaskly`

3. **If not found, check file**:
   ```bash
   ls -la /usr/local/bin/octaskly
   ```

4. **Try full path**:
   ```bash
   /usr/local/bin/octaskly --version
   ```

### "Permission denied" when running installer

**Cause**: File doesn't have execute permissions

**Solutions**:

1. **Set executable permission**:
   ```bash
   chmod +x ~/Downloads/octaskly-installer-1.0.0.run
   ```
   Then try again

2. **Always use `sudo` for system installation**:
   ```bash
   sudo ./octaskly-installer-1.0.0.run
   ```

### "File not found" or "No such file"

**Cause**: File path is incorrect

**Solutions**:

1. **Check file exists**:
   ```bash
   ls -la ~/Downloads/octaskly*
   ```

2. **Use correct filename as shown above**:
   ```bash
   sudo apt install ./octaskly_1.0.0_amd64.deb
   ```

3. **If in same directory as file**:
   ```bash
   sudo apt install ./octaskly_1.0.0_amd64.deb
   ```

### "Unmet dependencies" or "Broken packages"

**Cause**: Missing system dependencies

**Solutions**:

1. **Try auto-fix**:
   ```bash
   sudo apt --fix-broken install
   ```

2. **Or remove and reinstall**:
   ```bash
   sudo apt remove octaskly
   sudo apt install ./octaskly_1.0.0_amd64.deb
   ```

### Package manager won't find octaskly after install

**Cause**: Cache not refreshed

**Solutions**:

1. **Refresh package cache**:
   ```bash
   # Debian/Ubuntu
   sudo apt update
   
   # Fedora/RHEL
   sudo dnf update
   ```

2. **Then verify**:
   ```bash
   octaskly --version
   ```

### 32-bit vs 64-bit Version

**Check your system**:

```bash
uname -m
```

- Shows `x86_64` = 64-bit (use `amd64` or `x86_64` version)
- Shows `i686` = 32-bit (use `i386` version if available)
- Shows `aarch64` = ARM 64-bit (use `arm64` version if available)

Download the correct version for your architecture.

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
