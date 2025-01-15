use yew::{
    function_component, html, use_state_eq, AttrValue, Callback, Html, MouseEvent, Properties,
};

#[derive(Properties, PartialEq)]
pub struct ModalButtonProps {
    pub children: Html,
    pub text: AttrValue,
    pub title: AttrValue,
}

#[function_component]
pub fn ModalButton(props: &ModalButtonProps) -> Html {
    let visible = use_state_eq(|| false);
    let onclick = {
        let visible = visible.clone();
        Callback::from(move |_: MouseEvent| {
            visible.set(!*visible);
        })
    };

    html! {
        <div>
            <button
                type="button"
                class="btn btn-primary"
                onclick={onclick.clone()}
            >
                {props.text.clone()}
            </button>
            if *visible {
                <div
                    class="modal fade show"
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
                                    onclick={onclick.clone()}
                                />
                            </div>
                            <div class="modal-body">
                                {props.children.clone()}
                            </div>
                            <div class="modal-footer">
                                <button
                                    type="button"
                                    class="btn btn-secondary"
                                    onclick={onclick.clone()}
                                >
                                    {"Close"}
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            }
        </div>
    }
}
