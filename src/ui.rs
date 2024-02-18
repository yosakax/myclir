use std::time::Instant;

// use crossterm::style::Stylize;
use crate::app::Mode;
use crate::query;
use itertools::Itertools;
use ratatui::widgets::{Cell, ListItem, Row, Table};
use ratatui::{
    layout::Alignment,
    prelude::*,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Clear, List, ListState, Paragraph, TableState, Wrap},
    Frame,
};

use crate::app::App;

pub struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn new() -> Self {
        StatefulList {
            state: ListState::default(),
            items: vec![],
        }
    }
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i))
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn unselect(&mut self) {
        self.state.select(None);
    }
}

#[derive(Debug, Clone)]
pub struct TableComponent {
    pub state: TableState,
    pub items: Vec<Vec<String>>,
}

impl TableComponent {
    pub fn new() -> Self {
        TableComponent {
            state: TableState::default(),
            items: vec![],
        }
    }
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Cursor {
    pub x: u16,
    pub y: u16,
    pub base_x: u16,
    pub base_y: u16,
}

impl Cursor {
    pub fn new(x: u16, y: u16) -> Self {
        Self {
            x,
            y,
            base_x: x,
            base_y: y,
        }
    }

    pub fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            base_x: 0,
            base_y: 0,
        }
    }

    pub fn initialize(&mut self) {
        self.x = self.base_x;
        self.y = self.base_y;
    }

    pub fn initialize_base(&mut self, base_x: u16, base_y: u16) {
        self.base_x = base_x;
        self.base_y = base_y;
    }
}

fn render_borders(title: &str, text: &str, border: Borders, frame: &mut Frame, area: Rect) {
    let block = Block::new().borders(border).title(title);
    let paragraph = Paragraph::new(text).wrap(Wrap { trim: true });
    frame.render_widget(paragraph.clone().block(block), area)
}

fn render_title(frame: &mut Frame, area: Rect) {
    frame.render_widget(
        Paragraph::new("This is Myclir. Press C-q to quit")
            .dark_gray()
            .alignment(Alignment::Center),
        area,
    );
}

pub fn calculate_layout(area: Rect) -> (Rect, Vec<Vec<Rect>>) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(1), Constraint::Min(0)])
        .split(area);
    let title_area = layout[0];
    // let outer_area = Layout::default()
    //     .direction(Direction::Horizontal)
    //     .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
    //     .split(layout[1])
    //     .to_vec();
    // let outer_area = Layout::default()
    //     .direction(Direction::Horizontal)
    //     .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
    //     .split(layout[1])
    //     .to_vec();
    // let left_area = outer_area.clone();
    let right_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(layout[1])
        .to_vec();
    // let main_area = vec![left_area, right_area];
    let main_area = vec![right_area];

    (title_area, main_area)
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let (title_area, layout) = calculate_layout(frame.size());
    if app.cursor.base_x == 0 {
        app.cursor = Cursor::new(layout[0][0].x + 1, layout[0][0].y + 1);
        app.edit_area = (layout[0][0].width, layout[0][0].height);
    }

    let texts: Vec<ListItem> = app
        .text
        .iter()
        .map(|text| {
            // let content = Line::from(Span::raw(format!("{}", text.to_owned())));
            ListItem::new(text.to_owned())
        })
        .collect();
    let editor_title = match app.mode {
        Mode::Normal => "Normal Mode",
        Mode::Insert => "Insert Mode",
        Mode::Command => "Command Mode",
    };
    let text_list =
        List::new(texts).block(Block::default().borders(Borders::ALL).title(editor_title));

    // lower right
    app.table_components.items = app.result.to_owned();
    render_borders(
        "Result Monitor",
        app.result
            .iter()
            .map(|vec| vec.iter().join(" | "))
            .join("\n")
            .as_str(),
        Borders::ALL,
        frame,
        layout[0][1],
    );

    // completion
    if app.is_popup {
        // popup block
        let block = Block::default().bg(Color::Gray);
        let area = Rect::new(app.cursor.x + 1, app.cursor.y + 1, 20, 5);
        let paragraph =
            Paragraph::new(app.completion.list.iter().join("\n")).wrap(Wrap { trim: true });
        frame.render_widget(Clear, area);
        frame.render_widget(paragraph.clone().block(block), area);
    } else {
        app.cursor.x = app.cursor.base_x;
        app.cursor.y = app.cursor.base_y;
    }

    // command window
    if app.popup_command {
        let area = centered_rect(10, 10, frame.size());
        let paragraph =
            Paragraph::new(Text::raw(app.command_input.as_str())).wrap(Wrap { trim: true });
        let block = Block::default().bg(Color::Green);
        frame.render_widget(paragraph.clone().block(block), area);
        app.cursor.x = area.x;
        app.cursor.y = area.y;
    }

    frame.render_widget(text_list, layout[0][0]);
    frame.set_cursor(app.cursor.x, app.cursor.y);
    render_title(frame, title_area)
}
