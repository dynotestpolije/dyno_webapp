use yew::{function_component, html, Html};

#[function_component(LandingIntro)]
pub fn landing_intro() -> Html {
    html! {
    <div class="hero min-h-full rounded-l-xl bg-base-200">
        <div class="hero-content py-12">
            <div class="max-w-md">

                <h1 class="text-3xl text-center font-bold ">
                    <img src="/assets/logo192.png" class="w-12 inline-block mr-2 mask mask-circle" alt="dynotests-logo" />
                    {"Aplikasi Kompetensi Dynotests"}
                </h1>

                <div class="text-center mt-12">
                    <img src="/assets/intro.png" alt="dynotests-application" class="w-48 inline-block"/>
                </div>

                <div class="text-center mt-12">
                    <h3 class="text-xl mt-8 font-bold">{"Laboratorium Mesin Otomotif"}</h3>
                    <h1 class="text-xl font-bold">{"Politeknik Negeri Jember"}</h1>
                </div>
            </div>
        </div>
    </div>
    }
}
