use crossterm::event::{self, Event};
use ratatui::{
    style::{Color, Style},
    text::Text,
    widgets::{Block, BorderType, Borders, List},
    Frame,
};

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
    let block = Block::default()
        .title("")
        .border_style(Style::default().fg(Color::Blue))
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL);

    let text = Text::styled("Hello, World!", Style::new().fg(Color::Magenta));

    let items = [text];

    let list = List::new(items).block(block);

    frame.render_widget(list, frame.area());
}
