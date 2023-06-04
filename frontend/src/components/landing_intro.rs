use yew::prelude::*;

#[function_component(Pointers)]
fn pointers() -> Html {
    html! {
    <>
        // <h1 className="text-2xl mt-8 font-bold">Admin Dashboard Starter Kit</h1>
        // <p className="py-2 mt-4">✓ <span className="font-semibold">Light/dark</span> mode toggle</p>
        // <p className="py-2 ">✓ <span className="font-semibold">Redux toolkit</span> and other utility libraries configured</p>
        // <p className="py-2">✓ <span className="font-semibold">Calendar, Modal, Sidebar </span> components</p>
        // <p className="py-2  ">✓ User-friendly <span className="font-semibold">documentation</span></p>
        // <p className="py-2  mb-4">✓ <span className="font-semibold">Daisy UI</span> components, <span className="font-semibold">Tailwind CSS</span> support</p>
    </>
    }
}

#[function_component(LandingIntro)]
pub fn landing_intro() -> Html {
    html! {
    <div class="hero min-h-full rounded-l-xl bg-base-200">
        <div class="hero-content py-12">
            <div class="max-w-md">

                <h1 class="text-3xl text-center font-bold ">
                    <img src="/img/logo192.png" class="w-12 inline-block mr-2 mask mask-circle" alt="dashwind-logo" />
                    {"DashWind"}
                </h1>

                <div class="text-center mt-12">
                    <img src="/img/intro.png" alt="Dashwind Admin Template" class="w-48 inline-block"/>
                </div>

                <Pointers />
            </div>

        </div>
    </div>
    }
}
