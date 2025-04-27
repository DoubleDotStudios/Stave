use serde::Deserialize;
use std::fs;

use crate::utils;

#[derive(Deserialize, Default, Clone)]
pub struct RawConfig {
    pub general: Option<RawGeneral>,
    pub branding: Option<RawBranding>,
    pub colors: Option<RawColors>,
    pub style: Option<RawStyle>,
}

#[derive(Deserialize, Default, Clone)]
pub struct RawGeneral {
    pub tabs: Option<Vec<String>>,
    pub tab_spacer: Option<String>,
}

#[derive(Deserialize, Default, Clone)]
pub struct RawBranding {
    pub logo_path: Option<String>,
    pub description: Option<String>,
    pub welcome_message: Option<String>,
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
    pub title_text: Option<String>,
    pub title_text_bg: Option<String>,
}

#[derive(Deserialize, Default, Clone)]
pub struct RawStyle {
    pub border_style: Option<String>,
}

#[derive(Deserialize, Default, Clone)]
pub struct Config {
    pub general: General,
    pub branding: Branding,
    pub colors: Colors,
    pub style: Style,
}

#[derive(Deserialize, Default, Clone)]
pub struct General {
    pub tabs: Vec<String>,
    pub tab_spacer: String,
}

#[derive(Deserialize, Default, Clone)]
pub struct Branding {
    pub logo_path: String,
    pub description: String,
    pub welcome_message: String,
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
    pub title_text: String,
    pub title_text_bg: String,
}

#[derive(Deserialize, Default, Clone)]
pub struct Style {
    pub border_style: String,
}

impl Config {
    pub fn get() -> Self {
        let config_path = utils::get_path();

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

            let style = if config.style.is_some() {
                Style::get(&config)
            } else {
                Style::default()
            };

            Config {
                branding,
                colors,
                general,
                style,
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
            style: Style::default(),
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
            "User".to_string(),
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
            "User".to_string(),
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
            logo_path: config
                .branding
                .clone()
                .unwrap()
                .logo_path
                .unwrap_or("".to_string()),
            description: config
                .branding
                .clone()
                .unwrap()
                .description
                .unwrap_or("".to_string()),
            welcome_message: config
                .branding
                .clone()
                .unwrap()
                .welcome_message
                .unwrap_or("Welcome!".to_string()),
        }
    }

    fn default() -> Self {
        Branding {
            logo_path: "".to_string(),
            description: "".to_string(),
            welcome_message: "Welcome!".to_string(),
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
            title_text: config
                .colors
                .clone()
                .unwrap()
                .title_text
                .unwrap_or("#f38ba8".to_string()),
            title_text_bg: config
                .colors
                .clone()
                .unwrap()
                .title_text_bg
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
            title_text: "#f38ba8".to_string(),
            title_text_bg: "reset".to_string(),
        }
    }
}

impl Style {
    fn get(config: &RawConfig) -> Self {
        Style {
            border_style: config
                .style
                .clone()
                .unwrap()
                .border_style
                .unwrap_or("plain".to_string()),
        }
    }

    fn default() -> Self {
        Style {
            border_style: "plain".to_string(),
        }
    }
}
