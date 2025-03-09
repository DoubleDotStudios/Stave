mod config;
mod container;
mod pages;
mod utils;

use crate::pages::brand_page;
use config::Config;
use crossterm::event::{self, Event};
use ratatui::Frame;

fn main() {
    let mut terminal = ratatui::init();
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

    brand_page::draw(frame, config);
}
