use ratatui::layout::{Constraint, Flex, Layout, Rect};

pub fn h_center(area: Rect, width: u16) -> Rect {
    let [area] = Layout::horizontal([Constraint::Length(width)])
        .flex(Flex::Center)
        .areas(area);
    area
}

pub fn v_center(area: Rect, height: u16) -> Rect {
    let [area] = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(area);
    area
}

pub fn center(area: Rect, horizontal: u16, vertical: u16) -> Rect {
    let [area] = Layout::horizontal([Constraint::Length(horizontal)])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([Constraint::Length(vertical)])
        .flex(Flex::Center)
        .areas(area);
    area
}

pub fn get_lines(string: &str) -> u16 {
    (string.matches("\n").count() + 1) as u16
}
