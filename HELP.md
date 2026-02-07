# Help Documentation

Complete guide to Octaskly commands, features, and troubleshooting.

---

## Table of Contents

- [Getting Started](#getting-started)
- [Command Reference](#command-reference)
- [Dispatcher Guide](#dispatcher-guide)
- [Worker Guide](#worker-guide)
- [P2P Networking](#p2p-networking)
- [Troubleshooting](#troubleshooting)
- [Advanced Configuration](#advanced-configuration)
- [FAQ](#faq)

---

## Getting Started

### Installation

Install Octaskly globally on your system:

```bash
bash scripts/install.sh install
```

Verify installation:

```bash
octaskly --version
```

### Basic Usage

Start a dispatcher (task orchestrator):

```bash
octaskly dispatcher --port 7878
```

Start a worker (task executor):

```bash
octaskly worker --name worker-01
```

---

## Command Reference

### Global Options

Available on all commands:

```bash
octaskly [COMMAND] [OPTIONS]

Flags:
  -h, --help              Show help message and exit
  -V, --version           Print version information
  -v, --verbose           Enable verbose/debug logging output
  -q, --quiet             Suppress output (error level only)
```

---

## Dispatcher Guide

### Starting a Dispatcher

The dispatcher is the central task orchestrator. It receives tasks and distributes them to workers.

### Command

```bash
octaskly dispatcher [OPTIONS]
```

### Options

Complete reference for dispatcher options:

| Short | Long | Default | Description |
|-------|------|---------|-------------|
| `-p` | `--port` | 7878 | Port for dispatcher to listen on |
| `-b` | `--bind` | 0.0.0.0 | Network interface to bind to |
| `-w` | `--max-workers` | 10 | Maximum concurrent workers to manage |
| `-t` | `--task-timeout` | 300 | Task timeout in seconds |
| | `--p2p-enabled` | true | Enable P2P peer distribution |
| | `--discovery-port` | 5555 | UDP port for peer discovery |
| `-v` | `--verbose` | false | Enable debug logging |

### Examples

Basic dispatcher:

```bash
octaskly dispatcher
```

Custom port:

```bash
octaskly dispatcher --port 8080
```

Bind to specific interface (networked setup):

```bash
octaskly dispatcher --bind 192.168.1.10 --port 7878
```

Limit workers and set timeout:

```bash
octaskly dispatcher --max-workers 5 --task-timeout 600
```

Disable P2P, use centralized only:

```bash
octaskly dispatcher --p2p-enabled false
```

With debug logging:

```bash
octaskly dispatcher --verbose
```

### Dispatcher Responsibilities

The dispatcher:

1. **Accept connections** from workers
2. **Discover P2P peers** on local network
3. **Queue tasks** for distribution
4. **Select workers** based on availability and resource requirements
5. **Monitor execution** and handle timeouts
6. **Aggregate results** and report back
7. **Retry failed tasks** with exponential backoff

### Configuration

Dispatcher state can be configured in code or via command-line options. Default configuration:

- Max workers: 10
- Task timeout: 300 seconds (5 minutes)
- P2P enabled: true
- Discovery interval: 30 seconds
- Worker heartbeat timeout: 60 seconds

---

## Worker Guide

### Starting a Worker

Workers execute tasks assigned by the dispatcher.

### Command

```bash
octaskly worker [OPTIONS]
```

### Options

Complete reference for worker options:

| Short | Long | Default | Description |
|-------|------|---------|-------------|
| `-n` | `--name` | worker-N | Unique worker identifier |
| `-d` | `--dispatcher` | localhost | Dispatcher address |
| `-p` | `--port` | 7878 | Dispatcher port |
| `-j` | `--max-jobs` | 4 | Maximum concurrent jobs |
| `-c` | `--cpu-cores` | auto | CPU cores available (auto-detect) |
| `-m` | `--memory` | auto | RAM in MB (auto-detect) |
| | `--gpu` | false | GPU available (true/false) |
| `-v` | `--verbose` | false | Enable debug logging |

### Examples

Basic worker:

```bash
octaskly worker --name worker-01
```

Worker with name and max jobs:

```bash
octaskly worker --name worker-prod-01 --max-jobs 8
```

Connect to remote dispatcher:

```bash
octaskly worker --name worker-remote --dispatcher 192.168.1.100 --port 7878
```

Declare hardware capabilities:

```bash
octaskly worker --name gpu-worker-01 \
  --cpu-cores 16 \
  --memory 32768 \
  --gpu true
```

Multiple workers on same machine:

```bash
# Terminal 1
octaskly worker --name worker-a --max-jobs 4

# Terminal 2
octaskly worker --name worker-b --max-jobs 4

# Terminal 3
octaskly worker --name worker-c --max-jobs 4
```

### Worker Naming Best Practices

Use descriptive, unique names:

```bash
Good names:
octaskly worker --name prod-worker-01
octaskly worker --name gpu-compute-01
octaskly worker --name api-background-01

Avoid:
octaskly worker --name worker        # Too generic
octaskly worker --name WORKER-01    # Case sensitivity issues
octaskly worker --name "worker 01"  # Spaces in names
```

### Hardware Declaration

Declare your hardware capabilities so dispatcher can match tasks appropriately:

High-performance worker:

```bash
octaskly worker --name compute-01 \
  --cpu-cores 16 \
  --memory 65536 \
  --gpu true \
  --max-jobs 8
```

Small worker (laptop, VM):

```bash
octaskly worker --name laptop-worker \
  --cpu-cores 4 \
  --memory 8192 \
  --gpu false \
  --max-jobs 2
```

### Job Concurrency

The `--max-jobs` parameter controls how many tasks run simultaneously:

Conservative (low resource usage):

```bash
octaskly worker --max-jobs 1
```

Moderate:

```bash
octaskly worker --max-jobs 4
```

Aggressive (high resource usage):

```bash
octaskly worker --max-jobs 16
```

Set based on your CPU cores and available RAM.

---

## P2P Networking

### How P2P Works

1. **Discovery** - Workers announce themselves on local network via UDP
2. **Resource Advertisement** - Each peer broadcasts available CPU, RAM, GPU
3. **Task Distribution** - Dispatcher can send tasks directly to peers
4. **Fallback** - If peer fails, task retries through dispatcher

### Enabling P2P

P2P is enabled by default:

```bash
# Dispatcher with P2P (default)
octaskly dispatcher

# Explicitly enable
octaskly dispatcher --p2p-enabled true

# Disable (centralized only)
octaskly dispatcher --p2p-enabled false
```

### Network Requirements

P2P discovery works on same local network (same subnet):

Works:
- All machines on 192.168.1.0/24
- All on same Wi-Fi network
- Machines on same ethernet subnet

Doesn't work:
- Across VPN (use explicit dispatcher IP)
- Different subnets without routing
- If firewall blocks UDP 5555

### Firewall Configuration

Ensure UDP port 5555 is open between machines:

Linux (ufw):

```bash
sudo ufw allow 5555/udp
```

macOS (pf):

```bash
sudo pfctl -s nat
# May require firewall settings GUI
```

Windows Firewall:

```powershell
New-NetFirewallRule -DisplayName "Octaskly P2P" `
  -Direction Inbound -Action Allow `
  -Protocol UDP -LocalPort 5555
```

### Monitoring P2P Status

View connected peers in dispatcher logs (verbose mode):

```bash
octaskly dispatcher --verbose

# Look for messages like:
# [P2P] Discovered peer: worker-02 (CPU: 8, RAM: 16384 MB)
# [P2P] Peer resource announcement received
```

---

## Troubleshooting

### Installation Issues

#### Command Not Found: `octaskly`

Windows:
```powershell
# Restart PowerShell/terminal
exit
```

macOS/Linux:
```bash
# Reload shell configuration
source ~/.bashrc  # or ~/.zshrc
```

#### Permission Denied

```bash
# Make script executable
chmod +x scripts/install.sh

# Try installing with sudo
sudo bash scripts/install.sh install
```

#### Cargo Not Found

Install Rust from https://rustup.rs:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Connection Issues

#### Worker Can't Connect to Dispatcher

Check dispatcher is running and accessible:

```bash
# Terminal 1 - start dispatcher
octaskly dispatcher --verbose

# Terminal 2 - verify connectivity
ping dispatcher-ip-address
```

Verify port is correct:

```bash
octaskly worker --dispatcher 192.168.1.100 --port 7878
```

#### Dispatcher Not Finding Workers

Ensure dispatcher listens on accessible interface:

```bash
# Good - accept connections from any interface
octaskly dispatcher --bind 0.0.0.0

# Bad - only local connections
octaskly dispatcher --bind 127.0.0.1
```

### P2P Discovery Issues

#### P2P Peers Not Discovered

Check 1: All machines on same subnet

```bash
# List network interfaces and their IPs
ipconfig    # Windows
ifconfig    # macOS/Linux
```

Check 2: UDP port is open

```bash
# Linux - check firewall
sudo ufw status

# If blocked:
sudo ufw allow 5555/udp
```

Check 3: Enable verbose logging

```bash
octaskly dispatcher --verbose

# Look for P2P discovery messages
```

#### Firewall Blocking P2P

Add firewall rule for UDP 5555:

Linux (iptables):

```bash
sudo iptables -A INPUT -p udp --dport 5555 -j ACCEPT
```

Windows PowerShell (as Administrator):

```powershell
New-NetFirewallRule -DisplayName "Octaskly" `
  -Direction Inbound -Action Allow `
  -Protocol UDP -LocalPort 5555
```

### Task Execution Issues

#### Tasks Timing Out

Increase timeout on dispatcher:

```bash
octaskly dispatcher --task-timeout 600  # 10 minutes
```

Check worker has sufficient resources:

```bash
# Verify CPU/memory not maxed out
# Reduce max-jobs if needed
octaskly worker --max-jobs 2
```

Check task logs:

```bash
octaskly dispatcher --verbose
# Look for timeout messages
```

#### Worker Rejecting Tasks

| Issue | Solution |
|-------|----------|
| All job slots full | Start more workers or increase `--max-jobs` |
| Insufficient CPU/RAM | Declare correct hardware with `--cpu-cores`, `--memory` |
| Task requirements mismatch | Verify task requirements vs worker capabilities |
| Hardware unavailable | Start worker with `--gpu true` if needed |

---

## Advanced Configuration

### Environment Variables

Configure via environment variables (overrides CLI args):

Dispatcher:

```bash
export OCTASKLY_PORT=7878
export OCTASKLY_BIND=0.0.0.0
export OCTASKLY_MAX_WORKERS=10
export OCTASKLY_TASK_TIMEOUT=300
export OCTASKLY_P2P_ENABLED=true

octaskly dispatcher
```

Worker:

```bash
export OCTASKLY_WORKER_NAME=my-worker
export OCTASKLY_DISPATCHER=192.168.1.10
export OCTASKLY_DISPATCHER_PORT=7878
export OCTASKLY_MAX_JOBS=4
export OCTASKLY_CPU_CORES=8
export OCTASKLY_MEMORY=16384
export OCTASKLY_GPU=false

octaskly worker
```

### Multi-Machine Setup

Example: 1 dispatcher, 3 workers on network

Machine A (Dispatcher) - IP: 192.168.1.10:

```bash
octaskly dispatcher --bind 192.168.1.10 --port 7878
```

Machine B (Worker) - IP: 192.168.1.11:

```bash
octaskly worker --name worker-b --dispatcher 192.168.1.10
```

Machine C (Worker) - IP: 192.168.1.12:

```bash
octaskly worker --name worker-c --dispatcher 192.168.1.10
```

Machine D (Worker) - IP: 192.168.1.13:

```bash
octaskly worker --name worker-d --dispatcher 192.168.1.10
```

Verify all connected (verbose mode on dispatcher):

```bash
octaskly dispatcher --verbose
```

---

## FAQ

### Q: Can I run dispatcher and worker on same machine?

A: Yes. Start dispatcher first, then worker(s):

```bash
# Terminal 1
octaskly dispatcher

# Terminal 2
octaskly worker --name local-worker-01

# Terminal 3 (optional)
octaskly worker --name local-worker-02
```

### Q: How many workers can one dispatcher manage?

A: Default limit is 10 (configurable with `--max-workers`). Can scale to hundreds with proper networking.

### Q: What happens if dispatcher goes down?

A: Workers disconnect and wait for dispatcher to restart. In-flight tasks may be lost (no persistent queue yet).

### Q: Can I use Octaskly across the internet?

A: Currently limited to local network (LAN). For WAN, explicitly specify dispatcher IP on each worker.

### Q: How do I monitor task progress?

A: Use `--verbose` flag on dispatcher:

```bash
octaskly dispatcher --verbose
```

Watch for task assignment and completion messages in output.

### Q: Can workers have different capabilities?

A: Yes. Declare different CPU/memory/GPU per worker:

```bash
# High-spec
octaskly worker --name compute-server \
  --cpu-cores 32 --memory 131072 --gpu true

# Low-spec
octaskly worker --name laptop \
  --cpu-cores 4 --memory 8192 --gpu false
```

Dispatcher matches task requirements to appropriate workers.

### Q: How do I uninstall?

A: Remove the binary from system PATH location:

Windows:

```powershell
Remove-Item "C:\Program Files\octaskly" -Recurse
```

macOS/Linux:

```bash
sudo rm /usr/local/bin/octaskly
```

### Q: Where are logs stored?

A: Currently output to console. For persistent logs, redirect:

```bash
octaskly dispatcher > dispatcher.log 2>&1 &
octaskly worker > worker.log 2>&1 &
```

### Q: Can I run multiple dispatchers?

A: Not recommended. Each dispatcher manages independent pool of workers.

### Q: What's the performance impact of P2P?

A: Minimal. P2P runs in background on separate thread, doesn't impact task throughput.

---

## See Also

- [Installation Guide](INSTALLATION.md) - Setup help
- [Deployment Guide](DEPLOYMENT.md) - Production setup
- [Implementation Details](IMPLEMENTATION.md) - Technical details
- [Security Guide](SECURITY.md) - Security guidelines
- [README](README.md) - Project overview

---

**Octaskly Help Documentation**  
Version 1.0.0
