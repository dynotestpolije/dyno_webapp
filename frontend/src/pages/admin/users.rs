use dyno_core::{
    chrono::{Local, TimeZone},
    AsStr,
};
use yew::{function_component, html, use_state, Html};
use yewdux::prelude::use_store;

use crate::{components::cards::TitleCard, state::AppState};

#[function_component(PageAdminUsers)]
pub fn page_admin_user() -> Html {
    let idx_open = use_state(|| Option::<usize>::None);
    let (state, dispatch) = use_store::<AppState>();
    let token = format!("Bearer {}", state.token_session().unwrap());
    let on_refresh = {
        let token = token.clone();
        dispatch.reduce_mut_future_callback_with(move |s, _| {
            let token = token.clone();
            Box::pin(async move { crate::fetch::fetch_user(s, token).await })
        })
    };
    let on_delete = {
        dispatch.reduce_mut_future_callback_with(move |s, idx: i64| {
            let token = token.clone();
            Box::pin(async move {
                if crate::fetch::fetch_delete_user(&token, idx).await {
                    crate::fetch::fetch_user(s, token).await
                }
            })
        })
    };

    let table_body = {
        state
            .get_data()
            .users()
            .clone()
            .into_iter()
            .enumerate()
            .map(|(k, d)| {
                html! {
                    <tr key={d.id}>
                        <td>{d.id}</td>
                        <td>{d.nim}</td>
                        <td>{d.name}</td>
                        <td>{d.email}</td>
                        <td>{d.role.as_str()}</td>
                        <td>{Local.from_utc_datetime(&d.updated_at).format("%r %v").to_string()}</td>
                        <td>{Local.from_utc_datetime(&d.created_at).format("%r %v").to_string()}</td>
                        <td>
                            <button class="btn" onclick={let cb = idx_open.clone(); move |_| {
                                cb.set(Some(k))
                            }}>
                                {"Update"}
                            </button>
                            <button class="btn" onclick={let cb = on_delete.clone(); move |_| {
                                cb.emit(d.id)
                            }}>
                                {"Delete"}
                            </button>
                        </td>
                    </tr>
                }
            })
    };

    html! {
    <>
        <TitleCard class="mt-2" title="Users Table Database" top_side_button={html!(
            <button class="btn px-6 btn-sm normal-case btn-primary" onclick={on_refresh}>{"Refresh"}</button>
        )}>
            <div class="overflow-x-auto">
                <table class="table w-full">
                    <thead>
                    <tr>
                        <th>{"Id"}</th>
                        <th>{"NIM"}</th>
                        <th>{"Nama"}</th>
                        <th>{"Email"}</th>
                        <th>{"Role"}</th>
                        <th>{"Update at"}</th>
                        <th>{"Created at"}</th>
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
