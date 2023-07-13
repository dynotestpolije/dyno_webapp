#![allow(non_camel_case_types)]

mod components;
mod containers;
mod fetch;
mod pages;
mod route;
mod state;
mod theme;

use wasm_bindgen::UnwrapThrowExt;
use yew::{function_component, html, Html, Suspense};
use yew_router::{prelude::use_route, BrowserRouter, Routable};

use route::{LinkTag, Route};
use yewdux::prelude::use_store;

use crate::{
    components::notification::{Notification, NotificationFactory, NotificationsProvider},
    containers::suspend::SuspenseContent,
    pages::PageLive,
};
use theme::Theme;

use crate::{
    containers::layout::Layout,
    pages::{
        admin::{PageAdminDynos, PageAdminHistory, PageAdminInfos, PageAdminUsers},
        PageActivities, PageDashboard, PageNotFound, PageSettingProfile, PageSignIn, PageSignUp,
        PageSop,
    },
};

macro_rules! with_layout {
    ($($args:tt)*) => (html! { <Layout> $($args)* </Layout> })
}

#[function_component(DynoRouter)]
fn router() -> Html {
    let (state, _) = use_store::<state::AppState>();
    let route = use_route::<Route>().map(|p| match state.token_session().is_none() {
        true => {
            if matches!(p, Route::SignIn | Route::SignUp) {
                p
            } else {
                Route::SignIn
            }
        }
        false => p,
    });
    match route {
        Some(route) => {
            dyno_core::log::debug!("switching router to: `{}`", route.to_path());
            match route {
                Route::NotFound => with_layout!(<PageNotFound />),
                Route::Dashboard => with_layout!(<PageDashboard/>),
                Route::Activities => with_layout!(<PageActivities/>),
                Route::Sop => with_layout!(<PageSop/>),
                Route::SignIn => html! { <PageSignIn /> },
                Route::SignUp => html! { <PageSignUp /> },
                Route::Live => with_layout!(<PageLive />),
                Route::SettingProfile => with_layout!(<PageSettingProfile />),
                Route::AdminDynos => with_layout!(<PageAdminDynos />),
                Route::AdminUsers => with_layout!(<PageAdminUsers />),
                Route::AdminInfos => with_layout!(<PageAdminInfos />),
                Route::AdminHistory => with_layout!(<PageAdminHistory />),
            }
        }
        None => {
            dyno_core::log::warn!("no route matched");
            html! { <PageNotFound /> }
        }
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter >
            <NotificationsProvider<Notification, NotificationFactory> component_creator={NotificationFactory}>
                <Suspense fallback={html!(<SuspenseContent />)}>
                    <DynoRouter />
                </Suspense>
            </NotificationsProvider<Notification, NotificationFactory>>
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(dyno_core::log::Level::Info));
    yew::Renderer::<App>::new().render();
}

#[inline]
fn get_host() -> String {
    gloo::utils::window()
        .location()
        .host()
        .expect_throw("this is should not be happen")
}
