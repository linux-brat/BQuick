@echo off
cls

echo ================================================
echo              BQuick Installer
echo          Professional Software Manager
echo ================================================
echo.

REM Check if cargo is installed
where cargo >nul 2>&1
if errorlevel 1 (
    echo [1/3] Rust/Cargo not found. Installing Rust...
    powershell -Command "Invoke-WebRequest https://win.rustup.rs -OutFile rustup-init.exe"
    start /wait rustup-init.exe -y
    del rustup-init.exe
    echo.
    echo Rust installed successfully!
    echo Please restart your terminal and rerun this script.
    pause
    exit /b
) else (
    echo [1/3] Rust and Cargo found.
)

echo [2/3] Installing BQuick from crates.io...
cargo install bquick
if errorlevel 1 (
    echo.
    echo Failed to install BQuick from crates.io.
    echo Trying alternative installation method...
    cargo install --git https://github.com/linux-brat/BQuick.git
    if errorlevel 1 (
        echo Installation failed completely.
        pause
        exit /b
    )
)

echo [3/3] Verifying installation...
where bquick >nul 2>&1
if errorlevel 1 (
    echo Warning: bquick not found in PATH.
    echo You may need to restart your terminal.
) else (
    echo BQuick found in PATH.
)

echo.
echo ================================================
echo            Installation Complete!
echo ================================================
echo.
echo To run BQuick, simply type:
echo.
echo    bquick
echo.
echo If command not found, restart your terminal.
echo.
pause
