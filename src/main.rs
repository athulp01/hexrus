mod editor;
mod event;

use editor::Editor;
use event::{Event, Events};
use std::env;
use std::fs;
use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Gauge, Paragraph, Row, Table},
    Terminal,
};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let data = fs::read(args[1].clone()).expect("Unable to read file");
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    let mut hexrus = Editor::from(&data);
    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default();

    // Input
    loop {
        terminal.draw(|f| {
            let size = f.size();
            let rects = Layout::default()
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                .margin(0)
                .direction(Direction::Horizontal)
                .split(Rect::new(size.x, size.y, size.width, size.height - 1));

            hexrus.width = rects[0].width;
            hexrus.height = rects[0].height;
            let col_count = ((rects[0].width - 1) / 3) as usize;
            hexrus.col_count = col_count as u16;
            let hex_col_width = vec![Constraint::Length(2); col_count];
            let char_col_width = vec![Constraint::Length(1); col_count];

            let hex_rows = editor::build_hex_rows(
                hexrus.bytes,
                hexrus.cursor_pos,
                hexrus.width,
                selected_style,
                normal_style,
            );

            let char_rows: Vec<Row> = editor::build_ascii_rows(
                hexrus.bytes,
                hexrus.cursor_pos,
                rects[0].width,
                selected_style,
                normal_style,
            );

            let ratio = (hexrus.cursor_pos + 1) as f64 / data.len() as f64;
            let gauge = Gauge::default()
                .gauge_style(
                    Style::default()
                        .fg(Color::Red)
                        .bg(Color::Black)
                        .add_modifier(Modifier::ITALIC),
                )
                .ratio(ratio);
            let status_rect = Rect::new(0, size.height - 1, size.width, 1);
            let status_layout = Layout::default()
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                .margin(0)
                .direction(Direction::Horizontal)
                .split(status_rect);
            let hex_table = Table::new(hex_rows)
                .block(Block::default().borders(Borders::ALL).title("Hex"))
                .widths(&hex_col_width)
                .column_spacing(1);

            let char_table = Table::new(char_rows)
                .block(Block::default().borders(Borders::ALL).title("ASCII"))
                .widths(&char_col_width)
                .column_spacing(0);
            let byte_status = Paragraph::new(format!("{}/{} bytes", hexrus.cursor_pos, data.len()))
                .alignment(Alignment::Center);

            f.render_stateful_widget(hex_table, rects[0], &mut hexrus.state);
            f.render_stateful_widget(char_table, rects[1], &mut hexrus.state);
            f.render_widget(gauge, status_layout[0]);
            f.render_widget(byte_status, status_layout[1]);
        })?;

        if let Event::Input(key) = events.next()? {
            match key {
                Key::Char('q') => {
                    break;
                }
                _ => {hexrus.move_cursor(key)}
            }
        };
    }

    Ok(())
}
