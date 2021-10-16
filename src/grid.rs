
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Visited,
    Current,
    Origin,
    Target,
    Wall,
}

#[derive(Clone, Debug)]
pub struct Grid {
    pub width: i32,
    pub height: i32,
    pub cells: Vec<Cell>
}

impl Grid {
    pub fn new() -> Grid {
        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map({ |i| Cell::Empty })
            .collect();

        Grid {
            width,
            height,
            cells
        }
    }

    fn get_index(&self, row: i32, column: i32) -> usize {
        (row * self.width + column) as usize
    }

    fn get_cell(&self, row: i32, column: i32) -> Cell {
        return self.cells[self.get_index(row, column)]
    }

    fn neighbours(&self, row: i32, column: i32) -> Vec<Cell> {
        let mut neighbours = Vec::new();

        let steps = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (y, x) in steps {
            // Check out-of-bounds
            if row - y < 0 || row + y >= self.height {
                continue;
            }
            if column - x < 0 || column + x >= self.width {
                continue;
            }
            neighbours.push(self.get_cell(row + y, column + x));
        }

        return neighbours
    }
}
