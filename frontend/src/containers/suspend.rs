use yew::{function_component, html, Html};

#[function_component(SuspenseContent)]
pub fn suspense_content() -> Html {
    html!(
        <div class="w-full h-screen text-gray-300 dark:text-gray-200 bg-base-100">
            {"Loading..."}
        </div>
    )
}
