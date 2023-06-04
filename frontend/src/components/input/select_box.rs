use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_icons::{Icon, IconId::HeroiconsOutlineInformationCircle};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct SelectBoxProps {
    #[prop_or(From::from("Check Box"))]
    pub title: AttrValue,
    pub desc: Option<AttrValue>,

    #[prop_or_default]
    pub value: AttrValue,
    #[prop_or_default]
    pub container_class: Classes,
    #[prop_or_default]
    pub label_class: Classes,
    #[prop_or(From::from("Search"))]
    pub placeholder: AttrValue,

    // (value, key)
    pub options: Vec<SelectOption>,

    pub update_callback: Callback<String>,
}

pub struct SelectBox;

impl Component for SelectBox {
    type Message = ();
    type Properties = SelectBoxProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let SelectBoxProps {
            value: default_value,
            placeholder,
            update_callback,
            title,
            desc,
            container_class,
            label_class,
            options,
        } = ctx.props();
        let description = if desc.is_none() {
            html! {
                <div class={classes!("tooltip", "tooltip-right")} data-tip={desc} >
                    <Icon icon_id={HeroiconsOutlineInformationCircle} class="w-4 h-4"/>
                </div>
            }
        } else {
            html!()
        };

        html! {
        <div class={classes!("inline-block", *container_class)}>
            <label class={classes!("label", *label_class)}>
                <div class="label-text"> {title} {description} </div>
            </label>

            <select
                class="select select-bordered w-full"
                value={default_value}
                onchange={|e: Event| {
                    e.prevent_default();
                    let input = e.target_dyn_into::<HtmlInputElement>();
                    if let Some(input) = input {
                        update_callback.emit(input.value().into());
                    }
                }}
            >
                <option disabled=true value="PLACEHOLDER">{placeholder}</option>
                {
                    for options
                        .into_iter()
                        .enumerate()
                        .map(|(key, SelectOption { name, value })| html!{
                            <option value={value} key={key}>{name}</option>
                        })
                }
            </select>
        </div>
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SelectOption {
    pub name: AttrValue,
    pub value: AttrValue,
}

impl SelectOption {
    pub fn new(key: impl Into<AttrValue>, value: impl Into<AttrValue>) -> Self {
        let key = key.into();
        let value = value.into();
        Self { name: key, value }
    }
}

impl<K, V> From<(K, V)> for SelectOption
where
    K: Into<AttrValue>,
    V: Into<AttrValue>,
{
    fn from(value: (K, V)) -> Self {
        Self::new(value.0, value.1)
    }
}
