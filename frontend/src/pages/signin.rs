use dyno_core::crypto::TokenDetails;
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use yewdux::prelude::Dispatch;

use crate::components::{input::TextInput, landing_intro::LandingIntro, typography::ErrorText};
use crate::state::AppState;
use crate::{LinkTag, Route};

use dyno_core::users::UserLogin;
use dyno_core::{ApiResponse, DynoErr, DynoResult, UserSession};

#[function_component(PageSignIn)]
pub fn signin() -> yew::Html {
    let loading = use_state(bool::default);
    let nim = use_state(AttrValue::default);
    let password = use_state(AttrValue::default);
    let error = use_state(AttrValue::default);

    let onsubmitlogin = {
        let nav = use_navigator();
        let nim = nim.to_string();
        let password = password.to_string();
        let loading = loading.setter();
        let error = error.setter();
        Dispatch::<AppState>::new().reduce_mut_future_callback_with(move |state, e: SubmitEvent| {
            e.prevent_default();
            let loading = loading.clone();
            let error = error.clone();
            let data = UserLogin {
                nim: nim.clone(),
                password: password.clone(),
            };
            let nav = nav.clone();

            Box::pin(async move {
                loading.set(true);
                match signin_submit(data).await {
                    Ok(token) => {
                        state.set_token_details(token);
                        if let Some(nav) = nav {
                            nav.push(&Route::Dashboard);
                        }
                    }
                    Err(err) => error.set(err.to_string().into()),
                }
                loading.set(false);
            })
        })
    };

    let nim_setter = use_callback(
        move |s, dp| {
            dp.set(s);
        },
        nim.clone(),
    );
    let password_setter = use_callback(
        move |s, dp| {
            dp.set(s);
        },
        password.clone(),
    );

    let form_input = html! {
    <form onsubmit={onsubmitlogin}>
        <div class="mb-4">
            <TextInput
                types="text"
                value={nim.to_string()}
                class="mt-4"
                title="NIM/Email"
                placeholder="Nim/Email"
                update_callback={nim_setter}
            />
            <TextInput
                types="password"
                value={password.to_string()}
                class="mt-4"
                title="Password"
                placeholder="Password"
                update_callback={password_setter}
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

        <ErrorText class="mt-8" > {error.as_ref()} </ErrorText>
        <button type="submit"
            class={classes!("btn", "mt-2", "w-full", "btn-primary",
                if *loading { "loading-dots loading-sm" } else { "" })
        }>
            {"Login"}
        </button>

        <div class="text-center mt-4">{"Don't have an account yet? "}
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

async fn signin_submit(data: UserLogin) -> DynoResult<TokenDetails> {
    if cfg!(debug_assertions) {
        return Ok(TokenDetails {
            user: UserSession {
                id: 1,
                uuid: dyno_core::uuid::Uuid::new_v4(),
                role: dyno_core::role::Roles::Admin,
            },
            token_id: dyno_core::uuid::Uuid::new_v4(),
            expires_in: None,
            token: None,
        });
    }
    match gloo::net::http::Request::post("/api/auth/login").json(&data) {
        Ok(req) => match req.send().await {
            Ok(response) => {
                if response.ok() {
                    response
                        .json::<ApiResponse<TokenDetails>>()
                        .await
                        .map(|x| x.payload)
                        .map_err(DynoErr::api_error)
                } else {
                    match response
                        .json::<ApiResponse<DynoErr>>()
                        .await
                        .map(|x| x.payload)
                    {
                        Ok(err) => Err(err),
                        Err(err) => Err(DynoErr::api_error(err)),
                    }
                }
            }
            Err(err) => Err(DynoErr::api_error(err)),
        },
        Err(err) => Err(DynoErr::api_error(err)),
    }
}
