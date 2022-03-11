use std::collections::HashSet;

use gloo_timers::callback::Interval;
use yew::prelude::*;
use yew::{html, html::Scope};

use crate::cell::CellType;
use crate::grid::Grid;

use crate::cell_component::CellModel;

use crate::algorithms::*;
use crate::algorithms::fan_out::FanOut;

pub enum GridMsg {
    Refresh,
    SelectTool(CellType),
    MouseDown,
    MouseUp,
    CellActivity(usize),
    CellClick(usize),
    Run,
    Tick,
}

pub struct GridModel {
    grid: Grid,
    current_tool: CellType,
    origin_index: usize,
    target_index: usize,
    wall_indexes: HashSet<usize>,
    recording_mouse: bool,
    _interval: Interval,
    running: bool,
    algorithm: Box<dyn Algorithm>,
}

impl GridModel {
    fn toggle_empty(&mut self, idx: usize, cell_type: CellType) {
        if self.grid.cells[idx].cell_type == cell_type {
            self.grid.cells[idx].cell_type = CellType::Empty;
        } else {
            self.grid.cells[idx].cell_type = cell_type;
        }
    }

    fn update_origin(&mut self, idx: usize) {
        self.toggle_empty(idx, self.current_tool);
        if self.origin_index != idx {
            self.grid.cells[self.origin_index].cell_type = CellType::Empty;
        }
        self.origin_index = idx;
    }

    fn update_target(&mut self, idx: usize) {
        self.toggle_empty(idx, self.current_tool);
        if self.target_index != idx {
            self.grid.cells[self.target_index].cell_type = CellType::Empty;
        }
        self.target_index = idx;
    }

    fn update_walls(&mut self, idx: usize) {
        if self.wall_indexes.contains(&idx) {
            self.wall_indexes.remove(&idx);
            self.grid.cells[idx].cell_type = CellType::Empty;
        } else {
            self.wall_indexes.insert(idx);
            self.grid.cells[idx].cell_type = CellType::Wall;
        }
    }

    fn cell_activity(&mut self, idx: usize) {
        match self.current_tool {
            CellType::Origin => self.update_origin(idx),
            CellType::Target => self.update_target(idx),
            CellType::Wall => self.update_walls(idx),
            _ => {}
        }
    }

    fn run(&mut self) {
        self.running = true;
        let algorithm = FanOut::new(self.grid.clone(), self.origin_index, self.target_index);
        self.algorithm = Box::new(algorithm);
    }

    fn button_select_view(&self, tool: CellType, text: &str, link: &Scope<GridModel>) -> Html {
        let callback = link.callback(move |_| GridMsg::SelectTool(tool));
        html! {
            <button type="button" onclick={callback}>{ text }</button>
        }
    }
}

impl Component for GridModel {
    type Message = GridMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(|_| GridMsg::Tick);
        let interval = Interval::new(200, move || callback.emit(()));
        let grid = Grid::new();
        let (y, x) = grid.reverse_index(672);
        log::info!("{} {}", y, x);
        log::info!("{}", grid.get_index(y, x));
        log::info!("{:?}", grid.neighbours(14, 0));
        Self {
            grid: Grid::new(),
            current_tool: CellType::Origin,
            origin_index: 0,
            target_index: 0,
            wall_indexes: HashSet::new(),
            recording_mouse: false,
            _interval: interval,
            running: false,
            algorithm: Box::new(FanOut::new(grid.clone(), 0, 0)),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            GridMsg::SelectTool(tool) => {
                log::info!("Select Tool..");
                self.current_tool = tool;
                false
            }
            GridMsg::MouseDown => {
                log::info!("Mouse Down..");
                self.recording_mouse = true;
                false
            }
            GridMsg::MouseUp => {
                log::info!("Mouse Up..");
                self.recording_mouse = false;
                false
            }
            GridMsg::CellActivity(idx) => {
                if self.recording_mouse {
                    ctx.link().send_message(GridMsg::CellClick(idx));
                }
                false
            }
            GridMsg::CellClick(idx) => {
                self.cell_activity(idx);
                true
            }
            GridMsg::Refresh => true,
            GridMsg::Run => {
                self.run();
                false
            }
            GridMsg::Tick => {
                if self.running {
                    log::info!("Tick..");
                    if self.algorithm.is_finished() {
                        self.running = false;
                    } else {
                        self.grid = self.algorithm.tick();
                        log::info!("{:?}", self.algorithm.path());
                        for idx in self.algorithm.path() {
                            let ct = self.grid.cells[idx].cell_type;
                            if ct != CellType::Origin && ct != CellType::Target {
                                self.grid.cells[idx].cell_type = CellType::Path;
                            }
                        }
                    }
                    return true;
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cell_rows = self.grid.cells
            .chunks(self.grid.width as usize)
            .enumerate()
            .map(|(y, cells)| {
                let idx_offset = y * self.grid.width as usize;

                let cell_views = cells
                    .iter()
                    .enumerate()
                    .map( |(x, cell)| {
                        let idx = idx_offset + x;
                        let refresh_callback = ctx.link().callback(|_| GridMsg::Refresh);
                        let click_callback = ctx.link().callback(move |_| GridMsg::CellClick(idx));
                        let activity_callback = ctx.link().callback(move |_| GridMsg::CellActivity(idx));
                        html! { <CellModel idx={idx} refresh_callback={refresh_callback} cell_type={cell.cell_type} click_callback={click_callback} activity_callback={activity_callback} /> }
                    });
                html! {
                    <div key={y} class="grid-row">
                        { for cell_views }
                    </div>
                }
            });

        let mouse_down = ctx.link().callback(|_| GridMsg::MouseDown);
        let mouse_up = ctx.link().callback(|_| GridMsg::MouseUp);
        let run_callback = ctx.link().callback(|_| GridMsg::Run);
        html! {
            <>
            <div id="button">
                { self.button_select_view(CellType::Origin, "Origin", ctx.link()) }
                { self.button_select_view(CellType::Target, "Target", ctx.link()) }
                { self.button_select_view(CellType::Wall, "Wall", ctx.link()) }
                <button type="button" onclick={run_callback}>{ "Run" }</button>
            </div>
            <div id="grid" onmousedown={mouse_down} onmouseup={mouse_up}>
                { for cell_rows }
            </div>
            </>
        }
    }
}
