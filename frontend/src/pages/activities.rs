use yew::{html, Component};

pub struct PageActivities {}

impl Component for PageActivities {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <h1>{"PAGES NOT IMPLEMENTED YET"}</h1>
        }
    }
}
