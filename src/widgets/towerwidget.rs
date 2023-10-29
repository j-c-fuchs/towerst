use ratatui::{
    prelude::{Buffer, Rect},
    style::Style,
    widgets::Widget,
};

const SYMBOLS: [char; 9] = ['█', '▇', '▆', '▅', '▄', '▃', '▂', '▁', ' '];

pub struct TowerWidget {
    tower: Option<usize>,
    size: usize,
    tower_style: Option<Style>,
    value_style: Option<Style>,
}

impl TowerWidget {
    pub fn new(tower: Option<usize>, size: usize) -> Self {
        Self {
            tower,
            size,
            tower_style: None,
            value_style: None,
        }
    }

    pub fn tower_style(mut self, style: Style) -> Self {
        self.tower_style = Some(style);
        self
    }
    pub fn value_style(mut self, style: Style) -> Self {
        self.value_style = Some(style);
        self
    }

    fn get_value_style(&self, filled_eights: u64) -> Style {
        match self.value_style {
            Some(s) => s,
            None => {
                let mut value_style = self.tower_style.unwrap_or_default();
                if filled_eights > 4 {
                    (value_style.fg, value_style.bg) = (value_style.bg, value_style.fg);
                };
                value_style
            }
        }
    }
}

impl Widget for TowerWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let total_eights = 8 * (area.height as u64);
        let filled_eights = match self.tower {
            None => 0,
            Some(h) => (h as u64) * total_eights / (self.size as u64),
        };
        let mut empty_top = total_eights.saturating_sub(filled_eights);

        for y in 0..area.height {
            let symbol = SYMBOLS[empty_top.min(8) as usize];
            empty_top = empty_top.saturating_sub(8);

            let string: String = std::iter::repeat(symbol)
                .take(area.width as usize)
                .collect();
            let style = self.tower_style.unwrap_or_default();
            buf.set_string(area.x, area.y + y, string, style);
        }

        if let Some(h) = self.tower {
            let value_string = format!("{}", h);
            let value_width = value_string.len();
            let start_x = area
                .right()
                .saturating_sub(area.width.saturating_sub(1) / 2)
                .saturating_sub(value_width.try_into().unwrap_or(u16::MAX));
            buf.set_string(
                start_x,
                area.bottom() - 1,
                value_string,
                self.get_value_style(filled_eights),
            );
        }
    }
}
