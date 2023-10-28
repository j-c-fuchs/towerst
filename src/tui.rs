use std::{io::Stdout, time::Duration};

use anyhow::Result;
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::CrosstermBackend, widgets::Paragraph};

use crate::event::EventHandler;
use crate::game::Game;

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

    fn show_hello(&mut self) -> Result<()> {
        self.terminal
            .draw(|frame| frame.render_widget(Paragraph::new("Hello, World!"), frame.size()))?;
        Ok(())
    }

    pub fn show_game(&mut self, game: &Game) -> Result<()> {
        let widget = game.render();
        self.terminal
            .draw(|frame| frame.render_widget(widget, frame.size()))?;
        Ok(())
    }

    pub fn handle_keypress(&mut self) -> Result<bool> {
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

impl Drop for Tui {
    fn drop(&mut self) {
        let _ = self.cleanup();
    }
}
