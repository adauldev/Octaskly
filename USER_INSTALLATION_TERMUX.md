# Octaskly Installation Guide - Termux (Android)

**Version**: 1.0.0  
**Date**: February 7, 2026  
**Platform**: Termux (Android Terminal Emulator)

---

## Overview

**What is Termux?**

Termux is a powerful terminal emulator for Android that lets you run a full Linux environment. With Termux, you can:
- Run Octaskly as a task scheduler on your Android device
- Connect to Linux/Windows/macOS devices over network
- Use Octaskly for distributed task processing on Android

**Device Requirements**:
- Android 5.0+
- ~200 MB free storage
- Termux app installed from Google Play or F-Droid

---

## Installation

### Option 1: One-Line Installation from GitHub (Easiest)

If Octaskly is available on GitHub:

```bash
curl -fsSL https://raw.githubusercontent.com/your-username/octaskly/main/scripts/install.sh | bash
```

**What it does**:
1. Downloads install script from GitHub
2. Auto-detects Termux environment
3. Builds binary if Rust is installed
4. Installs to `$PREFIX/bin/` (automatic PATH)

**Done!** Type: `octaskly --version`

---

### Option 2: Clone and Install from GitHub

```bash
# Clone repository
git clone https://github.com/your-username/octaskly.git
cd octaskly

# Run installer
bash scripts/install.sh install
```

---

### Option 3: Using wget Instead of curl

If `curl` is not available:

```bash
# One-line
wget -qO- https://raw.githubusercontent.com/your-username/octaskly/main/scripts/install.sh | bash
```

Or clone:
```bash
git clone https://github.com/your-username/octaskly.git
cd octaskly
bash scripts/install.sh install
```

---

## Step-by-Step: Build and Install Manually

For complete control over the process:

### Step 1: Install Rust (First Time Only)

```bash
pkg update
pkg install rust
```

**Or install Rust via rustup**:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash
source $HOME/.cargo/env
```

### Step 2: Clone Repository

```bash
git clone https://github.com/your-username/octaskly.git
cd octaskly
```

### Step 3: Build Release Binary

```bash
cargo build --release
```

**Building...**  
This may take a few minutes. The binary will be at:
```
target/release/octaskly
```

### Step 4: Install to System

Termux automatically handles PATH - just copy to `$PREFIX/bin/`:

```bash
cp target/release/octaskly $PREFIX/bin/
chmod +x $PREFIX/bin/octaskly
```

### Step 5: Verify Installation

```bash
octaskly --version
```

You should see:
```
octaskly 1.0.0
```

✅ **Success!** Octaskly is installed and available.

---

## What is `$PREFIX` in Termux?

In Termux, `$PREFIX` is a special variable pointing to your Termux environment:

```bash
# See your $PREFIX
echo $PREFIX
# Usually: /data/data/com.termux/files/usr
```

**Why use `$PREFIX`?**
- Writable by your user account
- Automatically in PATH
- Only location for system utilities in Termux
- Survives reinstalls with `pkg update`

---

## Installing Rust in Termux

Termux has two ways to install Rust:

### Option A: Package Manager (Easier)

```bash
pkg update
pkg install rust
```

**Pros**: Simple, one command  
**Cons**: May be older version

### Option B: Rustup (Recommended)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash
source $HOME/.cargo/env
```

**Pros**: Latest version, easy updates  
**Cons**: Takes more space initially

---

## First Time Setup (Termux)

If this is your first time in Termux:

### Step 1: Update Packages

```bash
pkg update
pkg upgrade
```

### Step 2: Install Basic Tools

```bash
pkg install git curl wget
```

### Step 3: Install Rust

```bash
pkg install rust
# or use rustup (see above)
```

### Step 4: Create Workspace

```bash
mkdir -p ~/src
cd ~/src
```

Now you can clone and build Octaskly!

---

## Using Octaskly in Termux

### Check Installation

```bash
octaskly --version
octaskly --help
```

### Common Commands

```bash
# Start dispatcher
octaskly dispatcher --port 7878

# Connect worker
octaskly worker --name myphone
```

### Running in Background

Keep Octaskly running while you close terminal:

```bash
# Using nohup
nohup octaskly dispatcher --port 7878 &

# Or use screen/tmux
screen -S octaskly
octaskly dispatcher --port 7878
# Detach: Ctrl+A then D
```

---

## Network Access

### Connecting from Another Device

If you're running Octaskly on Termux and want to access it from PC:

**Find your Termux device IP**:
```bash
ifconfig
# Look for inet address (e.g., 192.168.1.100)
```

**Connect from another device**:
```bash
# Linux/macOS/WSL
octaskly --connect 192.168.1.100:7878

# Or see docs for your platform
```

### Port Forwarding

Running behind NAT? Set up port forwarding:
```bash
# Forward port 7878 to your router
# (See your router documentation)
```

---

## Storage and Permissions

### Important: Storage Access in Termux

Termux has **limited access** to Android storage. To access downloads:

```bash
# Check available storage
ls -la /sdcard/

# Or use shared storage
ls -la $HOME/storage/
```

If storage is not linked:
```bash
# Grant storage permission (one time)
termux-setup-storage
```

### Installing Files

If you download an installer to Android:
```bash
# Find file
ls -la $HOME/storage/downloads/

# Copy to Termux
cp $HOME/storage/downloads/octaskly-1.0.0.deb .
```

---

## Troubleshooting

### "curl: command not found"

**Solution**: Install curl
```bash
pkg install curl
```

Then try installation again.

### "git: command not found"

**Solution**: Install git
```bash
pkg install git
```

Then clone the repository.

### "cargo: command not found"

**Solution**: Rust not installed
```bash
pkg install rust
# or use rustup (see above)
```

### Build fails with memory errors

**Cause**: Termux limited resources

**Solutions**:
1. **Close other apps** to free memory
2. **Build with less parallelism**:
   ```bash
   cargo build --release -j 1
   ```
3. **Use pre-built binary** instead (if available)

### "octaskly: command not found" after install

**Cause**: PATH not reloaded

**Solution**:
```bash
# Reload shell
bash
# or
exit
# Then reopen Termux
```

### Permission denied when running

**Cause**: Binary not executable

**Solution**:
```bash
chmod +x $PREFIX/bin/octaskly
octaskly --version
```

### Slow building

Termux on mobile is slower than desktop:
- First build: 10-20 minutes
- Subsequent builds: 2-5 minutes

This is normal. Let it finish.

### "No space left on device"

**Cause**: Termux storage full

**Solutions**:
1. **Check space**:
   ```bash
   df -h
   ```

2. **Remove build cache**:
   ```bash
   cargo clean
   ```

3. **Reinstall Termux** if very full

---

## Uninstallation

### Remove Octaskly

```bash
rm $PREFIX/bin/octaskly
```

### Uninstall Rust (if not using it)

```bash
# If installed via pkg
pkg remove rust

# If installed via rustup
rustup self uninstall
```

### Uninstall Termux (Android)

1. Open Settings
2. Apps
3. Find "Termux"
4. Uninstall

---

## Advanced: Building with Custom Options

### With Custom Features

```bash
cargo build --release --features feature1,feature2
```

### With Optimization

Slower build, faster runtime:
```bash
RUSTFLAGS="-C target-cpu=native -C lto=fat" cargo build --release -j 1
```

### Cross-Compiling to Another Platform

From Termux, build for desktop:
```bash
# This is complex - see Rust docs
```

---

## Performance Notes

### Termux vs Desktop

| Task | Desktop | Termux |
|------|---------|--------|
| Build binary | 2-3 min | 15-20 min |
| Run tasks | 10ms latency | 20-50ms latency |
| Memory usage | Normal | Limited |
| CPU usage | Normal | Limited |

Termux works great for:
- ✅ Development/testing
- ✅ Lightweight task processing
- ✅ Network coordination

Termux limitations:
- ❌ CPU-intensive work
- ❌ Large file processing
- ❌ Always-on server (battery drain)

---

## Next Steps

1. **Installation successful?** Run:
   ```bash
   octaskly --help
   ```

2. **See main README** for usage:
   ```bash
   cat README.md
   ```

3. **More help?**
   - [GitHub Installation](USER_INSTALLATION_GITHUB.md)
   - [Main Installation Guide](USER_INSTALLATION.md)
   - [Termux Support Docs](TERMUX_SUPPORT.md)

---

**Back to**: [User Installation Guide](USER_INSTALLATION.md)
