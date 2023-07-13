use std::ops::Deref;

use dyno_core::{
    log::{error, info},
    role::Roles,
    users::UserUpdate,
};
use web_sys::MouseEvent;
use yew::{function_component, html, use_callback, use_state, AttrValue, Html};
use yew_hooks::use_async;
use yewdux::prelude::use_store;

use crate::{
    components::{
        cards::TitleCard,
        input::{TextInput, ToggleInput},
    },
    state::AppState,
};

#[function_component(PageSettingProfile)]
pub fn setting_profile() -> Html {
    let (state, _) = use_store::<AppState>();
    let Some(user) = state.me().cloned() else {
        return html!{<h1>{"No User"}</h1>}
    };
    let nim = use_state(|| Option::<String>::None);
    let name = use_state(|| Option::<String>::None);
    let password = use_state(|| Option::<String>::None);
    let role = use_state(|| Option::<String>::None);
    let email = use_state(|| Option::<String>::None);

    let nim_val = nim.deref().clone();
    let name_val = name.deref().clone();
    let password_val = password.deref().clone();
    let role_val = role.deref().clone();
    let email_val = email.deref().clone();
    let on_update_async = {
        let id = user.id;
        let nim = nim_val.clone();
        let name = name_val.clone();
        let password = password_val.clone();
        let role = role_val.clone();
        let email = email_val.clone();
        use_async::<_, _, ()>(async move {
            let user_update = UserUpdate {
                nim,
                name,
                password,
                role: role.map(Roles::from),
                email,
                photo: None,
            };
            let ret = crate::fetch::update_user(
                id,
                user_update,
                state.token_session().cloned().unwrap_or_default(),
            )
            .await;
            match ret {
                Ok(ok) => info!("Success update user with id: {id}, Response: {ok}"),
                Err(err) => error!("Failed update user with id: {id} - {err}"),
            }
            Ok(())
        })
    };

    let on_update = {
        use_callback(
            move |e: MouseEvent, _| {
                e.prevent_default();
                on_update_async.run();
            },
            (),
        )
    };

    let nim_val = nim_val.unwrap_or(user.nim);
    let name_val = name_val.unwrap_or(user.name);
    let role_val = role_val.unwrap_or(user.role.to_string());
    let email_val = email_val.unwrap_or(user.email.unwrap_or_default());

    html! {
    <TitleCard title="Profile Settings" class="mt-2">

        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <TextInput
                title="Name"
                value={name_val}
                update_callback={move |e: AttrValue| {
                    name.set(Some(e.as_str().to_owned()))
                }}
            />
            <TextInput
                title="Email"
                types="email"
                value={email_val}
                update_callback={move |e: AttrValue| {
                    email.set(Some(e.as_str().to_owned()))
                }}
            />
            <TextInput
                title="NIM"
                value={nim_val}
                update_callback={move |e: AttrValue| {
                    nim.set(Some(e.as_str().to_owned()))
                }}
            />
            <TextInput
                title="Role"
                value={role_val}
                disabled={true}
            />
            <TextInput
                title="Password"
                types="password"
                value={password_val.unwrap_or("inipassword".to_owned())}
                update_callback={move |e: AttrValue| {
                    password.set(Some(e.as_str().to_owned()))
                }}
            />
        </div>
        <div class="divider" ></div>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <TextInput title="Language" value="Indonesia" update_callback={move |_e| {}}/>
            <TextInput title="Timezone" value="Western Indonesia Time" update_callback={move |_e| {}}/>
            <ToggleInput title="Sync Data" value={true} update_callback={move |_e| {}}/>
        </div>

        <div class="mt-16"><button class="btn btn-primary float-right" onclick={on_update}>{"Update"}</button></div>
    </TitleCard>
    }
}
