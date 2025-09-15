use super::{Category, Software};

pub fn get_browsers_category() -> Category {
    let mut category = Category::new("Web Browsers", "[WEB]");

    let browsers = vec![
        Software::new(
            "Google Chrome",
            "winget install Google.Chrome --silent --accept-package-agreements --accept-source-agreements",
            "choco install googlechrome -y",
            "winget uninstall Google.Chrome --silent",
            "winget list --id Google.Chrome",
            "Popular web browser by Google"
        ).with_repair("winget install Google.Chrome --force --silent"),

        Software::new(
            "Mozilla Firefox",
            "winget install Mozilla.Firefox --silent --accept-package-agreements --accept-source-agreements",
            "choco install firefox -y",
            "winget uninstall Mozilla.Firefox --silent",
            "winget list --id Mozilla.Firefox",
            "Open-source web browser"
        ).with_repair("winget install Mozilla.Firefox --force --silent"),

        Software::new(
            "Microsoft Edge",
            "winget install Microsoft.Edge --silent --accept-package-agreements --accept-source-agreements",
            "choco install microsoft-edge -y",
            "winget uninstall Microsoft.Edge --silent",
            "winget list --id Microsoft.Edge",
            "Microsoft's modern web browser"
        ).with_repair("winget install Microsoft.Edge --force --silent"),

        Software::new(
            "Brave Browser",
            "winget install Brave.Brave --silent --accept-package-agreements --accept-source-agreements",
            "choco install brave -y",
            "winget uninstall Brave.Brave --silent",
            "winget list --id Brave.Brave",
            "Privacy-focused web browser"
        ).with_repair("winget install Brave.Brave --force --silent"),

        Software::new(
            "Opera",
            "winget install Opera.Opera --silent --accept-package-agreements --accept-source-agreements",
            "choco install opera -y",
            "winget uninstall Opera.Opera --silent",
            "winget list --id Opera.Opera",
            "Feature-rich web browser"
        ).with_repair("winget install Opera.Opera --force --silent"),

        Software::new(
            "Opera GX",
            "winget install Opera.OperaGX --silent --accept-package-agreements --accept-source-agreements",
            "choco install opera-gx -y",
            "winget uninstall Opera.OperaGX --silent",
            "winget list --id Opera.OperaGX",
            "Gaming-focused web browser"
        ).with_repair("winget install Opera.OperaGX --force --silent"),

        Software::new(
            "Vivaldi",
            "winget install VivaldiTechnologies.Vivaldi --silent --accept-package-agreements --accept-source-agreements",
            "choco install vivaldi -y",
            "winget uninstall VivaldiTechnologies.Vivaldi --silent",
            "winget list --id VivaldiTechnologies.Vivaldi",
            "Highly customizable web browser"
        ).with_repair("winget install VivaldiTechnologies.Vivaldi --force --silent"),

        Software::new(
            "Chromium",
            "winget install Hibbiki.Chromium --silent --accept-package-agreements --accept-source-agreements",
            "choco install chromium -y",
            "winget uninstall Hibbiki.Chromium --silent",
            "winget list --id Hibbiki.Chromium",
            "Open-source web browser project"
        ).with_repair("winget install Hibbiki.Chromium --force --silent"),

        Software::new(
            "Waterfox",
            "winget install Waterfox.Waterfox --silent --accept-package-agreements --accept-source-agreements",
            "choco install waterfox -y",
            "winget uninstall Waterfox.Waterfox --silent",
            "winget list --id Waterfox.Waterfox",
            "Privacy-focused Firefox-based browser"
        ).with_repair("winget install Waterfox.Waterfox --force --silent"),

        Software::new(
            "Pale Moon",
            "winget install MoonchildProductions.PaleMoon --silent --accept-package-agreements --accept-source-agreements",
            "choco install palemoon -y",
            "winget uninstall MoonchildProductions.PaleMoon --silent",
            "winget list --id MoonchildProductions.PaleMoon",
            "Goanna-based web browser"
        ).with_repair("winget install MoonchildProductions.PaleMoon --force --silent"),

        Software::new(
            "Tor Browser",
            "winget install TorProject.TorBrowser --silent --accept-package-agreements --accept-source-agreements",
            "choco install tor-browser -y",
            "winget uninstall TorProject.TorBrowser --silent",
            "winget list --id TorProject.TorBrowser",
            "Privacy and anonymity focused browser"
        ).with_repair("winget install TorProject.TorBrowser --force --silent"),

        Software::new(
            "Maxthon",
            "winget install Maxthon.Maxthon --silent --accept-package-agreements --accept-source-agreements",
            "choco install maxthon -y",
            "winget uninstall Maxthon.Maxthon --silent",
            "winget list --id Maxthon.Maxthon",
            "Cloud-based web browser"
        ).with_repair("winget install Maxthon.Maxthon --force --silent"),

        Software::new(
            "Yandex Browser",
            "winget install Yandex.Browser --silent --accept-package-agreements --accept-source-agreements",
            "choco install yandex-browser -y",
            "winget uninstall Yandex.Browser --silent",
            "winget list --id Yandex.Browser",
            "Russian web browser with integrated services"
        ).with_repair("winget install Yandex.Browser --force --silent"),

        Software::new(
            "Slimjet",
            "winget install FlashPeak.Slimjet --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall FlashPeak.Slimjet --silent",
            "winget list --id FlashPeak.Slimjet",
            "Fast and powerful web browser"
        ).with_repair("winget install FlashPeak.Slimjet --force --silent"),

        Software::new(
            "Firefox Developer Edition",
            "winget install Mozilla.Firefox.DeveloperEdition --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall Mozilla.Firefox.DeveloperEdition --silent",
            "winget list --id Mozilla.Firefox.DeveloperEdition",
            "Firefox with developer tools"
        ).with_repair("winget install Mozilla.Firefox.DeveloperEdition --force --silent"),

        Software::new(
            "Firefox Nightly",
            "winget install Mozilla.Firefox.Nightly --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall Mozilla.Firefox.Nightly --silent",
            "winget list --id Mozilla.Firefox.Nightly",
            "Experimental Firefox builds"
        ).with_repair("winget install Mozilla.Firefox.Nightly --force --silent"),

        Software::new(
            "Brave Nightly",
            "winget install Brave.Brave.Nightly --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall Brave.Brave.Nightly --silent",
            "winget list --id Brave.Brave.Nightly",
            "Experimental Brave builds"
        ).with_repair("winget install Brave.Brave.Nightly --force --silent"),

        Software::new(
            "Falkon",
            "winget install KDE.Falkon --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall KDE.Falkon --silent",
            "winget list --id KDE.Falkon",
            "Lightweight Qt-based browser"
        ).with_repair("winget install KDE.Falkon --force --silent"),

        Software::new(
            "Min Browser",
            "winget install PalmerCluff.Min --silent --accept-package-agreements --accept-source-agreements",
            "",
            "winget uninstall PalmerCluff.Min --silent",
            "winget list --id PalmerCluff.Min",
            "Minimalist web browser"
        ).with_repair("winget install PalmerCluff.Min --force --silent"),
    ];

    for browser in browsers {
        category.add_software(browser);
    }

    category
}
