use web_sys::wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{function_component, html, use_state_eq, AttrValue, Callback, Event, Html, Properties};

use crate::ui::components::form_container::FormContainer;
use crate::ui::data::UserConfig;

#[derive(Properties, PartialEq)]
pub struct UserEditFormProps {
    pub user: UserConfig,

    pub oncancel: Callback<()>,
    pub onsubmit: Callback<UserConfig>,
}

#[function_component]
pub fn UserEditForm(props: &UserEditFormProps) -> Html {
    let user = use_state_eq(|| props.user.clone());
    let onsubmit = {
        let user = user.clone();
        let onsubmit = props.onsubmit.clone();
        Callback::from(move |_| {
            onsubmit.emit((*user).clone());
        })
    };
    let oncancel = {
        let oncancel = props.oncancel.clone();
        Callback::from(move |_| {
            oncancel.emit(());
        })
    };
    let onchange = {
        let user = user.clone();
        Callback::from(move |e: Event| {
            let input = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
            user.set(UserConfig {
                id: user.id,
                name: AttrValue::from(input.value()),
            });
        })
    };

    html! {
        <FormContainer
            oncancel={oncancel.clone()}
            onsubmit={onsubmit.clone()}
        >
            <label class="form-label">
                {"Name"}
            </label>
            <input
                type="text"
                class="form-control"
                value={user.name.clone()}
                onchange={onchange.clone()}
            />
        </FormContainer>
    }
}
