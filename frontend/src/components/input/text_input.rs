use web_sys::{Event, HtmlInputElement};
use yew::{classes, html, AttrValue, Callback, Classes, Component, Properties, TargetCast};

#[derive(PartialEq, Properties)]
pub struct TextInputProps {
    #[prop_or_default]
    pub value: Option<AttrValue>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or(AttrValue::from("text"))]
    pub title: AttrValue,
    #[prop_or(AttrValue::from("text"))]
    pub types: AttrValue,
    #[prop_or_default]
    pub placeholder: AttrValue,
    #[prop_or_default]
    pub border: Classes,
    #[prop_or_default]
    pub required: bool,
    #[prop_or(false)]
    pub disabled: bool,
    #[prop_or_default]
    pub update_callback: Callback<AttrValue>,
}

pub struct TextInput;

impl Component for TextInput {
    type Message = ();
    type Properties = TextInputProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let TextInputProps {
            value,
            class,
            title,
            types,
            placeholder,
            required,
            update_callback,
            border,
            disabled,
        } = ctx.props();

        let onupdatevalue = update_callback.clone();

        html! {
        <div class={classes!("form-control", "w-full", class.clone())}>
            <label class="label">
                <span class={"label-text text-base-content"}>
                    {title.clone()}
                </span>
            </label>
            <input
                id={types.clone()}
                name={types.clone()}
                type={types.clone()}
                autoComplete={types.clone()}
                value={value.clone()}
                placeholder={placeholder.clone()}
                onchange={move |e: Event| {
                    e.prevent_default();
                    let input = e.target_dyn_into::<HtmlInputElement>();
                    if let Some(input) = input {
                        onupdatevalue.emit(input.value().into());
                    }
                }}
                class={classes!("input", "input-bordered", "w-full", border.clone())}
                required={*required}
                disabled={*disabled}
            />
        </div>
        }
    }
}
