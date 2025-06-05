use crate::events::event::{AppEvent, Event, EventHandler};
use crossterm::event::KeyEventKind;
use rand::seq::IteratorRandom;
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
};
/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,

    pub state: State,

    pub selected_button: usize,

    pub board: [[u32; 4]; 4],

    /// Event handler.
    pub events: EventHandler,
}
#[derive(Debug)]
pub enum State {
    Menu,
    Playing,
    Done,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            state: State::Menu,
            selected_button: 0,
            board: [[0; 4]; 4],
            events: EventHandler::new(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    fn init_board(&mut self) {
        let mut rng = rand::rng();

        let empty_positions: Vec<(usize, usize)> = (0..4)
            .flat_map(|row| (0..4).map(move |col| (row, col)))
            .collect();

        for &(row, col) in empty_positions.iter().choose_multiple(&mut rng, 2).iter() {
            self.board[*row][*col] = 2;
        }
    }
    fn move_all_down(&mut self) {
        for i in 0..self.board.len() {
            let mut stack: Vec<(u32, usize)> = Vec::new();
            let mut j = 0;
            while j < self.board[0].len() {
                stack.push((self.board[j][i], j));
                j += 1;
            }
            while let Some(value) = stack.pop() {
                let mut index = value.1;
                while index + 1 < j {
                    if self.board[index + 1][i] == 0 {
                        self.board[index + 1][i] = self.board[index][i];
                        self.board[index][i] = 0;
                        index += 1;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    fn move_all_up(&mut self) {
        for i in 0..self.board.len() {
            let mut stack: Vec<(u32, usize)> = Vec::new();
            let mut j = self.board[0].len() - 1;
            while j > 0 {
                stack.push((self.board[j][i], j));
                j -= 1;
            }
            while let Some(value) = stack.pop() {
                let mut index = value.1;
                while index > 0 {
                    if self.board[index - 1][i] == 0 {
                        self.board[index - 1][i] = self.board[index][i];
                        self.board[index][i] = 0;
                        index -= 1;
                    } else {
                        break;
                    }
                }
            }
        }
    }
    fn move_all_right(&mut self) {
        for i in 0..self.board.len() {
            let mut stack: Vec<(u32, usize)> = Vec::new();
            let mut j = 0;
            while j < self.board[0].len() {
                stack.push((self.board[i][j], j));
                j += 1;
            }
            while let Some(value) = stack.pop() {
                let mut index = value.1;
                while index + 1 < j {
                    if self.board[i][index + 1] == 0 {
                        self.board[i][index + 1] = self.board[i][index];
                        self.board[i][index] = 0;
                        index += 1;
                    } else {
                        break;
                    }
                }
            }
        }
    }

        fn move_all_left(&mut self) {
        for i in 0..self.board.len() {
            let mut stack: Vec<(u32, usize)> = Vec::new();
            let mut j = self.board[0].len() - 1;
            while j > 0 {
                stack.push((self.board[i][j], j));
                j -= 1;
            }
            while let Some(value) = stack.pop() {
                let mut index = value.1;
                while index > 0 {
                    if self.board[i][index - 1] == 0 {
                        self.board[i][index - 1] = self.board[i][index];
                        self.board[i][index] = 0;
                        index -= 1;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            self.handle_events()?;

            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
        }
        Ok(())
    }

    pub fn handle_events(&mut self) -> color_eyre::Result<()> {
        match self.events.next()? {
            Event::Tick => self.tick(),
            Event::Crossterm(event) => match event {
                crossterm::event::Event::Key(key_event) => self.handle_key_event(key_event)?,
                _ => {}
            },
            Event::App(app_event) => match app_event {
                AppEvent::Quit => self.quit(),
            },
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        if key_event.kind != KeyEventKind::Press {
            return Ok(());
        }

        match self.state {
            State::Menu => match key_event.code {
                KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
                KeyCode::Char('c') if key_event.modifiers == KeyModifiers::CONTROL => {
                    self.events.send(AppEvent::Quit)
                }
                KeyCode::Left | KeyCode::Char('h') => {
                    if self.selected_button > 0 {
                        self.selected_button -= 1;
                    }
                }
                KeyCode::Right | KeyCode::Char('l') => {
                    if self.selected_button < 1 {
                        self.selected_button += 1;
                    }
                }

                KeyCode::Char('e') | KeyCode::Char('E') => match self.selected_button {
                    0 => {
                        self.state = State::Playing;
                        self.init_board();
                    }
                    1 => self.events.send(AppEvent::Quit),
                    _ => {}
                },
                _ => {}
            },
            State::Playing => match key_event.code {
                KeyCode::Down => self.move_all_down(),
                KeyCode::Up => self.move_all_up(),
                KeyCode::Right => self.move_all_right(),
                KeyCode::Left => self.move_all_left(), 
                KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
                _ => {}
            },
            _ => {
                // For other states, handle quit keys
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
                    KeyCode::Char('c') if key_event.modifiers == KeyModifiers::CONTROL => {
                        self.events.send(AppEvent::Quit)
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
