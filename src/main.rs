mod algorithms;
mod cell;
mod cell_component;
mod grid;
mod grid_component;

use crate::grid_component::GridModel;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<GridModel>();
}
