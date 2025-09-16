$ErrorActionPreference = "Stop"

function Write-Status ($msg) {
    Write-Host "$msg"
}

Write-Status "Starting installation..."

Write-Status "Checking Rust installation..."

if (-not (Get-Command rustc -ErrorAction SilentlyContinue)) {
    Write-Status "Rust not found. Installing rustup..."
    $url = "https://win.rustup.rs"
    $installer = "$env:TMP\rustup-init.exe"
    Invoke-WebRequest -Uri $url -OutFile $installer
    & $installer -y
    Remove-Item $installer
    Write-Status "Rust installed. Please restart the terminal and re-run this script."
    exit
} else {
    Write-Status "Rust found."
}

Write-Status "Installing BQuick..."

try {
    cargo install bquick
    Write-Status "BQuick successfully installed."
} catch {
    Write-Status "Failed to install BQuick: $_"
    exit 1
}

Write-Status "Installation complete! Run with: bquick"

Read-Host "Press Enter to exit"
