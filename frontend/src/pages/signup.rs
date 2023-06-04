use crate::{
    components::{input::TextInput, landing_intro::LandingIntro, typography::ErrorText},
    LinkTag, Route,
};
use dyno_core::{users::UserRegistration, ApiResponse, DynoErr};

use yew::prelude::*;
enum DataMsg {
    Nim(String),
    Email(String),
    Password(String),
    ConfirmPassword(String),
    Role(String),
}

enum SignUpMsg {
    OnResponseSubmit(ApiResponse<String>),
    OnResponseError(ApiResponse<DynoErr>),
    OnUpdate(DataMsg),
    OnErrorMsg(String),
    OnLoading(bool),
}

impl SignUpMsg {
    #[inline]
    fn on_err(err: impl ToString) -> Self {
        Self::OnErrorMsg(err.to_string())
    }
    #[inline]
    const fn nim(v: String) -> Self {
        Self::OnUpdate(DataMsg::Nim(v))
    }
    #[inline]
    const fn pswd(v: String) -> Self {
        Self::OnUpdate(DataMsg::Password(v))
    }
    #[inline]
    const fn confirm_pswd(v: String) -> Self {
        Self::OnUpdate(DataMsg::Password(v))
    }
}

#[derive(Debug, Default, Clone)]
pub struct PageSignUp {
    data: UserRegistration,
    loading: bool,
    error_msg: String,
}

impl Component for PageSignUp {
    type Message = SignUpMsg;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        todo!()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let class_loading = if self.loading { "loading" } else { "" };
        let nim_value = AttrValue::from(self.data.nim.as_str());
        let email_value = AttrValue::from(self.data.email.as_str());
        let pswd_value = AttrValue::from(self.data.password.as_str());
        let confirm_pswd_value = AttrValue::from(self.data.confirm_password.as_str());
        let role_value = AttrValue::from(self.data.role.to_string());

        let div_form = html! {
            <form onsubmit={|e| {}}>
                <div class="mb-4">
                    <TextInput
                        defaultValue={registerObj.name}
                        updateType="name"
                        containerStyle="mt-4"
                        labelTitle="Name"
                        updateFormValue={updateFormValue}
                    />
                    <TextInput
                        defaultValue={registerObj.emailId}
                        updateType="emailId"
                        containerStyle="mt-4"
                        labelTitle="Email Id"
                        updateFormValue={updateFormValue}
                    />
                    <TextInput
                        defaultValue={registerObj.password}
                        type="password"
                        updateType="password"
                        containerStyle="mt-4"
                        labelTitle="Password"
                        updateFormValue={updateFormValue}
                    />
                </div>

                <ErrorText class="mt-8" >
                    {self.error_msg.clone()}
                </ErrorText>
                <button type="submit" class={classes!("btn", "mt-2", "w-full", "btn-primary", class_loading)}>{"Register"}</button>

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
}
