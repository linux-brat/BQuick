use super::{Category, Software};

pub fn get_gaming_category() -> Category {
    let mut category = Category::new("Gaming", "[GAME]");

    let gaming_software = vec![
        Software::new(
            "Steam",
            "winget install Valve.Steam --silent --accept-package-agreements --accept-source-agreements",
            "choco install steam -y",
            "winget uninstall Valve.Steam --silent",
            "winget list --id Valve.Steam",
            "Gaming platform and digital distribution service"
        ).with_repair("winget install Valve.Steam --force --silent"),

        Software::new(
            "Epic Games Launcher",
            "winget install EpicGames.EpicGamesLauncher --silent --accept-package-agreements --accept-source-agreements",
            "choco install epicgameslauncher -y",
            "winget uninstall EpicGames.EpicGamesLauncher --silent",
            "winget list --id EpicGames.EpicGamesLauncher",
            "Epic Games digital distribution platform"
        ).with_repair("winget install EpicGames.EpicGamesLauncher --force --silent"),

        Software::new(
            "Discord",
            "winget install Discord.Discord --silent --accept-package-agreements --accept-source-agreements",
            "choco install discord -y",
            "winget uninstall Discord.Discord --silent",
            "winget list --id Discord.Discord",
            "Voice and text chat for gamers"
        ).with_repair("winget install Discord.Discord --force --silent"),
    ];

    for software in gaming_software {
        category.add_software(software);
    }

    category
}
