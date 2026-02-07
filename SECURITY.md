# Octaskly Security Documentation

**Version**: 1.0.0  
**Last Updated**: February 7, 2026  
**Status**: Production Ready

---

## Overview

Octaskly implements **defense-in-depth security** with five independent security layers, each providing its own security guarantees:

1. ✅ **Transport Security** - AES-256-GCM encryption
2. ✅ **Authentication & Authorization** - JWT tokens & RBAC
3. ✅ **Task Isolation** - Four-level sandboxing
4. ✅ **Resource Control** - CPU, memory, disk limits
5. ✅ **Audit Logging** - Complete activity tracking

---

## 🔐 Transport Layer Security

### AES-256-GCM Encryption

**Standard**: NIST-approved authenticated encryption

| Property | Value |
|----------|-------|
| **Algorithm** | AES-256-GCM |
| **Key Size** | 256 bits (32 bytes) |
| **Nonce Size** | 96 bits (12 bytes, random per message) |
| **Authentication Tag** | 128 bits (16 bytes) |
| **Mode** | AEAD (Authenticated Encryption with Associated Data) |

**Characteristics**:
- ✅ Symmetric encryption (same key for encrypt/decrypt)
- ✅ Authenticated (detects tampering automatically)
- ✅ Constant-time operations (resistant to timing attacks)
- ✅ All node-to-node communication encrypted by default

### Message Encryption Flow

```
Sender:
1. Serialize message (JSON/Bincode)
2. Generate random 96-bit nonce
3. Encrypt with AES-256-GCM
4. Transmit: [nonce (12B) + ciphertext + auth-tag (16B)]

Receiver:
1. Extract nonce from message header
2. Decrypt and verify authentication tag
3. Reject if tag invalid (tampering detected)
4. Deserialize message
```

### Key Derivation

**Process**:
```
Password (user input)
    ↓
SHA-256 hash
    ↓
256-bit AES key
```

**Implementation** (Rust):
```rust
pub fn derive_key(password: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let result = hasher.finalize();
    let mut key = [0u8; 32];
    key.copy_from_slice(&result[..32]);
    key
}
```

**Best Practices**:
- ✅ Use strong passwords (16+ characters minimum)
- ✅ Include: uppercase, lowercase, numbers, symbols
- ✅ Never reuse passwords across different systems
- ✅ Rotate keys monthly (recommended)
- ✅ Store passwords in secure password manager

---

## 🔑 Authentication & Authorization

### JWT Token System

**Standard**: RFC 7519 (JSON Web Tokens) with HMAC-SHA256

**Token Structure**:
```
eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.
[header].[payload].[signature]
```

**Header**:
```json
{
  "alg": "HS256",
  "typ": "JWT"
}
```

**Payload (Claims)**:
```json
{
  "sub": "username",
  "role": "dispatcher",
  "exp": 1708867200,
  "iat": 1708780800,
  "permissions": ["read", "write"],
  "node_id": "dispatcher-01"
}
```

**Signature**:
```
HMAC-SHA256(header + payload, secret_key)
```

### Token Lifecycle

| Stage | Description |
|-------|-------------|
| **Generation** | Issued on successful authentication with credentials |
| **Usage** | Sent in HTTP `Authorization: Bearer <token>` header |
| **Verification** | Signature and expiration checked on every API request |
| **Expiration** | Default 24 hours (configurable per role) |
| **Refresh** | Requires re-authentication (future: refresh token support) |

### HMAC-SHA256 for Integrity

Used for worker authentication and message integrity verification:

```
Input data → HMAC-SHA256 with shared key → Token
Verification → Recalculate hash → Compare hashes → ✓ or ✗
```

**Use Cases**:
- Worker peer authentication
- Message integrity verification
- Timestamp validation

---

## 🛡️ Role-Based Access Control (RBAC)

### Four Role Types

#### Admin Role
- **Purpose**: System administration
- **Permissions**: Full system access
- **Capabilities**:
  - Manage all users and roles
  - Configure security parameters
  - Access all resources and logs
  - Shutdown system
- **Permission String**: `["admin", "dispatcher", "worker", "client"]`

#### Dispatcher Role
- **Purpose**: Task orchestration
- **Permissions**: Task and worker management
- **Capabilities**:
  - Submit and monitor tasks
  - View all worker status
  - Access task results
  - Manage task queue
- **Permission String**: `["dispatcher", "client"]`

#### Worker Role
- **Purpose**: Task execution
- **Permissions**: Limited to assigned work
- **Capabilities**:
  - Execute assigned tasks
  - Report task results
  - Register with dispatcher
  - Report resource availability
- **Permission String**: `["worker"]`

#### Client Role
- **Purpose**: Task submission
- **Permissions**: Submit own tasks only
- **Capabilities**:
  - Submit tasks to dispatcher
  - View own task results
  - Retrieve task status
- **Permission String**: `["client"]`

### Permission Matrix

| Resource | Admin | Dispatcher | Worker | Client |
|----------|:-----:|:----------:|:------:|:------:|
| Create Task | ✅ | ✅ | ❌ | ✅ |
| List All Tasks | ✅ | ✅ | ❌ | ❌ |
| List Own Tasks | ✅ | ✅ | ❌ | ✅ |
| Cancel Task | ✅ | ✅ | ❌ | Own only |
| View Worker Stats | ✅ | ✅ | ❌ | ❌ |
| Register Worker | ✅ | ✅ | ✅ | ❌ |
| Manage Users | ✅ | ❌ | ❌ | ❌ |
| System Shutdown | ✅ | ❌ | ❌ | ❌ |

### Permission Checking

```rust
// Check permission before allowing operation
let permitted = auth.has_permission(&claims, "task:create")?;
if !permitted {
    return Err(AuthError::InsufficientPermissions);
}
```

---

## 🔒 Task Sandboxing & Isolation

### Four Isolation Levels

#### Level 1: None
- **Security**: No isolation
- **System Access**: Full
- **Use Case**: Trusted, internal tasks only
- **Risk**: High - task can access system resources

```bash
octaskly worker --sandbox-level none
```

#### Level 2: Basic
- **Security**: Basic environment isolation
- **Features**:
  - Clear sensitive environment variables
  - Restricted PATH (no system dirs)
  - Standard `/tmp` access allowed
  - Limited file system access
- **Use Case**: Semi-trusted tasks
- **Risk**: Medium

```bash
octaskly worker --sandbox-level basic
```

#### Level 3: Strict
- **Security**: Strong isolation
- **Features**:
  - Only `/tmp` writable
  - Minimal environment variables
  - No direct shell access
  - No system utilities available
  - Read-only home directory
- **Use Case**: Untrusted code execution
- **Risk**: Low

```bash
octaskly worker --sandbox-level strict
```

#### Level 4: VeryStrict
- **Security**: Maximum isolation
- **Features**:
  - Empty environment
  - No write permissions (except stdout/stderr)
  - Read-only execution
  - Severely restricted system calls
  - No network access
- **Use Case**: Hostile/malicious code
- **Risk**: Minimal (execution fails gracefully)

```bash
octaskly worker --sandbox-level very-strict
```

### Implementation Details

**Process Isolation**:
- Separate process group per task
- Resource limits via `rlimit`
- File descriptor limits enforced
- Process count limited (prevent fork bombs)

**Environment Control**:
- Clear all inherited environment
- Whitelist specific variables
- Secure defaults applied
- No secrets/tokens exposed

**File System**:
- Allowlist specific accessible paths
- Deny access to system files (`/etc`, `/sys`, `/proc`)
- Prevent symlink traversal attacks
- Restrict temporary directory access

---

## 📊 Resource Limits

### Per-Task Limit Types

#### CPU Time Limit
- **Default**: 3600 seconds (1 hour)
- **Minimum**: 1 second
- **Maximum**: 86400 seconds (24 hours)
- **Enforcement**: Timeout mechanism with SIGALRM
- **Configurable**: Yes, per task

```bash
# Task runs maximum 5 minutes
octaskly submit-task --timeout 300 "./long-job.sh"
```

#### Memory Limit
- **Default**: 2048 MB (2 GB)
- **Minimum**: 64 MB
- **Maximum**: System available
- **Enforcement**: `rlimit` RLIMIT_AS
- **Behavior**: Task killed if exceeded

```bash
# Task limited to 512 MB RAM
octaskly submit-task --memory 512 "./memory-intensive.sh"
```

#### Disk Usage Limit
- **Default**: 10000 MB (10 GB)
- **Minimum**: 100 MB
- **Maximum**: Filesystem available
- **Enforcement**: `rlimit` RLIMIT_FSIZE
- **Behavior**: Task killed if exceeded

```bash
# Task can write max 1 GB to disk
octaskly submit-task --disk 1000 "./write-heavy.sh"
```

#### Open File Descriptors
- **Default**: 1024 files
- **Minimum**: 64
- **Maximum**: System max FDs
- **Enforcement**: `rlimit` RLIMIT_NOFILE

#### Child Processes
- **Default**: 256 processes
- **Minimum**: 4
- **Maximum**: System limit
- **Enforcement**: `rlimit` RLIMIT_NPROC
- **Purpose**: Prevent fork bomb attacks

### Preset Configurations

#### Default Limits (General Tasks)
```
CPU Time:     3600 seconds (1 hour)
Memory:       2048 MB
Disk Usage:   10000 MB
Open Files:   1024
Processes:    256
```

#### Strict Limits (Untrusted Code)
```
CPU Time:     300 seconds (5 minutes)
Memory:       512 MB
Disk Usage:   1000 MB
Open Files:   100
Processes:    10
```

#### Relaxed Limits (Long Builds)
```
CPU Time:     86400 seconds (24 hours)
Memory:       8192 MB
Disk Usage:   102400 MB
Open Files:   4096
Processes:    1024
```

### Setting Resource Limits

**Per Dispatcher Configuration**:
```bash
octaskly dispatcher \
  --default-timeout 600 \
  --default-memory 4096 \
  --default-disk 20000
```

**Per Task**:
```bash
# Using task submission config
curl -X POST http://localhost:3000/api/v1/tasks \
  -H "Content-Type: application/json" \
  -d '{
    "command": "./build.sh",
    "timeout": 1800,
    "memory_limit_mb": 4096,
    "disk_limit_mb": 20000,
    "sandbox_level": "strict"
  }'
```

---

## 📝 Audit Logging

### What Gets Logged

**Security Events**:
- ✅ Authentication attempts (success/failure)
- ✅ Authorization denials
- ✅ Token generation and expiration
- ✅ User role changes
- ✅ Permission grants/revokes

**Task Events**:
- ✅ Task submission
- ✅ Task execution start/completion
- ✅ Task failures and timeouts
- ✅ Resource limit violations
- ✅ Sandbox policy enforcements

**System Events**:
- ✅ Worker registration/unregistration
- ✅ Node connection/disconnection
- ✅ System shutdown/startup
- ✅ Configuration changes

### Log Location

```
Linux/macOS:   ~/.octaskly/logs/audit.log
Windows:       %APPDATA%\octaskly\logs\audit.log
Development:   ./logs/audit.log
```

### Log Format

```json
{
  "timestamp": "2026-02-07T14:30:45.123Z",
  "event_type": "task_execution",
  "severity": "info",
  "user": "dispatcher-01",
  "action": "task_started",
  "task_id": "task-abc123",
  "details": {
    "worker": "worker-01",
    "command": "build.sh",
    "timeout": 3600
  },
  "source_ip": "192.168.1.100"
}
```

### Accessing Logs

**View recent logs**:
```bash
tail -f ~/.octaskly/logs/audit.log
```

**Filter by severity**:
```bash
grep "ERROR" ~/.octaskly/logs/audit.log
```

**Filter by event type**:
```bash
grep "task_execution" ~/.octaskly/logs/audit.log
```

**Export to JSON for analysis**:
```bash
curl http://localhost:3000/api/v1/audit-logs \
  -H "Authorization: Bearer $TOKEN" \
  > audit-export.json
```

---

## 🔒 Best Practices & Deployment

### Production Configuration

**Security Checklist**:

- ✅ Change default secret key
  ```bash
  octaskly dispatcher --secret "$(openssl rand -hex 32)"
  ```

- ✅ Enable HTTPS for API
  ```bash
  octaskly dispatcher --use-https true \
    --cert-path /etc/octaskly/server.crt \
    --key-path /etc/octaskly/server.key
  ```

- ✅ Set strong authentication
  ```bash
  octaskly dispatcher --require-auth true
  ```

- ✅ Configure firewall
  ```bash
  # Allow only trusted networks
  ufw allow from 192.168.1.0/24 to any port 7878
  ```

- ✅ Enable audit logging
  ```bash
  octaskly dispatcher --audit-log true --log-level debug
  ```

- ✅ Use restrictive sandbox levels
  ```bash
  octaskly worker --sandbox-level strict
  ```

- ✅ Set appropriate resource limits
  ```bash
  octaskly dispatcher --default-timeout 300
  ```

### Credential Management

**Never commit credentials**:
```bash
# ❌ BAD
export OCTASKLY_SECRET="my-secret-key-123"

# ✅ GOOD
export OCTASKLY_SECRET=$(cat /etc/octaskly/secret.key)
```

**Use environment variables or secret managers**:
```bash
# AWS Secrets Manager
aws secretsmanager get-secret-value --secret-id octaskly-key

# HashiCorp Vault
vault kv get secret/octaskly
```

### Installer & Release Integrity

**Verify downloads**:
```bash
# Download SHA256 checksums from GitHub releases
shasum -a 256 -c SHA256SUMS.txt

# Verify file matches
# octaskly-setup-1.0.0.exe: OK
```

**GitHub Release Process**:
1. All installer artifacts published on GitHub Releases
2. Distributed over HTTPS (encrypted transport)
3. SHA256 checksums provided in release notes
4. Releases signed (future: GPG signatures)

**For high-security environments**:
- Validate release tags on GitHub (git verify-tag)
- Verify commits before deployment
- Test installers in isolated environment first
- Compare checksums on multiple networks

---

## 🚨 Security Incident Response

### Suspected Compromise

**Immediate Actions**:
1. Stop all Octaskly services
   ```bash
   octaskly shutdown --force
   ```

2. Revoke all tokens
   ```bash
   octaskly revoke-tokens --all
   ```

3. Check audit logs for unauthorized access
   ```bash
   grep "ERROR\|WARN" ~/.octaskly/logs/audit.log
   ```

4. Change secret key
   ```bash
   octaskly update-secret --new-secret "$(openssl rand -hex 32)"
   ```

5. Rotate all worker credentials

### Vulnerability Reporting

**Found a security issue?**
- **DO NOT** open public GitHub issue
- Email: security@octaskly.io
- Include: Description, reproduction steps, impact assessment
- GPG key available for sensitive communication

---

## 📚 Additional Resources

| Document | Topic |
|----------|-------|
| [DEPLOYMENT.md](DEPLOYMENT.md) | Production deployment guide |
| [IMPLEMENTATION.md](IMPLEMENTATION.md) | Technical architecture |
| [API_REFERENCE.md](API_REFERENCE.md) | REST API security details |
| [HELP.md](HELP.md) | Troubleshooting & security FAQs |

---

**Version**: 1.0.0 | **Last Reviewed**: February 7, 2026 | **Next Review**: August 7, 2026
