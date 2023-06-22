use yew::{classes, function_component, html, AttrValue, Children, Classes, Html, Properties};

use super::typography::Subtitle;

#[derive(Properties, PartialEq)]
pub struct TitleCardProps {
    pub title: AttrValue,
    #[prop_or(From::from("mt-6"))]
    pub class: Classes,
    #[prop_or_default]
    pub top_side_button: Option<Html>,
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn TitleCard(props: &TitleCardProps) -> Html {
    let TitleCardProps {
        title,
        class: class_div,
        children,
        top_side_button,
    } = props;

    html! {
    <div class={classes!("card", "w-full", "p-6", "bg-base-100", "shadow-xl", class_div.clone())}>
        <Subtitle class={if top_side_button.is_some() { "inline-block" } else {"" }}>
            {title}
            if let Some(top_btn) = top_side_button { <div className="inline-block float-right">{top_btn.clone()}</div> }
        </Subtitle>
        <div class="divider mt-2"></div>
        <div class="h-full w-full pb-6 bg-base-100">
            {children.clone()}
        </div>
    </div>
    }
}
