use crate::grid::Grid;
use crate::grid::Cell;
use crate::algorithms::Algorithm;

pub struct FanOut {
    grid: Grid,
    start: Cell,
    current: Vec<Cell>
}

impl FanOut {
    pub fn new(grid: Grid, start: Cell) -> FanOut {
        Self {
            grid: grid.clone(),
            start,
            current: vec![start]
        }
    }
}

impl Algorithm for FanOut {
    fn tick(&self) -> Grid {
        self.grid.clone()
    }
}
