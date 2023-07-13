use std::ops::Deref;

use dyno_core::DynoPlot;
use yew::{function_component, html, use_callback, use_mut_ref, use_state, Html};
use yew_hooks::UseWebSocketReadyState;
use yew_icons::{Icon, IconId};
use yewdux::prelude::use_store;

use crate::{
    components::{button::Button, chart::Chart},
    state::AppState,
};

#[function_component(PageLive)]
pub fn live() -> Html {
    let (state, _) = use_store::<AppState>();

    let plot = use_state(DynoPlot::new);
    let data = use_mut_ref(dyno_core::BufferData::new);

    let ws = {
        let plot = plot.clone();
        let color = state.plot_color();
        yew_hooks::use_websocket_with_options(
            format!("ws://{}/ws", crate::get_host()),
            yew_hooks::UseWebSocketOptions {
                onmessage: {
                    let color = color.clone();
                    let data = data.clone();
                    let plot = plot.clone();
                    Some(Box::new(move |strings| {
                        if let Ok(de) =
                            dyno_core::serde_json::from_str::<Vec<dyno_core::Data>>(&strings)
                        {
                            data.borrow_mut().extend_data(de);
                            plot.set(
                                DynoPlot::new()
                                    .set_color(color.clone())
                                    .create_dyno_plot(&data.borrow()),
                            );
                        }
                    }))
                },
                onmessage_bytes: {
                    Some(Box::new(move |bytes| {
                        if let Ok(de) =
                            dyno_core::serde_json::from_slice::<Vec<dyno_core::Data>>(&bytes)
                        {
                            data.borrow_mut().extend_data(de);
                            plot.set(
                                DynoPlot::new()
                                    .set_color(color.clone())
                                    .create_dyno_plot(&data.borrow()),
                            );
                        }
                    }))
                },
                onerror: Some(Box::new(move |msg| {
                    msg.prevent_default();
                    dyno_core::log::error!("{:?}", msg.as_string());
                })),
                manual: Some(true),
                ..Default::default()
            },
        )
    };
    let start_callback = {
        let ws = ws.clone();
        use_callback(
            move |_, _| {
                ws.open();
            },
            (),
        )
    };
    let stop_callback = {
        let ws = ws.clone();
        use_callback(
            move |_, _| {
                ws.close();
            },
            (),
        )
    };

    let top_side_button = html! {
        <div class="inline-block float-right">
            <Button
                class="btn px-6 btn-sm normal-case normal-case btn-primary"
                onclick={start_callback}
                disabled={*ws.ready_state == UseWebSocketReadyState::Open}
            >
                <Icon icon_id={IconId::HeroiconsOutlineRocketLaunch} class="w-4 mr-2"/>
                {"Start Stream"}
            </Button>
            <Button
                class="btn px-6 btn-sm normal-case normal-case btn-primary"
                onclick={stop_callback}
                disabled={*ws.ready_state == UseWebSocketReadyState::Closed}
            >
                <Icon icon_id={IconId::HeroiconsOutlineStopCircle} class="w-4 mr-2"/>
                {"Stop Stream"}
            </Button>
        </div>
    };

    let plot = plot.deref();
    html! {
        <Chart
            id={state.me().map(|x| x.uuid.to_string()).unwrap_or("chart_live".to_owned())}
            title="Dynotest Live Data"
            plot={plot.clone()}
            {top_side_button}
        />
    }
}
