use super::{Category, Software};

pub fn get_utilities_category() -> Category {
    let mut category = Category::new("Utilities & System", "[UTL]");

    let utilities = vec![
        // === SYSTEM UTILITIES ===
        Software::new(
            "7-Zip",
            "winget install 7zip.7zip --silent --accept-package-agreements --accept-source-agreements",
            "choco install 7zip -y",
            "winget uninstall 7zip.7zip --silent",
            "winget list --id 7zip.7zip",
            "File archiver and compressor"
        ).with_repair("winget install 7zip.7zip --force --silent"),

        Software::new(
            "WinRAR",
            "winget install RARLab.WinRAR --silent --accept-package-agreements --accept-source-agreements",
            "choco install winrar -y",
            "winget uninstall RARLab.WinRAR --silent",
            "winget list --id RARLab.WinRAR",
            "Archive manager for RAR and ZIP files"
        ).with_repair("winget install RARLab.WinRAR --force --silent"),

        Software::new(
            "PeaZip",
            "winget install Peazip.Peazip --silent --accept-package-agreements --accept-source-agreements",
            "choco install peazip -y",
            "winget uninstall Peazip.Peazip --silent",
            "winget list --id Peazip.Peazip",
            "Free archiver with strong encryption"
        ).with_repair("winget install Peazip.Peazip --force --silent"),

        Software::new(
            "Sysinternals Suite",
            "winget install Microsoft.Sysinternals --silent --accept-package-agreements --accept-source-agreements",
            "choco install sysinternals -y",
            "winget uninstall Microsoft.Sysinternals --silent",
            "winget list --id Microsoft.Sysinternals",
            "Advanced system utilities from Microsoft"
        ).with_repair("winget install Microsoft.Sysinternals --force --silent"),

        Software::new(
            "Everything",
            "winget install voidtools.Everything --silent --accept-package-agreements --accept-source-agreements",
            "choco install everything -y",
            "winget uninstall voidtools.Everything --silent",
            "winget list --id voidtools.Everything",
            "Instant file and folder search tool"
        ).with_repair("winget install voidtools.Everything --force --silent"),

        Software::new(
            "PowerToys",
            "winget install Microsoft.PowerToys --silent --accept-package-agreements --accept-source-agreements",
            "choco install powertoys -y",
            "winget uninstall Microsoft.PowerToys --silent",
            "winget list --id Microsoft.PowerToys",
            "Windows system utilities and power tools"
        ).with_repair("winget install Microsoft.PowerToys --force --silent"),

        Software::new(
            "CPU-Z",
            "winget install CPUID.CPU-Z --silent --accept-package-agreements --accept-source-agreements",
            "choco install cpu-z -y",
            "winget uninstall CPUID.CPU-Z --silent",
            "winget list --id CPUID.CPU-Z",
            "System information and CPU details"
        ).with_repair("winget install CPUID.CPU-Z --force --silent"),

        Software::new(
            "HWMonitor",
            "winget install CPUID.HWMonitor --silent --accept-package-agreements --accept-source-agreements",
            "choco install hwmonitor -y",
            "winget uninstall CPUID.HWMonitor --silent",
            "winget list --id CPUID.HWMonitor",
            "Hardware monitoring and temperature tool"
        ).with_repair("winget install CPUID.HWMonitor --force --silent"),

        Software::new(
            "Speccy",
            "winget install Piriform.Speccy --silent --accept-package-agreements --accept-source-agreements",
            "choco install speccy -y",
            "winget uninstall Piriform.Speccy --silent",
            "winget list --id Piriform.Speccy",
            "System information and hardware details"
        ).with_repair("winget install Piriform.Speccy --force --silent"),

        Software::new(
            "CrystalDiskInfo",
            "winget install CrystalDewWorld.CrystalDiskInfo --silent --accept-package-agreements --accept-source-agreements",
            "choco install crystaldiskinfo -y",
            "winget uninstall CrystalDewWorld.CrystalDiskInfo --silent",
            "winget list --id CrystalDewWorld.CrystalDiskInfo",
            "Hard drive health monitoring"
        ).with_repair("winget install CrystalDewWorld.CrystalDiskInfo --force --silent"),

        Software::new(
            "CrystalDiskMark",
            "winget install CrystalDewWorld.CrystalDiskMark --silent --accept-package-agreements --accept-source-agreements",
            "choco install crystaldiskmark -y",
            "winget uninstall CrystalDewWorld.CrystalDiskMark --silent",
            "winget list --id CrystalDewWorld.CrystalDiskMark",
            "Disk benchmarking and performance testing"
        ).with_repair("winget install CrystalDewWorld.CrystalDiskMark --force --silent"),

        Software::new(
            "HWiNFO",
            "winget install REALiX.HWiNFO --silent --accept-package-agreements --accept-source-agreements",
            "choco install hwinfo -y",
            "winget uninstall REALiX.HWiNFO --silent",
            "winget list --id REALiX.HWiNFO",
            "Hardware information and diagnostics"
        ).with_repair("winget install REALiX.HWiNFO --force --silent"),

        Software::new(
            "Autoruns",
            "winget install Microsoft.Sysinternals.Autoruns --silent --accept-package-agreements --accept-source-agreements",
            "choco install autoruns -y",
            "winget uninstall Microsoft.Sysinternals.Autoruns --silent",
            "winget list --id Microsoft.Sysinternals.Autoruns",
            "Manage Windows startup programs"
        ).with_repair("winget install Microsoft.Sysinternals.Autoruns --force --silent"),

        Software::new(
            "Revo Uninstaller",
            "winget install RevoUninstaller.RevoUninstaller --silent --accept-package-agreements --accept-source-agreements",
            "choco install revo-uninstaller -y",
            "winget uninstall RevoUninstaller.RevoUninstaller --silent",
            "winget list --id RevoUninstaller.RevoUninstaller",
            "Advanced software uninstaller"
        ).with_repair("winget install RevoUninstaller.RevoUninstaller --force --silent"),

        Software::new(
            "Geek Uninstaller",
            "winget install CrystalRich.GeekUninstaller --silent --accept-package-agreements --accept-source-agreements",
            "choco install geekuninstaller -y",
            "winget uninstall CrystalRich.GeekUninstaller --silent",
            "winget list --id CrystalRich.GeekUninstaller",
            "Lightweight software uninstaller"
        ).with_repair("winget install CrystalRich.GeekUninstaller --force --silent"),

        Software::new(
            "CCleaner",
            "winget install Piriform.CCleaner --silent --accept-package-agreements --accept-source-agreements",
            "choco install ccleaner -y",
            "winget uninstall Piriform.CCleaner --silent",
            "winget list --id Piriform.CCleaner",
            "System cleaner and privacy tool"
        ).with_repair("winget install Piriform.CCleaner --force --silent"),

        Software::new(
            "Defraggler",
            "winget install Piriform.Defraggler --silent --accept-package-agreements --accept-source-agreements",
            "choco install defraggler -y",
            "winget uninstall Piriform.Defraggler --silent",
            "winget list --id Piriform.Defraggler",
            "Disk defragmentation utility"
        ).with_repair("winget install Piriform.Defraggler --force --silent"),

        Software::new(
            "TreeSize Free",
            "winget install JAMSoftware.TreeSize.Free --silent --accept-package-agreements --accept-source-agreements",
            "choco install treesizefree -y",
            "winget uninstall JAMSoftware.TreeSize.Free --silent",
            "winget list --id JAMSoftware.TreeSize.Free",
            "Disk space analyzer and visualizer"
        ).with_repair("winget install JAMSoftware.TreeSize.Free --force --silent"),

        Software::new(
            "WinDirStat",
            "winget install WinDirStat.WinDirStat --silent --accept-package-agreements --accept-source-agreements",
            "choco install windirstat -y",
            "winget uninstall WinDirStat.WinDirStat --silent",
            "winget list --id WinDirStat.WinDirStat",
            "Directory statistics and disk usage"
        ).with_repair("winget install WinDirStat.WinDirStat --force --silent"),

        Software::new(
            "Microsoft PC Manager",
            "winget install Microsoft.PCManager --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall Microsoft.PCManager --silent",
            "winget list --id Microsoft.PCManager",
            "Official Microsoft PC maintenance utility"
        ).with_repair("winget install Microsoft.PCManager --force --silent"),

        // === FILE & DISK TOOLS ===
        Software::new(
            "Rufus",
            "winget install Rufus.Rufus --silent --accept-package-agreements --accept-source-agreements",
            "choco install rufus -y",
            "winget uninstall Rufus.Rufus --silent",
            "winget list --id Rufus.Rufus",
            "Bootable USB drive creator"
        ).with_repair("winget install Rufus.Rufus --force --silent"),

        Software::new(
            "Etcher",
            "winget install Balena.Etcher --silent --accept-package-agreements --accept-source-agreements",
            "choco install balenaetcher -y",
            "winget uninstall Balena.Etcher --silent",
            "winget list --id Balena.Etcher",
            "USB and SD card imaging utility"
        ).with_repair("winget install Balena.Etcher --force --silent"),

        Software::new(
            "HashCheck",
            "winget install Kai.Liu.HashCheck --silent --accept-package-agreements --accept-source-agreements",
            "choco install hashcheck -y",
            "winget uninstall Kai.Liu.HashCheck --silent",
            "winget list --id Kai.Liu.HashCheck",
            "File hash verification tool"
        ).with_repair("winget install Kai.Liu.HashCheck --force --silent"),

        Software::new(
            "TeraCopy",
            "winget install CodeSector.TeraCopy --silent --accept-package-agreements --accept-source-agreements",
            "choco install teracopy -y",
            "winget uninstall CodeSector.TeraCopy --silent",
            "winget list --id CodeSector.TeraCopy",
            "Enhanced file copy utility"
        ).with_repair("winget install CodeSector.TeraCopy --force --silent"),

        Software::new(
            "FreeFileSync",
            "winget install FreeFileSync.FreeFileSync --silent --accept-package-agreements --accept-source-agreements",
            "choco install freefilesync -y",
            "winget uninstall FreeFileSync.FreeFileSync --silent",
            "winget list --id FreeFileSync.FreeFileSync",
            "File synchronization and backup tool"
        ).with_repair("winget install FreeFileSync.FreeFileSync --force --silent"),

        Software::new(
            "SyncBackFree",
            "winget install 2BrightSparks.SyncBackFree --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall 2BrightSparks.SyncBackFree --silent",
            "winget list --id 2BrightSparks.SyncBackFree",
            "File backup and synchronization"
        ).with_repair("winget install 2BrightSparks.SyncBackFree --force --silent"),

        Software::new(
            "UltraISO",
            "winget install EZBSystems.UltraISO --silent --accept-package-agreements --accept-source-agreements",
            "choco install ultraiso -y",
            "winget uninstall EZBSystems.UltraISO --silent",
            "winget list --id EZBSystems.UltraISO",
            "CD/DVD image file editor and creator"
        ).with_repair("winget install EZBSystems.UltraISO --force --silent"),

        Software::new(
            "ImgBurn",
            "winget install ImgBurn.ImgBurn --silent --accept-package-agreements --accept-source-agreements",
            "choco install imgburn -y",
            "winget uninstall ImgBurn.ImgBurn --silent",
            "winget list --id ImgBurn.ImgBurn",
            "Lightweight CD/DVD burning software"
        ).with_repair("winget install ImgBurn.ImgBurn --force --silent"),

        Software::new(
            "WinCDEmu",
            "winget install Bazis.WinCDEmu --silent --accept-package-agreements --accept-source-agreements",
            "choco install wincdemu -y",
            "winget uninstall Bazis.WinCDEmu --silent",
            "winget list --id Bazis.WinCDEmu",
            "Virtual CD/DVD drive emulator"
        ).with_repair("winget install Bazis.WinCDEmu --force --silent"),

        Software::new(
            "WizTree",
            "winget install AntibodySoftware.WizTree --silent --accept-package-agreements --accept-source-agreements",
            "choco install wiztree -y",
            "winget uninstall AntibodySoftware.WizTree --silent",
            "winget list --id AntibodySoftware.WizTree",
            "Fast disk space analyzer"
        ).with_repair("winget install AntibodySoftware.WizTree --force --silent"),

        // === NETWORKING TOOLS ===
        Software::new(
            "PuTTY",
            "winget install PuTTY.PuTTY --silent --accept-package-agreements --accept-source-agreements",
            "choco install putty -y",
            "winget uninstall PuTTY.PuTTY --silent",
            "winget list --id PuTTY.PuTTY",
            "SSH and telnet client"
        ).with_repair("winget install PuTTY.PuTTY --force --silent"),

        Software::new(
            "OpenSSH",
            "winget install Microsoft.OpenSSH.Beta --silent --accept-package-agreements --accept-source-agreements",
            "choco install openssh -y",
            "winget uninstall Microsoft.OpenSSH.Beta --silent",
            "winget list --id Microsoft.OpenSSH.Beta",
            "Secure Shell protocol implementation"
        ).with_repair("winget install Microsoft.OpenSSH.Beta --force --silent"),

        Software::new(
            "WinSCP",
            "winget install WinSCP.WinSCP --silent --accept-package-agreements --accept-source-agreements",
            "choco install winscp -y",
            "winget uninstall WinSCP.WinSCP --silent",
            "winget list --id WinSCP.WinSCP",
            "SFTP, FTP and SCP client"
        ).with_repair("winget install WinSCP.WinSCP --force --silent"),

        Software::new(
            "FileZilla",
            "winget install TimKosse.FileZilla.Client --silent --accept-package-agreements --accept-source-agreements",
            "choco install filezilla -y",
            "winget uninstall TimKosse.FileZilla.Client --silent",
            "winget list --id TimKosse.FileZilla.Client",
            "Cross-platform FTP client"
        ).with_repair("winget install TimKosse.FileZilla.Client --force --silent"),

        Software::new(
            "Nmap",
            "winget install Insecure.Nmap --silent --accept-package-agreements --accept-source-agreements",
            "choco install nmap -y",
            "winget uninstall Insecure.Nmap --silent",
            "winget list --id Insecure.Nmap",
            "Network discovery and security scanner"
        ).with_repair("winget install Insecure.Nmap --force --silent"),

        Software::new(
            "NetSpeedMonitor",
            "winget install Florian.Gilles.NetSpeedMonitor --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall Florian.Gilles.NetSpeedMonitor --silent",
            "winget list --id Florian.Gilles.NetSpeedMonitor",
            "Network speed monitoring tool"
        ).with_repair("winget install Florian.Gilles.NetSpeedMonitor --force --silent"),

        Software::new(
            "Angry IP Scanner",
            "winget install AngryIPScanner.AngryIPScanner --silent --accept-package-agreements --accept-source-agreements",
            "choco install angryip -y",
            "winget uninstall AngryIPScanner.AngryIPScanner --silent",
            "winget list --id AngryIPScanner.AngryIPScanner",
            "Fast network IP scanner"
        ).with_repair("winget install AngryIPScanner.AngryIPScanner --force --silent"),

        Software::new(
            "TCPView",
            "winget install Microsoft.Sysinternals.TCPView --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall Microsoft.Sysinternals.TCPView --silent",
            "winget list --id Microsoft.Sysinternals.TCPView",
            "TCP and UDP endpoint viewer"
        ).with_repair("winget install Microsoft.Sysinternals.TCPView --force --silent"),

        Software::new(
            "NetLimiter",
            "winget install Locktime.NetLimiter --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall Locktime.NetLimiter --silent",
            "winget list --id Locktime.NetLimiter",
            "Internet traffic control and monitoring"
        ).with_repair("winget install Locktime.NetLimiter --force --silent"),

        // === SECURITY & PRIVACY TOOLS ===
        Software::new(
            "KeePass",
            "winget install DominikReichl.KeePass --silent --accept-package-agreements --accept-source-agreements",
            "choco install keepass -y",
            "winget uninstall DominikReichl.KeePass --silent",
            "winget list --id DominikReichl.KeePass",
            "Free password manager"
        ).with_repair("winget install DominikReichl.KeePass --force --silent"),

        Software::new(
            "Bitwarden",
            "winget install Bitwarden.Bitwarden --silent --accept-package-agreements --accept-source-agreements",
            "choco install bitwarden -y",
            "winget uninstall Bitwarden.Bitwarden --silent",
            "winget list --id Bitwarden.Bitwarden",
            "Open-source password manager"
        ).with_repair("winget install Bitwarden.Bitwarden --force --silent"),

        Software::new(
            "VeraCrypt",
            "winget install IDRIX.VeraCrypt --silent --accept-package-agreements --accept-source-agreements",
            "choco install veracrypt -y",
            "winget uninstall IDRIX.VeraCrypt --silent",
            "winget list --id IDRIX.VeraCrypt",
            "Disk encryption software"
        ).with_repair("winget install IDRIX.VeraCrypt --force --silent"),

        Software::new(
            "NordVPN",
            "winget install NordVPN.NordVPN --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall NordVPN.NordVPN --silent",
            "winget list --id NordVPN.NordVPN",
            "VPN service client"
        ).with_repair("winget install NordVPN.NordVPN --force --silent"),

        Software::new(
            "OpenVPN Connect",
            "winget install OpenVPNTechnologies.OpenVPNConnect --silent --accept-package-agreements --accept-source-agreements",
            "choco install openvpn -y",
            "winget uninstall OpenVPNTechnologies.OpenVPNConnect --silent",
            "winget list --id OpenVPNTechnologies.OpenVPNConnect",
            "Open-source VPN client"
        ).with_repair("winget install OpenVPNTechnologies.OpenVPNConnect --force --silent"),

        Software::new(
            "Malwarebytes",
            "winget install Malwarebytes.Malwarebytes --silent --accept-package-agreements --accept-source-agreements",
            "choco install malwarebytes -y",
            "winget uninstall Malwarebytes.Malwarebytes --silent",
            "winget list --id Malwarebytes.Malwarebytes",
            "Anti-malware protection"
        ).with_repair("winget install Malwarebytes.Malwarebytes --force --silent"),

        Software::new(
            "GlassWire",
            "winget install SecondGuard.GlassWire --silent --accept-package-agreements --accept-source-agreements",
            "choco install glasswire -y",
            "winget uninstall SecondGuard.GlassWire --silent",
            "winget list --id SecondGuard.GlassWire",
            "Network security monitoring and firewall"
        ).with_repair("winget install SecondGuard.GlassWire --force --silent"),

        Software::new(
            "Gpg4win",
            "winget install GnuPG.Gpg4win --silent --accept-package-agreements --accept-source-agreements",
            "choco install gpg4win -y",
            "winget uninstall GnuPG.Gpg4win --silent",
            "winget list --id GnuPG.Gpg4win",
            "GNU Privacy Guard for Windows"
        ).with_repair("winget install GnuPG.Gpg4win --force --silent"),

        // === ADDITIONAL UTILITIES ===
        Software::new(
            "Notepad++",
            "winget install Notepad++.Notepad++ --silent --accept-package-agreements --accept-source-agreements",
            "choco install notepadplusplus -y",
            "winget uninstall Notepad++.Notepad++ --silent",
            "winget list --id Notepad++.Notepad++",
            "Advanced text editor with syntax highlighting"
        ).with_repair("winget install Notepad++.Notepad++ --force --silent"),

        Software::new(
            "Process Monitor",
            "winget install Microsoft.Sysinternals.ProcessMonitor --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall Microsoft.Sysinternals.ProcessMonitor --silent",
            "winget list --id Microsoft.Sysinternals.ProcessMonitor",
            "Real-time file system and registry monitoring"
        ).with_repair("winget install Microsoft.Sysinternals.ProcessMonitor --force --silent"),
    ];

    for utility in utilities {
        category.add_software(utility);
    }

    category
}
