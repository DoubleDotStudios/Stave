use std::str::FromStr;

use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Padding},
};

use crate::config::Config;

pub struct Container {
    fg: Color,
    bg: Color,
    border_style: BorderType,
    borders: Borders,
    title: String,
}

impl Container {
    pub fn new(
        title: String,
        fg: Color,
        bg: Color,
        border_type: BorderType,
        borders: Borders,
    ) -> Block<'static> {
        Block::default()
            .title(title)
            .border_style(Style::default().fg(fg).bg(bg))
            .border_type(border_type)
            .borders(borders)
            .padding(Padding::ZERO)
    }

    pub fn default(config: Config) -> Block<'static> {
        Container::new(
            "".to_string(),
            Color::from_str(&config.colors.border).expect("Invalid color!"),
            Color::Reset,
            BorderType::Rounded,
            Borders::ALL,
        )
    }
}

pub fn area(rect: Block, area: Rect) -> Rect {
    return rect.inner(area);
}
