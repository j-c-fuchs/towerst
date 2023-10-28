use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

use crate::game::{Game, Size};

#[derive(Debug)]
pub struct App {
    pub game: Game,
    pub should_quit: bool,
}

impl App {
    pub fn new<T: Into<Size>>(size: T) -> Self {
        Self {
            game: Game::empty(size),
            should_quit: false,
        }
    }

    pub fn handle_keypress(&mut self, key_event: KeyEvent) {
        match key_event {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers,
                kind: KeyEventKind::Press,
                state,
            } => self.quit(),
            _ => {}
        };
    }

    pub fn quit(&mut self) {
        self.should_quit = true
    }
}
