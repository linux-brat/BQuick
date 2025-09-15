use super::{Category, Software};

pub fn get_media_category() -> Category {
    let mut category = Category::new("Media & Entertainment", "[MED]");

    let media_tools = vec![
        // Video & Screen Recording
        Software::new(
            "OBS Studio",
            "winget install OBSProject.OBSStudio --silent --accept-package-agreements --accept-source-agreements",
            "choco install obs-studio -y",
            "winget uninstall OBSProject.OBSStudio --silent",
            "winget list --id OBSProject.OBSStudio",
            "Live streaming and recording software"
        ).with_repair("winget install OBSProject.OBSStudio --force --silent"),

        Software::new(
            "Shotcut",
            "winget install Meltytech.Shotcut --silent --accept-package-agreements --accept-source-agreements",
            "choco install shotcut -y",
            "winget uninstall Meltytech.Shotcut --silent",
            "winget list --id Meltytech.Shotcut",
            "Free, open-source video editor"
        ).with_repair("winget install Meltytech.Shotcut --force --silent"),

        Software::new(
            "OpenShot",
            "winget install OpenShot.OpenShot --silent --accept-package-agreements --accept-source-agreements",
            "choco install openshot -y",
            "winget uninstall OpenShot.OpenShot --silent",
            "winget list --id OpenShot.OpenShot",
            "User-friendly video editor"
        ).with_repair("winget install OpenShot.OpenShot --force --silent"),

        Software::new(
            "VSDC Free Video Editor",
            "winget install FlashIntegro.VSDCFreeVideoEditor --silent --accept-package-agreements --accept-source-agreements",
            "choco install vsdc-free-video-editor -y",
            "winget uninstall FlashIntegro.VSDCFreeVideoEditor --silent",
            "winget list --id FlashIntegro.VSDCFreeVideoEditor",
            "Non-linear video editing software"
        ).with_repair("winget install FlashIntegro.VSDCFreeVideoEditor --force --silent"),

        Software::new(
            "DaVinci Resolve",
            "winget install Blackmagic.DaVinciResolve --silent --accept-package-agreements --accept-source-agreements",
            "choco install davinci-resolve -y",
            "winget uninstall Blackmagic.DaVinciResolve --silent",
            "winget list --id Blackmagic.DaVinciResolve",
            "Professional video editing and color grading"
        ).with_repair("winget install Blackmagic.DaVinciResolve --force --silent"),

        Software::new(
            "CamStudio",
            "winget install CamStudio.CamStudio --silent --accept-package-agreements --accept-source-agreements",
            "choco install camstudio -y",
            "winget uninstall CamStudio.CamStudio --silent",
            "winget list --id CamStudio.CamStudio",
            "Lightweight screen recording software"
        ).with_repair("winget install CamStudio.CamStudio --force --silent"),

        Software::new(
            "ShareX",
            "winget install ShareX.ShareX --silent --accept-package-agreements --accept-source-agreements",
            "choco install sharex -y",
            "winget uninstall ShareX.ShareX --silent",
            "winget list --id ShareX.ShareX",
            "Screen capture and sharing tool"
        ).with_repair("winget install ShareX.ShareX --force --silent"),

        Software::new(
            "Bandicam",
            "winget install Bandicam.Bandicam --silent --accept-package-agreements --accept-source-agreements",
            "choco install bandicam -y",
            "winget uninstall Bandicam.Bandicam --silent",
            "winget list --id Bandicam.Bandicam",
            "High-quality screen recording software"
        ).with_repair("winget install Bandicam.Bandicam --force --silent"),

        Software::new(
            "Lightworks",
            "winget install LWKS.Lightworks --silent --accept-package-agreements --accept-source-agreements",
            "choco install lightworks -y",
            "winget uninstall LWKS.Lightworks --silent",
            "winget list --id LWKS.Lightworks",
            "Professional video editing software"
        ).with_repair("winget install LWKS.Lightworks --force --silent"),

        Software::new(
            "ScreenToGif",
            "winget install NickeManarin.ScreenToGif --silent --accept-package-agreements --accept-source-agreements",
            "choco install screentogif -y",
            "winget uninstall NickeManarin.ScreenToGif --silent",
            "winget list --id NickeManarin.ScreenToGif",
            "Screen recorder and GIF creator"
        ).with_repair("winget install NickeManarin.ScreenToGif --force --silent"),

        // Audio Editing & Production
        Software::new(
            "Audacity",
            "winget install Audacity.Audacity --silent --accept-package-agreements --accept-source-agreements",
            "choco install audacity -y",
            "winget uninstall Audacity.Audacity --silent",
            "winget list --id Audacity.Audacity",
            "Open source audio editor"
        ).with_repair("winget install Audacity.Audacity --force --silent"),

        Software::new(
            "Voicemeeter Banana",
            "winget install VB-Audio.Voicemeeter --silent --accept-package-agreements --accept-source-agreements",
            "choco install voicemeeter-banana -y",
            "winget uninstall VB-Audio.Voicemeeter --silent",
            "winget list --id VB-Audio.Voicemeeter",
            "Virtual audio mixer"
        ).with_repair("winget install VB-Audio.Voicemeeter --force --silent"),

        Software::new(
            "Reaper",
            "winget install Cockos.REAPER --silent --accept-package-agreements --accept-source-agreements",
            "choco install reaper -y",
            "winget uninstall Cockos.REAPER --silent",
            "winget list --id Cockos.REAPER",
            "Digital audio workstation"
        ).with_repair("winget install Cockos.REAPER --force --silent"),

        Software::new(
            "Ocenaudio",
            "winget install Ocenaudio.Ocenaudio --silent --accept-package-agreements --accept-source-agreements",
            "choco install ocenaudio -y",
            "winget uninstall Ocenaudio.Ocenaudio --silent",
            "winget list --id Ocenaudio.Ocenaudio",
            "User-friendly audio editor"
        ).with_repair("winget install Ocenaudio.Ocenaudio --force --silent"),

        Software::new(
            "LMMS",
            "winget install LMMS.LMMS --silent --accept-package-agreements --accept-source-agreements",
            "choco install lmms -y",
            "winget uninstall LMMS.LMMS --silent",
            "winget list --id LMMS.LMMS",
            "Music production software"
        ).with_repair("winget install LMMS.LMMS --force --silent"),

        Software::new(
            "FL Studio",
            "winget install ImageLine.FLStudio --silent --accept-package-agreements --accept-source-agreements",
            "choco install fl-studio -y",
            "winget uninstall ImageLine.FLStudio --silent",
            "winget list --id ImageLine.FLStudio",
            "Popular digital audio workstation"
        ).with_repair("winget install ImageLine.FLStudio --force --silent"),

        // Image Editing & Design
        Software::new(
            "GIMP",
            "winget install GIMP.GIMP --silent --accept-package-agreements --accept-source-agreements",
            "choco install gimp -y",
            "winget uninstall GIMP.GIMP --silent",
            "winget list --id GIMP.GIMP",
            "GNU Image Manipulation Program"
        ).with_repair("winget install GIMP.GIMP --force --silent"),

        Software::new(
            "Paint.NET",
            "winget install dotPDN.PaintDotNet --silent --accept-package-agreements --accept-source-agreements",
            "choco install paint.net -y",
            "winget uninstall dotPDN.PaintDotNet --silent",
            "winget list --id dotPDN.PaintDotNet",
            "Simple yet powerful image editor"
        ).with_repair("winget install dotPDN.PaintDotNet --force --silent"),

        Software::new(
            "Krita",
            "winget install KDE.Krita --silent --accept-package-agreements --accept-source-agreements",
            "choco install krita -y",
            "winget uninstall KDE.Krita --silent",
            "winget list --id KDE.Krita",
            "Digital painting application"
        ).with_repair("winget install KDE.Krita --force --silent"),

        Software::new(
            "Inkscape",
            "winget install Inkscape.Inkscape --silent --accept-package-agreements --accept-source-agreements",
            "choco install inkscape -y",
            "winget uninstall Inkscape.Inkscape --silent",
            "winget list --id Inkscape.Inkscape",
            "Vector graphics editor"
        ).with_repair("winget install Inkscape.Inkscape --force --silent"),

        Software::new(
            "Affinity Photo",
            "winget install Serif.AffinityPhoto --silent --accept-package-agreements --accept-source-agreements",
            "choco install affinity-photo -y",
            "winget uninstall Serif.AffinityPhoto --silent",
            "winget list --id Serif.AffinityPhoto",
            "Professional photo editing software"
        ).with_repair("winget install Serif.AffinityPhoto --force --silent"),

        Software::new(
            "IrfanView",
            "winget install IrfanSkiljan.IrfanView --silent --accept-package-agreements --accept-source-agreements",
            "choco install irfanview -y",
            "winget uninstall IrfanSkiljan.IrfanView --silent",
            "winget list --id IrfanSkiljan.IrfanView",
            "Fast and compact image viewer"
        ).with_repair("winget install IrfanSkiljan.IrfanView --force --silent"),

        Software::new(
            "XnView",
            "winget install XnSoft.XnViewMP --silent --accept-package-agreements --accept-source-agreements",
            "choco install xnviewmp -y",
            "winget uninstall XnSoft.XnViewMP --silent",
            "winget list --id XnSoft.XnViewMP",
            "Photo viewer and organizer"
        ).with_repair("winget install XnSoft.XnViewMP --force --silent"),

        Software::new(
            "FastStone Image Viewer",
            "winget install FastStone.ImageViewer --silent --accept-package-agreements --accept-source-agreements",
            "choco install faststone -y",
            "winget uninstall FastStone.ImageViewer --silent",
            "winget list --id FastStone.ImageViewer",
            "Image viewer with editing capabilities"
        ).with_repair("winget install FastStone.ImageViewer --force --silent"),

        // Media Players & Utilities
        Software::new(
            "VLC Media Player",
            "winget install VideoLAN.VLC --silent --accept-package-agreements --accept-source-agreements",
            "choco install vlc -y",
            "winget uninstall VideoLAN.VLC --silent",
            "winget list --id VideoLAN.VLC",
            "Versatile media player"
        ).with_repair("winget install VideoLAN.VLC --force --silent"),

        Software::new(
            "MPV Player",
            "winget install mpv.net --silent --accept-package-agreements --accept-source-agreements",
            "choco install mpv -y",
            "winget uninstall mpv.net --silent",
            "winget list --id mpv.net",
            "Minimalist media player"
        ).with_repair("winget install mpv.net --force --silent"),

        Software::new(
            "PotPlayer",
            "winget install Daum.PotPlayer --silent --accept-package-agreements --accept-source-agreements",
            "choco install potplayer -y",
            "winget uninstall Daum.PotPlayer --silent",
            "winget list --id Daum.PotPlayer",
            "Feature-rich media player"
        ).with_repair("winget install Daum.PotPlayer --force --silent"),

        Software::new(
            "KMPlayer",
            "winget install KMPlayer.KMPlayer --silent --accept-package-agreements --accept-source-agreements",
            "choco install kmplayer -y",
            "winget uninstall KMPlayer.KMPlayer --silent",
            "winget list --id KMPlayer.KMPlayer",
            "Multimedia player with codec support"
        ).with_repair("winget install KMPlayer.KMPlayer --force --silent"),

        Software::new(
            "Media Player Classic",
            "winget install clsid2.mpc-hc --silent --accept-package-agreements --accept-source-agreements",
            "choco install mpc-hc -y",
            "winget uninstall clsid2.mpc-hc --silent",
            "winget list --id clsid2.mpc-hc",
            "Lightweight media player"
        ).with_repair("winget install clsid2.mpc-hc --force --silent"),

        Software::new(
            "K-Lite Codec Pack",
            "winget install CodecGuide.K-LiteCodecPack.Full --silent --accept-package-agreements --accept-source-agreements",
            "choco install k-litecodecpack-full -y",
            "winget uninstall CodecGuide.K-LiteCodecPack.Full --silent",
            "winget list --id CodecGuide.K-LiteCodecPack.Full",
            "Comprehensive codec pack"
        ).with_repair("winget install CodecGuide.K-LiteCodecPack.Full --force --silent"),

        Software::new(
            "Plex",
            "winget install Plex.PlexMediaServer --silent --accept-package-agreements --accept-source-agreements",
            "choco install plex -y",
            "winget uninstall Plex.PlexMediaServer --silent",
            "winget list --id Plex.PlexMediaServer",
            "Media server and streaming platform"
        ).with_repair("winget install Plex.PlexMediaServer --force --silent"),

        Software::new(
            "Kodi",
            "winget install XBMCFoundation.Kodi --silent --accept-package-agreements --accept-source-agreements",
            "choco install kodi -y",
            "winget uninstall XBMCFoundation.Kodi --silent",
            "winget list --id XBMCFoundation.Kodi",
            "Open source media center"
        ).with_repair("winget install XBMCFoundation.Kodi --force --silent"),

        Software::new(
            "Foobar2000",
            "winget install PeterPawlowski.foobar2000 --silent --accept-package-agreements --accept-source-agreements",
            "choco install foobar2000 -y",
            "winget uninstall PeterPawlowski.foobar2000 --silent",
            "winget list --id PeterPawlowski.foobar2000",
            "Advanced audio player"
        ).with_repair("winget install PeterPawlowski.foobar2000 --force --silent"),

        Software::new(
            "iTunes",
            "winget install Apple.iTunes --silent --accept-package-agreements --accept-source-agreements",
            "choco install itunes -y",
            "winget uninstall Apple.iTunes --silent",
            "winget list --id Apple.iTunes",
            "Apple media player and library"
        ).with_repair("winget install Apple.iTunes --force --silent"),

        // Media Conversion & Processing
        Software::new(
            "FFmpeg",
            "winget install Gyan.FFmpeg --silent --accept-package-agreements --accept-source-agreements",
            "choco install ffmpeg -y",
            "winget uninstall Gyan.FFmpeg --silent",
            "winget list --id Gyan.FFmpeg",
            "Complete multimedia framework"
        ).with_repair("winget install Gyan.FFmpeg --force --silent"),

        Software::new(
            "HandBrake",
            "winget install HandBrake.HandBrake --silent --accept-package-agreements --accept-source-agreements",
            "choco install handbrake -y",
            "winget uninstall HandBrake.HandBrake --silent",
            "winget list --id HandBrake.HandBrake",
            "Open source video transcoder"
        ).with_repair("winget install HandBrake.HandBrake --force --silent"),

        Software::new(
            "XMedia Recode",
            "winget install SebastianDorfler.XMediaRecode --silent --accept-package-agreements --accept-source-agreements",
            "choco install xmedia-recode -y",
            "winget uninstall SebastianDorfler.XMediaRecode --silent",
            "winget list --id SebastianDorfler.XMediaRecode",
            "Universal media converter"
        ).with_repair("winget install SebastianDorfler.XMediaRecode --force --silent"),

        Software::new(
            "Format Factory",
            "winget install PCFreetime.FormatFactory --silent --accept-package-agreements --accept-source-agreements",
            "choco install formatfactory -y",
            "winget uninstall PCFreetime.FormatFactory --silent",
            "winget list --id PCFreetime.FormatFactory",
            "Multimedia converter"
        ).with_repair("winget install PCFreetime.FormatFactory --force --silent"),

        Software::new(
            "MakeMKV",
            "winget install GuinpinSoft.MakeMKV --silent --accept-package-agreements --accept-source-agreements",
            "choco install makemkv -y",
            "winget uninstall GuinpinSoft.MakeMKV --silent",
            "winget list --id GuinpinSoft.MakeMKV",
            "DVD and Blu-ray ripping tool"
        ).with_repair("winget install GuinpinSoft.MakeMKV --force --silent"),

        Software::new(
            "DVDStyler",
            "winget install DVDStyler.DVDStyler --silent --accept-package-agreements --accept-source-agreements",
            "choco install dvdstyler -y",
            "winget uninstall DVDStyler.DVDStyler --silent",
            "winget list --id DVDStyler.DVDStyler",
            "DVD authoring application"
        ).with_repair("winget install DVDStyler.DVDStyler --force --silent"),

        Software::new(
            "MKVToolNix",
            "winget install MoritzBunkus.MKVToolNix --silent --accept-package-agreements --accept-source-agreements",
            "choco install mkvtoolnix -y",
            "winget uninstall MoritzBunkus.MKVToolNix --silent",
            "winget list --id MoritzBunkus.MKVToolNix",
            "Set of tools for MKV files"
        ).with_repair("winget install MoritzBunkus.MKVToolNix --force --silent"),

        Software::new(
            "Avidemux",
            "winget install Avidemux.Avidemux --silent --accept-package-agreements --accept-source-agreements",
            "choco install avidemux -y",
            "winget uninstall Avidemux.Avidemux --silent",
            "winget list --id Avidemux.Avidemux",
            "Video editing and processing tool"
        ).with_repair("winget install Avidemux.Avidemux --force --silent"),

        Software::new(
            "Any Video Converter",
            "winget install Anvsoft.AnyVideoConverter --silent --accept-package-agreements --accept-source-agreements",
            "choco install any-video-converter -y",
            "winget uninstall Anvsoft.AnyVideoConverter --silent",
            "winget list --id Anvsoft.AnyVideoConverter",
            "Video converter and editor"
        ).with_repair("winget install Anvsoft.AnyVideoConverter --force --silent"),

        Software::new(
            "Freemake Video Converter",
            "winget install Freemake.VideoConverter --silent --accept-package-agreements --accept-source-agreements",
            "choco install freemake-video-converter -y",
            "winget uninstall Freemake.VideoConverter --silent",
            "winget list --id Freemake.VideoConverter",
            "Free video conversion software"
        ).with_repair("winget install Freemake.VideoConverter --force --silent"),

        // Streaming & Virtual Camera
        Software::new(
            "Streamlabs",
            "winget install Streamlabs.Streamlabs --silent --accept-package-agreements --accept-source-agreements",
            "choco install streamlabs-obs -y",
            "winget uninstall Streamlabs.Streamlabs --silent",
            "winget list --id Streamlabs.Streamlabs",
            "Live streaming software with overlays"
        ).with_repair("winget install Streamlabs.Streamlabs --force --silent"),

        Software::new(
            "NDI Tools",
            "winget install NewTek.NDITools --silent --accept-package-agreements --accept-source-agreements",
            "choco install ndi-tools -y",
            "winget uninstall NewTek.NDITools --silent",
            "winget list --id NewTek.NDITools",
            "Network Device Interface tools"
        ).with_repair("winget install NewTek.NDITools --force --silent"),

        // Other Media Apps
        Software::new(
            "Spotify",
            "winget install Spotify.Spotify --silent --accept-package-agreements --accept-source-agreements",
            "choco install spotify -y",
            "winget uninstall Spotify.Spotify --silent",
            "winget list --id Spotify.Spotify",
            "Music streaming service"
        ).with_repair("winget install Spotify.Spotify --force --silent"),

        Software::new(
            "Discord",
            "winget install Discord.Discord --silent --accept-package-agreements --accept-source-agreements",
            "choco install discord -y",
            "winget uninstall Discord.Discord --silent",
            "winget list --id Discord.Discord",
            "Voice and text chat for gamers"
        ).with_repair("winget install Discord.Discord --force --silent"),

        Software::new(
            "Vesktop",
            "winget install Vencord.Vesktop --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall Vencord.Vesktop --silent",
            "winget list --id Vencord.Vesktop",
            "Lightweight Discord client with Vencord pre-installed"
        ).with_repair("winget install Vencord.Vesktop --force --silent"),
    ];

    for tool in media_tools {
        category.add_software(tool);
    }

    category
}
