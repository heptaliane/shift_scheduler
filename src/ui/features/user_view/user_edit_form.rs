use web_sys::wasm_bindgen::JsCast;
use web_sys::{HtmlButtonElement, HtmlInputElement};
use yew::{
    function_component, html, use_state_eq, AttrValue, Callback, Event, Html, MouseEvent, Properties
};

use crate::ui::data::{UserConfig, UserTag};

#[derive(Properties, PartialEq)]
pub struct UserEditFormProps {
    pub user: UserConfig,
    pub tags: Vec<UserTag>,

    pub onsubmit: Callback<UserConfig>,
    pub oncancel: Callback<()>,
}

#[function_component]
pub fn UserEditForm(props: &UserEditFormProps) -> Html {
    let user = use_state_eq(|| props.user.clone());
    let handle_tag_change = {
        let user = user.clone();
        let tags = props.tags.clone();
        Callback::from(move |e: MouseEvent| {
            let btn = e.target().unwrap().dyn_into::<HtmlButtonElement>().unwrap();
            let idx: usize = btn.name().parse().unwrap();
            let tag_id = tags[idx].id;
            let mut new_user = (*user).clone();

            if let Some(i) = new_user.tags.iter().position(|&i| i == tag_id) {
                new_user.tags.remove(i);
            } else {
                new_user.tags.push(tag_id);
            }
            user.set(new_user);
        })
    };
    let handle_name_change = {
        let user = user.clone();
        Callback::from(move |e: Event| {
            let mut new_user = (*user).clone();
            let inp = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
            new_user.name = AttrValue::from(inp.value());
            user.set(new_user);
        })
    };

    html! {
        <div>
            <label class="form-label">
                {"Name"}
            </label>
            <input
                type="text"
                class="form-control"
                value={(*user).name.clone()}
                onchange={handle_name_change.clone()}
            />
            <label class="form-label">
                {"Tags"}
            </label>
            <div class="gap-2">
                {
                    props.tags.iter().enumerate().map(|(i, t)| {
                        let class_str = match user.tags.contains(&t.id) {
                            true => "btn btn-primary btn-sm active",
                            _ => "btn btn-primary btn-sm",
                        };
                        html! {
                            <button
                                type="button"
                                class={class_str}
                                name={i.to_string()}
                                onclick={handle_tag_change.clone()}
                            >
                                {t.label.clone()}
                            </button>
                        }
                    }).collect::<Html>()
                }
            </div>
        </div>
    }
}
