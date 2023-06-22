use crate::{
    components::{
        input::{SelectBox, SelectOption, TextInput},
        landing_intro::LandingIntro,
        typography::ErrorText,
    },
    LinkTag, Route,
};
use dyno_core::{
    role::Roles, users::UserRegistration, validate_email, validate_nim, validate_password,
    ApiResponse, DynoErr, DynoResult,
};

use gloo::net::http::Request;
use web_sys::SubmitEvent;
use yew::{
    classes, function_component, html, platform::spawn_local, use_callback, use_state, AttrValue,
    Html,
};
use yew_router::prelude::use_navigator;

macro_rules! create_cb {
    ($validfunc:ident) => {
        move |s, (dp, v)| {
            v.set($validfunc(&s).err().map(|x| x.desc.into()));
            dp.set(s);
        }
    };
}

#[function_component(PageSignUp)]
pub fn signup() -> Html {
    let loading = use_state(bool::default);
    let nim = use_state(AttrValue::default);
    let email = use_state(AttrValue::default);
    let pswd = use_state(AttrValue::default);
    let confirm_pswd = use_state(AttrValue::default);
    let role = use_state(Roles::default);

    let validation_nim = use_state(Option::<AttrValue>::default);
    let validation_email = use_state(Option::<AttrValue>::default);
    let validation_pswd = use_state(Option::<AttrValue>::default);

    let error = use_state(AttrValue::default);

    let onupdate_nim = use_callback(
        create_cb!(validate_nim),
        (nim.clone(), validation_nim.clone()),
    );
    let onupdate_email = use_callback(
        create_cb!(validate_email),
        (email.clone(), validation_email.clone()),
    );
    let onupdate_pswd = use_callback(
        create_cb!(validate_password),
        (pswd.clone(), validation_pswd.clone()),
    );
    let onupdate_confirm_pswd = use_callback(
        create_cb!(validate_password),
        (confirm_pswd.clone(), validation_pswd.clone()),
    );
    let onupdate_role = use_callback(
        move |s: String, dp| {
            dp.set(Roles::from(s));
        },
        role.clone(),
    );

    let onsubmitsignup = {
        let navigator = use_navigator();
        let loading = loading.setter();
        let error = error.setter();
        let data = UserRegistration {
            nim: nim.to_string(),
            email: nim.to_string(),
            password: nim.to_string(),
            confirm_password: nim.to_string(),
            role: *role,
        };
        let validation_ok =
            validation_nim.is_none() && validation_pswd.is_none() && validation_nim.is_none();

        use_callback(
            move |e: SubmitEvent, nav| {
                e.prevent_default();
                if validation_ok {
                    let loading = loading.clone();
                    let error = error.clone();
                    let nav = nav.clone();
                    let data = data.clone();

                    spawn_local(async move {
                        loading.set(true);
                        match signup_submit(data).await {
                            Ok(_ok) => {
                                if let Some(nav) = nav {
                                    nav.push(&Route::Dashboard)
                                }
                            }
                            Err(err) => error.set(err.to_string().into()),
                        }
                        loading.set(false);
                    })
                } else {
                    error.set("Input Error, Check your input again".into());
                }
            },
            navigator,
        )
    };

    let div_form = html! {
        <form onsubmit={onsubmitsignup}>
            <div class="mb-4">
                <TextInput
                    value={nim.to_string()}
                    types="nim"
                    class="mt-4"
                    title="Nim"
                    border={if validation_nim.is_some() { "input-error" } else { "" }}
                    required={true}
                    update_callback={onupdate_nim}
                />
                if let Some(err) = validation_nim.as_ref() {
                    <span class="inline-flex text-sm text-red-700">
                        {err}
                        <svg xmlns="http://www.w3.org/2000/svg"
                            class="w-6 h-6"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                        >
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                        </svg>
                    </span>
                }
                <TextInput
                    value={email.to_string()}
                    types="email"
                    class="mt-4"
                    title="Email"
                    border={if validation_email.is_some() { "input-error" } else { "" }}
                    update_callback={onupdate_email}
                />
                if let Some(err) = validation_email.as_ref() {
                    <span class="inline-flex text-sm text-red-700">
                        {err}
                        <svg xmlns="http://www.w3.org/2000/svg"
                            class="w-6 h-6"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                        >
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                        </svg>
                    </span>
                }
                <TextInput
                    value={pswd.to_string()}
                    types="password"
                    class="mt-4"
                    title="Password"
                    border={if validation_pswd.is_some() { "input-error" } else { "" }}
                    required={true}
                    update_callback={onupdate_pswd}
                />
                if let Some(err) = validation_pswd.as_ref() {
                    <span class="inline-flex text-sm text-red-700">
                        {err}
                        <svg xmlns="http://www.w3.org/2000/svg"
                            class="w-6 h-6"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                        >
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                        </svg>
                    </span>
                }
                <TextInput
                    value={confirm_pswd.to_string()}
                    types="password"
                    class="mt-4"
                    title="Confirm Password"
                    border={ if validation_pswd.is_some() || pswd != confirm_pswd { "input-error" } else { "" } }
                    required={true}
                    update_callback={onupdate_confirm_pswd}
                />
                if let Some(err) = validation_pswd.as_ref() {
                    <span class="inline-flex text-sm text-red-700">
                        {err}
                        <svg xmlns="http://www.w3.org/2000/svg"
                            class="w-6 h-6"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                        >
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                        </svg>
                    </span>
                }
                <SelectBox
                    value={role.to_string()}
                    placeholder={"Choose Role"}
                    title={"Role"}
                    desc={"Choose role for register user, admin is disabled, only admin can change role to admin"}
                    container_class={"w-72"}
                    update_callback={onupdate_role}
                >
                    <SelectOption name={"Admin"} value={"admin"} disable={true} />
                    <SelectOption name={"User"} value={"user"} />
                    <SelectOption name={"Guest"} value={"guest"} />
                </SelectBox>
            </div>

            <ErrorText class="mt-8" > {error.as_ref()} </ErrorText>
            <button
                type="submit"
                class={classes!("btn", "mt-2", "w-full", "btn-primary",
                    if *loading { "loading loading-dots loading-sm" } else { "" }
                )}
            >
                {"Register"}
            </button>

            <div class="text-center mt-4">
                {"Already have an account? "}
                <LinkTag to={Route::SignIn}>
                    <span class="inline-block hover:text-primary hover:underline hover:cursor-pointer transition duration-200">
                        {"Login"}
                    </span>
                </LinkTag>
            </div>
        </form>
    };

    html! {
    <div class="min-h-screen bg-base-200 flex items-center">
        <div class="card mx-auto w-full max-w-5xl shadow-xl">
            <div class="grid  md:grid-cols-2 grid-cols-1  bg-base-100 rounded-xl">
                <div class="">
                        <LandingIntro />
                </div>
                <div class="py-24 px-10">
                    <h2 class="text-2xl font-semibold mb-2 text-center">{"Register"}</h2>
                    {div_form}
                </div>
            </div>
        </div>
    </div>
    }
}

async fn signup_submit(data: UserRegistration) -> DynoResult<usize> {
    match Request::post("/api/auth/register")
        .json(&data)
        .map_err(DynoErr::api_error)
    {
        Ok(req) => match req.send().await.map_err(DynoErr::api_error) {
            Ok(response) => {
                if response.ok() {
                    match response
                        .json::<ApiResponse<usize>>()
                        .await
                        .map_err(DynoErr::api_error)
                    {
                        Ok(d) => Ok(d.payload),
                        Err(err) => {
                            dyno_core::log::error!("{err}");
                            Err(err)
                        }
                    }
                } else {
                    match response
                        .json::<ApiResponse<DynoErr>>()
                        .await
                        .map_err(DynoErr::api_error)
                    {
                        Ok(json) => Err(json.payload),
                        Err(err) => {
                            dyno_core::log::error!("{err}");
                            Err("Failed no submit, something bad happen.".into())
                        }
                    }
                }
            }
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}
