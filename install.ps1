# install.ps1 - BQuick Installation Script with Minimal UI

$ErrorActionPreference = "Stop"

# Configuration
$AppName = "BQuick"
$CrateName = "bquick"

# Colors for UI
$Green = "Green"
$Yellow = "Yellow"
$Red = "Red"
$Cyan = "Cyan"

function Write-Header {
    Clear-Host
    Write-Host "╔══════════════════════════════════════════════╗" -ForegroundColor Cyan
    Write-Host "║              BQuick Installer                ║" -ForegroundColor Cyan  
    Write-Host "║          Professional Software Manager       ║" -ForegroundColor Cyan
    Write-Host "╚══════════════════════════════════════════════╝" -ForegroundColor Cyan
    Write-Host ""
}

function Write-Step($StepNumber, $Message, $Status = "RUNNING") {
    $StatusColor = switch($Status) {
        "RUNNING" { $Yellow }
        "SUCCESS" { $Green }
        "ERROR" { $Red }
        default { $Yellow }
    }
    
    $StatusSymbol = switch($Status) {
        "RUNNING" { "⚡" }
        "SUCCESS" { "✅" }
        "ERROR" { "❌" }
        default { "⚡" }
    }
    
    Write-Host "[$StepNumber/4] $StatusSymbol $Message" -ForegroundColor $StatusColor
}

function Install-Rust {
    Write-Step 1 "Checking Rust and Cargo installation..." "RUNNING"
    
    $rustc = Get-Command rustc -ErrorAction SilentlyContinue
    $cargo = Get-Command cargo -ErrorAction SilentlyContinue
    
    if (-not $rustc -or -not $cargo) {
        Write-Host "   Rust not found. Installing automatically..." -ForegroundColor Yellow
        
        try {
            # Download rustup installer
            $rustupUrl = "https://win.rustup.rs/x86_64"
            $rustupPath = "$env:TEMP\rustup-init.exe"
            
            Write-Host "   Downloading Rust installer..." -ForegroundColor Yellow
            Invoke-WebRequest -Uri $rustupUrl -OutFile $rustupPath
            
            Write-Host "   Installing Rust (this may take a few minutes)..." -ForegroundColor Yellow
            Start-Process -FilePath $rustupPath -ArgumentList "-y" -Wait -NoNewWindow
            
            # Clean up installer
            Remove-Item $rustupPath -Force
            
            # Refresh PATH for current session
            $env:PATH = [System.Environment]::GetEnvironmentVariable("PATH", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("PATH", "User")
            
            Write-Step 1 "Rust and Cargo installed successfully!" "SUCCESS"
            Write-Host "   Note: You may need to restart your terminal after installation completes." -ForegroundColor Yellow
            
        } catch {
            Write-Step 1 "Failed to install Rust: $_" "ERROR"
            throw "Rust installation failed"
        }
    } else {
        Write-Step 1 "Rust and Cargo are already installed!" "SUCCESS"
        $rustVersion = & rustc --version
        $cargoVersion = & cargo --version
        Write-Host "   $rustVersion" -ForegroundColor Gray
        Write-Host "   $cargoVersion" -ForegroundColor Gray
    }
}

function Install-BQuick {
    Write-Step 2 "Installing BQuick from crates.io..." "RUNNING"
    
    try {
        # First, try to install from crates.io
        Write-Host "   Downloading and compiling BQuick..." -ForegroundColor Yellow
        cargo install $CrateName
        
        Write-Step 2 "BQuick installed successfully from crates.io!" "SUCCESS"
        
    } catch {
        Write-Host "   Failed to install from crates.io, trying GitHub..." -ForegroundColor Yellow
        
        try {
            # Fallback: install from GitHub
            cargo install --git "https://github.com/linux-brat/BQuick.git"
            Write-Step 2 "BQuick installed successfully from GitHub!" "SUCCESS"
            
        } catch {
            Write-Step 2 "Failed to install BQuick: $_" "ERROR"
            throw "BQuick installation failed"
        }
    }
}

function Verify-Installation {
    Write-Step 3 "Verifying installation..." "RUNNING"
    
    try {
        # Check if bquick is available
        $bquickPath = Get-Command bquick -ErrorAction SilentlyContinue
        
        if ($bquickPath) {
            Write-Step 3 "Installation verified successfully!" "SUCCESS"
            Write-Host "   BQuick is installed at: $($bquickPath.Source)" -ForegroundColor Gray
        } else {
            Write-Step 3 "BQuick command not found in PATH" "ERROR"
            Write-Host "   You may need to restart your terminal or add Cargo's bin directory to your PATH." -ForegroundColor Yellow
            Write-Host "   Default location: $env:USERPROFILE\.cargo\bin" -ForegroundColor Gray
        }
        
    } catch {
        Write-Step 3 "Verification failed: $_" "ERROR"
    }
}

function Show-Completion {
    Write-Step 4 "Installation completed!" "SUCCESS"
    Write-Host ""
    Write-Host "╔══════════════════════════════════════════════╗" -ForegroundColor Green
    Write-Host "║            Installation Complete!            ║" -ForegroundColor Green
    Write-Host "╚══════════════════════════════════════════════╝" -ForegroundColor Green
    Write-Host ""
    Write-Host "🚀 To run BQuick, simply type:" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "   bquick" -ForegroundColor Yellow -BackgroundColor Black
    Write-Host ""
    Write-Host "If the command is not found, restart your terminal and try again." -ForegroundColor Gray
    Write-Host ""
    Write-Host "Enjoy using BQuick - Your Professional Software Manager!" -ForegroundColor Green
}

# Main installation process
try {
    Write-Header
    
    Install-Rust
    Start-Sleep -Seconds 1
    
    Install-BQuick  
    Start-Sleep -Seconds 1
    
    Verify-Installation
    Start-Sleep -Seconds 1
    
    Show-Completion
    
} catch {
    Write-Host ""
    Write-Host "❌ Installation failed: $_" -ForegroundColor Red
    Write-Host ""
    Write-Host "Please try the following:" -ForegroundColor Yellow
    Write-Host "1. Run PowerShell as Administrator" -ForegroundColor Gray
    Write-Host "2. Ensure you have internet connectivity" -ForegroundColor Gray
    Write-Host "3. Check Windows firewall/antivirus settings" -ForegroundColor Gray
    Write-Host "4. Try installing Rust manually from https://rustup.rs/" -ForegroundColor Gray
    Write-Host ""
    Exit 1
}

# Pause to let user read the completion message
Write-Host "Press any key to exit..." -ForegroundColor Gray
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
