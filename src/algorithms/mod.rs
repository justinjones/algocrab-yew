pub mod fan_out;
use crate::algorithms::fan_out::FanOut;
use crate::grid::Grid;

pub trait Algorithm {
    fn tick(&self) -> Grid;
}
