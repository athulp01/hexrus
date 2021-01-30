mod editor;

use editor::Editor;
use std::env;
use std::error::Error;
use std::fs;
use std::io;
use termion::{event::Key, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Gauge, Paragraph, Row, Table},
    Terminal,
};

use termion::input::TermRead;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let data = fs::read(args[1].clone()).expect("Unable to read file");
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut hexrus = Editor::from(&data);

    let mut start: usize = 0;
    // Input
    loop {
        let stdin = io::stdin();
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

            if hexrus.state.offset > 10 && start != 0{
                let select = hexrus.state.selected().unwrap() - hexrus.state.offset;
                start += hexrus.state.offset;
                hexrus.state.offset = 0;
                hexrus.state.select(Some(select));
            }

            if hexrus.state.selected().unwrap_or(11) < 10 && hexrus.state.offset <= 1 && start != 0{
                let select = if start > 1000 {start - 1000} else {0};
                hexrus.state.offset =start + hexrus.state.offset;
                hexrus.state.select(Some(1 + hexrus.state.selected().unwrap() + hexrus.state.offset));
                start = select;
            } 

            let hex_rows =
                editor::build_hex_rows(hexrus.bytes, hexrus.cursor_pos, hexrus.width, start);

            let char_rows: Vec<Row> =
                editor::build_ascii_rows(hexrus.bytes, hexrus.cursor_pos, rects[0].width, start);

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

        for evt in stdin.keys() {
            if let Ok(key) = evt {
                match key {
                    Key::Char('q') => {
                        return Ok(())
                    }
                    _ => {hexrus.move_cursor(key);
                    break;
                }
            }
        }
    }}
}
