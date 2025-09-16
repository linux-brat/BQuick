<div align="center">

# BQuick

**Professional Software Installer for Windows**

[![Windows](https://img.shields.io/badge/Windows-10%2F11-0078d4?style=for-the-badge&ds.io/badge/Rust-1.70+-orange?style=for-the-badge//img.shields.io![Crates.io](https://img.shields.io/crates/v/bquick?style=formanager for Windows. Install, update, and manage applications easily through WinGet and Chocolatey.

</div>

---

## What is BQuick?

BQuick simplifies software management on Windows. Instead of visiting multiple websites to download software, BQuick provides one interface to install applications from trusted sources like Microsoft's WinGet and Chocolatey package managers.

**Key Features:**

- Install 200+ popular applications instantly
- Uninstall and reinstall software safely
- Smart detection of already installed programs
- Real-time search across all software
- Professional terminal interface with progress tracking

---

## Installation

### Quick Install (Recommended)

Download and run our installer script:

```batch
# Download installer
curl -L -o install.bat https://raw.githubusercontent.com/linux-brat/bquick/master/install.bat

# Run installer (Right-click → Run as Administrator)
install.bat
```

**What happens:** The installer checks for Rust, installs it if needed, then installs BQuick automatically.

### Manual Install

If you have Rust installed:

```bash
cargo install bquick
```

If you don't have Rust:

1. Install Rust from [rustup.rs](https://rustup.rs/)
2. Restart your terminal
3. Run `cargo install bquick`

---

## Basic Usage

**Start BQuick:**

```bash
bquick
```

**Navigation:**
| Key | Action |
|-----|--------|
| `↑↓` | Navigate up/down |
| `Space` | Select/deselect software |
| `Enter` | Start installation |
| `A` | Select all in category |
| `U` | Switch to uninstall mode |
| `F5` | Scan for installed software |
| `/` | Search |
| `Q` | Quit |

**Simple Workflow:**

1. Launch BQuick
2. Choose installation method (WinGet recommended)
3. Browse software categories
4. Select applications with `Space`
5. Press `Enter` to install

---

## Available Software

BQuick includes popular applications across these categories:

- **Web Browsers:** Chrome, Firefox, Edge, Brave
- **Development:** VS Code, Git, Node.js, Docker, Python
- **Media:** VLC, OBS Studio, GIMP, Audacity
- **Utilities:** 7-Zip, PowerToys, Everything
- **Communication:** Discord, Zoom, Teams

Total: 200+ applications

---

## Common Tasks

**Install Multiple Programs:**

1. Navigate to a category
2. Use `Space` to select each program you want
3. Press `Enter` to install all selected

**Remove Software:**

1. Press `U` to switch to uninstall mode
2. Select programs to remove
3. Press `Enter` to uninstall

**Find Specific Software:**

1. Press `/` to search
2. Type the program name
3. Select from results

---

## Troubleshooting

**Installation fails:**

- Run terminal as Administrator
- Check internet connection
- Restart terminal if Rust was just installed

**Software not detected:**

- Press `F5` for deep scan
- Some portable apps may not be detected

**Need help:**

- Press `F1` in the app for help
- Visit [GitHub Issues](https://github.com/linux-brat/bquick/issues)

---

## Contributing

Want to help improve BQuick?

**Add New Software:**

1. Fork the repository
2. Edit `src/categories.rs`
3. Add your software entry
4. Submit a pull request

**Report Issues:**

- Use [GitHub Issues](https://github.com/linux-brat/bquick/issues) for bugs
- Include your Windows version and error details

**Development:**

```bash
git clone https://github.com/linux-brat/bquick.git
cd bquick
cargo run
```

---

## License

MIT License - Use freely for personal and commercial projects.

---

**Built with Rust - Designed for Windows - Open Source**

[View on GitHub](https://github.com/linux-brat/bquick) - [Report Issues](https://github.com/linux-brat/bquick/issues) - [Documentation](https://github.com/linux-brat/bquick/wiki)
