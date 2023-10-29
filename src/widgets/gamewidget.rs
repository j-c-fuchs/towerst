use ratatui::{
    prelude::{Buffer, Rect},
    style::Style,
    widgets::{Bar, BarChart, BarGroup, Widget},
};

use crate::game::Game;

use super::{towerwidget, TowerWidget};

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
        let render_width = area.width / (self.game.size() as u16);
        let render_height = area.height / (self.game.size() as u16);
        if render_height == 0 {
            if area.height > 0 {
                buf.set_stringn(
                    area.x,
                    area.y,
                    "error: not enough space to show the grid".to_owned(),
                    area.width as usize,
                    Style::default(),
                );
            }
            return;
        }

        let tower_width = render_width.saturating_sub(1).max(1);
        let tower_height = render_height.saturating_sub(if render_height > 3 { 1 } else { 0 });

        let mut grid_x = 0;
        let mut grid_y = 0;
        for tower in self.game.board().iter() {
            let tower_widget = TowerWidget::new(*tower, self.game.size());
            let tower_widget = if let Some(tower_style) = self.tower_style {
                tower_widget.tower_style(tower_style)
            } else {
                tower_widget
            };
            let tower_widget = if let Some(value_style) = self.value_style {
                tower_widget.value_style(value_style)
            } else {
                tower_widget
            };

            let tower_area = Rect {
                x: area.x + grid_x * render_width,
                y: area.y + grid_y * render_height,
                width: tower_width,
                height: tower_height,
            };

            grid_x += 1;
            if grid_x as usize >= self.game.size() {
                grid_x = 0;
                grid_y += 1;
            }

            tower_widget.render(tower_area, buf);
        }
    }
}
