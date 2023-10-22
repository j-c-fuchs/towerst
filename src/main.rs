use std::io::{stdout, Result, Stdout};

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::{CrosstermBackend, Terminal},
    widgets::Paragraph,
};

use crate::game::Game;

mod game;

struct UI {
    pub(crate) stdout: Stdout,
    pub(crate) terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl UI {
    pub(crate) fn create() -> Result<Self> {
        let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        let stdout = stdout();
        let mut ui = Self { stdout, terminal };
        ui.init()?;
        Ok(ui)
    }

    fn init(&mut self) -> Result<()> {
        execute!(self.stdout, EnterAlternateScreen)?;
        terminal::enable_raw_mode()?;
        self.terminal.clear()?;
        Ok(())
    }

    fn show_hello(&mut self) -> Result<()> {
        self.terminal
            .draw(|frame| frame.render_widget(Paragraph::new("Hello, World!"), frame.size()))?;
        Ok(())
    }

    fn show_game(&mut self, game: &Game) -> Result<()> {
        let widget = game.render();
        self.terminal
            .draw(|frame| frame.render_widget(widget, frame.size()))?;
        Ok(())
    }

    fn handle_keypress(&mut self) -> Result<bool> {
        if event::poll(std::time::Duration::from_millis(100))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }

    fn cleanup(&mut self) -> Result<()> {
        self.terminal.clear()?;
        terminal::disable_raw_mode()?;
        execute!(self.stdout, LeaveAlternateScreen)?;
        Ok(())
    }
}

impl Drop for UI {
    fn drop(&mut self) {
        let _ = self.cleanup();
    }
}

fn main() -> Result<()> {
    let mut ui = UI::create()?;
    let game = Game::empty((5usize, 5usize));
    loop {
        ui.show_game(&game)?;
        if ui.handle_keypress()? {
            break;
        }
    }
    Ok(())
}
