use yew::{function_component, html, AttrValue, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct BadgeProps {
    pub text: AttrValue,

    #[prop_or(AttrValue::from("primary"))]
    pub variant: AttrValue,
}


#[function_component]
pub fn Badge(props: &BadgeProps) -> Html {
    html! {
        <span class={format!("badge text-bg-{}", props.variant)}>
            {props.text.clone()}
        </span>
    }
}
