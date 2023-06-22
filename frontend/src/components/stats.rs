use yew::{classes, function_component, html, AttrValue, Classes, Html, Properties};
use yew_icons::{Icon, IconId};

#[derive(Properties, PartialEq, Clone)]
pub struct StatDashboardProps {
    pub icon: IconId,
    #[prop_or(From::from("title"))]
    pub title: AttrValue,
    #[prop_or(From::from("value"))]
    pub value: AttrValue,
    #[prop_or(From::from("description"))]
    pub desc: AttrValue,
}

pub fn get_desc_style(desc: &AttrValue) -> Classes {
    if desc.contains("↗︎") {
        From::from("font-bold text-green-700 dark:text-green-300")
    } else if desc.contains('↙') {
        From::from("font-bold text-rose-500 dark:text-red-400")
    } else {
        Default::default()
    }
}

#[function_component(Stats)]
pub fn stat_dashboard(
    StatDashboardProps {
        icon,
        title,
        value,
        desc,
    }: &StatDashboardProps,
) -> Html {
    html! {
        <div class="stats shadow">
            <div class="stat">
                <div class="stat-figure dark:text-slate-300 text-primary">
                    <Icon icon_id={*icon} class="w-8 h-8" />
                </div>
                <div class="stat-title dark:text-slate-300">{title}</div>
                <div class="stat-value dark:text-slate-300 text-primary">{value}</div>
                <div class={classes!("stat-desc", get_desc_style(desc))}>{desc}</div>
            </div>
        </div>
    }
}
