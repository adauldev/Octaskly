# Changelog

All notable changes to Octaskly are documented in this file. The format follows [Keep a Changelog](https://keepachangelog.com/) conventions, and this project adheres to [Semantic Versioning](https://semver.org/).

---

## [1.0.0] - 2026-02-07

### Added

#### Core Features
- **P2P Peer Discovery**: Complete peer-to-peer network discovery using UDP multicast and mDNS service announcements. Workers automatically detect and announce availability on local networks.
  - Resource advertisement: CPU cores, RAM available, GPU status, current load, task slots
  - Peer registry with automatic timeout-based cleanup
  - Periodic broadcast announcements every 30 seconds

- **P2P Task Distribution**: Intelligent task distribution algorithm that matches task requirements to available peer resources
  - Resource-aware peer selection based on CPU, RAM, and GPU requirements
  - Load-balancing through least-loaded peer selection
  - Task queuing with exponential backoff retry mechanism
  - Per-peer slot tracking for concurrent task limits

- **Protocol Extensions**: Message types for distributed task sharing
  - `ResourceAvailability`: Peer resource status broadcasts
  - `P2PShareTask`: Task distribution to remote peers
  - `P2PTaskResponse`: Peer acceptance/rejection responses
  - `PeerDiscoveryRequest/Response`: Explicit peer discovery protocol

- **Message Protocol**: Extended message protocol supporting hybrid execution patterns
  - Backward compatible with existing dispatcher-worker messages
  - Support for both centralized and distributed task execution
  - Graceful fallback when peer assignment fails

#### Installation & Distribution
- **Universal Cross-Platform Installer**: Single bash script with OS auto-detection
  - Automatic architecture detection (x86_64, ARM64)
  - Conditional binary building: `cargo build --release` if pre-built not available
  - Platform-specific installation paths:
    - Windows: `C:\Program Files\octaskly\`
    - macOS/Linux: `/usr/local/bin/`
  - Automatic PATH environment variable updates per platform
  - Supports: Windows (PowerShell/WSL), macOS (Intel/Apple Silicon), Linux (Debian, Ubuntu, RHEL, CentOS, Arch, Alpine)

#### Documentation
- **Installation Guide** (`INSTALLATION.md`): Comprehensive user-facing installation instructions
  - Quick start for each OS
  - Manual installation methods
  - Troubleshooting section
  - System requirements

- **Installer Architecture** (`INSTALLER_ARCHITECTURE.md`): Technical documentation of installation design
  - Design philosophy: One script vs. OS-specific duplication
  - Platform detection logic and implementation details
  - Security considerations
  - Future enhancement roadmap
  - Comparison with industry-standard tools (npm, git, cargo, rustup)

- **Code Comments**: Bilingual professional documentation (Indonesian/English)
  - All public APIs documented
  - Complex algorithms explained
  - Module-level documentation
  - Function signatures with parameter descriptions

### Changed

#### Architecture
- Dispatcher now spawns background P2P management tasks
  - `manage_p2p_distribution()`: Continuous task distribution loop
  - `handle_p2p_peer_updates()`: Periodic resource monitoring
  - `start_p2p_discovery()`: Peer discovery initialization

- Worker extended to execute P2P distributed tasks
  - `handle_p2p_task()`: Safe execution of remote tasks
  - Result reporting back to task distributor

#### Build System
- Single unified installer script replaces 6 platform-specific installers
- Simplified workflow: no more confusion about which script to run
- Automatic binary building integrated into installation process

#### Documentation
- README updated: All references point to global `octaskly` command
- Removed `.target/release/` path references in examples
- Consolidated installation instructions to single method

### Fixed

- Compilation warnings:
  - Marked unused `SERVICE_TYPE` constant with `#[allow(dead_code)]`
  - Marked unused `get_local_ip()` function with `#[allow(dead_code)]`
  
### Removed

#### Deprecated Installation Files
- `installer/` folder (all contents)
- `octaskly-installer.iss` (Windows installer configuration)
- `octaskly-path.reg` (Windows registry file)
- Old installer scripts:
  - `scripts/install-windows.ps1` (replaced by universal `install.sh`)
  - `scripts/install-macos.sh` (replaced by universal `install.sh`)
  - `scripts/install-linux.sh` (replaced by universal `install.sh`)
  - `scripts/install-universal.sh` (consolidated into `install.sh`)
  - `scripts/build-release.ps1` (integrated into `install.sh`)
  - `scripts/build-release.sh` (integrated into `install.sh`)
  - `scripts/package-release.ps1` (not needed with simplified approach)
  - `scripts/create-installer.ps1` (replaced by source build)
  - `scripts/build-installer.ps1` (not applicable)
  - `scripts/cleanup-installers.ps1` (manual cleanup)
  - `scripts/cleanup-installers.sh` (manual cleanup)

### Technical Improvements

#### Code Quality
- Added comprehensive error handling in P2P distribution
- Implemented timeout-based cleanup for stale peer connections
- Added exponential backoff for task retry attempts

#### Performance
- P2P task distribution loads balanced across available peers
- Reduced overhead from multiple discovery mechanisms converging to single mDNS approach
- Minimal resource footprint for peer tracking

#### Security
- P2P distribution validates resource requirements before peer assignment
- Peer cleanup prevents memory leaks from inactive connections
- Installation script uses local cargo builds (no remote binary execution)

---

## Project Status

### Completed Milestones
- ✅ P2P peer discovery and resource announcement
- ✅ Intelligent task distribution with resource matching
- ✅ Cross-platform universal installer
- ✅ Automatic PATH environment setup
- ✅ Comprehensive documentation

### Known Limitations
- P2P network limited to local network (no WAN support yet)
- No authentication between peers (assumes trusted network)
- Manual configuration for custom installation paths
- No GUI management tool (CLI-only)

### Planned Future Features
- **Package Manager Integration**:
  - Homebrew formula for macOS
  - APT/DEB support for Debian/Ubuntu
  - RPM support for CentOS/RHEL
  - Pacman for Arch Linux
  - Chocolatey for Windows

- **Automatic Updates**: `octaskly update` command for self-updating binaries

- **Configuration Management**: Persistent config file support for custom settings

- **Monitoring Dashboard**: Real-time visualization of peer status and task distribution

- **Cross-Compilation**: Pre-built binaries for multiple platforms/architectures

---

## Version History

| Version | Date | Status |
|---------|------|--------|
| 1.0.0 | 2026-02-07 | Release |

---

## Contributing

For changes and improvements, please refer to `IMPLEMENTATION.md` for architectural guidelines and `PROJECT_STRUCTURE.md` for file organization.

---

## Migration Notes

### For Users Upgrading from Pre-Release
- Run: `bash scripts/install.sh install`
- Global `octaskly` command is now available from any directory
- No need to specify full paths to executable

### For Developers
- Use `bash scripts/install.sh build` to compile release binary
- Use `bash scripts/install.sh install` for system-wide installation
- All platform-specific scripts have been consolidated
