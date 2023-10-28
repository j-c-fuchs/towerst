use std::time::Duration;

use anyhow::Result;

use crate::app::App;
use crate::tui::Tui;

mod app;
mod event;
mod game;
mod tui;
mod ui;

fn main() -> Result<()> {
    let tick_rate = Duration::from_millis(100);
    let mut tui = Tui::create(tick_rate)?;
    let mut app = App::new((5usize, 5usize));
    tui.init()?;
    while !app.should_quit {
        tui.draw(&mut app)?;
    }
    tui.cleanup()?;
    Ok(())
}
