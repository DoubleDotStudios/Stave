mod config;
mod container;
mod pages;
mod utils;

use std::str::FromStr;

use color_eyre::{eyre::Ok, Result};
use config::Config;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, ModifierKeyCode};
use ratatui::{
    layout::{self, Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Tabs},
    DefaultTerminal, Frame,
};
use utils::render_page;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::default().run(terminal);
    ratatui::restore();
    app_result
}

#[derive(Default)]
struct App {
    tab: usize,
}

impl App {
    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Tab => self.tab += 1,
                        KeyCode::BackTab => self.tab -= 1,
                        _ => {}
                    }
                }
            }
        }
    }

    fn draw(&self, frame: &mut Frame) {
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
        render_page(
            config.general.tabs[self.tab].clone(),
            frame,
            layout[1],
            config,
        );
    }
}
