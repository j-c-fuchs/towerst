use rand::prelude::*;
use ratatui::{
    prelude::{Alignment, Rect},
    widgets::{Block, BorderType, Borders, Paragraph},
};

mod render;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Size {
    pub width: usize,
    pub height: usize,
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

    pub fn size(&self) -> Size {
        self.size
    }

    // TODO: make Game correct
    pub fn random<T: Into<Size>>(size: T) -> Self {
        let mut rng = rand::thread_rng();
        let size = size.into();
        let constraints = (0..2 * (size.width + size.height))
            .map(|_| Some(rng.gen_range(1..size.width + 1)))
            .collect();
        let board = (0..size.width * size.height)
            .map(|_| Some(rng.gen_range(1..size.width + 1)))
            .collect();
        Self {
            size,
            constraints,
            board,
        }
    }
}
