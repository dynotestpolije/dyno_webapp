use gloo::storage::{LocalStorage, Storage};
use yew::prelude::*;

use crate::components::{input::TextInput, landing_intro::LandingIntro, typography::ErrorText};
use crate::{LinkTag, Route};

use dyno_core::users::UserLogin;
use dyno_core::{ApiResponse, DynoErr, UserSession};

enum SignInMsg {
    OnResponseSubmit(ApiResponse<UserSession>),
    OnResponseError(ApiResponse<DynoErr>),
    OnUpdateNim(String),
    OnUpdatePassword(String),
    OnErrorMsg(String),
    OnLoading(bool),
}
impl SignInMsg {
    #[inline]
    const fn pswd(v: String) -> Self {
        Self::OnUpdatePassword(v)
    }
    #[inline]
    const fn nim(v: String) -> Self {
        Self::OnUpdateNim(v)
    }

    #[inline]
    fn on_err(err: impl ToString) -> Self {
        Self::OnErrorMsg(err.to_string())
    }
}

#[derive(Debug, Clone, Properties, PartialEq)]
pub struct Name {
    setter: UseStateSetter<Option<UserSession>>,
}

#[derive(Debug, Default, Clone)]
pub struct PageSignIn {
    data: UserLogin,
    loading: bool,
    error_msg: String,
}

impl PageSignIn {
    async fn submit_login(data: UserLogin) -> SignInMsg {
        match gloo::net::http::Request::get("/api/auth/login").json(&data) {
            Ok(req) => match req.send().await {
                Ok(response) => {
                    let headers = response.headers();
                    match headers.get("Authorization") {
                        Some(token) => {
                            if let Err(err) = LocalStorage::set(crate::USER_TOKEN, token) {
                                dyno_core::log::error!("Failed set data in LocalStorage: {err}")
                            }
                        }
                        None => return SignInMsg::on_err("Something Bad Happen."),
                    }
                    if response.ok() {
                        match response.json::<ApiResponse<UserSession>>().await {
                            Ok(json) => SignInMsg::OnResponseSubmit(json),
                            Err(err) => SignInMsg::on_err(err),
                        }
                    } else {
                        match response.json::<ApiResponse<DynoErr>>().await {
                            Ok(json) => SignInMsg::OnResponseError(json),
                            Err(err) => SignInMsg::on_err(err),
                        }
                    }
                }
                Err(err) => SignInMsg::on_err(err),
            },
            Err(err) => SignInMsg::OnErrorMsg(err.to_string()),
        }
    }
}

impl Component for PageSignIn {
    type Message = SignInMsg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SignInMsg::OnUpdateNim(nim) => self.data.nim = nim,
            SignInMsg::OnUpdatePassword(password) => self.data.password = password,
            SignInMsg::OnErrorMsg(error) => self.error_msg = error,
            SignInMsg::OnLoading(loading) => self.loading = loading,
            SignInMsg::OnResponseSubmit(submit) => {}
            SignInMsg::OnResponseError(_) => {}
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let class_loading = if self.loading { "loading" } else { "" };
        let nim_value = AttrValue::from(self.data.nim.clone());
        let pswd_value = AttrValue::from(self.data.password.clone());

        let data = self.data.clone();
        let onsubmitlogin = link.callback_future(move |e: SubmitEvent| {
            e.prevent_default();
            Self::submit_login(data)
        });

        let onupdate_nim = link.callback(move |val: String| Self::Message::nim(val));
        let onupdate_pswd = link.callback(move |val: String| Self::Message::pswd(val));

        let form_input = html! {
        <form onsubmit={onsubmitlogin}>
            <div class="mb-4">
                <TextInput
                    types="text"
                    value={nim_value}
                    class="mt-4"
                    title="NIM/NIS"
                    update_callback={onupdate_nim}
                />
                <TextInput
                    types="password"
                    value={pswd_value}
                    class="mt-4"
                    title="Password"
                    update_callback={onupdate_pswd}
                />
            </div>

            <div class="text-right text-primary">
                <LinkTag to={Route::NotFound}>
                <span class=
                    "text-sm inline-block hover:text-primary hover:underline hover:cursor-pointer transition duration-200"
                >
                    {"Forgot Password?"}
                </span>
                </LinkTag>
            </div>

            <ErrorText class="mt-8" >
                {self.error_msg.clone()}
            </ErrorText>
            <button type="submit" class={classes!("btn", "mt-2", "w-full", "btn-primary", class_loading)}>{"Login"}</button>

            <div class="text-center mt-4">{"Don't have an account yet?"}
                <LinkTag to={Route::SignUp}>
                <span class="inline-block  hover:text-primary hover:underline hover:cursor-pointer transition duration-200">
                    {"Register"}
                </span>
                </LinkTag>
            </div>
        </form>
        };

        html! {
        <div class="min-h-screen bg-base-200 flex items-center">
            <div class="card mx-auto w-full max-w-5xl  shadow-xl">
                <div class="grid  md:grid-cols-2 grid-cols-1  bg-base-100 rounded-xl">
                    <div class="">
                        <LandingIntro />
                    </div>
                    <div class="py-24 px-10">
                        <h2 class="text-2xl font-semibold mb-2 text-center">{"Login"}</h2>
                        {form_input}
                    </div>
                </div>
            </div>
        </div>
        }
    }
}
