use yew::{function_component, html, AttrValue, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct UserTagInputProps {
    pub labels: Vec<AttrValue>,
}

#[function_component]
pub fn UserTagInput(props: &UserTagInputProps) -> Html {
    html! {
        <div class="card">
            <div class="card-body">
            {
                props.labels.iter().map(|l| html! {
                    <span
                        class="badge text-bg-primary"
                        style="margin: 2px;"
                    >
                    {l}
                    </span>
                }).collect::<Html>()
            }
            </div>
        </div>
    }
}
