use dyno_core::chrono::{Local, TimeZone};

use yew::{function_component, html, use_state, Html};
use yew_icons::{Icon, IconId};
use yewdux::prelude::use_store;

use crate::{components::cards::TitleCard, state::AppState};

#[function_component(PageAdminDynos)]
pub fn page_admin_dynos() -> Html {
    let idx_open = use_state(|| Option::<usize>::None);
    let (state, dispatch) = use_store::<AppState>();
    let on_refresh = dispatch.reduce_mut_future_callback_with(move |s, _| {
        let token = format!("Bearer {}", s.token_session().unwrap());
        Box::pin(async move { crate::fetch::fetch_dyno(s, token).await })
    });
    let _token = format!("Bearer {}", state.token_session().unwrap());
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
                            <button class="btn" onclick={|_e| {}}>
                                {"Validate"}
                            </button>
                            <button class="btn" onclick={let cb = idx_open.clone(); move |_| {
                                cb.set(Some(k))
                            }}>
                                {"Detail"}
                            </button>
                        </td>
                    </tr>
                }
            })
    };

    html! {
    <>
        <TitleCard class="mt-2" title="Dynotest Table Database" top_side_button={html!(
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
    </>
    }
}
