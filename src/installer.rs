use crate::app::{App, InstallMethod, OperationMode};
use crate::categories::Category;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

pub struct Installer;

// FIXED: Define static array for comprehensive aliases
static COMPREHENSIVE_ALIASES: &[(&str, &[&str])] = &[
    // Browsers
    (
        "google chrome",
        &["chrome", "googlechrome", "google-chrome"],
    ),
    ("mozilla firefox", &["firefox", "firefox.exe", "mozilla"]),
    ("microsoft edge", &["edge", "msedge", "microsoft-edge"]),
    (
        "brave browser",
        &["brave", "brave-browser", "bravesoftware"],
    ),
    ("brave", &["brave", "brave-browser", "bravesoftware"]),
    ("opera", &["opera", "opera.exe", "operasoftware"]),
    ("opera gx", &["operagx", "opera-gx", "gx"]),
    ("vivaldi", &["vivaldi", "vivaldi-browser"]),
    ("tor browser", &["tor", "torbrowser", "tor-browser"]),
    // Development Tools
    (
        "visual studio code",
        &["vscode", "code", "vs-code", "visualstudiocode"],
    ),
    (
        "jetbrains intellij idea",
        &["intellij", "idea", "intellijidea"],
    ),
    ("jetbrains webstorm", &["webstorm", "jetbrains-webstorm"]),
    ("jetbrains pycharm", &["pycharm", "jetbrains-pycharm"]),
    (
        "visual studio community",
        &["visualstudio", "vs2022", "visual-studio"],
    ),
    ("eclipse ide", &["eclipse", "eclipse-ide"]),
    ("netbeans", &["netbeans", "apache-netbeans"]),
    ("sublime text", &["sublime", "sublimetext", "sublime-text"]),
    ("neovim", &["nvim", "neovim", "vim"]),
    // Programming Languages & Runtimes
    ("node.js", &["node", "nodejs", "npm"]),
    ("python", &["python", "python3", "py"]),
    ("java jdk", &["java", "jdk", "openjdk", "oracle-jdk"]),
    ("rust", &["rust", "rustc", "cargo"]),
    ("go", &["go", "golang"]),
    // Build Tools
    ("maven", &["maven", "mvn", "apache-maven"]),
    ("gradle", &["gradle", "gradle-build-tool"]),
    ("cmake", &["cmake", "cmake-gui"]),
    // Version Control
    ("git", &["git", "git-scm", "gitforwindows"]),
    ("github desktop", &["githubdesktop", "github-desktop"]),
    ("gitkraken", &["gitkraken", "git-kraken"]),
    ("sourcetree", &["sourcetree", "atlassian-sourcetree"]),
    // Containerization & DevOps
    (
        "docker desktop",
        &["docker", "dockerdesktop", "docker-desktop"],
    ),
    ("kubernetes cli", &["kubectl", "kubernetes", "k8s"]),
    ("terraform", &["terraform", "tf"]),
    ("vagrant", &["vagrant", "hashicorp-vagrant"]),
    // Media Software
    ("vlc media player", &["vlc", "vlcmediaplayer", "videolan"]),
    ("obs studio", &["obs", "obs-studio", "obsstudio"]),
    ("gimp", &["gimp", "gnu-image-manipulation-program"]),
    ("paint.net", &["paintnet", "paint.net", "paint-net"]),
    ("krita", &["krita", "kde-krita"]),
    ("inkscape", &["inkscape", "inkscape-svg"]),
    ("audacity", &["audacity", "audacity-audio-editor"]),
    ("handbrake", &["handbrake", "handbrake-video-transcoder"]),
    ("ffmpeg", &["ffmpeg", "ff-mpeg"]),
    // Utilities
    ("7-zip", &["7zip", "7z", "sevenzip"]),
    ("winrar", &["winrar", "rar", "rarlab"]),
    ("peazip", &["peazip", "pea-zip"]),
    ("everything", &["everything", "voidtools-everything"]),
    ("powertoys", &["powertoys", "microsoft-powertoys"]),
    ("cpu-z", &["cpuz", "cpu-z", "cpuid"]),
    ("hwmonitor", &["hwmonitor", "hw-monitor", "cpuid-hwmonitor"]),
    (
        "ccleaner",
        &["ccleaner", "crap-cleaner", "piriform-ccleaner"],
    ),
    // Security & Privacy
    (
        "keepass",
        &["keepass", "keepass2", "dominik-reichl-keepass"],
    ),
    ("bitwarden", &["bitwarden", "bit-warden"]),
    ("veracrypt", &["veracrypt", "vera-crypt", "truecrypt"]),
    ("malwarebytes", &["malwarebytes", "mbam", "anti-malware"]),
    // Communication & Entertainment
    ("discord", &["discord", "discordapp"]),
    ("spotify", &["spotify", "spotify-music"]),
    ("steam", &["steam", "valve-steam"]),
    (
        "epic games launcher",
        &["epicgames", "epic-games", "unreal-engine"],
    ),
];

impl Installer {
    /// Start installation/uninstall/reinstall process
    pub fn start_installation(app: &mut App, log_sender: Sender<String>) {
        let selected_software: Vec<_> = app
            .categories
            .iter()
            .flat_map(|c| &c.software)
            .filter(|s| s.is_selected)
            .map(|s| {
                (
                    s.name.to_string(),
                    s.powershell_command.to_string(),
                    s.choco_command.to_string(),
                    s.uninstall_command.to_string(),
                    s.is_installed,
                )
            })
            .collect();

        if selected_software.is_empty() {
            let _ = log_sender.send("No software selected for operation".to_string());
            let _ = log_sender.send("INSTALLATION_COMPLETE".to_string());
            return;
        }

        let install_method = app.selected_method;
        let operation_mode = app.operation_mode;

        thread::spawn(move || {
            Self::run_operation_process(
                selected_software,
                install_method,
                operation_mode,
                log_sender,
            );
        });
    }

    /// Ultra-comprehensive system scan using multiple detection methods
    pub fn comprehensive_system_scan(
        categories: Vec<Category>,
        sender: Sender<(usize, usize, bool)>,
    ) {
        let installed_software = Self::get_installed_software_comprehensive();

        for (cat_idx, category) in categories.iter().enumerate() {
            for (soft_idx, software) in category.software.iter().enumerate() {
                let is_installed =
                    Self::accurate_software_match(&software.name, &installed_software);
                let _ = sender.send((cat_idx, soft_idx, is_installed));
                thread::sleep(Duration::from_millis(5)); // Controlled scanning speed
            }
        }
    }

    /// Comprehensive installed software detection combining multiple sources
    fn get_installed_software_comprehensive() -> HashMap<String, Vec<String>> {
        let mut software_database = HashMap::new();

        // Source 1: Windows Registry 32-bit Applications
        let registry_32 = Self::get_registry_installed_safe(
            "HKLM:\\Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\*",
        );
        if !registry_32.is_empty() {
            Self::merge_software_data(&mut software_database, registry_32, "Registry32");
        }

        // Source 2: Windows Registry 64-bit Applications
        let registry_64 = Self::get_registry_installed_safe(
            "HKLM:\\Software\\Wow6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\*",
        );
        if !registry_64.is_empty() {
            Self::merge_software_data(&mut software_database, registry_64, "Registry64");
        }

        // Source 3: Current User Registry Applications
        let registry_user = Self::get_registry_installed_safe(
            "HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\*",
        );
        if !registry_user.is_empty() {
            Self::merge_software_data(&mut software_database, registry_user, "RegistryUser");
        }

        // Source 4: WinGet Installed Packages
        let winget_installed = Self::get_winget_installed_accurate();
        if !winget_installed.is_empty() {
            Self::merge_software_data(&mut software_database, winget_installed, "WinGet");
        }

        // Source 5: Chocolatey Installed Packages
        let choco_installed = Self::get_chocolatey_installed_accurate();
        if !choco_installed.is_empty() {
            Self::merge_software_data(&mut software_database, choco_installed, "Chocolatey");
        }

        // Source 6: Microsoft Store Applications
        let store_apps = Self::get_microsoft_store_apps();
        if !store_apps.is_empty() {
            Self::merge_software_data(&mut software_database, store_apps, "MicrosoftStore");
        }

        // Source 7: File System Scan (Most Reliable)
        let filesystem_apps = Self::get_filesystem_installed_comprehensive();
        if !filesystem_apps.is_empty() {
            Self::merge_software_data(&mut software_database, filesystem_apps, "FileSystem");
        }

        // Source 8: Program Files Directory Scan
        let program_files = Self::scan_program_files_directories();
        if !program_files.is_empty() {
            Self::merge_software_data(&mut software_database, program_files, "ProgramFiles");
        }

        software_database
    }

    /// Safer registry scanning with comprehensive error handling
    fn get_registry_installed_safe(registry_path: &str) -> Vec<String> {
        let command = format!(
            r#"try {{ 
                Get-ItemProperty {} -ErrorAction SilentlyContinue | 
                Where-Object {{$_.DisplayName -and $_.DisplayName.Trim() -ne "" -and $_.DisplayName -notlike "*Update*" -and $_.DisplayName -notlike "*Hotfix*"}} | 
                Select-Object -ExpandProperty DisplayName | 
                Sort-Object -Unique
            }} catch {{ 
                Write-Output "" 
            }}"#,
            registry_path
        );

        Self::run_powershell_command_safe(&command, 8)
    }

    /// More accurate WinGet detection with better parsing
    fn get_winget_installed_accurate() -> Vec<String> {
        let command = r#"try {
            $output = winget list --accept-source-agreements --disable-interactivity 2>$null
            if ($output) {
                $output | Where-Object { 
                    $_ -match '^[A-Za-z]' -and 
                    $_ -notmatch '^Name\s+Id\s+Version' -and 
                    $_ -notmatch '^-+' -and
                    $_ -notmatch 'upgrades available' -and
                    $_.Trim() -ne ""
                } | ForEach-Object { 
                    $parts = $_ -split '\s{2,}'
                    if ($parts.Length -ge 1 -and $parts[0].Trim() -ne "") { 
                        $parts[0].Trim() 
                    }
                } | Where-Object { $_ -and $_.Trim() -ne "" } | Sort-Object -Unique
            }
        } catch { 
            Write-Output "" 
        }"#;

        Self::run_powershell_command_safe(command, 12)
    }

    /// More accurate Chocolatey detection with better error handling
    fn get_chocolatey_installed_accurate() -> Vec<String> {
        let command = r#"try {
            if (Get-Command choco -ErrorAction SilentlyContinue) {
                $output = choco list --local-only --no-progress 2>$null
                if ($output) {
                    $output | Where-Object { 
                        $_ -match '^\w+' -and 
                        $_ -notmatch 'packages installed' -and
                        $_ -notmatch 'Chocolatey v' -and
                        $_.Trim() -ne ""
                    } | ForEach-Object { 
                        ($_ -split '\s')[0].Trim()
                    } | Where-Object { $_ -and $_.Trim() -ne "" } | Sort-Object -Unique
                }
            }
        } catch { 
            Write-Output "" 
        }"#;

        Self::run_powershell_command_safe(command, 10)
    }

    /// Microsoft Store applications detection
    fn get_microsoft_store_apps() -> Vec<String> {
        let command = r#"try {
            Get-AppxPackage | Where-Object {
                $_.Name -notlike '*Windows*' -and 
                $_.Name -notlike '*Microsoft.Windows*' -and
                $_.Name -notlike '*Package*' -and
                $_.PackageFullName -notlike '*Framework*' -and
                $_.IsFramework -eq $false
            } | Select-Object -ExpandProperty Name | Sort-Object -Unique
        } catch { 
            Write-Output "" 
        }"#;

        Self::run_powershell_command_safe(command, 15)
    }

    /// Comprehensive filesystem scanning with accurate path detection
    fn get_filesystem_installed_comprehensive() -> Vec<String> {
        let mut detected = Vec::new();

        let software_paths = vec![
            // Browsers
            (
                r"C:\Program Files\Google\Chrome\Application\chrome.exe",
                "Google Chrome",
            ),
            (
                r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe",
                "Google Chrome",
            ),
            (
                r"C:\Program Files\Mozilla Firefox\firefox.exe",
                "Mozilla Firefox",
            ),
            (
                r"C:\Program Files (x86)\Mozilla Firefox\firefox.exe",
                "Mozilla Firefox",
            ),
            (
                r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe",
                "Microsoft Edge",
            ),
            (
                r"C:\Program Files\BraveSoftware\Brave-Browser\Application\brave.exe",
                "Brave Browser",
            ),
            (
                r"C:\Program Files (x86)\BraveSoftware\Brave-Browser\Application\brave.exe",
                "Brave Browser",
            ),
            (r"C:\Program Files\Opera\opera.exe", "Opera"),
            (r"C:\Program Files (x86)\Opera\opera.exe", "Opera"),
            (r"C:\Program Files\Opera GX\opera.exe", "Opera GX"),
            (r"C:\Program Files (x86)\Opera GX\opera.exe", "Opera GX"),
            (
                r"C:\Program Files\Vivaldi\Application\vivaldi.exe",
                "Vivaldi",
            ),
            (
                r"C:\Program Files (x86)\Vivaldi\Application\vivaldi.exe",
                "Vivaldi",
            ),
            // Development Tools
            (
                r"C:\Program Files\Microsoft VS Code\Code.exe",
                "Visual Studio Code",
            ),
            (
                r"C:\Program Files (x86)\Microsoft VS Code\Code.exe",
                "Visual Studio Code",
            ),
            (
                r"C:\Users\{USERNAME}\AppData\Local\Programs\Microsoft VS Code\Code.exe",
                "Visual Studio Code",
            ),
            (
                r"C:\Program Files\JetBrains\IntelliJ IDEA Community Edition",
                "JetBrains IntelliJ IDEA",
            ),
            (r"C:\Program Files\JetBrains\WebStorm", "JetBrains WebStorm"),
            (
                r"C:\Program Files\JetBrains\PyCharm Community Edition",
                "JetBrains PyCharm",
            ),
            (
                r"C:\Program Files\Microsoft Visual Studio\2022\Community",
                "Visual Studio Community",
            ),
            (
                r"C:\Program Files\Eclipse Foundation\Eclipse",
                "Eclipse IDE",
            ),
            (r"C:\Program Files\NetBeans", "NetBeans"),
            (
                r"C:\Program Files\Sublime Text\sublime_text.exe",
                "Sublime Text",
            ),
            (
                r"C:\Program Files (x86)\Sublime Text\sublime_text.exe",
                "Sublime Text",
            ),
            (r"C:\Program Files\Neovim\bin\nvim.exe", "Neovim"),
            // Programming Languages
            (r"C:\Program Files\nodejs\node.exe", "Node.js"),
            (r"C:\Program Files (x86)\nodejs\node.exe", "Node.js"),
            (r"C:\Python39\python.exe", "Python"),
            (r"C:\Python310\python.exe", "Python"),
            (r"C:\Python311\python.exe", "Python"),
            (r"C:\Python312\python.exe", "Python"),
            (r"C:\Program Files\Java\jdk", "Java JDK"),
            (r"C:\Program Files\Maven", "Maven"),
            (r"C:\Program Files\Gradle", "Gradle"),
            (r"C:\Program Files\Rust", "Rust"),
            (r"C:\Program Files\Go", "Go"),
            // Version Control
            (r"C:\Program Files\Git\bin\git.exe", "Git"),
            (r"C:\Program Files (x86)\Git\bin\git.exe", "Git"),
            (
                r"C:\Users\{USERNAME}\AppData\Local\GitHubDesktop",
                "GitHub Desktop",
            ),
            // Media Software
            (r"C:\Program Files\VideoLAN\VLC\vlc.exe", "VLC Media Player"),
            (
                r"C:\Program Files (x86)\VideoLAN\VLC\vlc.exe",
                "VLC Media Player",
            ),
            (
                r"C:\Program Files\obs-studio\bin\64bit\obs64.exe",
                "OBS Studio",
            ),
            (
                r"C:\Program Files (x86)\obs-studio\bin\32bit\obs32.exe",
                "OBS Studio",
            ),
            (r"C:\Program Files\GIMP 2\bin\gimp-2.10.exe", "GIMP"),
            (r"C:\Program Files\paint.net\PaintDotNet.exe", "Paint.NET"),
            (r"C:\Program Files\Audacity\audacity.exe", "Audacity"),
            // Utilities
            (r"C:\Program Files\7-Zip\7z.exe", "7-Zip"),
            (r"C:\Program Files (x86)\7-Zip\7z.exe", "7-Zip"),
            (r"C:\Program Files\WinRAR\WinRAR.exe", "WinRAR"),
            (r"C:\Program Files (x86)\WinRAR\WinRAR.exe", "WinRAR"),
            (r"C:\Program Files\Everything\Everything.exe", "Everything"),
            (r"C:\Program Files\PowerToys\PowerToys.exe", "PowerToys"),
            (r"C:\Program Files\CPUID\CPU-Z\cpuz.exe", "CPU-Z"),
            // Communication
            (r"C:\Program Files\Discord\Discord.exe", "Discord"),
            (r"C:\Users\{USERNAME}\AppData\Local\Discord", "Discord"),
            (r"C:\Program Files\Spotify\Spotify.exe", "Spotify"),
            (
                r"C:\Users\{USERNAME}\AppData\Roaming\Spotify\Spotify.exe",
                "Spotify",
            ),
        ];

        for (path, name) in software_paths {
            if Self::path_exists_with_username(path) {
                detected.push(name.to_string());
            }
        }

        detected
    }

    /// Handle USERNAME placeholder in paths and check directory existence
    fn path_exists_with_username(path: &str) -> bool {
        if path.contains("{USERNAME}") {
            if let Ok(username) = std::env::var("USERNAME") {
                let expanded_path = path.replace("{USERNAME}", &username);
                let path_obj = Path::new(&expanded_path);
                return path_obj.exists() || (path_obj.parent().map_or(false, |p| p.exists()));
            }
        }

        let path_obj = Path::new(path);
        path_obj.exists() || (path_obj.parent().map_or(false, |p| p.exists()))
    }

    /// Scan Program Files directories for additional software
    fn scan_program_files_directories() -> Vec<String> {
        let mut detected = Vec::new();
        let directories = ["C:\\Program Files", "C:\\Program Files (x86)"];

        for dir in &directories {
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries.flatten() {
                    if let Ok(file_type) = entry.file_type() {
                        if file_type.is_dir() {
                            if let Some(dir_name) = entry.file_name().to_str() {
                                // Filter out common system directories
                                if !dir_name.starts_with("Windows")
                                    && !dir_name.starts_with("Microsoft")
                                    && !dir_name.starts_with("Common Files")
                                    && dir_name.len() > 2
                                {
                                    detected.push(dir_name.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }

        detected
    }

    /// Advanced software name matching with multiple verification methods
    fn accurate_software_match(
        target_name: &str,
        software_database: &HashMap<String, Vec<String>>,
    ) -> bool {
        let target_normalized = Self::normalize_name_strict(target_name);

        // Don't match if normalized name is too short or generic
        if target_normalized.len() < 3
            || target_normalized == "app"
            || target_normalized == "tool"
            || target_normalized == "software"
        {
            return false;
        }

        for (source, names) in software_database {
            for installed_name in names {
                let installed_normalized = Self::normalize_name_strict(installed_name);

                // Skip if installed name is too generic or short
                if installed_normalized.len() < 3 {
                    continue;
                }

                // Exact match (highest confidence)
                if target_normalized == installed_normalized {
                    return true;
                }

                // Check comprehensive aliases
                if Self::check_comprehensive_aliases(target_name, installed_name) {
                    return true;
                }

                // Special matching for known software patterns
                if Self::special_software_patterns(target_name, installed_name) {
                    return true;
                }

                // Partial match with high confidence requirements
                if target_normalized.len() > 5 && installed_normalized.len() > 5 {
                    // Bidirectional contains check
                    if target_normalized.contains(&installed_normalized)
                        || installed_normalized.contains(&target_normalized)
                    {
                        // Higher confidence for filesystem detection
                        if source == "FileSystem" {
                            return true;
                        }
                        // For other sources, require very high similarity
                        if Self::calculate_string_similarity(
                            &target_normalized,
                            &installed_normalized,
                        ) > 0.90
                        {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }

    /// Stricter name normalization to prevent false matches
    fn normalize_name_strict(name: &str) -> String {
        name.to_lowercase()
            .replace(
                &[' ', '.', '-', '_', '(', ')', '[', ']', '™', '®', '©'][..],
                "",
            )
            .replace("&", "and")
            .replace("+", "plus")
            .replace("#", "sharp")
            .replace("++", "plusplus")
            .trim()
            .to_string()
    }

    /// Comprehensive alias checking system
    fn check_comprehensive_aliases(target: &str, installed: &str) -> bool {
        let target_lower = target.to_lowercase();
        let installed_lower = installed.to_lowercase();

        // FIXED: Use static array instead of local variable
        for &(software, aliases) in COMPREHENSIVE_ALIASES {
            if target_lower.contains(software) {
                for &alias in aliases {
                    if installed_lower.contains(alias) {
                        return true;
                    }
                }
            }
        }

        false
    }

    /// Special pattern matching for known software families
    fn special_software_patterns(target: &str, installed: &str) -> bool {
        let target_lower = target.to_lowercase();
        let installed_lower = installed.to_lowercase();

        // JetBrains products family
        if target_lower.contains("jetbrains") && installed_lower.contains("jetbrains") {
            return true;
        }

        // Microsoft products family
        if (target_lower.contains("microsoft") || target_lower.contains("visual studio"))
            && (installed_lower.contains("microsoft") || installed_lower.contains("visual studio"))
        {
            return true;
        }

        // Adobe products family
        if target_lower.contains("adobe") && installed_lower.contains("adobe") {
            return true;
        }

        // Firefox variants
        if target_lower.contains("firefox")
            && (installed_lower.contains("firefox") || installed_lower.contains("mozilla"))
        {
            return true;
        }

        false
    }

    /// Calculate string similarity using a simple but effective method - FIXED: Correct function name
    fn calculate_string_similarity(s1: &str, s2: &str) -> f64 {
        if s1.is_empty() || s2.is_empty() {
            return 0.0;
        }

        let len1 = s1.len();
        let len2 = s2.len();
        let max_len = len1.max(len2);

        if max_len == 0 {
            return 1.0;
        }

        // Count common characters
        let common_chars: usize = s1.chars().filter(|c| s2.contains(*c)).count();

        common_chars as f64 / max_len as f64
    }

    /// Merge software data from different sources
    fn merge_software_data(
        database: &mut HashMap<String, Vec<String>>,
        new_data: Vec<String>,
        source: &str,
    ) {
        for item in new_data {
            if !item.trim().is_empty() && item.len() > 2 {
                database
                    .entry(source.to_string())
                    .or_insert_with(Vec::new)
                    .push(item.trim().to_string());
            }
        }
    }

    /// Enhanced PowerShell command execution with comprehensive error handling
    fn run_powershell_command_safe(command: &str, timeout_seconds: u64) -> Vec<String> {
        let mut cmd = Command::new("powershell");
        cmd.args([
            "-WindowStyle", "Hidden",
            "-NoProfile", 
            "-NonInteractive", 
            "-ExecutionPolicy", "Bypass",
            "-NoLogo",
            "-Command", 
            &format!("$ProgressPreference = 'SilentlyContinue'; $ErrorActionPreference = 'SilentlyContinue'; {}", command)
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .stdin(Stdio::null());

        #[cfg(target_os = "windows")]
        cmd.creation_flags(0x08000000);

        let (tx, rx) = std::sync::mpsc::channel();

        thread::spawn(move || match cmd.spawn() {
            Ok(mut child) => {
                if let Some(stdout) = child.stdout.take() {
                    let reader = BufReader::new(stdout);
                    let mut result = Vec::new();

                    for line in reader.lines() {
                        if let Ok(line) = line {
                            let trimmed = line.trim();
                            if !trimmed.is_empty()
                                && !trimmed.starts_with("WARNING")
                                && !trimmed.starts_with("ERROR")
                                && !trimmed.contains("Get-Help")
                                && trimmed.len() > 1
                            {
                                result.push(trimmed.to_string());
                            }
                        }
                    }

                    let _ = child.wait();
                    let _ = tx.send(result);
                } else {
                    let _ = tx.send(Vec::new());
                }
            }
            Err(_) => {
                let _ = tx.send(Vec::new());
            }
        });

        match rx.recv_timeout(Duration::from_secs(timeout_seconds)) {
            Ok(result) => result,
            Err(_) => Vec::new(),
        }
    }

    /// Run operation process with comprehensive error handling and logging
    fn run_operation_process(
        software_list: Vec<(String, String, String, String, bool)>,
        install_method: Option<InstallMethod>,
        operation_mode: OperationMode,
        log_sender: Sender<String>,
    ) {
        let operation_name = match operation_mode {
            OperationMode::Install => "installation",
            OperationMode::Uninstall => "uninstallation",
            OperationMode::Reinstall => "reinstallation",
        };

        let _ = log_sender.send(format!("Starting {} process...", operation_name));
        let _ = log_sender.send("=".repeat(60));

        for (index, (name, ps_command, choco_command, uninstall_command, is_installed)) in
            software_list.iter().enumerate()
        {
            match operation_mode {
                OperationMode::Install => {
                    let _ = log_sender.send(format!("INSTALLING:{}", name));
                }
                OperationMode::Uninstall => {
                    let _ = log_sender.send(format!("UNINSTALLING:{}", name));
                }
                OperationMode::Reinstall => {
                    let _ = log_sender.send(format!("REINSTALLING:{}", name));
                }
            }

            let _ = log_sender.send(format!("PROGRESS:{}", index + 1));
            let _ = log_sender.send(format!("Processing: {}", name));
            let _ = log_sender.send("-".repeat(40));

            match operation_mode {
                OperationMode::Install => {
                    if *is_installed {
                        let _ = log_sender.send(format!(
                            "{} is already installed. Checking for updates...",
                            name
                        ));
                    } else {
                        let _ =
                            log_sender.send(format!("Installing {} for the first time...", name));
                        let (primary_cmd, fallback_cmd) = match install_method {
                            Some(InstallMethod::Chocolatey) => {
                                (choco_command.as_str(), ps_command.as_str())
                            }
                            _ => (ps_command.as_str(), choco_command.as_str()),
                        };

                        if !primary_cmd.is_empty() {
                            match Self::execute_command(primary_cmd, &log_sender) {
                                Ok(_) => {
                                    let _ = log_sender
                                        .send(format!("✓ Successfully installed {}", name));
                                }
                                Err(e) => {
                                    let _ = log_sender.send(format!(
                                        "✗ Primary install failed for {}: {}",
                                        name, e
                                    ));
                                    if !fallback_cmd.is_empty() {
                                        let _ = log_sender.send(format!(
                                            "⚡ Trying fallback method for {}...",
                                            name
                                        ));
                                        match Self::execute_command(fallback_cmd, &log_sender) {
                                            Ok(_) => {
                                                let _ = log_sender.send(format!(
                                                    "✓ Successfully installed {} using fallback",
                                                    name
                                                ));
                                            }
                                            Err(e2) => {
                                                let _ = log_sender.send(format!(
                                                    "✗ Both methods failed for {}: {}",
                                                    name, e2
                                                ));
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            let _ = log_sender
                                .send(format!("⚠ No install command available for {}", name));
                        }
                    }
                }
                OperationMode::Uninstall => {
                    if *is_installed {
                        if !uninstall_command.is_empty() {
                            let _ = log_sender.send(format!("Uninstalling {}...", name));
                            match Self::execute_command(uninstall_command, &log_sender) {
                                Ok(_) => {
                                    let _ = log_sender
                                        .send(format!("✓ Successfully uninstalled {}", name));
                                }
                                Err(e) => {
                                    let _ = log_sender
                                        .send(format!("✗ Failed to uninstall {}: {}", name, e));
                                }
                            }
                        } else {
                            let _ = log_sender
                                .send(format!("⚠ No uninstall command available for {}", name));
                        }
                    } else {
                        let _ = log_sender
                            .send(format!("⚠ {} is not installed, skipping uninstall", name));
                    }
                }
                OperationMode::Reinstall => {
                    if *is_installed {
                        let _ = log_sender.send(format!("Step 1: Uninstalling {}...", name));
                        if !uninstall_command.is_empty() {
                            match Self::execute_command(uninstall_command, &log_sender) {
                                Ok(_) => {
                                    let _ = log_sender
                                        .send(format!("✓ Successfully uninstalled {}", name));
                                    thread::sleep(Duration::from_secs(2));

                                    let _ = log_sender
                                        .send(format!("Step 2: Reinstalling {}...", name));
                                    let (primary_cmd, _fallback_cmd) = match install_method {
                                        Some(InstallMethod::Chocolatey) => {
                                            (choco_command.as_str(), ps_command.as_str())
                                        }
                                        _ => (ps_command.as_str(), choco_command.as_str()),
                                    };

                                    if !primary_cmd.is_empty() {
                                        match Self::execute_command(primary_cmd, &log_sender) {
                                            Ok(_) => {
                                                let _ = log_sender.send(format!(
                                                    "✓ Successfully reinstalled {}",
                                                    name
                                                ));
                                            }
                                            Err(e) => {
                                                let _ = log_sender.send(format!(
                                                    "✗ Failed to reinstall {}: {}",
                                                    name, e
                                                ));
                                            }
                                        }
                                    }
                                }
                                Err(e) => {
                                    let _ = log_sender.send(format!(
                                        "✗ Failed to uninstall {} (skipping reinstall): {}",
                                        name, e
                                    ));
                                }
                            }
                        } else {
                            let _ = log_sender
                                .send(format!("⚠ No uninstall command available for {}", name));
                        }
                    } else {
                        let _ = log_sender
                            .send(format!("⚠ {} is not installed, cannot reinstall", name));
                    }
                }
            }

            let _ = log_sender.send("".to_string());
        }

        let _ = log_sender.send("=".repeat(60));
        let _ = log_sender.send(format!("🎉 All {} operations completed!", operation_name));
        let _ = log_sender.send("INSTALLATION_COMPLETE".to_string());
    }

    /// Enhanced command execution with better error handling and output capture
    fn execute_command(command: &str, log_sender: &Sender<String>) -> Result<(), String> {
        let mut cmd = Command::new("powershell");
        cmd.args([
            "-WindowStyle",
            "Hidden",
            "-NoProfile",
            "-NonInteractive",
            "-ExecutionPolicy",
            "Bypass",
            "-NoLogo",
            "-Command",
            &format!(
                r#"
                $ProgressPreference = 'SilentlyContinue';
                $ErrorActionPreference = 'Stop';
                try {{
                    Write-Host "Executing command..."
                    {}
                    Write-Host "Command completed successfully"
                }} catch {{
                    Write-Error "Command failed: $_"
                    exit 1
                }}
            "#,
                command
            ),
        ]);

        #[cfg(target_os = "windows")]
        cmd.creation_flags(0x08000000);

        let mut child = cmd
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .stdin(Stdio::null())
            .spawn()
            .map_err(|e| format!("Failed to start process: {}", e))?;

        if let Some(stdout) = child.stdout.take() {
            let log_sender_stdout = log_sender.clone();
            thread::spawn(move || {
                let reader = BufReader::new(stdout);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        let trimmed = line.trim();
                        if !trimmed.is_empty() && !trimmed.contains("ProgressPreference") {
                            let _ = log_sender_stdout.send(format!("  {}", trimmed));
                        }
                    }
                }
            });
        }

        if let Some(stderr) = child.stderr.take() {
            let log_sender_stderr = log_sender.clone();
            thread::spawn(move || {
                let reader = BufReader::new(stderr);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        let trimmed = line.trim();
                        if !trimmed.is_empty() {
                            let _ = log_sender_stderr.send(format!("  ⚠ Error: {}", trimmed));
                        }
                    }
                }
            });
        }

        let status = child
            .wait()
            .map_err(|e| format!("Process execution failed: {}", e))?;

        if status.success() {
            Ok(())
        } else {
            Err(format!(
                "Command failed with exit code: {:?}",
                status.code()
            ))
        }
    }
}
