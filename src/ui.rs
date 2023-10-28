use ratatui::prelude::Alignment;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType, Borders};

use crate::app::App;
use crate::tui::Frame;

pub fn render(app: &mut App, f: &mut Frame) {
    let widget = app
        .game
        .render()
        .block(
            Block::default()
                .title("Towers")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::LightGreen))
        .alignment(Alignment::Center);
    f.render_widget(widget, f.size())
}
