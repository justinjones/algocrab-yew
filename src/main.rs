mod grid;
mod algorithms;

use yew::prelude::*;

use crate::grid::Grid;
use crate::grid::Cell;
//use crate::algorithms::fan_out::FanOut;

struct GridModel {
    grid: Grid,
}

impl GridModel {
    fn view_cell(&self, idx: usize, _cell: &Cell) -> Html {
        html!{
            <div key={idx} class={classes!("cell", "ba")}>
            </div>
        }
    }
}

impl Component for GridModel {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            grid: Grid::new()
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let cell_rows = self.grid.cells
            .chunks(self.grid.width as usize)
            .enumerate()
            .map(|(y, cells)| {
                let idx_offset = y * self.grid.width as usize;

                let cell_views = cells
                    .iter()
                    .enumerate()
                    .map(|(x, cell)| self.view_cell(idx_offset + x, cell));
                html! {
                    <div key={y} class="grid-row">
                        { for cell_views }
                    </div>
                }
            });
        html! {
            <div id="grid">
                { "Hello World" }
                { for cell_rows }
            </div>
        }
    }
}

fn main() {
    yew::start_app::<GridModel>();
}
