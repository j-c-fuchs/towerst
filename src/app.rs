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

    pub fn quit(&mut self) {
        self.should_quit = true
    }
}
