use yew::{html, Component};

pub struct PageAdminDynos {}

impl Component for PageAdminDynos {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <h1>{"SOP PAGES NOT IMPLEMENTED YET"}</h1>
        }
    }
}