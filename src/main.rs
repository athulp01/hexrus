mod editor;
#[allow(dead_code)]
mod event;

use editor::Editor;
use std::env;
use event::{Event, Events};
use std::fs;
use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Gauge, Paragraph, Row, Table},
    Terminal,
};

fn main() -> Result<(), Box<dyn Error>> {
    let args:Vec<String> = env::args().collect();
    let data = fs::read(args[1].clone()).expect("Unable to read file");
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    let mut editor = Editor::from(&data);

    // Input
    loop {
        terminal.draw(|f| {
            let size = f.size();
            let rects = Layout::default()
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                .margin(0)
                .direction(Direction::Horizontal)
                .split(Rect::new(size.x, size.y, size.width, size.height - 1));

            let selected_style = Style::default().add_modifier(Modifier::REVERSED);
            let normal_style = Style::default();
            editor.width = size.width;
            editor.height = size.height;
            let col_size = ((rects[0].width - 1) / 3) as usize;
            editor.col_size = col_size as u16;
            let hex_col_width = vec![Constraint::Length(2); col_size];
            let char_col_width = vec![Constraint::Length(1); col_size];

            let mut hex_rows: Vec<Row> = Vec::new();
            let mut char_rows: Vec<Row> = Vec::new();

            for r_idx in 0..(data.len() as f32 / col_size as f32).ceil() as usize {
                let mut hex_cells: Vec<Cell> = Vec::new();
                let mut char_cells: Vec<Cell> = Vec::new();
                for c_idx in 0..col_size {
                    let idx = r_idx * col_size + c_idx;
                    hex_cells.push(match idx {
                        i if i >= data.len() => Cell::from(" ").style(normal_style),
                        _ => Cell::from(format!("{:02X}", editor.items[idx])).style(
                            if idx == editor.cursor_pos {
                                selected_style
                            } else {
                                normal_style
                            },
                        ),
                    });

                    char_cells.push(match idx {
                        i if i >= data.len() => Cell::from(" ").style(normal_style),
                        _ => Cell::from(format!(
                            "{}",
                            if editor.items[idx].is_ascii_control()
                            {
                                '.'.to_owned()
                            } else {
                                editor.items[idx] as char
                            }
                        ))
                        .style(if idx == editor.cursor_pos {
                            selected_style
                        } else {
                            normal_style
                        }),
                    });
                }
                hex_rows.push(Row::new(hex_cells).height(1).bottom_margin(0));
                char_rows.push(Row::new(char_cells).height(1).bottom_margin(0));
            }
            let ratio = (editor.cursor_pos + 1) as f64 / data.len() as f64;
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
            let byte_status = Paragraph::new(format!("{}/{} bytes", editor.cursor_pos, data.len()))
                .alignment(Alignment::Center);

            f.render_stateful_widget(hex_table, rects[0], &mut editor.state);
            f.render_stateful_widget(char_table, rects[1], &mut editor.state);
            f.render_widget(gauge, status_layout[0]);
            f.render_widget(byte_status, status_layout[1]);
        })?;

        if let Event::Input(key) = events.next()? {
            match key {
                Key::Char('q') => {
                    break;
                }
                Key::Down => {
                    editor.move_cursor(editor::Direction::DOWN);
                }
                Key::Up => {
                    editor.move_cursor(editor::Direction::UP);
                }
                Key::Left => {
                    editor.move_cursor(editor::Direction::LEFT);
                }
                Key::Right => {
                    editor.move_cursor(editor::Direction::RIGHT);
                }
                _ => {}
            }
        };
    }

    Ok(())
}
