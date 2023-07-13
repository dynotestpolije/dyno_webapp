use yew::prelude::*;
use yew_icons::{Icon, IconId};
use yew_router::{hooks::use_location, Routable};
use yewdux::prelude::use_store;

use crate::{route::{sidebar_routes, LinkTag, Route, RouteSideBar}, state::AppState};

#[derive(Debug, Properties, PartialEq)]
pub struct SideBarProps {
    pub open_callback: Callback<()>,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Sidebar)]
pub fn sidebar(props: &SideBarProps) -> Html {
    let location = use_location()
        .and_then(|loc| Route::recognize(loc.path()))
        .unwrap_or_default();

    let callback = props.open_callback.clone();
    let (state, _) = use_store::<AppState>();
    let role = state.user_session().map(|x| x.role).unwrap_or_default();

    html! {
        <div class="drawer-side">
            <label for="left-sidebar-drawer" class="drawer-overlay"> </label>
            <ul class="menu pt-2 w-80 bg-base-100 text-base-content">
            // <ul class="menu p-4 w-80 bg-base-100 text-base-content">
                <button
                    class="btn btn-ghost bg-base-300  btn-circle z-50 top-0 right-0 mt-4 mr-2 absolute lg:hidden"
                    onclick={move |e: MouseEvent| {
                        e.prevent_default();
                        callback.emit(())
                    }}
                >
                    <Icon icon_id={IconId::HeroiconsOutlineXMark} class="h-5 inline-block w-5"/>
                </button>
                <li class="mb-2 font-semibold text-xl">
                    <LinkTag to={Route::Dashboard}>
                        <img class="mask mask-squircle w-10" src="/assets/logo.png" alt="Dynotest Logo"/>
                        {"Dynotests"}
                    </LinkTag>
                </li>
                {for sidebar_routes().borrow().iter().filter(|x| x.filter_role == role || role.is_admin()).enumerate().map(|(idx, route)| {
                    html!{
                    <li class="" key={idx}>
                        if let Some(ref submenu) = route.submenu {
                            <LeftSidebarSub icon={route.icon.clone()} name={route.name} submenu={submenu.clone()} location={location}/>
                        } else {
                            <NavLink to={route.route.unwrap()} routes={route.clone()} selected={location == route.route.unwrap()}/>
                        }
                    </li>
                    }
                })
                }
            {props.children.clone()}
            </ul>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct NavLinkProps {
    pub to: Route,
    pub routes: RouteSideBar<'static>,
    #[prop_or(false)]
    pub selected: bool,
}

#[function_component(NavLink)]
fn navlink(props: &NavLinkProps) -> Html {
    html! {
        <LinkTag
            to={props.to}
            classes={classes!(if props.selected { "font-semibold  bg-base-200" } else { "font-normal" })}
        >
            {props.routes.icon.clone()}{props.routes.name}
            if props.selected {
                <span class="absolute inset-y-0 left-0 w-1 rounded-tr-md rounded-br-md bg-primary" aria-hidden="true"></span>
            } else {
                <></>
            }
        </LinkTag>
    }
}

#[derive(Properties, PartialEq)]
pub struct LeftSidebarSubProps {
    pub location: Route,
    pub icon: Html,
    pub name: AttrValue,
    pub submenu: Vec<RouteSideBar<'static>>,
}

#[function_component(LeftSidebarSub)]
fn sidebar_submenu(props: &LeftSidebarSubProps) -> Html {
    let expanded_state = use_state(|| false);
    let expanded_setter = expanded_state.setter();
    let expanded = *expanded_state;

    html! {
        <details open={expanded}>
            <summary onclick={move |_| expanded_setter.set(!expanded)}>{props.icon.clone()}{props.name.clone()}</summary>
            <ul class="menu menu-compact">
            {for props.submenu.iter().enumerate().map(|(idx, route)| html!{
                <li key={idx}>
                    <LinkTag to={route.route.unwrap()} >
                        {route.icon.clone()} {route.name}
                        if props.location == route.route.unwrap() {
                            <span
                                class="absolute mt-1 mb-1 inset-y-0 left-0 w-1 rounded-tr-md rounded-br-md bg-primary"
                                aria-hidden="true">
                            </span>
                        } else {
                            <></>
                        }
                    </LinkTag>
                </li>
            })}
            </ul>
        </details>
    }
}
