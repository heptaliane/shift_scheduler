use yew::{function_component, html, AttrValue, Callback, Html, MouseEvent, Properties};

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
        <div>
            <div class="modal show" style="display: block;">
                <div class="modal-dialog">
                    <div class="modal-content">
                        <div class="modal-header">
                            <h1 class="modal-title">
                                {props.header.clone()}
                            </h1>
                            <button
                                type="button"
                                class="btn-close"
                                onclick={handle_cancel.clone()}
                            />
                        </div>
                        <div class="modal-body">
                            {props.children.clone()}
                        </div>
                        <div class="modal-footer">
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
                    </div>
                </div>
            </div>
            <div class="modal-backdrop show" />
        </div>
    }
}
