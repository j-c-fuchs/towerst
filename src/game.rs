use rand::prelude::*;

mod render;

pub type Size = usize;

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
            constraints: vec![None; 4 * size],
            board: vec![None; size * size],
        }
    }

    pub fn size(&self) -> Size {
        self.size
    }

    // TODO: make Game correct
    pub fn random<T: Into<Size>>(size: T) -> Self {
        let mut rng = rand::thread_rng();
        let size = size.into();
        let constraints = (0..4 * size)
            .map(|_| Some(rng.gen_range(1..size + 1)))
            .collect();
        let board = (0..size * size)
            .map(|_| Some(rng.gen_range(1..size + 1)))
            .collect();
        Self {
            size,
            constraints,
            board,
        }
    }
}
