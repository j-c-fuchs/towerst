use std::{io::Stdout, panic, time::Duration};

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
        Ok(Self {
            stdout,
            terminal,
            events,
        })
    }

    pub fn init(&mut self) -> Result<()> {
        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            Self::reset().expect("failed to reset the terminal");
            panic_hook(panic);
        }));

        execute!(self.stdout, EnterAlternateScreen)?;
        terminal::enable_raw_mode()?;
        self.terminal.clear()?;
        self.terminal.hide_cursor()?;
        Ok(())
    }

    pub fn draw(&mut self, app: &mut App) -> Result<()> {
        self.terminal.draw(|frame| ui::render(app, frame))?;
        Ok(())
    }

    fn reset() -> Result<()> {
        terminal::disable_raw_mode()?;
        execute!(std::io::stdout(), LeaveAlternateScreen)?;
        Ok(())
    }

    pub fn cleanup(&mut self) -> Result<()> {
        Self::reset()?;
        self.terminal.clear()?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}
