use yew::{html, Component};

pub struct PageSop {}

impl Component for PageSop {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <h1>{"SOP PAGE NOT IMPLEMENTED YET"}</h1>
        }
    }
}
