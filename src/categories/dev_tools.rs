use super::{Category, Software};

pub fn get_dev_tools_category() -> Category {
    let mut category = Category::new("Development Tools", "[DEV]");

    let dev_tools = vec![
        // Code Editors & IDEs
        Software::new(
            "Visual Studio Code",
            "winget install Microsoft.VisualStudioCode --silent --accept-package-agreements --accept-source-agreements",
            "choco install vscode -y",
            "winget uninstall Microsoft.VisualStudioCode --silent",
            "winget list --id Microsoft.VisualStudioCode",
            "Powerful and popular code editor"
        ).with_repair("winget install Microsoft.VisualStudioCode --force --silent"),

        Software::new(
            "JetBrains IntelliJ IDEA",
            "winget install JetBrains.IntelliJIDEA.Community --silent --accept-package-agreements --accept-source-agreements",
            "choco install intellijidea-community -y",
            "winget uninstall JetBrains.IntelliJIDEA.Community --silent",
            "winget list --id JetBrains.IntelliJIDEA.Community",
            "Intelligent Java IDE"
        ).with_repair("winget install JetBrains.IntelliJIDEA.Community --force --silent"),

        Software::new(
            "JetBrains WebStorm",
            "winget install JetBrains.WebStorm --silent --accept-package-agreements --accept-source-agreements",
            "choco install webstorm -y",
            "winget uninstall JetBrains.WebStorm --silent",
            "winget list --id JetBrains.WebStorm",
            "JavaScript and TypeScript IDE"
        ).with_repair("winget install JetBrains.WebStorm --force --silent"),

        Software::new(
            "JetBrains PyCharm",
            "winget install JetBrains.PyCharm.Community --silent --accept-package-agreements --accept-source-agreements",
            "choco install pycharm-community -y",
            "winget uninstall JetBrains.PyCharm.Community --silent",
            "winget list --id JetBrains.PyCharm.Community",
            "Python IDE"
        ).with_repair("winget install JetBrains.PyCharm.Community --force --silent"),

        Software::new(
            "Visual Studio Community",
            "winget install Microsoft.VisualStudio.2022.Community --silent --accept-package-agreements --accept-source-agreements",
            "choco install visualstudio2022community -y",
            "winget uninstall Microsoft.VisualStudio.2022.Community --silent",
            "winget list --id Microsoft.VisualStudio.2022.Community",
            "Free IDE for .NET development"
        ).with_repair("winget install Microsoft.VisualStudio.2022.Community --force --silent"),

        Software::new(
            "Eclipse IDE",
            "winget install EclipseAdoptium.Temurin.8.JDK --silent --accept-package-agreements --accept-source-agreements",
            "choco install eclipse -y",
            "winget uninstall EclipseAdoptium.Temurin.8.JDK --silent",
            "winget list --id EclipseAdoptium.Temurin.8.JDK",
            "Popular Java development environment"
        ).with_repair("winget install EclipseAdoptium.Temurin.8.JDK --force --silent"),

        Software::new(
            "NetBeans",
            "winget install Apache.NetBeans --silent --accept-package-agreements --accept-source-agreements",
            "choco install netbeans -y",
            "winget uninstall Apache.NetBeans --silent",
            "winget list --id Apache.NetBeans",
            "Java IDE by Apache"
        ).with_repair("winget install Apache.NetBeans --force --silent"),

        Software::new(
            "Sublime Text",
            "winget install SublimeHQ.SublimeText.4 --silent --accept-package-agreements --accept-source-agreements",
            "choco install sublimetext4 -y",
            "winget uninstall SublimeHQ.SublimeText.4 --silent",
            "winget list --id SublimeHQ.SublimeText.4",
            "Sophisticated text editor for code"
        ).with_repair("winget install SublimeHQ.SublimeText.4 --force --silent"),

        Software::new(
            "Neovim",
            "winget install Neovim.Neovim --silent --accept-package-agreements --accept-source-agreements",
            "choco install neovim -y",
            "winget uninstall Neovim.Neovim --silent",
            "winget list --id Neovim.Neovim",
            "Hyperextensible Vim-based text editor"
        ).with_repair("winget install Neovim.Neovim --force --silent"),

        // Build & Dependency Management
        Software::new(
            "Node.js",
            "winget install OpenJS.NodeJS --silent --accept-package-agreements --accept-source-agreements",
            "choco install nodejs -y",
            "winget uninstall OpenJS.NodeJS --silent",
            "winget list --id OpenJS.NodeJS",
            "JavaScript runtime with npm"
        ).with_repair("winget install OpenJS.NodeJS --force --silent"),

        Software::new(
            "Yarn",
            "winget install Yarn.Yarn --silent --accept-package-agreements --accept-source-agreements",
            "choco install yarn -y",
            "winget uninstall Yarn.Yarn --silent",
            "winget list --id Yarn.Yarn",
            "Fast package manager for Node.js"
        ).with_repair("winget install Yarn.Yarn --force --silent"),

        Software::new(
            "Python",
            "winget install Python.Python.3.12 --silent --accept-package-agreements --accept-source-agreements",
            "choco install python -y",
            "winget uninstall Python.Python.3.12 --silent",
            "winget list --id Python.Python.3.12",
            "Python programming language"
        ).with_repair("winget install Python.Python.3.12 --force --silent"),

        Software::new(
            "Java JDK",
            "winget install Oracle.JDK.21 --silent --accept-package-agreements --accept-source-agreements",
            "choco install openjdk -y",
            "winget uninstall Oracle.JDK.21 --silent",
            "winget list --id Oracle.JDK.21",
            "Java Development Kit"
        ).with_repair("winget install Oracle.JDK.21 --force --silent"),

        Software::new(
            "Maven",
            "winget install Apache.Maven --silent --accept-package-agreements --accept-source-agreements",
            "choco install maven -y",
            "winget uninstall Apache.Maven --silent",
            "winget list --id Apache.Maven",
            "Java project management tool"
        ).with_repair("winget install Apache.Maven --force --silent"),

        Software::new(
            "Gradle",
            "winget install Gradle.Gradle --silent --accept-package-agreements --accept-source-agreements",
            "choco install gradle -y",
            "winget uninstall Gradle.Gradle --silent",
            "winget list --id Gradle.Gradle",
            "Build automation tool"
        ).with_repair("winget install Gradle.Gradle --force --silent"),

        Software::new(
            "Rust",
            "winget install Rustlang.Rust.MSVC --silent --accept-package-agreements --accept-source-agreements",
            "choco install rust -y",
            "winget uninstall Rustlang.Rust.MSVC --silent",
            "winget list --id Rustlang.Rust.MSVC",
            "Rust programming language with Cargo"
        ).with_repair("winget install Rustlang.Rust.MSVC --force --silent"),

        Software::new(
            "Go",
            "winget install GoLang.Go --silent --accept-package-agreements --accept-source-agreements",
            "choco install golang -y",
            "winget uninstall GoLang.Go --silent",
            "winget list --id GoLang.Go",
            "Go programming language"
        ).with_repair("winget install GoLang.Go --force --silent"),

        Software::new(
            "CMake",
            "winget install Kitware.CMake --silent --accept-package-agreements --accept-source-agreements",
            "choco install cmake -y",
            "winget uninstall Kitware.CMake --silent",
            "winget list --id Kitware.CMake",
            "Cross-platform build system generator"
        ).with_repair("winget install Kitware.CMake --force --silent"),

        // Debugging & Testing
        Software::new(
            "Postman",
            "winget install Postman.Postman --silent --accept-package-agreements --accept-source-agreements",
            "choco install postman -y",
            "winget uninstall Postman.Postman --silent",
            "winget list --id Postman.Postman",
            "API development and testing platform"
        ).with_repair("winget install Postman.Postman --force --silent"),

        Software::new(
            "Insomnia",
            "winget install Insomnia.Insomnia --silent --accept-package-agreements --accept-source-agreements",
            "choco install insomnia-rest-api-client -y",
            "winget uninstall Insomnia.Insomnia --silent",
            "winget list --id Insomnia.Insomnia",
            "API client and design tool"
        ).with_repair("winget install Insomnia.Insomnia --force --silent"),

        Software::new(
            "Fiddler",
            "winget install Telerik.Fiddler.Everywhere --silent --accept-package-agreements --accept-source-agreements",
            "choco install fiddler -y",
            "winget uninstall Telerik.Fiddler.Everywhere --silent",
            "winget list --id Telerik.Fiddler.Everywhere",
            "Web debugging proxy"
        ).with_repair("winget install Telerik.Fiddler.Everywhere --force --silent"),

        Software::new(
            "Wireshark",
            "winget install WiresharkFoundation.Wireshark --silent --accept-package-agreements --accept-source-agreements",
            "choco install wireshark -y",
            "winget uninstall WiresharkFoundation.Wireshark --silent",
            "winget list --id WiresharkFoundation.Wireshark",
            "Network protocol analyzer"
        ).with_repair("winget install WiresharkFoundation.Wireshark --force --silent"),

        // Version Control
        Software::new(
            "Git",
            "winget install Git.Git --silent --accept-package-agreements --accept-source-agreements",
            "choco install git -y",
            "winget uninstall Git.Git --silent",
            "winget list --id Git.Git",
            "Distributed version control system"
        ).with_repair("winget install Git.Git --force --silent"),

        Software::new(
            "GitHub Desktop",
            "winget install GitHub.GitHubDesktop --silent --accept-package-agreements --accept-source-agreements",
            "choco install github-desktop -y",
            "winget uninstall GitHub.GitHubDesktop --silent",
            "winget list --id GitHub.GitHubDesktop",
            "Git client for GitHub"
        ).with_repair("winget install GitHub.GitHubDesktop --force --silent"),

        Software::new(
            "GitKraken",
            "winget install Axosoft.GitKraken --silent --accept-package-agreements --accept-source-agreements",
            "choco install gitkraken -y",
            "winget uninstall Axosoft.GitKraken --silent",
            "winget list --id Axosoft.GitKraken",
            "Git GUI client"
        ).with_repair("winget install Axosoft.GitKraken --force --silent"),

        Software::new(
            "Sourcetree",
            "winget install Atlassian.Sourcetree --silent --accept-package-agreements --accept-source-agreements",
            "choco install sourcetree -y",
            "winget uninstall Atlassian.Sourcetree --silent",
            "winget list --id Atlassian.Sourcetree",
            "Git client by Atlassian"
        ).with_repair("winget install Atlassian.Sourcetree --force --silent"),

        Software::new(
            "GitHub CLI",
            "winget install GitHub.cli --silent --accept-package-agreements --accept-source-agreements",
            "choco install gh -y",
            "winget uninstall GitHub.cli --silent",
            "winget list --id GitHub.cli",
            "Command-line tool for GitHub"
        ).with_repair("winget install GitHub.cli --force --silent"),

        // Containerization & DevOps
        Software::new(
            "Docker Desktop",
            "winget install Docker.DockerDesktop --silent --accept-package-agreements --accept-source-agreements",
            "choco install docker-desktop -y",
            "winget uninstall Docker.DockerDesktop --silent",
            "winget list --id Docker.DockerDesktop",
            "Containerization platform"
        ).with_repair("winget install Docker.DockerDesktop --force --silent"),

        Software::new(
            "Kubernetes CLI",
            "winget install Kubernetes.kubectl --silent --accept-package-agreements --accept-source-agreements",
            "choco install kubernetes-cli -y",
            "winget uninstall Kubernetes.kubectl --silent",
            "winget list --id Kubernetes.kubectl",
            "Command-line tool for Kubernetes"
        ).with_repair("winget install Kubernetes.kubectl --force --silent"),

        Software::new(
            "Terraform",
            "winget install Hashicorp.Terraform --silent --accept-package-agreements --accept-source-agreements",
            "choco install terraform -y",
            "winget uninstall Hashicorp.Terraform --silent",
            "winget list --id Hashicorp.Terraform",
            "Infrastructure as code tool"
        ).with_repair("winget install Hashicorp.Terraform --force --silent"),

        Software::new(
            "Vagrant",
            "winget install Hashicorp.Vagrant --silent --accept-package-agreements --accept-source-agreements",
            "choco install vagrant -y",
            "winget uninstall Hashicorp.Vagrant --silent",
            "winget list --id Hashicorp.Vagrant",
            "Development environment manager"
        ).with_repair("winget install Hashicorp.Vagrant --force --silent"),

        // Web Development Tools
        Software::new(
            "ngrok",
            "winget install ngrok.ngrok --silent --accept-package-agreements --accept-source-agreements",
            "choco install ngrok -y",
            "winget uninstall ngrok.ngrok --silent",
            "winget list --id ngrok.ngrok",
            "Secure tunneling to localhost"
        ).with_repair("winget install ngrok.ngrok --force --silent"),

        // Environment Management
        Software::new(
            "NVM for Windows",
            "winget install CoreyButler.NVMforWindows --silent --accept-package-agreements --accept-source-agreements",
            "choco install nvm -y",
            "winget uninstall CoreyButler.NVMforWindows --silent",
            "winget list --id CoreyButler.NVMforWindows",
            "Node.js version manager for Windows"
        ).with_repair("winget install CoreyButler.NVMforWindows --force --silent"),

        Software::new(
            "Windows Terminal",
            "winget install Microsoft.WindowsTerminal --silent --accept-package-agreements --accept-source-agreements",
            "choco install microsoft-windows-terminal -y",
            "winget uninstall Microsoft.WindowsTerminal --silent",
            "winget list --id Microsoft.WindowsTerminal",
            "Modern terminal application"
        ).with_repair("winget install Microsoft.WindowsTerminal --force --silent"),

        Software::new(
            "PowerShell Core",
            "winget install Microsoft.PowerShell --silent --accept-package-agreements --accept-source-agreements",
            "choco install powershell-core -y",
            "winget uninstall Microsoft.PowerShell --silent",
            "winget list --id Microsoft.PowerShell",
            "Cross-platform PowerShell"
        ).with_repair("winget install Microsoft.PowerShell --force --silent"),

        // Additional Tools
        Software::new(
            "JetBrains Toolbox",
            "winget install JetBrains.Toolbox --silent --accept-package-agreements --accept-source-agreements",
            "choco install jetbrainstoolbox -y",
            "winget uninstall JetBrains.Toolbox --silent",
            "winget list --id JetBrains.Toolbox",
            "JetBrains tools manager"
        ).with_repair("winget install JetBrains.Toolbox --force --silent"),

        Software::new(
            "Azure CLI",
            "winget install Microsoft.AzureCLI --silent --accept-package-agreements --accept-source-agreements",
            "choco install azure-cli -y",
            "winget uninstall Microsoft.AzureCLI --silent",
            "winget list --id Microsoft.AzureCLI",
            "Command-line interface for Azure"
        ).with_repair("winget install Microsoft.AzureCLI --force --silent"),

        Software::new(
            "AWS CLI",
            "winget install Amazon.AWSCLI --silent --accept-package-agreements --accept-source-agreements",
            "choco install awscli -y",
            "winget uninstall Amazon.AWSCLI --silent",
            "winget list --id Amazon.AWSCLI",
            "Command-line interface for AWS"
        ).with_repair("winget install Amazon.AWSCLI --force --silent"),

        Software::new(
            "Redis",
            "winget install Redis.Redis --silent --accept-package-agreements --accept-source-agreements",
            "choco install redis-64 -y",
            "winget uninstall Redis.Redis --silent",
            "winget list --id Redis.Redis",
            "In-memory data structure store"
        ).with_repair("winget install Redis.Redis --force --silent"),

        Software::new(
            "MongoDB Compass",
            "winget install MongoDB.Compass --silent --accept-package-agreements --accept-source-agreements",
            "choco install mongodb-compass -y",
            "winget uninstall MongoDB.Compass --silent",
            "winget list --id MongoDB.Compass",
            "GUI for MongoDB"
        ).with_repair("winget install MongoDB.Compass --force --silent"),

        Software::new(
            "MySQL Workbench",
            "winget install Oracle.MySQLWorkbench --silent --accept-package-agreements --accept-source-agreements",
            "choco install mysql.workbench -y",
            "winget uninstall Oracle.MySQLWorkbench --silent",
            "winget list --id Oracle.MySQLWorkbench",
            "Visual database design tool"
        ).with_repair("winget install Oracle.MySQLWorkbench --force --silent"),
    ];

    for tool in dev_tools {
        category.add_software(tool);
    }

    category
}
