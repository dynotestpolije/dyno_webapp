use web_sys::Event;
use yew::{classes, function_component, html, AttrValue, Callback, Classes, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct ToggleInputProps {
    pub title: AttrValue,
    pub value: bool,
    #[prop_or(From::from("checkbox"))]
    pub types: AttrValue,
    #[prop_or_default]
    pub checked: bool,
    #[prop_or_default]
    pub container_class: Classes,
    #[prop_or_default]
    pub label_class: Classes,
    #[prop_or_default]
    pub placeholder: AttrValue,
    #[prop_or_default]
    pub update_callback: Callback<()>,
}

#[function_component(ToggleInput)]
pub fn toggle_input(
    ToggleInputProps {
        title,
        types,
        value,
        checked,
        container_class,
        label_class,
        placeholder,
        update_callback,
    }: &ToggleInputProps,
) -> Html {
    let update_callback = update_callback.clone();

    html! {
    <div class={classes!("form-control", "w-full", container_class.clone())}>
        <label class="label cursor-pointer">
            <span class={classes!("label-text", "text-base-content", label_class.clone())}>
                {title}
            </span>
            <input
                placeholder={placeholder.clone()}
                type={types.clone()}
                class="toggle"
                checked={*checked}
                value={value.to_string()}
                onchange={move |e: Event| {
                    e.prevent_default();
                    update_callback.emit(())
                }
            }/>
        </label>
    </div>
    }
}
