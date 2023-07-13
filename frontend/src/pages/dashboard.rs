use std::ops::Deref;

use dyno_core::{chrono::Utc, ActiveResponse, DynoPlot};
use yew::{function_component, html, use_effect_with_deps, use_state, Html, UseStateSetter};
use yew_icons::{Icon, IconId};
use yewdux::prelude::{use_store, Dispatch};

use crate::{
    components::{button::Button, chart::Chart, stats::Stats},
    fetch,
    route::{LinkTag, Route},
    state::AppState,
};
pub async fn fetch_dashboard_all(
    state: &mut AppState,
    active: UseStateSetter<Option<ActiveResponse>>,
    history: UseStateSetter<DynoPlot>,
) {
    let token = format!("Bearer {}", state.token_session().unwrap());
    fetch::fetch_status(&active, &token).await;
    fetch::fetch_dashboard(state, &token).await;
    fetch::fetch_dyno(state, &token).await;

    let plot = DynoPlot::new()
        .set_color(state.theme().plot_color())
        .create_history_dyno(state.get_data().dyno());
    history.set(plot);
}

#[function_component(PageDashboard)]
pub fn page_dashboard() -> Html {
    let (state, _) = use_store::<AppState>();
    let history_plot = use_state(DynoPlot::new);
    let active_user = use_state(|| Option::<ActiveResponse>::None);

    let on_refresh = {
        let active = active_user.setter();
        let history = history_plot.setter();
        Dispatch::<AppState>::new().reduce_mut_future_callback_with(move |s, _| {
            let active = active.clone();
            let history = history.clone();
            Box::pin(fetch_dashboard_all(s, active, history))
        })
    };
    {
        let d = on_refresh.clone();
        use_effect_with_deps(
            move |_| {
                d.emit(());
                || ()
            },
            state.theme(),
        );
    }

    let status_active = match active_user.as_ref() {
        Some(act) => html! {
            <Stats icon={IconId::HeroiconsOutlineUserCircle}
                title="Active Dynotest"
                value={match &act.user {
                    Some(user) => format!("{} ({})", user.name, user.nim),
                    None => "Not Logined".to_owned(),
                }}
                desc={format!("start: {} ({}m)", act.start.naive_local().format("%r"), (act.start - Utc::now()).num_minutes())}
            />
        },
        None => html! {
            <Stats icon={IconId::HeroiconsOutlineUserCircle}
                title="Active Dynotest"
                value="None"
                desc="No active usage in Dynotest"
            />
        },
    };

    let history_plot = history_plot.deref();

    html! {
    <>
        <div class="grid grid-cols-1 sm:grid-cols-1 gap-4">
            <div class="text-right ">
                if active_user.is_some() {
                    <LinkTag to={Route::Live}>
                        <Button class="btn-ghost normal-case">
                            <Icon icon_id={IconId::HeroiconsOutlineWifi} class="w-4 mr-2"/>
                            {"Stream Data"}
                        </Button>
                    </LinkTag>
                }
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
            <Stats
                icon={IconId::HeroiconsOutlineAcademicCap}
                title="Total Dynotest Usage"
                value={state.get_data().dyno().len().to_string()}
                desc={
                    let data = state.get_data().dyno();
                    let verified_len = data.iter().filter(|x| x.verified).count();
                    let len = data.len();
                    format!("{} Verified {}/{} ({} %)",
                        dyno_core::ternary!((verified_len < (len/2))? ("↙") : ("↗︎")),
                        verified_len, len, (verified_len as f32 / len as f32) * 100.)
                }
            />
            {status_active}
        </div>
        <div class="grid lg:grid-cols-1 mt-1 md:grid-cols-1 grid-cols-1">
            <Chart id="chart_activity" title="Chart Activities" plot={history_plot.clone()} />
        </div>
    </>
    }
}
