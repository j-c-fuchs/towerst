use ratatui::{
    layout::Constraint,
    prelude::{Direction, Layout},
};

use crate::app::App;
use crate::tui::Frame;

pub fn render(app: &mut App, f: &mut Frame) {
    let mut constraints = Vec::new();
    constraints.push(Constraint::Min(1));
    constraints.append(&mut vec![Constraint::Length(3); app.game.size().height]);
    constraints.push(Constraint::Min(1));
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(f.size());
    let charts = app.game.render();
    for (i, chart) in charts.into_iter().enumerate() {
        f.render_widget(chart, chunks[i + 1]);
    }
}
