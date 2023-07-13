use dyno_core::{DynoErr, DynoResult};
use gloo::net::http::Request;
use web_sys::MouseEvent;
use yew::{classes, function_component, html, use_effect_with_deps, AttrValue, Properties};
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
    {
        use_effect_with_deps(
            move |d| match gloo::utils::document_element()
                .set_attribute("data-theme", d.to_str())
                .map_err(|j| j.as_string().unwrap_or(String::new()))
            {
                Ok(()) => (),
                Err(err) => dyno_core::log::error!("Failed to set `data-theme`: {err}"),
            },
            theme,
        )
    }

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
            let token = format!("Bearer {}", state.token_session().unwrap());
            Box::pin(async move {
                match logout(&token).await {
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

    let (name, nim) = state
        .me()
        .map(|x| (x.name.clone(), x.nim.clone()))
        .unwrap_or_default();

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
                            <a class="font-bold">{name}</a>
                        </li>
                        <li class="justify-between">
                            <a class="font-light">{nim}</a>
                        </li>
                        <div class="divider mt-0 mb-0"></div>
                        <li class="justify-between">
                            <LinkTag to={Route::SettingProfile}> {"Profile Settings"}</LinkTag>
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

async fn logout(token: impl AsRef<str>) -> DynoResult<()> {
    match Request::get("/api/auth/logout")
        .header("Authorization", token.as_ref())
        .send()
        .await
    {
        Ok(ok) if ok.ok() || ok.status() == 401 => Ok(()),
        Ok(ok) => Err(DynoErr::api_error(ok.text().await.unwrap_or_default())),
        Err(err) => Err(DynoErr::api_error(err)),
    }
}
