use web_sys::HtmlInputElement;
use yew::prelude::*;

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

pub struct InputTextArea;

impl Component for InputTextArea {
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

        html! {
            <div class={classes!("form-control", "w-full", *container_class)}>
                <label class="label">
                    <span class={classes!("label-text", "text-base-content", *label_class)}>
                        {title}
                    </span>
                </label>
                <textarea
                    value={value}
                    class="textarea textarea-bordered w-full"
                    placeholder={placeholder}
                    onchange={|e: Event| {
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
