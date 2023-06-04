use crate::{
    components::notification::{use_notification, Notification},
    pages::{PageNotFound, PageSignIn, PageSignUp},
};
use dyno_core::{role::Roles, UserSession};
use gloo::storage::{LocalStorage, Storage};
use yew::{html, use_state, AttrValue, Callback, Properties};
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/")]
    Dashboard,
    #[at("/activities")]
    Activities,
    #[at("/sop")]
    Sop,
    #[at("/about")]
    About,
    #[at("/calendar")]
    Kalendar,
    #[at("/signin")]
    SignIn,
    #[at("/signup")]
    SignUp,

    #[at("/settings/profile")]
    SettingProfile,
    #[at("/settings/app")]
    SettingApp,

    #[at("/administration/dynos")]
    AdminDynos,
    #[at("/administration/users")]
    AdminUsers,
    #[at("/administration/infos")]
    AdminInfos,
    #[at("/administration/history")]
    AdminHistory,
}

pub struct RouteSideBar<'a> {
    pub icon: yew::Html,
    pub name: &'a str,
    pub filter_role: Roles,
    pub route: Option<Route>,
    pub submenu: Option<&'a [RouteSideBar<'a>]>,
}

pub type LinkTag = Link<Route>;

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct SwitchProp {
    pub session: Option<UserSession>,
    pub path_name: Option<AttrValue>,
}

#[yew::function_component(Switch)]
pub fn switch(prop: &SwitchProp) -> yew::Html {
    let session = use_state(|| LocalStorage::get::<UserSession>(crate::USER_SESSION_OBJ_KEY).ok());
    let route = use_route::<Route>();
    let route = prop
        .path_name
        .as_ref()
        .and_then(|p| Route::recognize(p))
        .or(route);
    let notif_manager = use_notification();
    let notif = Callback::from(move |v: Notification| notif_manager.spawn(v));

    let callback_session_change = session.setter();
    if session.is_none() {
        return html! { <PageSignIn /> };
    }

    match route {
        Some(route) => {
            dyno_core::log::debug!("switching router to: `{}`", route.to_path());
            match route {
                Route::SignIn => html! { <PageSignIn /> },
                Route::SignUp => html! { <PageSignUp /> },
                _ => {
                    html! { <PageNotFound /> }
                }
            }
        }
        None => html! { <PageNotFound /> },
    }
}

macro_rules! route_sidebar {
    ($icon:ident, $name:literal, $route:ident, $role:ident) => {
        RouteSideBar {
            icon: yew::html! { <yew_icons::Icon icon_id={yew_icons::IconId::$icon} class={"h-6 w-6"} />  },
            name: $name,
            route: Some(Route::$route),
            filter_role: Roles::$role,
            submenu: None,
        }
    };
    ($($icon:ident, $name:literal, $role:ident, [$($args:tt)*]),*) => {$(
        RouteSideBar {
            icon: yew::html! { <yew_icons::Icon icon_id={yew_icons::IconId::$icon} class={"h-6 w-6"} />  },
            name: $name,
            route: None,
            filter_role: Roles::$role,
            submenu: Some(&[$($args)*])
        }
    ),*
    };
}

pub const SIDE_BAR: [RouteSideBar<'static>; 7] = [
    route_sidebar!(HeroiconsOutlineSquares2X2, "Dashboard", Dashboard, User),
    route_sidebar!(HeroiconsOutlineSquares2X2, "Aktivitas", Activities, User),
    route_sidebar!(HeroiconsOutlineSquares2X2, "SOP", Sop, User),
    route_sidebar!(
        HeroiconsOutlineSquares2X2,
        "Administration",
        Admin,
        [
            route_sidebar!(HeroiconsOutlineSquares2X2, "Users", AdminUsers, User),
            route_sidebar!(HeroiconsOutlineSquares2X2, "Dynos", AdminDynos, User),
            route_sidebar!(HeroiconsOutlineSquares2X2, "Infos", AdminInfos, User),
            route_sidebar!(HeroiconsOutlineSquares2X2, "History", AdminHistory, User),
        ]
    ),
    route_sidebar!(
        HeroiconsOutlineSquares2X2,
        "Settings",
        User,
        [
            route_sidebar!(HeroiconsOutlineSquares2X2, "Profil", SettingProfile, User),
            route_sidebar!(HeroiconsOutlineSquares2X2, "App", SettingApp, User)
        ]
    ),
    route_sidebar!(HeroiconsOutlineSquares2X2, "Kalendar", Dashboard, User),
    route_sidebar!(HeroiconsOutlineSquares2X2, "About", About, User),
];
