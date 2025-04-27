use std::{fs, process::exit, str::FromStr};

use ansi_to_tui::IntoText;
use color_eyre::eyre::Result;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Text,
    Frame,
};

use crate::{
    components::container::{area, Container},
    config::Config,
    utils::{center, get_lines},
};

pub fn draw(frame: &mut Frame, rect: Rect, config: Config) {
    let description = &config.branding.description.clone();
    let welcome_message = &config.branding.welcome_message.clone();
    let logo_path = &config.branding.logo_path.clone();
    let text = Color::from_str(&config.colors.text).expect("Invalid color!");
    let text_bg = Color::from_str(&config.colors.text_bg).expect("Invalid color!");
    let title_text = Color::from_str(&config.colors.title_text).expect("Invalid color!");
    let title_text_bg = Color::from_str(&config.colors.title_text_bg).expect("Invalid color!");

    let outer = Layout::new(
        Direction::Vertical,
        vec![
            Constraint::Min(7),
            Constraint::Percentage(100),
            Constraint::Min(7),
        ],
    )
    .split(rect);

    let outer_alt = Layout::new(
        Direction::Vertical,
        vec![Constraint::Min(7), Constraint::Percentage(100)],
    )
    .split(rect);

    let container = Container::default(&config);

    let mut welcome_area = area(container.clone(), outer[0]);
    let mut top_area = area(container.clone(), outer[1]);
    let mut bot_area = area(container.clone(), outer[2]);

    if logo_path == "" && description == "" && welcome_message == "" {
        ratatui::restore();
        eprintln!("\x1b[31mMust have at least one component on the page.\x1b[0m");
        exit(1);
    } else if logo_path == "" && description == "" {
        welcome_area = rect
    } else if logo_path == "" {
        welcome_area = area(container.clone(), outer_alt[0]);
        bot_area = area(container.clone(), outer_alt[1]);
    } else if description == "" {
        welcome_area = area(container.clone(), outer_alt[0]);
        top_area = area(container.clone(), outer_alt[1]);
    }

    if logo_path != "" {
        let logo_text =
            fs::read_to_string(logo_path).expect(format!("Invalid path {}", logo_path).as_str());

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

    if description != "" {
        let desc = Text::raw(description)
            .style(
                Style::default()
                    .fg(text)
                    .bg(text_bg)
                    .add_modifier(Modifier::BOLD),
            )
            .centered();

        let area = center(bot_area, desc.width() as u16, get_lines(&description));

        frame.render_widget(desc, area);
    }

    let welcome = Text::raw(welcome_message)
        .style(
            Style::default()
                .fg(title_text)
                .bg(title_text_bg)
                .add_modifier(Modifier::BOLD),
        )
        .centered();

    let area = center(
        welcome_area,
        welcome.width() as u16,
        get_lines(&welcome_message),
    );

    frame.render_widget(&container, rect);
    frame.render_widget(welcome, area);
}
