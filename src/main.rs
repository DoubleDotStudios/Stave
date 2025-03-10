mod config;
mod container;
mod pages;
mod utils;

use std::str::FromStr;

use config::Config;
use crossterm::event::{self, Event};
use ratatui::{
    layout::{self, Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Tabs},
    Frame,
};
use utils::render_page;

fn main() {
    let mut terminal = ratatui::init();
    let mut page = 0;
    loop {
        terminal.draw(draw).expect("failed to draw frame");
        if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
            break;
        }
    }
    ratatui::restore();
}

fn draw(frame: &mut Frame) {
    let config = Config::get();

    let pages = Tabs::new(config.general.tabs.clone())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(
                    Style::default()
                        .fg(Color::from_str(&config.colors.border).expect("Invalid color!")),
                ),
        )
        .style(
            Style::default()
                .fg(Color::from_str(&config.colors.inactive_text).expect("Invalid color!"))
                .bg(Color::from_str(&config.colors.inactive_text_bg).expect("Invalid color!")),
        )
        .highlight_style(
            Style::default()
                .fg(Color::from_str(&config.colors.active_text).expect("Invalid color!"))
                .bg(Color::from_str(&config.colors.active_text_bg).expect("Invalid color!")),
        )
        .divider(config.general.tab_spacer.clone())
        .select(0);

    let layout = Layout::new(
        layout::Direction::Vertical,
        vec![Constraint::Min(3), Constraint::Percentage(100)],
    )
    .spacing(0)
    .split(frame.area());

    frame.render_widget(pages, layout[0]);
    render_page(config.general.tabs[0].clone(), frame, layout[1], config);
}
