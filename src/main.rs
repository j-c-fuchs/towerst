use std::time::Duration;

use anyhow::Result;

use crate::game::Game;
use crate::tui::Tui;

mod app;
mod event;
mod game;
mod tui;

fn main() -> Result<()> {
    let tick_rate = Duration::from_millis(100);
    let mut tui = Tui::create(tick_rate)?;
    let game = Game::empty((5usize, 5usize));
    loop {
        tui.show_game(&game)?;
        if tui.handle_keypress()? {
            break;
        }
    }
    Ok(())
}
