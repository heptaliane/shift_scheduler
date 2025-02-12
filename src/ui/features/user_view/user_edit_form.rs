use std::collections::HashMap;

use yew::{function_component, html, use_state_eq, AttrValue, Callback, Html, Properties};

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

    html! {
        <div>
            <label class="form-label">
                {"Name"}
            </label>
            <input
                type="text"
                class="form-control"
                value={(*user).name.clone()}
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
