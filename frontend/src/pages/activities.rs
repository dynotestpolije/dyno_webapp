use std::ops::Deref;

use crate::{
    components::{cards::TitleCard, chart::Chart, stats::Stats, typography::Title},
    fetch::{fetch_and_save, fetch_data_dyno, fetch_info_byid},
    state::AppState,
};
use dyno_core::{
    chrono::{offset::TimeZone, Local},
    dynotests::DynoTest,
    DynoConfig, DynoPlot, MotorType, PlotColor,
};
use web_sys::MouseEvent;
use yew::{
    classes, function_component, html, platform::spawn_local, use_callback, use_effect_with_deps,
    use_state, AttrValue, Callback, Html, Properties, UseStateHandle,
};
use yew_icons::{Icon, IconId};
use yewdux::prelude::use_store;

#[function_component(PageActivities)]
pub fn activities() -> Html {
    let idx_open = use_state(|| Option::<usize>::None);
    let (state, dispatch) = use_store::<AppState>();
    let token = format!("Bearer {}", state.token_session().unwrap());

    let on_refresh = {
        let token = token.clone();
        dispatch.reduce_mut_future_callback_with(move |s, _| {
            let token = token.clone();
            Box::pin(async move { crate::fetch::fetch_dyno(s, token).await })
        })
    };

    let ondownload = {
        let token = token.clone();
        Callback::from(move |(url, tp): (String, String)| {
            dyno_core::log::info!("Callback fetch data: {url}");
            let token = token.clone();
            spawn_local(async move { fetch_and_save(url, tp, token).await.unwrap() })
        })
    };
    let is_admin = state.me().is_some_and(|x| x.role.is_admin());

    let table_body = {
        state
            .get_data()
            .dyno()
            .clone()
            .into_iter()
            .enumerate()
            .map(|(k, d)| {
                html! {
                    <tr key={k}>
                        <td>{d.id}</td>
                        <td>{d.info_id}</td>
                        <td>
                            if d.verified {
                                <Icon icon_id={IconId::HeroiconsOutlineCheck} />
                            } else {
                                <Icon icon_id={IconId::HeroiconsOutlineXMark} />
                            }
                        </td>
                        <td>{(d.stop - d.start).num_minutes()}</td>
                        <td>{Local.from_utc_datetime(&d.updated_at).format("%r %v").to_string()}</td>
                        <td>{Local.from_utc_datetime(&d.created_at).format("%r %v").to_string()}</td>
                        <td>
                            <button class="btn" onclick={let cb = idx_open.clone(); move |_| {
                                cb.set(Some(k))
                            }}>
                                {"Detail"}
                            </button>
                            if is_admin {
                                <button class="btn"> 
                                    {"Verify"}
                                </button>
                                <button class="btn"> 
                                    {"Delete"}
                                </button>
                            }
                        </td>
                    </tr>
                }
            })
    };

    let plot_color = state.plot_color();

    html! {
    <>
        <TitleCard class="mt-2" title="Aktivitas Mahasiswa" top_side_button={html!(
            <div class="inline-block float-right">
                <button class="btn px-6 btn-sm normal-case btn-primary" onclick={on_refresh}>{"Refresh"}</button>
            </div>
        )}>
        <div class="overflow-x-auto">
            <table class="table w-full">
                <thead>
                <tr>
                    <th>{"Id"}</th>
                    <th>{"Info Id"}</th>
                    <th>{"Verified"}</th>
                    <th>{"Duration (m)"}</th>
                    <th>{"Update At"}</th>
                    <th>{"Create At"}</th>
                    <th></th>
                </tr>
                </thead>
                <tbody>
                {for table_body}
                </tbody>
            </table>
        </div>

        </TitleCard>
        if let Some(open) = *idx_open {
            if let Some(data) = state.get_data().dyno().get(open) {
                <ModalAct
                    open={idx_open.clone()}
                    data={data.clone()}
                    token={token}
                    on_download={ondownload}
                    {plot_color}
                />
            }
        }
    </>
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct ModalActProps {
    pub open: UseStateHandle<Option<usize>>,
    pub data: DynoTest,
    pub token: String,
    pub on_download: Callback<(String, String)>,
    pub plot_color: PlotColor,
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
enum ModalTab {
    Info,
    Graph,
}
impl ModalTab {
    const fn to_str(self) -> &'static str {
        match self {
            ModalTab::Info => "Info",
            ModalTab::Graph => "Graph",
        }
    }
}

#[function_component(ModalAct)]
pub fn modal_activitas(props: &ModalActProps) -> Html {
    let info = use_state(|| Option::<DynoConfig>::None);
    let tabs = use_state(|| ModalTab::Info);
    {
        let token = props.token.clone();
        let info_setter = info.setter();
        let id = props.data.info_id;
        use_effect_with_deps(
            move |_| {
                let token = token.clone();
                spawn_local(async move {
                    let fetched = fetch_info_byid(token, id.unwrap_or_default()).await;
                    info_setter.set(fetched);
                })
            },
            (),
        );
    }
    let (name, tp, t) = if let Some(info) = info.as_ref() {
        match &info.motor_type {
            MotorType::Electric => (info.motor_info.name.clone(), " (Electric)", None),
            MotorType::Engine => (
                info.motor_info.name.clone(),
                " (Engine)",
                Some(info.motor_info.clone()),
            ),
        }
    } else {
        ("..".to_owned(), "..", None)
    };

    let on_download = {
        let on_download = props.on_download.clone();
        let data_url = props.data.data_url.clone();
        use_callback(
            move |e: MouseEvent, _| {
                e.prevent_default();
                on_download.emit((data_url.clone(), "bin".to_owned()));
            },
            (),
        )
    };

    let on_download_csv = {
        let on_download = props.on_download.clone();
        let data_url = props.data.data_url.clone();
        use_callback(
            move |e: MouseEvent, _| {
                let data_url = data_url.clone();
                e.prevent_default();
                on_download.emit((data_url, "csv".to_owned()));
            },
            (),
        )
    };
    let on_download_excel = {
        let on_download = props.on_download.clone();
        let data_url = props.data.data_url.clone();
        use_callback(
            move |e: MouseEvent, _| {
                let data_url = data_url.clone();
                e.prevent_default();
                on_download.emit((data_url, "excel".to_owned()));
            },
            (),
        )
    };

    let tabsetter = tabs.setter();
    html! {
        <dialog id={format!("modal_dyno_{}", props.open.unwrap_or(0))} class="modal modal_middle" open={props.open.is_some()}>
            <form method="dialog" class="modal-box w-11/12 max-w-5xl">
                <div class="tabs tabs-boxed">
                    <a
                        class={classes!("tab", if *tabs == ModalTab::Info { "tab-active" } else {""})}
                        onclick={let setter = tabsetter.clone(); move |e: MouseEvent|  {
                            e.prevent_default();
                            setter.set(ModalTab::Info);
                        }}
                    >
                        {ModalTab::Info.to_str()}
                    </a>
                    <a
                        class={classes!("tab", if *tabs == ModalTab::Graph { "tab-active" } else {""})}
                        onclick={let setter = tabsetter.clone(); move |e: MouseEvent|  {
                            e.prevent_default();
                            setter.set(ModalTab::Graph);
                        }}
                    >
                        {ModalTab::Graph.to_str()}
                    </a>
                </div>
                if *tabs == ModalTab::Info {
                    <Title class="text-center"> {"Info: "} {name} {tp} </Title>
                    if let Some(conf) = t {
                        <div class="grid lg:grid-cols-3 mt-1 md:grid-cols-1 grid-cols-3 gap-2">
                            <Stats
                                icon={IconId::HeroiconsOutlineCog}
                                title="Stroke"
                                value={(conf.stroke as u8).to_string()}
                                desc={"Stroke of the engine"}
                            />
                            <Stats
                                icon={IconId::HeroiconsOutlineCog}
                                title="Cylinder"
                                value={(conf.cylinder as u8).to_string()}
                                desc={"count of Cylinder of the engine"}
                            />
                            <Stats
                                icon={IconId::HeroiconsOutlineCog}
                                title="CC"
                                value={(conf.cc as u8).to_string()}
                                desc={"CC of the engine"}
                            />
                        </div>
                    }
                    if let Some(info) = info.as_ref() {
                        <table class="table w-full">
                            <thead>
                                <tr>
                                    <th>{"Name"}</th>
                                    <th>{"Value"}</th>
                                </tr>
                            </thead>
                            <tbody>
                                <tr>
                                    <th>{"Diameter Roller"}</th>
                                    <th>{info.diameter_roller.to_string()}</th>
                                </tr>
                                <tr>
                                    <th>{"Diameter Roller Beban"}</th>
                                    <th>{info.diameter_roller_beban.to_string()}</th>
                                </tr>
                                <tr>
                                    <th>{"Diameter Gear Encoder"}</th>
                                    <th>{info.diameter_gear_encoder.to_string()}</th>
                                </tr>
                                <tr>
                                    <th>{"Diameter Gear Beban"}</th>
                                    <th>{info.diameter_gear_beban.to_string()}</th>
                                </tr>
                                <tr>
                                    <th>{"Jarak Gear"}</th>
                                    <th>{info.jarak_gear.to_string()}</th>
                                </tr>
                                <tr>
                                    <th>{"Berat Beban"}</th>
                                    <th>{info.berat_beban.to_string()}</th>
                                </tr>
                            </tbody>
                        </table>
                    }
                } else {
                    <ChartDyno
                        id={props.data.uuid.to_string()}
                        name={name}
                        tp={tp}
                        color={props.plot_color.clone()}
                        token={props.token.clone()}
                        url={props.data.data_url.clone()}
                    />
                }
                <div class="modal-action">
                    <button class="btn" onclick={on_download}>
                        {"Download Bin"}
                    </button>
                    <button class="btn" onclick={on_download_csv}>
                        {"Download Csv"}
                    </button>
                    <button class="btn" onclick={on_download_excel}>
                        {"Download Excel"}
                    </button>
                    <button class="btn" onclick={let cb = props.open.clone(); move |e: MouseEvent| {
                        e.prevent_default();
                        cb.set(None);
                    }}>{"Close"}</button>
                </div>
            </form>
            <form method="dialog" class="modal-backdrop">
                <button class="btn" onclick={let cb = props.open.clone(); move |e: MouseEvent| {
                    e.prevent_default();
                    cb.set(None);
                }}>{"Close"}</button>
            </form>
        </dialog>
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct ChartDynoProps {
    #[prop_or(From::from("graph_dyno"))]
    id: AttrValue,
    #[prop_or(From::from("Graph Dyno"))]
    name: AttrValue,
    #[prop_or_default]
    tp: AttrValue,
    #[prop_or(PlotColor::dark())]
    color: PlotColor,
    #[prop_or_default]
    token: String,
    #[prop_or_default]
    url: String,
}
#[function_component(ChartDyno)]
fn chart_dyno(props: &ChartDynoProps) -> Html {
    let title = format!("Graph: {} {}", props.name, props.tp);
    let plot = use_state(DynoPlot::new);
    let fut_handle = yew_hooks::use_async::<_, _, ()>({
        let plot = plot.clone();
        let url = props.url.clone();
        let token = props.token.clone();
        let color = props.color.clone();
        async move {
            let response_data = fetch_data_dyno(url, token).await;
            plot.set(
                DynoPlot::new()
                    .set_color(color)
                    .create_dyno_plot(&response_data),
            );
            Ok(())
        }
    });

    {
        use_effect_with_deps(
            move |_| {
                fut_handle.run();
                || ()
            },
            props.color.clone(),
        );
    }

    let plot = plot.deref();

    html! {
        <Chart id={props.id.clone()} {title} plot={plot.clone()} />
    }
}
