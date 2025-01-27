use yew::{function_component, html, AttrValue, Callback, Html, MouseEvent, Properties};

use crate::ui::components::card::Card;

#[derive(Properties, PartialEq)]
pub struct FormContainerProps {
    #[prop_or(None)]
    pub header: Option<AttrValue>,

    pub children: Html,

    pub oncancel: Callback<()>,
    pub onsubmit: Callback<()>,
}

#[function_component]
pub fn FormContainer(props: &FormContainerProps) -> Html {
    let handle_cancel = {
        let oncancel = props.oncancel.clone();
        Callback::from(move |_: MouseEvent| {
            oncancel.emit(());
        })
    };
    let handle_submit = {
        let onsubmit = props.onsubmit.clone();
        Callback::from(move |_: MouseEvent| {
            onsubmit.emit(());
        })
    };

    html! {
        <Card
            header={
                html! {
                    <div>
                        <h5 class="card-title">{props.header.clone()}</h5>
                        <button
                            type="button"
                            class="btn-close"
                            onclick={handle_cancel.clone()}
                        />
                    </div>
                }
            }
            footer={
                html! {
                    <div>
                        <button
                            type="button"
                            class="btn btn-secondary"
                            onclick={handle_cancel.clone()}
                        >
                            {"Close"}
                        </button>
                        <button
                            type="button"
                            class="btn btn-primary"
                            onclick={handle_submit.clone()}
                        >
                            {"Submit"}
                        </button>
                    </div>
                }
            }
        >
            {props.children.clone()}
        </Card>
    }
}
