# Octaskly Installer for Windows PowerShell
# Requirements: PowerShell 5.1+ running as Administrator
# Usage: powershell -ExecutionPolicy Bypass -File install.ps1

param(
    [string]$Command = "install",
    [switch]$Force = $false
)

$ErrorActionPreference = "Stop"

# Color definitions
class Colors {
    static [string] $Reset = "`e[0m"
    static [string] $Red = "`e[31m"
    static [string] $Green = "`e[32m"
    static [string] $Yellow = "`e[33m"
    static [string] $Blue = "`e[34m"
    static [string] $Bold = "`e[1m"
}

# Helper functions
function Write-Error-Msg {
    param([string]$Message)
    Write-Host "$([Colors]::Red)✗ $Message$([Colors]::Reset)"
}

function Write-Success-Msg {
    param([string]$Message)
    Write-Host "$([Colors]::Green)✓ $Message$([Colors]::Reset)"
}

function Write-Info-Msg {
    param([string]$Message)
    Write-Host "$([Colors]::Blue)$Message$([Colors]::Reset)"
}

function Write-Warning-Msg {
    param([string]$Message)
    Write-Host "$([Colors]::Yellow)⚠ $Message$([Colors]::Reset)"
}

function Check-Administrator {
    $currentUser = [Security.Principal.WindowsIdentity]::GetCurrent()
    $principal = New-Object Security.Principal.WindowsPrincipal($currentUser)
    
    if (-not $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
        Write-Error-Msg "This script must be run as Administrator"
        Write-Host ""
        Write-Info-Msg "Right-click PowerShell and select 'Run as administrator', then run:"
        Write-Host ""
        Write-Host "  powershell -ExecutionPolicy Bypass -File install.ps1"
        Write-Host ""
        exit 1
    }
}

function Check-Rust {
    if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
        Write-Error-Msg "Cargo not found. Install Rust first:"
        Write-Host ""
        Write-Info-Msg "Download from: https://rustup.rs"
        Write-Host ""
        Write-Info-Msg "Or: winget install Rustlang.Rust.MSVC"
        exit 1
    }
    Write-Success-Msg "Cargo found at: $(rustc --version)"
}

function Build-Release {
    Write-Info-Msg "Building Octaskly release binary..."
    Write-Host ""
    
    if (-not (Test-Path "Cargo.toml")) {
        Write-Error-Msg "Cargo.toml not found. Are you in the project root?"
        exit 1
    }
    
    Write-Host "Running: cargo build --release"
    try {
        & cargo build --release 2>&1
        if ($LASTEXITCODE -ne 0) {
            Write-Error-Msg "Build failed with exit code $LASTEXITCODE"
            exit 1
        }
    } catch {
        Write-Error-Msg "Build error: $_"
        exit 1
    }
    
    $binary = ".\target\release\octaskly.exe"
    
    if (-not (Test-Path $binary)) {
        Write-Error-Msg "Binary not found after build: $binary"
        exit 1
    }
    
    Write-Success-Msg "Build successful: $binary"
    return $binary
}

function Install-Windows {
    param([string]$Binary)
    
    $installPath = "C:\Program Files\octaskly"
    
    Write-Host ""
    Write-Info-Msg "┌─ Installing for Windows"
    Write-Host ""
    
    # Create installation directory
    if (-not (Test-Path $installPath)) {
        Write-Host "Creating directory: $installPath"
        New-Item -ItemType Directory -Path $installPath -Force | Out-Null
        Write-Success-Msg "Directory created"
    }
    
    # Copy binary
    $targetBinary = Join-Path $installPath "octaskly.exe"
    Write-Host "Copying binary to: $targetBinary"
    Copy-Item -Path $Binary -Destination $targetBinary -Force
    Write-Success-Msg "Binary installed"
    
    # Add to PATH
    Write-Host "Updating PATH environment variable..."
    
    $currentPath = [Environment]::GetEnvironmentVariable("Path", "Machine")
    
    if ($currentPath -notlike "*$installPath*") {
        $newPath = "$installPath;$currentPath"
        [Environment]::SetEnvironmentVariable("Path", $newPath, "Machine")
        Write-Success-Msg "Added $installPath to system PATH"
        Write-Warning-Msg "Restart PowerShell or CMD to apply changes"
    } else {
        Write-Success-Msg "Already in PATH: $installPath"
    }
    
    Write-Host ""
    Write-Info-Msg "└─"
}

function Show-Usage {
    Write-Host ""
    Write-Info-Msg "Octaskly Installer for Windows PowerShell"
    Write-Host ""
    Write-Host "Usage: powershell -ExecutionPolicy Bypass -File install.ps1 [command]"
    Write-Host ""
    Write-Host "Commands:"
    Write-Host "  install  Install binary to C:\Program Files\octaskly (default)"
    Write-Host "  build    Build release binary only"
    Write-Host "  help     Show this message"
    Write-Host ""
    Write-Host "Flags:"
    Write-Host "  -Force   Force overwrite if already installed"
    Write-Host ""
    Write-Host "Examples:"
    Write-Host "  powershell -ExecutionPolicy Bypass -File install.ps1 build"
    Write-Host "  powershell -ExecutionPolicy Bypass -File install.ps1 install"
    Write-Host ""
    Write-Host "Requirements:"
    Write-Host "  • Windows 10/11"
    Write-Host "  • PowerShell 5.1 or later"
    Write-Host "  • Administrator privileges"
    Write-Host "  • Rust/Cargo installed (for building)"
    Write-Host ""
}

function Main {
    Write-Host ""
    Write-Info-Msg "╔════════════════════════════════════════╗"
    Write-Info-Msg "║    OCTASKLY - Windows Installer        ║"
    Write-Info-Msg "║         Distributed Task System        ║"
    Write-Info-Msg "╚════════════════════════════════════════╝"
    Write-Host ""
    
    try {
        switch ($Command.ToLower()) {
            "build" {
                Write-Info-Msg "Build mode selected"
                Write-Host ""
                Check-Rust
                Build-Release
                Write-Host ""
            }
            
            "install" {
                Write-Info-Msg "Install mode selected"
                Write-Host ""
                
                Check-Administrator
                Check-Rust
                
                $binary = $null
                
                if (Test-Path ".\target\release\octaskly.exe") {
                    $binary = ".\target\release\octaskly.exe"
                    Write-Success-Msg "Found pre-built binary"
                } else {
                    Write-Warning-Msg "Binary not found, building..."
                    Write-Host ""
                    $binary = Build-Release
                }
                
                Write-Host ""
                Install-Windows -Binary $binary
                Write-Host ""
                
                Write-Host ""
                Write-Info-Msg "╔════════════════════════════════════════╗"
                Write-Success-Msg "║    Installation Complete! ✓            ║"
                Write-Success-Msg "║    Run: octaskly --help                ║"
                Write-Info-Msg "╚════════════════════════════════════════╝"
                Write-Host ""
                
                Write-Host "Testing installation..."
                Start-Sleep -Milliseconds 500
                
                if (Get-Command octaskly -ErrorAction SilentlyContinue) {
                    Write-Success-Msg "octaskly is globally accessible!"
                    Write-Host ""
                    & octaskly --version
                } else {
                    Write-Warning-Msg "Please restart PowerShell for changes to take effect"
                    Write-Host ""
                    Write-Host "Then run: octaskly --version"
                }
                
                Write-Host ""
            }
            
            "help" {
                Show-Usage
            }
            
            default {
                Write-Error-Msg "Unknown command: $Command"
                Write-Host ""
                Show-Usage
                exit 1
            }
        }
    } catch {
        Write-Error-Msg "Error: $_"
        Write-Host ""
        exit 1
    }
}

# Run main
Main
