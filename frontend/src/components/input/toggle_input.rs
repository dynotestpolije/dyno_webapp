use std::{marker::PhantomData, str::FromStr};

use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct SearchBarProps<V: PartialEq> {
    pub title: AttrValue,
    pub types: AttrValue,
    pub value: V,
    #[prop_or_default]
    pub checked: bool,
    #[prop_or_default]
    pub container_class: Classes,
    #[prop_or_default]
    pub label_class: Classes,
    #[prop_or_default]
    pub placeholder: AttrValue,

    pub update_callback: Callback<Option<V>>,
}

pub struct SearchBar<V: Sized>(PhantomData<V>);

impl<V> Component for SearchBar<V>
where
    V: FromStr + Clone + PartialEq + std::fmt::Display + 'static,
{
    type Message = ();
    type Properties = SearchBarProps<V>;

    fn create(_ctx: &yew::Context<Self>) -> SearchBar<V> {
        Self(PhantomData)
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let SearchBarProps {
            title,
            types,
            value,
            checked,
            container_class,
            label_class,
            placeholder,
            update_callback,
        } = ctx.props();

        html! {
        <div class={classes!("form-control", "w-full", *container_class)}>
            <label class="label cursor-pointer">
                <span class={classes!("label-text", "text-base-content", *label_class)}>
                    {title}
                </span>
                <input
                    type="checkbox"
                    class="toggle"
                    checked={*checked}
                    value={value.to_string()}
                    onchange={|e: Event| {
                        e.prevent_default();
                        let input = e.target_dyn_into::<HtmlInputElement>();
                        if let Some(input) = input {
                            let input_value = input.value();
                            update_callback.emit(input.value().parse().ok())
                        }
                    }
                }/>
            </label>
        </div>
        }
    }
}
