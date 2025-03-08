use std::ops::Index;

use ratatui::{
    layout::{Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders},
};

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
    }
}

pub fn area(rect: Block, area: Rect) -> Rect {
    return rect.inner(area);
}
