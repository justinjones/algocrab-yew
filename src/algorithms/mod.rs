pub mod fan_out;
use crate::grid::Grid;

pub trait Algorithm {
    fn tick(&mut self) -> Grid;
    fn is_finished(&self) -> bool;
    fn path(&self) -> Vec<usize>;
}
