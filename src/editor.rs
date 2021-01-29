use tui::widgets::TableState;
use tui::{
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Gauge, Row, Table},
};

pub struct Editor<'a> {
    pub cursor_pos: usize,
    pub width: u16,
    pub height: u16,
    pub col_size: u16,
    pub items: &'a Vec<u8>,
    pub state: TableState,
}

pub enum Direction {
    LEFT,
    RIGHT,
    DOWN,
    UP,
}

impl<'a> Editor<'a> {
    pub fn from(items: &'a Vec<u8>) -> Editor<'a> {
        Editor {
            cursor_pos: 0,
            state: TableState::default(),
            width: 0,
            height: 0,
            col_size: 0,
            items: items,
        }
    }

    pub fn move_cursor(&mut self, direction: Direction) {
        match direction {
            Direction::LEFT => {
                self.cursor_pos = if self.cursor_pos == 0 {
                    0
                } else {
                    self.cursor_pos - 1
                };
            }
            Direction::RIGHT => {
                self.cursor_pos = if self.cursor_pos >= self.items.len() - 1 {
                    self.cursor_pos
                } else {
                    self.cursor_pos + 1
                };
            }
            Direction::DOWN => {
                self.cursor_pos =
                    if self.cursor_pos + self.col_size as usize >= self.items.len() - 1 {
                        self.cursor_pos
                    } else {
                        self.cursor_pos + self.col_size as usize
                    };
            }
            Direction::UP => {
                self.cursor_pos = if self.cursor_pos < self.col_size as usize {
                    0
                } else {
                    self.cursor_pos - self.col_size as usize
                };
            }
        }
        self.state
            .select(Some(self.cursor_pos / self.col_size as usize));
    }
}

fn build_hex_rows(
    items: &Vec<u8>,
    cursor_pos: usize,
    width: u16,
    select_style: Style,
    normal_style: Style,
) -> Vec<Row> {
    let col_size = ((width - 1) / 3) as usize;
    let mut hex_rows: Vec<Row> = Vec::new();

    for r_idx in 0..(items.len() as f32 / col_size as f32).ceil() as usize {
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

pub fn build_ascii_rows(
    items: &Vec<u8>,
    cursor_pos: usize,
    width: u16,
    select_style: Style,
    normal_style: Style,
) -> Vec<Row> {
    let col_size = (width - 1) as usize;
    let mut ascii_rows: Vec<Row> = Vec::new();

    for r_idx in 0..(items.len() as f32 / col_size as f32).ceil() as usize {
        let mut char_cells: Vec<Cell> = Vec::new();
        for c_idx in 0..col_size {
            let idx = r_idx * col_size + c_idx;
            char_cells.push(match idx {
                i if i >= items.len() => Cell::from(" ").style(normal_style),
                _ => Cell::from(format!("{}", items[idx] as char)).style(if idx == cursor_pos {
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
