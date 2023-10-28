use ratatui::{
    prelude::{Alignment, Rect},
    widgets::{Block, BorderType, Borders, Paragraph},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Size {
    width: usize,
    height: usize,
}

impl From<Rect> for Size {
    fn from(value: Rect) -> Self {
        Self {
            width: value.width as usize,
            height: value.height as usize,
        }
    }
}

impl<T: Into<usize>> From<(T, T)> for Size {
    fn from(value: (T, T)) -> Self {
        Self {
            width: value.0.into(),
            height: value.1.into(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Game {
    size: Size,
    constraints: Vec<Option<usize>>,
    board: Vec<Option<usize>>,
}

impl Game {
    pub fn empty<T: Into<Size>>(size: T) -> Self {
        let size = size.into();
        Self {
            size,
            constraints: vec![None; 2 * (size.width + size.height) as usize],
            board: vec![None; (size.width * size.height) as usize],
        }
    }

    pub fn render(&self) -> Paragraph<'_> {
        Paragraph::new("game board")
            .block(
                Block::default()
                    .borders(Borders::all())
                    .border_type(BorderType::Rounded),
            )
            .alignment(Alignment::Center)
    }
}
