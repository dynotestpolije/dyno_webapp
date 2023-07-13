use web_sys::{Element, HtmlInputElement};
use yew::{
    function_component, html, use_callback, use_effect_with_deps, use_node_ref, use_state,
    Children, Html, Properties,
};
use yew_router::prelude::use_route;

use super::{header::Header, sidebar::Sidebar};
use crate::route::Route;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct LayoutProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    let title = use_state(String::default);

    let clicked_ref = use_node_ref();
    let main_ref = use_node_ref();
    let route = use_route::<Route>();

    let title_value = (*title).clone();

    use_effect_with_deps(
        move |(r, t, refs)| {
            t.set(r.unwrap_or_default().to_string());
            if let Some(refs) = refs.cast::<Element>() {
                refs.scroll_into_view();
            }
        },
        (route, title, main_ref.clone()),
    );

    let leftsidebar_open_callback = use_callback(
        move |(), refs| {
            if let Some(refs) = refs.cast::<HtmlInputElement>() {
                refs.click();
            }
        },
        clicked_ref.clone(),
    );

    html! {
      <div class="drawer lg:drawer-open">
        <input
            id="left-sidebar-drawer"
            type="checkbox"
            class="drawer-toggle  lg:drawer-open"
            ref={clicked_ref.clone()}
        />

        <div class="drawer-content flex flex-col ">
            <Header title={title_value} />
            <main class="flex-1 overflow-y-auto pt-8 px-6  bg-base-200"  ref={main_ref.clone()}>
                {props.children.clone()}
                <div class="h-16"></div>
            </main>
        </div>
        <Sidebar open_callback={leftsidebar_open_callback} />
      </div>
    }
}
