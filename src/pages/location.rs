use std::{fs, process::exit, str::FromStr};

use ansi_to_tui::IntoText;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Text,
    Frame,
};

use crate::{
    components::container::{area, Container},
    config::Config,
    utils::{center, get_lines, get_tz},
};

pub fn draw(frame: &mut Frame, rect: Rect, config: Config) {
    let text = Color::from_str(&config.colors.text).expect("Invalid color!");
    let text_bg = Color::from_str(&config.colors.text_bg).expect("Invalid color!");
    let title_text = Color::from_str(&config.colors.title_text).expect("Invalid color!");
    let title_text_bg = Color::from_str(&config.colors.title_text_bg).expect("Invalid color!");

    let outer = Layout::new(Direction::Vertical, vec![Constraint::Percentage(100)]).split(rect);

    let tz = get_tz();

    let container = Container::default(&config);

    frame.render_widget(&container, rect);
}
