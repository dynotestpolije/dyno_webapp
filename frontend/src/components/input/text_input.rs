use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct TextInputProps {
    pub value: AttrValue,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or(AttrValue::from("text"))]
    pub title: AttrValue,
    #[prop_or(AttrValue::from("text"))]
    pub types: AttrValue,
    #[prop_or_default]
    pub placeholder: AttrValue,

    pub update_callback: Callback<String>,
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
            update_callback: onupdatevalue,
        } = ctx.props();

        html! {
        <div class={classes!("form-control", "w-full", class.clone())}>
            <label class="label"> 
                <span class={"label-text text-base-content"}> 
                    {title}
                </span>
            </label>
            <input
                type={types}
                value={value}
                placeholder={placeholder}
                onchange={|e: Event| {
                    e.prevent_default();
                    let input = e.target_dyn_into::<HtmlInputElement>();
                    if let Some(input) = input {
                        onupdatevalue.emit(input.value().into());
                    }
                }}
                class="input  input-bordered w-full "
            />
        </div>
        }
    }
}
