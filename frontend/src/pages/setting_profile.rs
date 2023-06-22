use yew::{function_component, html, Html};
use yewdux::prelude::use_store;

use crate::{
    components::{
        cards::TitleCard,
        input::{TextInput, TextInputArea, ToggleInput},
    },
    state::AppState,
};

#[function_component(PageSettingProfile)]
pub fn setting_profile() -> Html {
    let (state, _) = use_store::<AppState>();
    let Some(user) = state.me().cloned() else {
        return html!{<h1>{"No User"}</h1>}
    };

    html! {
    <TitleCard title="Profile Settings" class="mt-2">

        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <TextInput title="Name" value={user.name} update_callback={move |_e| {}}/>
            <TextInput title="Email Id" value={user.email.unwrap_or_default()} update_callback={move |_e| {}}/>
            <TextInput title="NIM" value={user.nim} update_callback={move |_e| {}}/>
            <TextInput title="Role" value={user.role.to_string()} update_callback={move |_e| {}}/>
            <TextInputArea title="About" value="" update_callback={move |_e| {}}/>
        </div>
        <div class="divider" ></div>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <TextInput title="Language" value="Indonesia" update_callback={move |_e| {}}/>
            <TextInput title="Timezone" value="Western Indonesia Time" update_callback={move |_e| {}}/>
            <ToggleInput title="Sync Data" value={true} update_callback={move |_e| {}}/>
        </div>

        <div class="mt-16"><button class="btn btn-primary float-right" onclick={move |_e| {}}>{"Update"}</button></div>
    </TitleCard>
    }
}
