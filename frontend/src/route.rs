use dyno_core::role::Roles;
use yew_router::{components::Link, Routable};

#[derive(dyno_core::derive_more::Display, Routable, PartialEq, Eq, Clone, Copy, Debug)]
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
    #[at("/signin")]
    SignIn,
    #[at("/signup")]
    SignUp,

    #[at("/live")]
    Live,

    #[at("/settings/profile")]
    SettingProfile,

    #[at("/administration/dynos")]
    AdminDynos,
    #[at("/administration/users")]
    AdminUsers,
    #[at("/administration/infos")]
    AdminInfos,
    #[at("/administration/history")]
    AdminHistory,
}

pub type LinkTag = Link<Route>;

macro_rules! route_sidebar {
    ($icon:ident, $name:literal, $route:ident, $role:ident, $classes:literal) => {
        RouteSideBar {
            icon: yew::html! { <yew_icons::Icon icon_id={yew_icons::IconId::$icon} class={$classes} />  },
            name: $name,
            route: Some(Route::$route),
            filter_role: Roles::$role,
            submenu: None,
        }
    };
    ($($icon:ident, $name:literal, $role:ident, [$($args:tt)*]),*) => {$(
        RouteSideBar {
            icon: yew::html! { <yew_icons::Icon icon_id={yew_icons::IconId::$icon} class={"h-6 w-6 inline"} />  },
            name: $name,
            route: None,
            filter_role: Roles::$role,
            submenu: Some(vec![$($args)*])
        }
    ),*
    };
}

#[derive(Debug, Clone, PartialEq)]
pub struct RouteSideBar<'a> {
    pub icon: yew::Html,
    pub name: &'a str,
    pub filter_role: Roles,
    pub route: Option<Route>,
    pub submenu: Option<Vec<RouteSideBar<'a>>>,
}

thread_local! {
    pub static SIDE_BAR: std::cell::RefCell<Vec<RouteSideBar<'static>>>  = std::cell::RefCell::new(vec![
        route_sidebar!(HeroiconsOutlineSquares2X2, "Dashboard", Dashboard, User, "h-6 w-6"),
        route_sidebar!(HeroiconsOutlineChartBar, "Aktivitas", Activities, User, "h-6 w-6"),
        route_sidebar!(HeroiconsOutlineUser, "Profil", SettingProfile, User, "h-5 w-5"),
        route_sidebar!(
            HeroiconsOutlineDocumentDuplicate,
            "Administration",
            Admin,
            [
                route_sidebar!(HeroiconsOutlineUsers, "Users", AdminUsers, Admin, "h-5 w-5"),
                route_sidebar!(HeroiconsOutlineTableCells, "Dynos", AdminDynos, Admin, "h-5 w-5"),
                route_sidebar!(HeroiconsOutlineCog, "Infos", AdminInfos, Admin, "h-5 w-5"),
                route_sidebar!(HeroiconsOutlineInboxStack, "History", AdminHistory, Admin, "h-5 w-5"),
            ]
        ),
        route_sidebar!(HeroiconsOutlineDocumentText, "SOP", Sop, User, "h-6 w-6"),
    ]);
}

pub fn sidebar_routes() -> std::cell::RefCell<Vec<RouteSideBar<'static>>> {
    SIDE_BAR.with(|inner| inner.clone())
}
