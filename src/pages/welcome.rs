use std::{fs, str::FromStr};

use ansi_to_tui::IntoText;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{BorderType, Borders},
    Frame,
};

use crate::{
    config::Config,
    container::{area, Container},
    utils::{center, get_lines},
};

pub fn draw(frame: &mut Frame, rect: Rect, config: Config) {
    let description = &config.branding.description;
    let use_logo = &config.branding.logo;
    let logo_path = &config.branding.logo_path;
    let text = Color::from_str(&config.colors.text).expect("Invalid color!");
    let text_bg = Color::from_str(&config.colors.text_bg).expect("Invalid color!");
    let border = Color::from_str(&config.colors.border).expect("Invalid color!");

    let mut logo_text: String = "Stave".to_string();
    let mut brand_page: Vec<Text> = vec![];

    let outer = Layout::new(
        Direction::Vertical,
        vec![Constraint::Percentage(90), Constraint::Percentage(10)],
    )
    .split(rect);

    let container = Container::new(
        "".to_string(),
        border,
        Color::Reset,
        BorderType::Rounded,
        Borders::ALL,
    );

    let top_area = area(container.clone(), outer[0]);
    let mut bot_area = area(container.clone(), outer[1]);

    if *use_logo {
        if logo_path != "" {
            logo_text = fs::read_to_string(logo_path)
                .expect(format!("Invalid path {}", logo_path).as_str());

            let logo_proto = fs::read(logo_path).unwrap();

            let logo: Text = logo_proto
                .into_text()
                .expect("Failed to convert logo to text widget!");

            let area = center(top_area, logo.width() as u16, get_lines(&logo_text));

            if logo.height() <= top_area.height.into() && logo.width() <= top_area.width.into() {
                frame.render_widget(logo, area);
            } else {
                bot_area = rect;
            }
        }
    } else {
        bot_area = rect;
    }

    let desc = Text::raw(description)
        .style(
            Style::default()
                .fg(text)
                .bg(text_bg)
                .add_modifier(Modifier::BOLD),
        )
        .centered();

    brand_page.push(desc.clone());

    let area = center(bot_area, desc.width() as u16, get_lines(&description));

    frame.render_widget(&container, rect);
    frame.render_widget(desc, area);
}
