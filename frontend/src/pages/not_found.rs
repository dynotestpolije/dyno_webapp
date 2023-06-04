use yew::{html, Component};

#[derive(Debug, Clone)]
pub struct PageNotFound;

impl Component for PageNotFound {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        use yew_icons::{Icon, IconId};
        html! {
        <div className="hero h-4/5 bg-base-200">
            <div className="hero-content text-accent text-center">
                <div className="max-w-md">
                <Icon icon_id={IconId::HeroiconsSolidFaceFrown} class="h-48 w-48 inline-block"/>
                <h1 className="text-5xl  font-bold">{"404 - Not Found"}</h1>
                </div>
            </div>
        </div>
        }
    }
}
