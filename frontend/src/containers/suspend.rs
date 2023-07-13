use yew::{function_component, html, Html};

#[function_component(SuspenseContent)]
pub fn suspense_content() -> Html {
    html!(
        <div class="fixed inset-0 flex items-center justify-center bg-opacity-50 bg-black">
            <div class="loading loading-spinner loading-xs"></div>
        </div>
    )
}
