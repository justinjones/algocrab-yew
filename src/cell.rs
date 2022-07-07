#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellType {
    Empty,
    Visited,
    Current,
    Origin,
    Target,
    Wall,
    Path,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cell {
    pub cell_type: CellType,
}

impl Cell {}
