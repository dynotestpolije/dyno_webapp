use web_sys::MouseEvent;
use yew::{
    classes, function_component, html, AttrValue, Callback, Children, Classes, Html, Properties, use_callback,
};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct ButtonProp {
    #[prop_or_default]
    pub r#type: AttrValue,
    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub onclick: Callback<()>,
    #[prop_or_default]
    pub children: Children,

    #[prop_or(false)]
    pub disabled: bool,
}

#[function_component(Button)]
pub fn button(
    ButtonProp {
        r#type: etype,
        class: eclass,
        onclick: eonclick,
        children: echildren,
        disabled,
    }: &ButtonProp,
) -> Html {
    let on_click = use_callback(|_e: MouseEvent, cb| { cb.emit(()) }, eonclick.clone());
    html! {
        <button
            type={etype.clone()}
            class={classes!{"btn", "btn-sm", eclass.clone()}}
            onclick={on_click}
            disabled={*disabled}
        >
            {echildren.clone()}
        </button>
    }
}
