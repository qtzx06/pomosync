use crate::{app::Cycle, ascii, menu};
use ratatui::{
    prelude::*,
    symbols,
    widgets::{Block, Borders, Paragraph},
};

pub fn ui(f: &mut Frame, app: &Cycle) {
    let area = f.area();

    if app.show_menu {
        let popup_area = Rect {
            x: area.width / 4,
            y: area.height / 4,
            width: area.width / 2,
            height: area.height / 2,
        };
        menu::render(f, popup_area);
        return;
    }

    let time_str = app.remaining_time();
    let time_width = ascii::get_text_width(&time_str) as u16;
    let time_height = ascii::FONT_HEIGHT as u16;

    let clock_box_width = time_width + 4;
    let clock_box_height = 14;

    if area.height < clock_box_height || area.width < clock_box_width {
        let message = "Terminal too small";
        f.render_widget(Paragraph::new(message).alignment(Alignment::Center), area);
        return;
    }

    let clock_area = Rect {
        x: area.x + (area.width - clock_box_width) / 2,
        y: area.y + (area.height - clock_box_height) / 2,
        width: clock_box_width,
        height: clock_box_height,
    };

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
    let inner_area = border.inner(clock_area);
    f.render_widget(border, clock_area);

    let inner_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(time_height),
            Constraint::Length(1), // minutes until
            Constraint::Length(1), // line
            Constraint::Length(1), // current state
            Constraint::Length(1), // bottom hints
        ])
        .margin(1)
        .split(inner_area);

    ascii::render_time(inner_chunks[0], f.buffer_mut(), &time_str);

    let until_text = format!("minutes until {}", app.next_state_name.to_lowercase());
    let until_paragraph = Paragraph::new(until_text).alignment(Alignment::Center);
    f.render_widget(until_paragraph, inner_chunks[1]);

    let dotted_line = "·".repeat(inner_chunks[2].width as usize);
    let line_paragraph = Paragraph::new(dotted_line).alignment(Alignment::Center);
    f.render_widget(line_paragraph, inner_chunks[2]);

    let state_text = if app.paused {
        format!("current state: {} (paused)", app.state_name.to_lowercase())
    } else {
        format!("current state: {}", app.state_name.to_lowercase())
    };
    let state_paragraph = Paragraph::new(state_text).alignment(Alignment::Center);
    f.render_widget(state_paragraph, inner_chunks[3]);

    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(inner_chunks[4]);

    let quit_hint = Paragraph::new("(q)uit").alignment(Alignment::Left);
    f.render_widget(quit_hint, bottom_chunks[0]);

    let menu_hint = Paragraph::new("(m)enu").alignment(Alignment::Right);
    f.render_widget(menu_hint, bottom_chunks[1]);
}
