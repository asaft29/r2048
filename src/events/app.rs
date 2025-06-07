use crate::events::event::{AppEvent, Event, EventHandler};

use crossterm::event::KeyEventKind;


use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
};

use r2048::game_logic::*;

/// Application.

#[derive(Debug)]

pub struct App {
    /// Is the application running?
    pub running: bool,

    pub state: State,

    pub selected_button: usize,

    pub board: Board,

    /// Event handler.
    pub events: EventHandler,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,

            state: State::Menu,

            selected_button: 0,

            board: Board::new(),

            events: EventHandler::new(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].

    pub fn new() -> Self {
        Self::default()
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

                        self.board.init_board();
                    }

                    1 => self.events.send(AppEvent::Quit),

                    _ => {}
                },

                _ => {}
            },

            State::Playing => match key_event.code {
                KeyCode::Down => {
                    self.board.move_all_down();
                    self.board.spawn_one_random();
                }

                KeyCode::Up => {
                    self.board.move_all_up();
                    self.board.spawn_one_random();
                }

                KeyCode::Right => {
                    self.board.move_all_right();
                    self.board.spawn_one_random();
                }

                KeyCode::Left => {
                    self.board.move_all_left();
                    self.board.spawn_one_random();
                }

                KeyCode::Esc | KeyCode::Char('q') => self.state = State::Menu, 

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
