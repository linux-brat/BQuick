# install.ps1 - Automated installer for BQuick

$ErrorActionPreference = "Stop"

# Variables
$RepoName = "BQuick"
$GitHubUser = "linux-brat"
$Branch = "main"
$TempDir = "$env:TEMP\BQuick_Install"
$ZipPath = "$TempDir\$RepoName.zip"
$ExtractPath = "$TempDir\$RepoName"

function Check-Rust {
    Write-Output "Checking for Rust installation..."
    $rustc = Get-Command rustc -ErrorAction SilentlyContinue
    $cargo = Get-Command cargo -ErrorAction SilentlyContinue
    if (-not $rustc -or -not $cargo) {
        Write-Error "Rust and Cargo are required but not found. Please install from https://rustup.rs/"
        Exit 1
    } else {
        Write-Output "Rust and Cargo found."
    }
}

function Download-Source {
    Write-Output "Downloading latest $RepoName source from GitHub..."
    if (Test-Path $TempDir) {
        Remove-Item -Recurse -Force $TempDir
    }
    New-Item -ItemType Directory -Force -Path $TempDir | Out-Null

    $url = "https://github.com/$GitHubUser/$RepoName/archive/refs/heads/$Branch.zip"
    Invoke-WebRequest -Uri $url -OutFile $ZipPath
    Write-Output "Download complete."
}

function Extract-Source {
    Write-Output "Extracting source..."
    Add-Type -AssemblyName System.IO.Compression.FileSystem
    [System.IO.Compression.ZipFile]::ExtractToDirectory($ZipPath, $TempDir)
    Write-Output "Extraction complete."
}

function Build-Install {
    Write-Output "Building and installing $RepoName..."
    $SourceDir = Join-Path $TempDir "$RepoName-$Branch"
    Push-Location $SourceDir
    cargo install --path .
    Pop-Location
    Write-Output "$RepoName installed successfully."
}

try {
    Check-Rust
    Download-Source
    Extract-Source
    Build-Install
} catch {
    Write-Error "Installation failed: $_"
    Exit 1
}

# Cleanup
if (Test-Path $TempDir) {
    Remove-Item -Recurse -Force $TempDir
}
