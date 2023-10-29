use std::time::Duration;

use anyhow::Result;
use game::Game;

use crate::app::App;
use crate::event::Event;
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
    app.game = Game::random(app.game.size());
    tui.init()?;
    while !app.should_quit {
        tui.draw(&mut app)?;
        if let Event::Key(e) = tui.events.next()? {
            app.handle_keypress(e);
        }
    }
    tui.cleanup()?;
    Ok(())
}
