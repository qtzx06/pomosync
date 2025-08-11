use crate::ascii;
use ratatui::{prelude::*,
    symbols,
    widgets::{Block, Borders, Clear, Paragraph},
};

pub fn render(f: &mut Frame, area: Rect) {
    let border = Block::default()
        .borders(Borders::ALL)
        .border_set(symbols::border::Set {
            top_left: "╭",
            top_right: "╮",
            bottom_left: "╰",
            bottom_right: "╯",
            vertical_left: "│",
            vertical_right: "│",
            horizontal_top: "─",
            horizontal_bottom: "─",
        });

    let inner_area = border.inner(area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Length(6), // "POMOSYNC" height
            Constraint::Length(6), // "MENU" height
            Constraint::Length(2), // description
            Constraint::Length(1), // dotted line
            Constraint::Length(5), // keybindings
            Constraint::Percentage(10),
        ])
        .split(inner_area);

    f.render_widget(Clear, area);
    f.render_widget(border, area);

    ascii::render_text(chunks[1], f.buffer_mut(), "POMOSYNC");
    ascii::render_text(chunks[2], f.buffer_mut(), "MENU");

    let description = "a universal pomodoro timer";
    let desc_para = Paragraph::new(description).alignment(Alignment::Center);
    f.render_widget(desc_para, chunks[3]);

    let dotted_line = "·".repeat(chunks[4].width as usize);
    let line_para = Paragraph::new(dotted_line).alignment(Alignment::Center);
    f.render_widget(line_para, chunks[4]);

    let keys_text = "(q)quit\n(p)ause\n(m)enu";
    let keys_para = Paragraph::new(keys_text).alignment(Alignment::Center);
    f.render_widget(keys_para, chunks[5]);
}
