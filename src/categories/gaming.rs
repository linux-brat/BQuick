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
            "Ubisoft Connect",
            "winget install Ubisoft.Connect --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall Ubisoft.Connect --silent",
            "winget list --id Ubisoft.Connect",
            "Ubisoft's official game launcher and DRM"
        ),

        Software::new(
            "EA App",
            "winget install ElectronicArts.EADesktop --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall ElectronicArts.EADesktop --silent",
            "winget list --id ElectronicArts.EADesktop",
            "EA's digital distribution platform"
        ),

        Software::new(
            "Battle.net",
            "winget install Blizzard.BattleNet --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall Blizzard.BattleNet --silent",
            "winget list --id Blizzard.BattleNet",
            "Blizzard and Activision launcher"
        ),

        Software::new(
            "GOG Galaxy",
            "winget install GOG.Galaxy --silent --accept-package-agreements --accept-source-agreements",
            "choco install goggalaxy -y",
            "winget uninstall GOG.Galaxy --silent",
            "winget list --id GOG.Galaxy",
            "DRM-free games launcher and manager"
        ),

        Software::new(
            "Itch.io App",
            "winget install ItchIo.Itch --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall ItchIo.Itch --silent",
            "winget list --id ItchIo.Itch",
            "Indie games launcher and downloader"
        ),

        Software::new(
            "Xbox App",
            "winget install Microsoft.XboxApp --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall Microsoft.XboxApp --silent",
            "winget list --id Microsoft.XboxApp",
            "Xbox Game Pass and Microsoft games launcher"
        ),

        Software::new(
            "Playnite",
            "winget install Playnite.Playnite --silent --accept-package-agreements --accept-source-agreements",
            "choco install playnite -y",
            "winget uninstall Playnite.Playnite --silent",
            "winget list --id Playnite.Playnite",
            "Unified game launcher and library manager"
        ),

        Software::new(
            "Lutris",
            "winget install Lutris.Lutris --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall Lutris.Lutris --silent",
            "winget list --id Lutris.Lutris",
            "Open-source gaming library manager"
        ),

        Software::new(
            "RetroArch",
            "winget install Libretro.RetroArch --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall Libretro.RetroArch --silent",
            "winget list --id Libretro.RetroArch",
            "Emulator frontend for retro gaming"
        ),

        Software::new(
            "Cemu",
            "winget install Cemu.Cemu --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall Cemu.Cemu --silent",
            "winget list --id Cemu.Cemu",
            "Wii U emulator"
        ),

        Software::new(
            "yuzu",
            "winget install yuzu-emu.yuzu --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall yuzu-emu.yuzu --silent",
            "winget list --id yuzu-emu.yuzu",
            "Nintendo Switch emulator"
        ),

        Software::new(
            "Ryujinx",
            "winget install Ryujinx.Ryujinx --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall Ryujinx.Ryujinx --silent",
            "winget list --id Ryujinx.Ryujinx",
            "Nintendo Switch emulator alternative"
        ),

        Software::new(
            "Dolphin Emulator",
            "winget install DolphinEmulator.Dolphin --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall DolphinEmulator.Dolphin --silent",
            "winget list --id DolphinEmulator.Dolphin",
            "GameCube and Wii emulator"
        ),

        Software::new(
            "PCSX2",
            "winget install PCSX2Team.PCSX2 --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall PCSX2Team.PCSX2 --silent",
            "winget list --id PCSX2Team.PCSX2",
            "PlayStation 2 emulator"
        ),

        Software::new(
            "RPCS3",
            "winget install RPCS3.RPCS3 --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall RPCS3.RPCS3 --silent",
            "winget list --id RPCS3.RPCS3",
            "PlayStation 3 emulator"
        ),

        Software::new(
            "PPSSPP",
            "winget install PPSSPPTeam.PPSSPP --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall PPSSPPTeam.PPSSPP --silent",
            "winget list --id PPSSPPTeam.PPSSPP",
            "PSP emulator"
        ),

        Software::new(
            "Project64",
            "winget install Project64.Project64 --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall Project64.Project64 --silent",
            "winget list --id Project64.Project64",
            "Nintendo 64 emulator"
        ),

        Software::new(
            "MAME",
            "winget install MAMEdev.MAME --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall MAMEdev.MAME --silent",
            "winget list --id MAMEdev.MAME",
            "Arcade game emulator"
        ),

        Software::new(
            "DOSBox",
            "winget install DOSBox.DOSBox --silent --accept-package-agreements --accept-source-agreements",
            "choco install dosbox -y",
            "winget uninstall DOSBox.DOSBox --silent",
            "winget list --id DOSBox.DOSBox",
            "Classic DOS emulator"
        ),

        Software::new(
            "ScummVM",
            "winget install ScummVM.ScummVM --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall ScummVM.ScummVM --silent",
            "winget list --id ScummVM.ScummVM",
            "Classic point-and-click adventure engine"
        ),

        Software::new(
            "VisualBoyAdvance-M",
            "winget install VisualBoyAdvance-M.Vbam --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall VisualBoyAdvance-M.Vbam --silent",
            "winget list --id VisualBoyAdvance-M.Vbam",
            "Game Boy Advance emulator"
        ),

        Software::new(
            "Nestopia",
            "winget install Nestopia.NestopiaUE --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall Nestopia.NestopiaUE --silent",
            "winget list --id Nestopia.NestopiaUE",
            "NES emulator"
        ),

        Software::new(
            "Snes9x",
            "winget install Snes9x.Snes9x --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall Snes9x.Snes9x --silent",
            "winget list --id Snes9x.Snes9x",
            "Super Nintendo emulator"
        ),

        Software::new(
            "MelonDS",
            "winget install MelonDS.MelonDS --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall MelonDS.MelonDS --silent",
            "winget list --id MelonDS.MelonDS",
            "Nintendo DS emulator"
        ),

        Software::new(
            "Citra",
            "winget install Citra.Citra --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall Citra.Citra --silent",
            "winget list --id Citra.Citra",
            "Nintendo 3DS emulator"
        ),

        Software::new(
            "BlueStacks",
            "winget install BlueStacks.BlueStacks --silent --accept-package-agreements --accept-source-agreements",
            "choco install bluestacks -y",
            "winget uninstall BlueStacks.BlueStacks --silent",
            "winget list --id BlueStacks.BlueStacks",
            "Android gaming emulator"
        ),

        Software::new(
            "LDPlayer",
            "winget install LDPlayer.LDPlayer --silent --accept-package-agreements --accept-source-agreements",
            "choco install ldplayer -y",
            "winget uninstall LDPlayer.LDPlayer --silent",
            "winget list --id LDPlayer.LDPlayer",
            "Android emulator optimized for gaming"
        ),

        Software::new(
            "NoxPlayer",
            "winget install Nox.NoxPlayer --silent --accept-package-agreements --accept-source-agreements",
            "choco install nox -y",
            "winget uninstall Nox.NoxPlayer --silent",
            "winget list --id Nox.NoxPlayer",
            "Android emulator for Windows"
        ),

        Software::new(
            "MEmu Play",
            "winget install MEmu.MEmu --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall MEmu.MEmu --silent",
            "winget list --id MEmu.MEmu",
            "Android emulator optimized for gaming"
        ),

        Software::new(
            "Gameloop",
            "winget install Tencent.Gameloop --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall Tencent.Gameloop --silent",
            "winget list --id Tencent.Gameloop",
            "Android emulator optimized for PUBG & COD"
        ),

        Software::new(
            "PCSX-Redux",
            "winget install PCSX-Redux.PCSX-Redux --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall PCSX-Redux.PCSX-Redux --silent",
            "winget list --id PCSX-Redux.PCSX-Redux",
            "PlayStation 1 emulator"
        ),

        Software::new(
            "OpenTTD",
            "winget install OpenTTD.OpenTTD --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall OpenTTD.OpenTTD --silent",
            "winget list --id OpenTTD.OpenTTD",
            "Open-source Transport Tycoon Deluxe game"
        ),

        Software::new(
            "Minetest",
            "winget install Minetest.Minetest --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall Minetest.Minetest --silent",
            "winget list --id Minetest.Minetest",
            "Open-source Minecraft-like game"
        ),

        Software::new(
            "0 A.D.",
            "winget install WildfireGames.0AD --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall WildfireGames.0AD --silent",
            "winget list --id WildfireGames.0AD",
            "Open-source strategy game"
        ),

        Software::new(
            "SuperTuxKart",
            "winget install SuperTuxKart.SuperTuxKart --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall SuperTuxKart.SuperTuxKart --silent",
            "winget list --id SuperTuxKart.SuperTuxKart",
            "Open-source racing game"
        ),

        Software::new(
            "Hedgewars",
            "winget install Hedgewars.Hedgewars --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall Hedgewars.Hedgewars --silent",
            "winget list --id Hedgewars.Hedgewars",
            "Worms-like artillery game"
        ),

        Software::new(
            "Xonotic",
            "winget install Xonotic.Xonotic --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall Xonotic.Xonotic --silent",
            "winget list --id Xonotic.Xonotic",
            "Free and open-source FPS"
        ),

        Software::new(
            "QuakeSpasm",
            "winget install QuakeSpasm.QuakeSpasm --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall QuakeSpasm.QuakeSpasm --silent",
            "winget list --id QuakeSpasm.QuakeSpasm",
            "Quake engine port"
        ),

        Software::new(
            "ioquake3",
            "winget install Ioquake3.Ioquake3 --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall Ioquake3.Ioquake3 --silent",
            "winget list --id Ioquake3.Ioquake3",
            "Quake III engine port"
        ),

        Software::new(
            "Unreal Tournament",
            "winget install UnrealTournament.UnrealTournament --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall UnrealTournament.UnrealTournament --silent",
            "winget list --id UnrealTournament.UnrealTournament",
            "Classic arena shooter"
        ),

        Software::new(
            "Urban Terror",
            "winget install UrbanTerror.UrbanTerror --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall UrbanTerror.UrbanTerror --silent",
            "winget list --id UrbanTerror.UrbanTerror",
            "Free multiplayer FPS"
        ),

        Software::new(
            "Warsow",
            "winget install Warsow.Warsow --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall Warsow.Warsow --silent",
            "winget list --id Warsow.Warsow",
            "Fast-paced FPS"
        ),

        Software::new(
            "Red Eclipse",
            "winget install RedEclipse.RedEclipse --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall RedEclipse.RedEclipse --silent",
            "winget list --id RedEclipse.RedEclipse",
            "Arena shooter"
        ),

        Software::new(
            "Tremulous",
            "winget install Tremulous.Tremulous --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall Tremulous.Tremulous --silent",
            "winget list --id Tremulous.Tremulous",
            "Aliens vs Humans FPS/RTS"
        ),

        Software::new(
            "OpenArena",
            "winget install OpenArena.OpenArena --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall OpenArena.OpenArena --silent",
            "winget list --id OpenArena.OpenArena",
            "Quake III-based open-source shooter"
        ),

        Software::new(
            "Battle for Wesnoth",
            "winget install Wesnoth.Wesnoth --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall Wesnoth.Wesnoth --silent",
            "winget list --id Wesnoth.Wesnoth",
            "Open-source turn-based strategy game"
        ),

        Software::new(
            "FreeCiv",
            "winget install Freeciv.Freeciv --silent --accept-package-agreements --accept-source-agreements",
            "choco install freeciv -y",
            "winget uninstall Freeciv.Freeciv --silent",
            "winget list --id Freeciv.Freeciv",
            "Civilization-style open-source strategy"
        ),

        Software::new(
            "Warzone 2100",
            "winget install Warzone2100.Warzone2100 --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall Warzone2100.Warzone2100 --silent",
            "winget list --id Warzone2100.Warzone2100",
            "Open-source real-time strategy"
        ),

        Software::new(
            "Zero-K",
            "winget install ZeroK.ZeroK --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall ZeroK.ZeroK --silent",
            "winget list --id ZeroK.ZeroK",
            "Free real-time strategy"
        ),

        Software::new(
            "Spring RTS",
            "winget install SpringRTS.SpringRTS --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall SpringRTS.SpringRTS --silent",
            "winget list --id SpringRTS.SpringRTS",
            "Real-time strategy engine"
        ),

        Software::new(
            "MegaGlest",
            "winget install MegaGlest.MegaGlest --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall MegaGlest.MegaGlest --silent",
            "winget list --id MegaGlest.MegaGlest",
            "Free 3D real-time strategy"
        ),
    ];

    for software in gaming_software {
        category.add_software(software);
    }

    category
}
