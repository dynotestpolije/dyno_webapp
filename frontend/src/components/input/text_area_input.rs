use web_sys::{Event, HtmlInputElement};
use yew::{classes, html, AttrValue, Callback, Classes, Component, Properties, TargetCast};

#[derive(PartialEq, Properties)]
pub struct InputTextAreaProps {
    pub title: AttrValue,
    pub value: AttrValue,
    #[prop_or_default]
    pub container_class: Classes,
    #[prop_or_default]
    pub label_class: Classes,
    #[prop_or_default]
    pub placeholder: AttrValue,

    pub update_callback: Callback<String>,
}

pub struct TextInputArea;

impl Component for TextInputArea {
    type Message = ();
    type Properties = InputTextAreaProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let InputTextAreaProps {
            title,
            value,
            container_class,
            label_class,
            placeholder,
            update_callback,
        } = ctx.props();
        let update_callback = update_callback.clone();

        html! {
            <div class={classes!("form-control", "w-full", container_class.clone())}>
                <label class="label">
                    <span class={classes!("label-text", "text-base-content", label_class.clone())}>
                        {title.clone()}
                    </span>
                </label>
                <textarea
                    value={value.clone()}
                    class="textarea textarea-bordered w-full"
                    placeholder={placeholder.clone()}
                    onchange={move |e: Event| {
                        e.prevent_default();
                        let input = e.target_dyn_into::<HtmlInputElement>();
                        if let Some(input) = input {
                            update_callback.emit(input.value().into());
                        }
                    }}
                >
                </textarea>
            </div>
        }
    }
}
