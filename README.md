<div align="center">

# BQuick

**Professional Software Installer for Windows**

[![Build Status](https://img.shields.io/github/actions/workflow/status/linux-brat/bquick/ci.yml?branch=main&ps://img.shields.io/github/v/release/linux-brat/b
[![License](https://img.shields.io/badge/license-MIT-blue.svg?style(https://img.shields.io/badge/rust-1.70+-orange.svg?style=for-theerminal-based software installation manager that streamlines Windows software management through WinGet and Chocolatey package managers.

</div>

## Features

- **Dual Package Manager Support** - WinGet/PowerShell and Chocolatey integration
- **Smart Software Detection** - Advanced scanning across registry, filesystem, and package databases
- **Professional Terminal UI** - Modern interface with real-time progress tracking
- **Multiple Operations** - Install, uninstall, and reinstall with automatic fallbacks
- **Extensive Catalog** - 200+ applications across browsers, development tools, media, and utilities
- **Advanced Search** - Real-time fuzzy search across all categories

## Installation

### Prerequisites

- **Rust & Cargo** ([Install Rust](https://rustup.rs/))
- **Windows 10/11** (64-bit)

### Quick Install

```powershell
# Automated installation script
Invoke-WebRequest -Uri "https://raw.githubusercontent.com/linux-brat/bquick/master/install.ps1" -OutFile "install.ps1"
powershell -ExecutionPolicy Bypass -File "install.ps1"
```

### Direct Cargo Install

```bash
cargo install bquick
```

or

```bash
cargo install --git https://github.com/linux-brat/bquick.git
```

## Usage

```bash
# Launch BQuick
bquick

# Navigate with arrow keys or j/k
# Space to select software
# Enter to start installation
# Q to quit
```

### Key Controls

| Key          | Action           |
| ------------ | ---------------- |
| `↑↓` / `j k` | Navigate         |
| `Space`      | Toggle selection |
| `Enter`      | Confirm/Start    |
| `A`          | Select all       |
| `U`          | Uninstall mode   |
| `F5`         | Deep scan        |
| `/`          | Search           |
| `Q`          | Quit             |

## Screenshots

_Application interface and installation progress screenshots_

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/name`)
3. Commit changes (`git commit -m 'Add feature'`)
4. Push to branch (`git push origin feature/name`)
5. Open a Pull Request

## License

MIT License - see [LICENSE](LICENSE) file for details.

---

<div align="center">

**Built with Rust** - **Designed for Windows** - **Powered by Community**

</div>
