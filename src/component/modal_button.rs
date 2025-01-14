use yew::{
    function_component, html, use_state_eq, AttrValue, Callback, Html, MouseEvent, Properties,
};

#[derive(Properties, PartialEq)]
pub struct ModalButtonProps {
    pub children: Html,
    pub text: AttrValue,
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
            <div
                class="modal fade"
                show={visible.to_string()}
            >
                <div class="modal-dialog">
                    <div class="modal-content">
                        <div class="modal-body">
                            {props.children.clone()}
                        </div>
                        <div class="modal-footer">
                            <button
                                type="button"
                                class="btn btn-primary"
                                onclick={onclick.clone()}
                            >
                                {"Close"}
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
