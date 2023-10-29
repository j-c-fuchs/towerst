use ratatui::{
    style::{Style, Stylize},
    widgets::{Bar, BarChart, BarGroup},
};

use super::*;

impl Game {
    pub fn render(&self) -> Vec<BarChart> {
        (0..self.size).map(|row| self.render_row(row)).collect()
    }

    fn render_row(&self, row: usize) -> BarChart {
        let data: Vec<_> = self.board[row * self.size..(row + 1) * self.size]
            .iter()
            .map(|tower| match tower {
                Some(height) => (format!("{}", height), *height as u64),
                None => (format!(" "), 0u64),
            })
            .collect();
        let bars: Vec<_> = data
            .into_iter()
            .map(|(s, h)| Bar::default().value(h).text_value(s))
            .collect();
        BarChart::default()
            .bar_width(3)
            .bar_gap(1)
            .bar_style(Style::new().green().on_black())
            .value_style(Style::new().black().on_green())
            .data(BarGroup::default().bars(&bars))
            .max(self.size as u64)
    }
}
