use ratatui::style::{Color, Style};

use crate::app::App;
use crate::gamewidget::GameWidget;
use crate::tui::Frame;

pub fn render(app: &mut App, f: &mut Frame) {
    let widget =
        GameWidget::new(&app.game).tower_style(Style::default().fg(Color::Green).bg(Color::Black));
    f.render_widget(widget, f.size())
}
