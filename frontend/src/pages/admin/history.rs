use yew::{function_component, html, Html};
use yewdux::prelude::use_store;

use crate::{components::cards::TitleCard, state::AppState};

#[function_component(PageAdminHistory)]
pub fn page_admin_history() -> Html {
    let (state, dispatch) = use_store::<AppState>();
    let token = format!("Bearer {}", state.token().unwrap());
    let on_refresh = {
        dispatch.reduce_mut_future_callback_with(move |s, _| {
            let token = token.clone();
            Box::pin(async move { crate::fetch::fetch_histories(s, token).await })
        })
    };

    let table_body = {
        state
            .get_data()
            .histories()
            .clone()
            .into_iter()
            .enumerate()
            .map(|(k, d)| {
                html! {
                    <tr key={k}>
                        <td>{d.user_id}</td>
                        <td>{d.user_uuid.to_string()}</td>
                        <td>{d.dyno_id}</td>
                        <td>{d.long_usage}</td>
                        <td>{d.created_at.format("%r %v").to_string()}</td>
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
                        <th>{"User Id"}</th>
                        <th>{"User Uuid"}</th>
                        <th>{"Dyno Id"}</th>
                        <th>{"Long Usage"}</th>
                        <th>{"Created at"}</th>
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
