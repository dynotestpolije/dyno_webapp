use dyno_core::MotorType;
use yew::{function_component, html, Html};
use yewdux::prelude::use_store;

use crate::{components::cards::TitleCard, state::AppState};

#[function_component(PageAdminInfos)]
pub fn page_admin_infos() -> Html {
    let (state, dispatch) = use_store::<AppState>();
    let token = format!("Bearer {}", state.token_session().unwrap());
    let on_refresh = {
        dispatch.reduce_mut_future_callback_with(move |s, _| {
            let token = token.clone();
            Box::pin(async move { crate::fetch::fetch_infos(s, token).await })
        })
    };

    let table_body = {
        state
            .get_data()
            .infos()
            .clone()
            .into_iter()
            .enumerate()
            .map(|(k, d)| {
                html! {
                    <tr key={k}>
                        <td>{k+1}</td>
                        <td>{d.motor_info.name.clone()}</td>
                        if let MotorType::Engine = &d.motor_type {
                            <td>{d.motor_info.cc.to_string()}</td>
                            <td>{d.motor_info.cylinder.to_string()}</td>
                            <td>{d.motor_info.stroke.to_string()}</td>
                        }
                    </tr>
                }
            })
    };
    html! {
    <>
        <TitleCard class="mt-2" title="Info Table Database" top_side_button={html!(
            <button class="btn px-6 btn-sm normal-case btn-primary" onclick={on_refresh}>{"Refresh"}</button>
        )}>
            <div class="overflow-x-auto">
                <table class="table w-full">
                    <thead>
                    <tr>
                        <th>{"Id"}</th>
                        <th>{"Name"}</th>
                        <th>{"CC"}</th>
                        <th>{"Cylinder"}</th>
                        <th>{"Stroke"}</th>
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
