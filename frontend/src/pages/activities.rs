use crate::components::cards::TitleCard;
use yew::{function_component, html, use_callback, use_state, Children, Html, Properties};

#[derive(Clone, Properties, PartialEq)]
pub struct PageActivitiesProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(PageActivities)]
pub fn activities(props: &PageActivitiesProps) -> Html {
    let on_refresh = use_callback(|_, _| {}, ());

    html! {
    <TitleCard class="mt-2" title="Aktivitas Mahasiswa" top_side_button={html!(
        <div class="inline-block float-right">
            <button class="btn px-6 btn-sm normal-case btn-primary" onclick={on_refresh}>{"Refresh"}</button>
        </div>
    )}>
    <div class="overflow-x-auto">
        <table class="table w-full">
            <thead>
            <tr>
                <th></th>
                <th>{"Name"}</th>
            </tr>
            </thead>
            <tbody>
            <tr>
                <th>{1}</th>
                <td>{"Cy Ganderton"}</td>
            </tr>
            <tr>
                <th>{2}</th>
                <td>{"Hart Hagerty"}</td>
            </tr>
            <tr>
                <th>{3}</th>
                <td>{"Brice Swyre"}</td>
            </tr>
            <tr>
                <th>{4}</th>
                <td>{"Marjy Ferencz"}</td>
            </tr>
            <tr>
                <th>{5}</th>
                <td>{"Yancy Tear"}</td>
            </tr>
            </tbody>
            <tfoot>
            <tr>
                <th></th>
                <th>{"Name"}</th>
            </tr>
            </tfoot>
        </table>
    </div>

    </TitleCard>
    }
}
