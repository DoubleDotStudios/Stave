use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Default, Clone)]
pub struct RawConfig {
    pub general: Option<RawGeneral>,
    pub branding: Option<RawBranding>,
    pub colors: Option<RawColors>,
}

#[derive(Deserialize, Default, Clone)]
pub struct RawGeneral {
    pub tabs: Option<Vec<String>>,
    pub tab_spacer: Option<String>,
}

#[derive(Deserialize, Default, Clone)]
pub struct RawBranding {
    pub logo: Option<bool>,
    pub logo_path: Option<String>,
    pub description: Option<String>,
}

#[derive(Deserialize, Default, Clone)]
pub struct RawColors {
    // Border colors
    pub border: Option<String>,
    pub inactive_border: Option<String>,
    pub active_border: Option<String>,

    // Text colors
    pub text: Option<String>,
    pub text_bg: Option<String>,
    pub active_text: Option<String>,
    pub active_text_bg: Option<String>,
    pub inactive_text: Option<String>,
    pub inactive_text_bg: Option<String>,
}

#[derive(Deserialize, Default)]
pub struct Config {
    pub general: General,
    pub branding: Branding,
    pub colors: Colors,
}

#[derive(Deserialize, Default, Clone)]
pub struct General {
    pub tabs: Vec<String>,
    pub tab_spacer: String,
}

#[derive(Deserialize, Default, Clone)]
pub struct Branding {
    pub logo: bool,
    pub logo_path: String,
    pub description: String,
}

#[derive(Deserialize, Default, Clone)]
pub struct Colors {
    // Border colors
    pub border: String,
    pub inactive_border: String,
    pub active_border: String,

    // Text colors
    pub text: String,
    pub text_bg: String,
    pub active_text: String,
    pub active_text_bg: String,
    pub inactive_text: String,
    pub inactive_text_bg: String,
}

impl Config {
    pub fn get() -> Self {
        let config_path = get_path();

        if config_path != "" {
            let raw_config: String =
                fs::read_to_string(config_path).expect("Unable to read config file.");
            let config = toml::from_str::<RawConfig>(&raw_config).unwrap();

            let general = if config.general.is_some() {
                General::get(&config)
            } else {
                General::default()
            };

            let branding = if config.branding.is_some() {
                Branding::get(&config)
            } else {
                Branding::default()
            };

            let colors = if config.colors.is_some() {
                Colors::get(&config)
            } else {
                Colors::default()
            };

            Config {
                branding,
                colors,
                general,
            }
        } else {
            Config::default()
        }
    }

    pub fn default() -> Self {
        Config {
            branding: Branding::default(),
            colors: Colors::default(),
            general: General::default(),
        }
    }
}

impl General {
    fn get(config: &RawConfig) -> Self {
        let def_tabs = vec![
            "Welcome".to_string(),
            "Location".to_string(),
            "Welcome".to_string(),
            "Keyboard".to_string(),
            "Partitions".to_string(),
            "GUI".to_string(),
            "Users".to_string(),
            "Overview".to_string(),
            "Install".to_string(),
            "Finish".to_string(),
        ];

        General {
            tabs: config
                .general
                .clone()
                .unwrap()
                .tabs
                .unwrap_or(def_tabs.clone()),
            tab_spacer: config
                .general
                .clone()
                .unwrap()
                .tab_spacer
                .unwrap_or("•".to_string()),
        }
    }

    fn default() -> Self {
        let def_tabs = vec![
            "Welcome".to_string(),
            "Location".to_string(),
            "Welcome".to_string(),
            "Keyboard".to_string(),
            "Partitions".to_string(),
            "GUI".to_string(),
            "Users".to_string(),
            "Overview".to_string(),
            "Install".to_string(),
            "Finish".to_string(),
        ];

        General {
            tabs: def_tabs.clone(),
            tab_spacer: "•".to_string(),
        }
    }
}

impl Branding {
    fn get(config: &RawConfig) -> Self {
        Branding {
                logo: config.branding.clone().unwrap().logo.unwrap_or(true),
                logo_path: config
                    .branding.clone().unwrap()
                    .logo_path.unwrap_or("".to_string())
                    ,
                description: config.branding.clone().unwrap().description.unwrap_or("Stave is a TUI OS installer built using Rust and Ratatui.\nCreated for SongbirdOS, it's name is taken from the note lines used in musical scores.".to_string()),
            }
    }

    fn default() -> Self {
        Branding {
                logo: true,
                logo_path:"".to_string(),
                description: "Stave is a TUI OS installer built using Rust and Ratatui.\nCreated for SongbirdOS, it's name is taken from the note lines used in musical scores.".to_string(),
            }
    }
}

impl Colors {
    fn get(config: &RawConfig) -> Self {
        Colors {
            border: config
                .colors
                .clone()
                .unwrap()
                .border
                .unwrap_or("#89b4fa".to_string()),
            inactive_border: config
                .colors
                .clone()
                .unwrap()
                .inactive_border
                .unwrap_or("#6c7086".to_string()),
            active_border: config
                .colors
                .clone()
                .unwrap()
                .active_border
                .unwrap_or("#cba6f7".to_string()),

            text: config
                .colors
                .clone()
                .unwrap()
                .text
                .unwrap_or("#cdd6f4".to_string()),
            text_bg: config
                .colors
                .clone()
                .unwrap()
                .text_bg
                .unwrap_or("reset".to_string()),
            active_text: config
                .colors
                .clone()
                .unwrap()
                .active_text
                .unwrap_or("#bac2de".to_string()),
            active_text_bg: config
                .colors
                .clone()
                .unwrap()
                .active_text_bg
                .unwrap_or("reset".to_string()),
            inactive_text: config
                .colors
                .clone()
                .unwrap()
                .inactive_text
                .unwrap_or("#a6adc8".to_string()),
            inactive_text_bg: config
                .colors
                .clone()
                .unwrap()
                .inactive_text_bg
                .unwrap_or("reset".to_string()),
        }
    }

    fn default() -> Self {
        Colors {
            border: "#89b4fa".to_string(),
            active_border: "#cba6f7".to_string(),
            inactive_border: "#6c7086".to_string(),

            text: "#cdd6f4".to_string(),
            text_bg: "reset".to_string(),
            active_text: "#bac2de".to_string(),
            active_text_bg: "reset".to_string(),
            inactive_text: "#a6adc8".to_string(),
            inactive_text_bg: "reset".to_string(),
        }
    }
}

fn get_path() -> String {
    if Result::expect(
        fs::exists("./config.toml"),
        "[WARN]: `config.toml` not found in current directory.",
    ) {
        return "./config.toml".to_string();
    } else if Result::expect(
        fs::exists("~/.config/stave/config.toml"),
        "[WARN]: `config.toml` not found in `~/.config/stave/`.",
    ) {
        return "~/.config/stave/config.toml".to_string();
    } else if Result::expect(
        fs::exists("/etc/stave/config.toml"),
        "[WARN]: `config.toml` not found in `/etc/stave/`.",
    ) {
        return "/etc/stave/config.toml".to_string();
    } else {
        println!("[WARN]: `config.toml` not found using the default config instead.");
        return "".to_string();
    }
}
