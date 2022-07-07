use crate::cell::{Cell, CellType};

#[derive(Clone, Debug)]
pub struct Grid {
    pub width: i32,
    pub height: i32,
    pub cells: Vec<Cell>,
}

impl Grid {
    pub fn new() -> Grid {
        let width = 48;
        let height = 32;

        let cells = (0..width * height)
            .map(|_i| Cell {
                cell_type: CellType::Empty,
            })
            .collect();

        Grid {
            width,
            height,
            cells,
        }
    }

    pub fn get_index(&self, row: i32, column: i32) -> usize {
        (row * self.width + column) as usize
    }

    pub fn reverse_index(&self, idx: usize) -> (i32, i32) {
        (idx as i32 / self.width, idx as i32 % self.width)
    }

    pub fn neighbours(&self, row: i32, column: i32) -> Vec<usize> {
        let mut neighbours = Vec::new();

        let steps = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (y, x) in steps {
            // Check out-of-bounds
            if row + y < 0 || row + y >= self.height {
                continue;
            }
            if column + x < 0 || column + x >= self.width {
                continue;
            }
            neighbours.push(self.get_index(row + y, column + x));
        }

        return neighbours;
    }
}
