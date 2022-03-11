use queues::*;
use std::collections::HashMap;

use crate::algorithms::Algorithm;
use crate::cell::CellType;
use crate::grid::Grid;

pub struct FanOut {
    grid: Grid,
    _origin: usize,
    _target: usize,
    current: Queue<usize>,
    visited: HashMap<usize, usize>,
    finished: bool,
    path: Vec<usize>,
}

impl FanOut {
    pub fn new(grid: Grid, _origin: usize, _target: usize) -> FanOut {
        Self {
            grid: grid,
            _origin,
            _target,
            current: queue![_origin],
            visited: HashMap::new(),
            finished: false,
            path: vec![],
        }
    }

    pub fn unpack_path(&mut self, target: usize) {
        let mut current = target.clone();
        while self.visited.contains_key(&current) {
            let prev = self.visited.get(&current);
            match prev {
                Some(idx) => {
                    self.path.push(*idx);
                    current = *idx;
                }
                _ => {}
            }
        }
        self.path.reverse();
    }
}

impl Algorithm for FanOut {
    fn is_finished(&self) -> bool {
        self.finished
    }

    fn path(&self) -> Vec<usize> {
        self.path.clone()
    }

    fn tick(&mut self) -> Grid {
        let mut next_tick: Queue<usize> = Queue::new();

        while self.current.size() > 0 {
            let next = self.current.remove();
            match next {
                Ok(idx) => {
                    let (y, x) = self.grid.reverse_index(idx);
                    let neighbours = self.grid.neighbours(y, x);
                    for n in neighbours.into_iter() {
                        if self.grid.cells[n].cell_type == CellType::Empty
                            && !self.visited.contains_key(&n)
                        {
                            self.grid.cells[n].cell_type = CellType::Visited;
                            self.visited.insert(n, idx);
                            next_tick.add(n);
                        }
                        if self.grid.cells[n].cell_type == CellType::Target {
                            self.visited.insert(n, idx);
                            self.unpack_path(n);
                            self.finished = true;
                            return self.grid.clone();
                        }
                    }
                }
                _ => {}
            }
        }

        self.current = next_tick;
        self.grid.clone()
    }
}
