use std::str::FromStr;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::Borders,
    Frame,
};
use tui_textarea::TextArea;

use crate::{
    components::container::{area, Container},
    config::Config,
    utils::str_to_border_type,
    App,
};

pub fn draw(frame: &mut Frame, rect: Rect, config: Config, app: &mut App) {
    let text = Color::from_str(&config.colors.text).expect("Invalid color!");
    let text_bg = Color::from_str(&config.colors.text_bg).expect("Invalid color!");
    let title_text = Color::from_str(&config.colors.title_text).expect("Invalid color!");
    let title_text_bg = Color::from_str(&config.colors.title_text_bg).expect("Invalid color!");

    let outer = Layout::new(
        Direction::Vertical,
        vec![
            Constraint::Max(7),
            Constraint::Max(1),
            Constraint::Max(5),
            Constraint::Max(1),
            Constraint::Max(5),
        ],
    )
    .split(rect);

    let user = Layout::new(Direction::Horizontal, vec![Constraint::Length(50)]).split(outer[2]);
    let passwd = Layout::new(Direction::Horizontal, vec![Constraint::Length(50)]).split(outer[4]);

    let container = Container::default(&config);

    let username_area = area(container.clone(), user[0]);
    let password_area = area(container.clone(), passwd[0]);

    let username = textarea(false, "Username...");
    let password = textarea(true, "Password...");

    let fields = [username.clone(), password.clone()];

    if app.tab
        == config
            .general
            .tabs
            .iter()
            .position(|r| r == "User")
            .unwrap()
    {
        app.input = true;
        app.fields = fields.to_vec();

        frame.render_widget(&container, rect);
        frame.render_widget(&app.fields[0], username_area);
        frame.render_widget(&app.fields[1], password_area);
    }
}

fn textarea(password: bool, placeholder_text: &str) -> TextArea<'static> {
    let mut textarea = TextArea::default();
    textarea.set_placeholder_text(placeholder_text);

    if password {
        textarea.set_mask_char('\u{2022}');
    }

    return textarea;
}

pub fn active(area: &mut TextArea<'_>, config: &Config) {
    area.set_cursor_line_style(Style::default().add_modifier(Modifier::UNDERLINED));
    area.set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
    area.set_block(Container::new(
        "".to_string(),
        Color::from_str(&config.colors.active_border).expect("Invalid color!"),
        Color::Reset,
        str_to_border_type(&config.style.border_style),
        Borders::ALL,
    ))
}

pub fn inactive(area: &mut TextArea<'_>, config: &Config) {
    area.set_cursor_line_style(Style::default());
    area.set_cursor_style(Style::default());
    area.set_block(Container::new(
        "".to_string(),
        Color::from_str(&config.colors.inactive_border).expect("Invalid color!"),
        Color::Reset,
        str_to_border_type(&config.style.border_style),
        Borders::ALL,
    ))
}
