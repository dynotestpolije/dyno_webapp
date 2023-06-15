use dyno_core::{crypto::TokenDetails, DynoErr, DynoResult};
use gloo::{
    net::http::Request,
    storage::{LocalStorage, Storage},
};
use web_sys::MouseEvent;
use yew::{classes, function_component, html, AttrValue, Properties};
use yew_icons::{Icon, IconId};
use yew_router::prelude::use_navigator;
use yewdux::prelude::use_store;

use crate::{
    components::notification::{use_notification, Notification},
    notif_error,
    route::{LinkTag, Route},
    state::AppState,
};

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub title: AttrValue,

    #[prop_or(From::from("https://clipground.com/images/blank-avatar-png-5.png"))]
    pub profile_photo: AttrValue,
}

#[function_component(Header)]
pub fn header(
    HeaderProps {
        profile_photo,
        title,
    }: &HeaderProps,
) -> yew::Html {
    let (state, dispatech) = use_store::<AppState>();
    let theme = state.theme();

    let onchange_theme = dispatech.reduce_mut_callback_with(move |state, e: MouseEvent| {
        e.prevent_default();
        state.swap_theme();
    });

    let onlogout = {
        let navigator = use_navigator();
        let notification = use_notification::<Notification>();
        dispatech.reduce_mut_future_callback_with(move |state, e: MouseEvent| {
            e.prevent_default();
            let navigator = navigator.clone();
            let notification = notification.clone();

            Box::pin(async move {
                match logout().await {
                    Ok(()) => {
                        state.delete_token();
                        if let Some(nav) = navigator {
                            nav.push(&Route::SignIn);
                        }
                    }
                    Err(err) => notification.spawn(notif_error!("Failed Logout", "{err}")),
                }
            })
        })
    };

    html! {
        <div class="navbar flex justify-between bg-base-100 z-10 shadow-md ">
            <div class="">
                <label for="left-sidebar-drawer" class="btn btn-primary drawer-button lg:hidden">
                <Icon icon_id={IconId::HeroiconsOutlineBars3} class="h-5 inline-block w-5"/></label>
                <h1 class="text-2xl font-semibold ml-2">{title.clone()}</h1>
            </div>
            <div class="order-last">
                <label class="swap">
                    <input class="hidden" type="checkbox" onclick={onchange_theme}/>
                    <Icon icon_id={IconId::HeroiconsOutlineSun}
                        class={classes!("fill-current", "w-6", "h-6", if theme.is_dark() { "swap-on" } else {"swap-off"})}
                    />
                    <Icon icon_id={IconId::HeroiconsOutlineMoon}
                        class={classes!("fill-current", "w-6", "h-6", if theme.is_light() { "swap-on" } else { "swap-off" })}
                    />
                </label>
                <div class="dropdown dropdown-end ml-4">
                    <label tabIndex={0} class="btn btn-ghost btn-circle avatar">
                        <div class="w-10 rounded-full">
                        <img
                            src={profile_photo.clone()}
                            alt="profile"
                        />
                        </div>
                    </label>
                    <ul tabIndex={0} class="menu menu-compact dropdown-content mt-3 p-2 shadow bg-base-100 rounded-box w-52">
                        <li class="justify-between">
                            <LinkTag to={Route::SettingProfile}> {"Profile Settings"} <span class="badge">{"New"}</span> </LinkTag>
                        </li>
                        <div class="divider mt-0 mb-0"></div>
                        <li>
                            <a onclick={onlogout}>
                            {"Logout"}
                            </a>
                        </li>
                    </ul>
                </div>
            </div>
        </div>
    }
}

async fn logout() -> DynoResult<()> {
    let data = LocalStorage::get::<TokenDetails>(crate::USER_SESSION_OBJ_KEY).unwrap();
    let token = format!("Bearer {}", data.token.unwrap_or_default());
    match Request::get("/api/auth/logout")
        .header("Authorization", &token)
        .send()
        .await
    {
        Ok(ok) if ok.ok() => Ok(()),
        Ok(ok) => Err(DynoErr::api_error(ok.text().await.unwrap_or_default())),
        Err(err) => Err(DynoErr::api_error(err)),
    }
}
