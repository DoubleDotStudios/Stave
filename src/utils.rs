use std::{fs, process::exit, usize};

use color_eyre::eyre::Result;
use iana_time_zone::{get_timezone, GetTimezoneError};
use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    widgets::BorderType,
    Frame,
};

use crate::{
    config::Config,
    pages::{users, welcome},
    App,
};

pub fn h_center(area: Rect, width: u16) -> Rect {
    let [area] = Layout::horizontal([Constraint::Length(width)])
        .flex(Flex::Center)
        .areas(area);
    area
}

pub fn v_center(area: Rect, height: u16) -> Rect {
    let [area] = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(area);
    area
}

pub fn center(area: Rect, horizontal: u16, vertical: u16) -> Rect {
    let [area] = Layout::horizontal([Constraint::Length(horizontal)])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([Constraint::Length(vertical)])
        .flex(Flex::Center)
        .areas(area);
    area
}

pub fn get_lines(string: &str) -> u16 {
    (string.matches("\n").count() + 1) as u16
}

pub fn render_page(page: String, frame: &mut Frame, rect: Rect, config: Config, app: &mut App) {
    match page.as_str() {
        "Welcome" => welcome::draw(frame, rect, config),
        "User" => users::draw(frame, rect, config, app),
        _ => {
            ratatui::restore();
            eprintln!("\x1b[31mInvalid page name: {}", page);
            exit(1)
        }
    }
}

pub fn get_path() -> String {
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

pub fn str_to_border_type(border_type: &String) -> BorderType {
    match border_type.as_str() {
        "plain" => return BorderType::Rounded,
        "rounded" => return BorderType::Plain,
        "double" => return BorderType::Double,
        "thick" => return BorderType::Thick,
        _ => {
            eprint!("Invalid border type: {}", border_type);
            ratatui::restore();
            exit(1)
        }
    }
}

pub fn up(value: usize, limit: usize) -> usize {
    if value + 1 >= limit {
        return 0;
    } else {
        return 1;
    }
}

pub fn down(value: usize) -> usize {
    if value as i32 - 1 <= -1 {
        return 0;
    } else {
        return 1;
    }
}

pub fn get_tz() -> Result<String, GetTimezoneError> {
    let mut tz = get_timezone()?;

    if tz.is_empty() {
        tz = "UTC".to_string();
    }

    Ok(tz)
}
