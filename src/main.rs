mod grid;
mod algorithms;

use yew::prelude::*;

use crate::grid::Grid;
use crate::grid::Cell;
use crate::algorithms::fan_out::FanOut;

struct GridModel {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    grid: Grid,
}

impl GridModel {
    fn view_cell(&self, idx: usize, cell: &Cell) -> Html {
        html!{
            <div key=idx class=classes!("cell", "ba")>
            </div>
        }
    }
}

impl Component for GridModel {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            grid: Grid::new()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
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
                    <div key=y class="grid-row">
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
