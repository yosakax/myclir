use crate::completion::Completion;
use crate::query::establish_connection;
use sqlx::mysql::MySqlPool;

// use crate::query::MyConnection;
use crate::ui::Cursor;
use crate::ui::{StatefulList, TableComponent};
use std::error;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub enum Mode {
    Normal,
    Insert,
    Command,
}

pub enum Window {
    Editor,
    Results,
    Command,
}

/// Application.
// #[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub mode: Mode,
    pub is_popup: bool,
    pub text: Vec<String>,
    pub line_number: usize,
    pub cursor: Cursor,
    pub edit_area: (u16, u16),
    pub result: Vec<Vec<String>>,
    pub conn: MySqlPool,
    pub databases: StatefulList<String>,
    pub completion: Completion,
    pub window: Window,
    pub table_components: TableComponent,
    pub popup_command: bool,
    pub command_input: String,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let connection = establish_connection().await?;
        let app = Self {
            running: true,
            mode: Mode::Normal,
            is_popup: false,
            text: vec![String::new()],
            line_number: 0,
            cursor: Cursor::default(),
            edit_area: (0, 0),
            result: vec![],
            conn: connection,
            databases: StatefulList::new(),
            completion: Completion::new(),
            window: Window::Editor,
            table_components: TableComponent::new(),
            popup_command: false,
            command_input: String::new(),
        };
        Ok(app)
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
