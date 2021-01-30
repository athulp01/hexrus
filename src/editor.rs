use termion::event::Key;
#[allow(dead_code)]
use tui::widgets::TableState;
use tui::{
    style::{Modifier, Style},
    widgets::{Cell, Row},
};

pub struct Editor<'a> {
    pub cursor_pos: usize,
    pub width: u16,
    pub height: u16,
    pub col_count: u16,
    pub bytes: &'a Vec<u8>,
    pub state: TableState,
}

impl<'a> Editor<'a> {
    pub fn from(items: &'a Vec<u8>) -> Editor<'a> {
        Editor {
            cursor_pos: 0,
            state: TableState::default(),
            width: 0,
            height: 0,
            col_count: 0,
            bytes: items,
        }
    }

    pub fn move_cursor(&mut self, direction: Key) {
        match direction {
            Key::Left => {
                self.cursor_pos = if self.cursor_pos == 0 {
                    0
                } else {
                    self.cursor_pos - 1
                };
            }
            Key::Right => {
                self.cursor_pos = if self.cursor_pos >= self.bytes.len() - 1 {
                    self.cursor_pos
                } else {
                    self.cursor_pos + 1
                };
            }
            Key::Down => {
                self.cursor_pos =
                    if self.cursor_pos + self.col_count as usize >= self.bytes.len() - 1 {
                        self.cursor_pos
                    } else {
                        self.cursor_pos + self.col_count as usize
                    };
                self.state.select(Some(self.state.selected().unwrap_or(0) + 1));
            }
            Key::Up => {
                self.cursor_pos = if self.cursor_pos < self.col_count as usize {
                    0
                } else {
                    self.cursor_pos - self.col_count as usize
                };
                self.state.select(Some(self.state.selected().unwrap() - 1));
            }

            _ => {}
        }
    }
}

pub fn build_hex_rows(items: &Vec<u8>, cursor_pos: usize, width: u16, start: usize) -> Vec<Row> {
    let col_size = ((width - 1) / 3) as usize;
    let mut hex_rows: Vec<Row> = Vec::new();
    let select_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default();

    for r_idx in start..1000 {
        let mut hex_cells: Vec<Cell> = Vec::new();
        for c_idx in 0..col_size {
            let idx = r_idx * col_size + c_idx;
            hex_cells.push(match idx {
                i if i >= items.len() => Cell::from(" ").style(normal_style),
                _ => {
                    Cell::from(format!("{:02X}", items[idx])).style(if idx == cursor_pos as usize {
                        select_style
                    } else {
                        normal_style
                    })
                }
            });
        }
        hex_rows.push(Row::new(hex_cells).height(1).bottom_margin(0));
    }
    hex_rows
}

pub fn build_ascii_rows(items: &Vec<u8>, cursor_pos: usize, width: u16, start: usize) -> Vec<Row> {
    let select_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default();
    let col_size = ((width - 1) / 3) as usize;
    let mut ascii_rows: Vec<Row> = Vec::new();

    for r_idx in start..1000 {
        let mut char_cells: Vec<Cell> = Vec::new();
        for c_idx in 0..col_size {
            let idx = r_idx * col_size + c_idx;
            char_cells.push(match idx {
                i if i >= items.len() => Cell::from(" ").style(normal_style),
                _ => Cell::from(format!(
                    "{}",
                    if items[idx].is_ascii_control() {
                        '.'.to_owned()
                    } else {
                        items[idx] as char
                    }
                ))
                .style(if idx == cursor_pos {
                    select_style
                } else {
                    normal_style
                }),
            });
        }
        ascii_rows.push(Row::new(char_cells).height(1).bottom_margin(0));
    }
    ascii_rows
}
