use yew::{classes, function_component, html, Children, Classes, Html, Properties};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct TypoGraphyComps {
    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub children: Children,
}

#[function_component(ErrorText)]
pub fn error_text(
    TypoGraphyComps {
        class: this_class,
        children,
    }: &TypoGraphyComps,
) -> Html {
    html! {
        <p class={classes!("text-center", "text-error", this_class.clone())}>
            {children.clone()}
        </p>
    }
}

#[function_component(HelperText)]
pub fn helper_text(
    TypoGraphyComps {
        class: this_class,
        children,
    }: &TypoGraphyComps,
) -> Html {
    html! {
        <p class={classes!("text-slate-400", this_class.clone())}>
            {children.clone()}
        </p>
    }
}

#[function_component(Subtitle)]
pub fn subtitle(
    TypoGraphyComps {
        class: this_class,
        children,
    }: &TypoGraphyComps,
) -> Html {
    html! {
        <p class={classes!("text-xl", "font-semibold", this_class.clone())}>
            {children.clone()}
        </p>
    }
}

#[function_component(Title)]
pub fn title(
    TypoGraphyComps {
        class: this_class,
        children,
    }: &TypoGraphyComps,
) -> Html {
    html! {
        <p class={classes!("text-2xl", "font-bold", this_class.clone())}>
            {children.clone()}
        </p>
    }
}
