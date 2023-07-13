use dyno_core::DynoPlot;
use yew::{function_component, html, AttrValue, Children, Html, Properties};

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
    let p = {
        let id = props.id.clone();
        let plot = props.plot.clone();
        yew_hooks::use_async::<_, _, ()>(async move {
            plot.render_to_canvas(id).await;
            Ok(())
        })
    };
    yew::use_effect_with_deps(
        move |_| {
            p.run();
            || ()
        },
        props.plot.clone(),
    );
    html! {
        <TitleCard title={props.title.clone()} top_side_button={props.top_side_button.clone()}>
            <div id={props.id.clone()}></div>
            {props.children.clone()}
        </TitleCard>
    }
}
