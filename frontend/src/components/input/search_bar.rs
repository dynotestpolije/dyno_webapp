use web_sys::{Event, HtmlInputElement};
use yew::{classes, html, AttrValue, Callback, Classes, Component, Properties, TargetCast};

#[derive(PartialEq, Properties)]
pub struct SearchBarProps {
    pub value: AttrValue,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub placeholder: AttrValue,

    pub update_callback: Callback<String>,
}

pub struct SearchBar;

impl Component for SearchBar {
    type Message = ();
    type Properties = SearchBarProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let SearchBarProps {
            value,
            class: style_class,
            placeholder,
            update_callback,
        } = ctx.props();

        let update_callback = update_callback.clone();
        html! {
            <div class={classes!("inline-block", style_class.clone())}>
                <div class="input-group  relative flex flex-wrap items-stretch w-full">
                <input
                    type="search"
                    value={value.clone()}
                    placeholder={placeholder.clone()}
                    onchange={move |e: Event| {
                        e.prevent_default();
                        let input = e.target_dyn_into::<HtmlInputElement>();
                        if let Some(input) = input {
                            update_callback.emit(input.value());
                        }
                    }}
                    class="input input-sm input-bordered  w-full max-w-xs" />
                </div>
            </div>
        }
    }
}
