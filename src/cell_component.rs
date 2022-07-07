use yew::html;
use yew::prelude::*;

use crate::cell::CellType;

pub enum CellMsg {
    SelectOrigin,
    HandleActivity,
}

#[derive(PartialEq, Properties)]
pub struct CellProps {
    pub idx: usize,
    pub refresh_callback: Callback<()>,
    pub click_callback: Callback<()>,
    pub activity_callback: Callback<()>,
    pub cell_type: CellType,
}

pub struct CellModel {
    cell_type: CellType,
}

impl CellModel {
    pub fn class_name(&self) -> String {
        match self.cell_type {
            CellType::Empty => String::from("empty"),
            CellType::Visited => String::from("visited"),
            CellType::Current => String::from("current"),
            CellType::Origin => String::from("origin"),
            CellType::Target => String::from("target"),
            CellType::Wall => String::from("wall"),
            CellType::Path => String::from("path"),
        }
    }
}

impl Component for CellModel {
    type Message = CellMsg;
    type Properties = CellProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            cell_type: ctx.props().cell_type,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(move |_| CellMsg::SelectOrigin);
        let onmousedown = ctx.link().callback(move |_| CellMsg::HandleActivity);
        let onmouseenter = ctx.link().callback(move |_| CellMsg::HandleActivity);
        html! {
            <div id={ctx.props().idx.to_string()} key={ctx.props().idx.to_string()} class={classes!("cell", "ba", self.class_name())} onclick={onclick} onmousedown={onmousedown} onmouseenter={onmouseenter}>
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CellMsg::SelectOrigin => {
                ctx.props().click_callback.emit(());
                ctx.props().refresh_callback.emit(());
                false
            }
            CellMsg::HandleActivity => {
                ctx.props().activity_callback.emit(());
                false
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        if ctx.props().cell_type != self.cell_type {
            self.cell_type = ctx.props().cell_type;
            return true;
        }
        false
    }
}
