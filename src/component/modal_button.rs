use yew::{
    function_component, html, use_state_eq, AttrValue, Callback, Html, MouseEvent, Properties,
};

#[derive(Properties, PartialEq)]
pub struct ModalButtonProps {
    pub children: Html,
    pub text: AttrValue,
    pub title: AttrValue,

    pub onsubmit: Callback<()>,
}

#[function_component]
pub fn ModalButton(props: &ModalButtonProps) -> Html {
    let visible = use_state_eq(|| false);
    let handle_toggle = {
        let visible = visible.clone();
        Callback::from(move |_: MouseEvent| {
            visible.set(!*visible);
        })
    };
    let handle_submit = {
        let visible = visible.clone();
        let onsubmit = props.onsubmit.clone();
        Callback::from(move |_: MouseEvent| {
            visible.set(false);
            onsubmit.emit(());
        })
    };

    html! {
        <div>
            <button
                type="button"
                class="btn btn-primary"
                onclick={handle_toggle.clone()}
            >
                {props.text.clone()}
            </button>
            if *visible {
                <div
                    class="modal show"
                    style="display: block;"
                >
                    <div class="modal-dialog">
                        <div class="modal-content">
                            <div class="modal-header">
                                <h1 class="modal-title fs-5">
                                    {props.title.clone()}
                                </h1>
                                <button
                                    class="btn-close"
                                    onclick={handle_toggle.clone()}
                                />
                            </div>
                            <div class="modal-body">
                                {props.children.clone()}
                            </div>
                            <div class="modal-footer">
                                <button
                                    type="button"
                                    class="btn btn-secondary"
                                    onclick={handle_toggle.clone()}
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
            }
        </div>
    }
}
