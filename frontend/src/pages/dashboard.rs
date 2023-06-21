use dyno_core::{chrono::Utc, users::UserResponse, ActiveResponse, ApiResponse};
use gloo::net::http::Request;
use yew::{
    classes, function_component, html, use_state, AttrValue, Classes, Html, Properties,
    UseStateSetter,
};
use yew_icons::{Icon, IconId};
use yewdux::prelude::{use_store, Dispatch};

use crate::{components::button::Button, state::AppState};

pub async fn fetch_dashboard(
    state: &mut AppState,
    token: impl AsRef<str>,
    error: &UseStateSetter<String>,
) {
    let fetched = match Request::get("/api/auth/me")
        .header("Authorization", token.as_ref())
        .send()
        .await
    {
        Ok(resp) if resp.status() == 200 => resp
            .json::<ApiResponse<UserResponse>>()
            .await
            .map(|x| x.payload)
            .ok(),
        Err(err) => {
            error.set(err.to_string());
            None
        }
        _ => None,
    };
    state.set_me(fetched);
}

pub async fn fetch_status(
    active: &UseStateSetter<Option<ActiveResponse>>,
    token: impl AsRef<str>,
    error: &UseStateSetter<String>,
) {
    let fetched_active = match Request::get("/api/active")
        .header("Authorization", token.as_ref())
        .send()
        .await
    {
        Ok(resp) if resp.status() == 200 => resp
            .json::<ApiResponse<ActiveResponse>>()
            .await
            .map(|x| x.payload)
            .ok(),
        Err(err) => {
            error.set(err.to_string());
            None
        }
        _ => None,
    };
    active.set(fetched_active)
}

#[function_component(PageDashboard)]
pub fn page_dashboard() -> Html {
    let active_user = use_state(|| Option::<ActiveResponse>::None);
    let error = use_state(String::new);

    let on_refresh = {
        let active = active_user.setter();
        let error = error.setter();
        Dispatch::<AppState>::new().reduce_mut_future_callback_with(move |s, _| {
            let active = active.clone();
            let error = error.clone();
            let token = format!("Bearer {}", s.token().unwrap());
            Box::pin(async move {
                fetch_status(&active, &token, &error).await;
                fetch_dashboard(s, token, &error).await;
            })
        })
    };

    let status_active = match active_user.as_ref() {
        Some(act) => html! {
            <StatDashboard icon={IconId::HeroiconsOutlineUserCircle}
                title="Active Dynotest"
                value={match &act.user {
                    Some(user) => format!("{} ({})", user.name, user.nim),
                    None => "Not Logined".to_owned(),
                }}
                desc={format!("start: {} ({}m)", act.start.format("%r"), (act.start - Utc::now()).num_minutes())}
            />
        },
        None => html! {
            <StatDashboard icon={IconId::HeroiconsOutlineUserCircle}
                title="Active Dynotest"
                value="None"
                desc="No active usage in Dynotest"
            />
        },
    };
    html! {
    <>
        <div class="grid grid-cols-1 sm:grid-cols-1 gap-4">
            <div class="text-right ">
                <Button class="btn-ghost normal-case" onclick={on_refresh}>
                    <Icon icon_id={IconId::HeroiconsOutlineArrowPath} class="w-4 mr-2"/>
                    {"Refresh Data"}
                </Button>
                <div class="dropdown dropdown-bottom dropdown-end ml-2">
                    <label tabIndex={0} class="btn btn-ghost btn-sm normal-case btn-square ">
                        <Icon icon_id={IconId::HeroiconsOutlineEllipsisVertical} class="w-5"/>
                    </label>
                    <ul tabIndex={0} class="dropdown-content menu menu-compact p-2 shadow bg-base-100 rounded-box w-52">
                        <li><a><Icon icon_id={IconId::HeroiconsOutlineEnvelope} class="w-4"/>{"Email Digests"}</a></li>
                        <li><a><Icon icon_id={IconId::HeroiconsOutlineArrowDownTray} class="w-4"/>{"Download"}</a></li>
                    </ul>
                </div>
            </div>
        </div>
        <div class="grid lg:grid-cols-2 mt-1 md:grid-cols-1 grid-cols-1 gap-6">
            <StatDashboard icon={IconId::HeroiconsOutlineAcademicCap} />
            {status_active}
        </div>
    </>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct StatDashboardProps {
    icon: IconId,
    #[prop_or(From::from("title"))]
    title: AttrValue,
    #[prop_or(From::from("value"))]
    value: AttrValue,
    #[prop_or(From::from("description"))]
    desc: AttrValue,
}

pub fn get_desc_style(desc: &AttrValue) -> Classes {
    if desc.contains("↗︎") {
        From::from("font-bold text-green-700 dark:text-green-300")
    } else if desc.contains('↙') {
        From::from("font-bold text-rose-500 dark:text-red-400")
    } else {
        Default::default()
    }
}

#[function_component(StatDashboard)]
fn stat_dashboard(
    StatDashboardProps {
        icon,
        title,
        value,
        desc,
    }: &StatDashboardProps,
) -> Html {
    html! {
        <div class="stats shadow">
            <div class="stat">
                <div class="stat-figure dark:text-slate-300 text-primary">
                    <Icon icon_id={*icon} class="w-8 h-8" />
                </div>
                <div class="stat-title dark:text-slate-300">{title}</div>
                <div class="stat-value dark:text-slate-300 text-primary">{value}</div>
                <div class={classes!("stat-desc", get_desc_style(desc))}>{desc}</div>
            </div>
        </div>
    }
}
