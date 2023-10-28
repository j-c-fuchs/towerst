use std::{io::Stdout, time::Duration};

use anyhow::Result;
use crossterm::{
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::CrosstermBackend;

use crate::{app::App, event::EventHandler, ui};

pub type Frame<'a> = ratatui::Frame<'a, CrosstermBackend<Stdout>>;
pub type Terminal = ratatui::Terminal<CrosstermBackend<Stdout>>;

pub struct Tui {
    stdout: Stdout,
    terminal: Terminal,
    pub events: EventHandler,
}

impl Tui {
    pub fn create(tick_rate: Duration) -> Result<Self> {
        let stdout = std::io::stdout();
        let terminal = Terminal::new(CrosstermBackend::new(std::io::stdout()))?;
        let events = EventHandler::new(tick_rate);
        let mut ui = Self {
            stdout,
            terminal,
            events,
        };
        ui.init()?;
        Ok(ui)
    }

    fn init(&mut self) -> Result<()> {
        execute!(self.stdout, EnterAlternateScreen)?;
        terminal::enable_raw_mode()?;
        self.terminal.clear()?;
        Ok(())
    }

    pub fn draw(&mut self, app: &mut App) -> Result<()> {
        self.terminal.draw(|frame| ui::render(app, frame))?;
        Ok(())
    }

    fn cleanup(&mut self) -> Result<()> {
        self.terminal.clear()?;
        terminal::disable_raw_mode()?;
        execute!(self.stdout, LeaveAlternateScreen)?;
        Ok(())
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        let _ = self.cleanup();
    }
}
