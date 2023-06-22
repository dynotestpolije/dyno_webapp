use crate::{
    components::{cards::TitleCard, stats::Stats, typography::Title},
    state::AppState,
};
use dyno_core::{
    chrono::{offset::TimeZone, Local},
    dynotests::DynoTest,
    log, ApiResponse, DynoConfig, MotorType,
};
use gloo::net::http::Request;
use web_sys::MouseEvent;
use yew::{
    function_component, html, platform::spawn_local, use_callback, use_effect_with_deps, use_state,
    Callback, Html, Properties, UseStateHandle,
};
use yew_icons::{Icon, IconId};
use yewdux::prelude::use_store;

#[function_component(PageActivities)]
pub fn activities() -> Html {
    let idx_open = use_state(|| Option::<usize>::None);
    let (state, dispatch) = use_store::<AppState>();
    let on_refresh = dispatch.reduce_mut_future_callback_with(move |s, _| {
        let token = format!("Bearer {}", s.token().unwrap());
        Box::pin(async move { crate::fetch::fetch_dyno(s, token).await })
    });
    let token = format!("Bearer {}", state.token().unwrap());
    let ondownload = {
        let token = token.clone();
        Callback::from(move |url: String| {
            let token = token.clone();
            spawn_local(async move { crate::fetch::fetch_and_save(url, token).await.unwrap() })
        })
    };

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
                                {"Verified"}
                            } else {
                                <Icon icon_id={IconId::HeroiconsOutlineXMark} />
                                {"Verified"}
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
                            <button class="btn" onclick={let cb = ondownload.clone(); move |e: MouseEvent| {
                                e.prevent_default();
                                cb.emit(d.data_url.clone())
                            }}>
                                {"Download"}
                            </button>
                        </td>
                    </tr>
                }
            })
    };

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
        if let Some(data) = state.get_data().dyno().get(idx_open.unwrap_or(0)) {
            <ModalAct open={idx_open.clone()} data={data.clone()} token={token} on_download={ondownload.clone()}/>
        }
    </>
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct ModalActProps {
    pub open: UseStateHandle<Option<usize>>,
    pub data: DynoTest,
    pub token: String,
    pub on_download: Callback<String>,
}

#[function_component(ModalAct)]
pub fn modal_activitas(props: &ModalActProps) -> Html {
    let info = use_state(|| Option::<DynoConfig>::None);
    {
        let token = props.token.clone();
        let info_setter = info.setter();
        use_effect_with_deps(
            move |d| {
                let token = token.clone();
                let d = d.clone();
                spawn_local(async move {
                    let url = format!("/api/info?id={}", d.info_id.unwrap_or_default());
                    let fetched = match Request::get(&url)
                        .header("Authorization", token.as_ref())
                        .send()
                        .await
                    {
                        Ok(resp) if resp.ok() => resp
                            .json::<ApiResponse<DynoConfig>>()
                            .await
                            .map(|x| x.payload)
                            .ok(),
                        Err(err) => {
                            log::error!("{err}");
                            None
                        }
                        _ => None,
                    };
                    info_setter.set(fetched);
                })
            },
            props.data.clone(),
        );
    }
    let (name, tp, t) = if let Some(info) = info.as_ref() {
        (
            info.motor_type.name(),
            if info.motor_type.is_electric() {
                " (Electric)"
            } else {
                " (Engine)"
            },
            match &info.motor_type {
                MotorType::Electric(_) => None,
                MotorType::Engine(e) => Some(e),
            },
        )
    } else {
        ("..".to_owned(), "..", None)
    };

    let on_download_csv = {
        let on_download = props.on_download.clone();
        let data_url = format!("{}/csv", props.data.data_url.clone());
        use_callback(
            move |e: MouseEvent, _| {
                let data_url = data_url.clone();
                e.prevent_default();
                on_download.emit(data_url);
            },
            (),
        )
    };
    let on_download_excel = {
        let on_download = props.on_download.clone();
        let data_url = format!("{}/xlsx", props.data.data_url.clone());
        use_callback(
            move |e: MouseEvent, _| {
                let data_url = data_url.clone();
                e.prevent_default();
                on_download.emit(data_url);
            },
            (),
        )
    };
    html! {
        <dialog id={format!("modal_dyno_{}", props.open.unwrap_or(0))} class="modal" open={props.open.is_some()}>
            <form method="dialog" class="modal-box w-11/12 max-w-5xl">
                <Title class="font-bold text-lg"> {"Dynotest Info: "} {name} {tp} </Title>
                if let Some(conf) = t {
                    <div class="grid grid-cols-3 sm:grid-cols-1 gap-4">
                        <Stats  icon={IconId::HeroiconsOutlineCog} title="Stroke" value={conf.stroke.to_string()} desc={"Stroke of the engine"}/>
                        <Stats  icon={IconId::HeroiconsOutlineCog} title="Cylinder" value={conf.cylinder.to_string()} desc={"count of Cylinder of the engine"}/>
                        <Stats  icon={IconId::HeroiconsOutlineCog} title="CC" value={conf.cc.to_string()} desc={"CC of the engine"}/>
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
                <div class="modal-action">
                    <button class="btn" onclick={let cb = props.open.clone(); move |e: MouseEvent| {
                        e.prevent_default();
                        cb.set(None);
                    }}>{"Close"}</button>
                    <button class="btn" onclick={on_download_csv}>
                        {"Download Csv"}
                    </button>
                    <button class="btn" onclick={on_download_excel}>
                        {"Download Excel"}
                    </button>
                </div>

            </form>
        </dialog>
    }
}
