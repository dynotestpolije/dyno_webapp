#![allow(non_camel_case_types)]

mod components;
mod containers;
mod pages;
mod route;

use yew::prelude::*;
use yew_router::BrowserRouter;

use route::{LinkTag, Route, Switch};

use components::notification::{Notification, NotificationFactory, NotificationsProvider};

const USER_SESSION_OBJ_KEY: &str = "dyno_user_session";
const USER_TOKEN: &str = "dyno_token";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AppMsg {
    Nope,
}

#[derive(Debug, Clone, Default)]
struct App;

impl Component for App {
    type Message = AppMsg;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let component_creator = NotificationFactory::default();

        html! {
            <NotificationsProvider<Notification, NotificationFactory> {component_creator}>

                <BrowserRouter basename="/">
                    <Switch  session={self.session} />
                </BrowserRouter>

            </NotificationsProvider<Notification, NotificationFactory>>
        }
    }
}

fn main() {
    #[cfg(debug_assertions)]
    wasm_logger::init(wasm_logger::Config::new(dyno_core::log::Level::Debug));
    #[cfg(not(debug_assertions))]
    wasm_logger::init(wasm_logger::Config::new(dyno_core::log::Level::Warn));

    yew::Renderer::<App>::new().render();
}
