use dyno_core::DynoPlot;
use yew::{function_component, html, platform::spawn_local, AttrValue, Children, Html, Properties};

use crate::components::cards::TitleCard;

#[derive(Clone, Properties, PartialEq)]
pub struct ChartProps {
    pub id: AttrValue,
    pub title: AttrValue,
    pub plot: DynoPlot,
    #[prop_or_default]
    pub top_side_button: Option<Html>,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Chart)]
pub fn chart(props: &ChartProps) -> Html {
    yew::use_effect_with_deps(
        move |d| {
            let d = d.clone();
            spawn_local(async move { d.1.render_to_canvas(d.0).await })
        },
        (props.id.to_string(), props.plot.clone()),
    );
    html! {
    <TitleCard title={props.title.clone()} top_side_button={props.top_side_button.clone()}>
      <div id={props.id.clone()}></div>
      {props.children.clone()}
    </TitleCard>
    }
}
