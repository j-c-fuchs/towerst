use ratatui::{
    prelude::{Buffer, Rect},
    style::Style,
    widgets::{Bar, BarChart, BarGroup, Widget},
};

use crate::game::Game;

pub struct GameWidget {
    // TODO make game a reference so that we don't have to clone it each time?
    game: Game,
    tower_style: Option<Style>,
    value_style: Option<Style>,
}

impl GameWidget {
    pub fn new(game: &Game) -> Self {
        Self {
            game: game.clone(),
            tower_style: None,
            value_style: None,
        }
    }

    pub fn tower_style(mut self, style: Style) -> GameWidget {
        self.tower_style = Some(style);
        self
    }
    pub fn value_style(mut self, style: Style) -> GameWidget {
        self.value_style = Some(style);
        self
    }

    pub fn get_value_style(&self) -> Style {
        self.value_style.unwrap_or_else(|| {
            let tower_style = self.tower_style.unwrap_or_default();
            let mut value_style = tower_style.clone();
            value_style.fg = tower_style.bg;
            value_style.bg = tower_style.fg;
            value_style
        })
    }

    fn row_widget(&self, row: usize) -> Option<BarChart> {
        let data: Vec<_> = self
            .game
            .row(row)?
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
        Some(
            BarChart::default()
                .bar_width(3)
                .bar_gap(1)
                .bar_style(self.tower_style.unwrap_or_default())
                .value_style(self.get_value_style())
                .data(BarGroup::default().bars(&bars))
                .max(self.game.size() as u64),
        )
    }
}

impl Widget for GameWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let row_height = area.height / (self.game.size() as u16);
        if row_height == 0 {
            if area.height > 0 {
                buf.set_string(
                    area.x,
                    area.y,
                    "error: not enough space to show the grid".to_owned(),
                    Style::default(),
                );
            }
            return;
        }

        let render_height = if row_height > 3 {
            // leave some space between the rows.
            row_height - 1
        } else {
            row_height
        };

        let collect_rows: Option<Vec<BarChart>> = (0..self.game.size())
            .map(|row| self.row_widget(row))
            .collect();

        if let Some(rows) = collect_rows {
            let mut inner_area = area.clone();
            inner_area.height = render_height;
            for row_widget in rows.into_iter() {
                row_widget.render(inner_area.clone(), buf);
                inner_area.y += render_height;
            }
        } else {
            buf.set_string(
                area.x,
                area.y,
                "error: unable to collect rows".to_owned(),
                Style::default(),
            );
        }
    }
}
