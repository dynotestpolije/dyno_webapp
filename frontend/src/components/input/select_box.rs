use web_sys::{Event, HtmlInputElement};
use yew::{
    classes, function_component, html, AttrValue, Callback, Children, Classes, Component,
    Properties, TargetCast,
};
use yew_icons::{Icon, IconId::HeroiconsOutlineInformationCircle};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct SelectBoxProps {
    #[prop_or(From::from("Check Box"))]
    pub title: AttrValue,

    #[prop_or_default]
    pub desc: Option<AttrValue>,
    #[prop_or_default]
    pub value: AttrValue,
    #[prop_or_default]
    pub container_class: Classes,
    #[prop_or_default]
    pub label_class: Classes,
    #[prop_or(From::from("Search"))]
    pub placeholder: AttrValue,

    #[prop_or_default]
    pub children: Children,

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
            children,
        } = ctx.props();

        let update_callback = update_callback.clone();

        let description = if desc.is_none() {
            html! {
                <div class={classes!("tooltip", "tooltip-right")} data-tip={desc.clone()} >
                    <Icon icon_id={HeroiconsOutlineInformationCircle} class="w-4 h-4"/>
                </div>
            }
        } else {
            html!()
        };

        html! {
        <div class={classes!("inline-block", container_class.clone())}>
            <label class={classes!("label", label_class.clone())}>
                <div class="label-text"> {title.clone()} {description} </div>
            </label>

            <select
                class="select select-bordered w-full"
                value={default_value.clone()}
                onchange={move |e: Event| {
                    e.prevent_default();
                    let input = e.target_dyn_into::<HtmlInputElement>();
                    if let Some(input) = input {
                        update_callback.emit(input.value());
                    }
                }}
            >
                <option disabled=true value={default_value.clone()}>{placeholder.clone()}</option>
                {children.clone()}
            </select>
        </div>
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Properties)]
pub struct SelectOptionProps {
    pub name: AttrValue,
    pub value: AttrValue,

    #[prop_or_default]
    pub disable: bool,
}

#[function_component(SelectOption)]
pub fn select_option(props: &SelectOptionProps) -> yew::Html {
    html! {
        <option disabled={props.disable} value={props.value.clone()} key={props.value.as_str()}>{props.name.clone()}</option>
    }
}
