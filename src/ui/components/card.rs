use yew::{function_component, html, Properties, Html};

#[derive(Properties, PartialEq)]
pub struct CardProp {
    #[prop_or(None)]
    pub header: Option<Html>,

    #[prop_or(None)]
    pub footer: Option<Html>,

    pub children: Html,
}

#[function_component]
pub fn Card(props: &CardProp) -> Html {
    html! {
        <div class="card">
        if let Some(header_elem) = props.header.clone() {
            <div class="card-header">
                {header_elem}
            </div>
        }
        <div class="card-body">
        {props.children.clone()}
        </div>
        if let Some(footer_elem) = props.footer.clone() {
            <div class="card-footer">
                {footer_elem}
            </div>
        }
        </div>
    }
}
