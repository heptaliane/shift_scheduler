use web_sys::wasm_bindgen::JsCast;
use web_sys::HtmlButtonElement;
use yew::{function_component, html, Callback, Html, MouseEvent, Properties};

use super::data::{UserConfig, UserTag};

#[derive(Properties, PartialEq)]
pub struct UserInputProps {
    pub users: Vec<UserConfig>,
    pub tags: Vec<UserTag>,

    pub onchange: Callback<Vec<UserConfig>>,
}

#[function_component]
pub fn UserInput(props: &UserInputProps) -> Html {
    let handle_remove = {
        let users = props.users.clone();
        let onchange = props.onchange.clone();

        Callback::from(move |e: MouseEvent| {
            let elem = e.target().unwrap().dyn_into::<HtmlButtonElement>().unwrap();
            let idx: usize = elem.name().parse().unwrap();
            let mut users = users.clone();
            users.remove(idx);
            onchange.emit(users);
        })
    };

    html! {
        <table class="table">
            <thead>
                <tr>
                    <th scope="col">{"#"}</th>
                    <th scope="col">{"Name"}</th>
                    <th scope="col">{"Tags"}</th>
                    <th scope="col"></th>
                </tr>
            </thead>
            <tbody>
                {
                    props.users.iter().enumerate().map(|(i, u)| {
                        let user = u.clone();
                        html! {
                            <tr>
                                <th scope="row">{user.id}</th>
                                <td>
                                    <input
                                        type="text"
                                        class="form-control"
                                        name={i.to_string()}
                                        value={user.name}
                                    />
                                </td>
                                <td>
                                    {
                                        user.tags.iter().map(|&j| {
                                            let tag = props.tags[j].clone();
                                            html!{
                                                <span
                                                    class="badge text-bg-primary"
                                                    style="margin: 2px;"
                                                >
                                                    {tag.label.clone()}
                                                </span>
                                            }
                                        }).collect::<Html>()
                                    }
                                </td>
                                <td>
                                    <button
                                        type="button"
                                        class="btn btn-primary"
                                        name={i.to_string()}
                                        onclick={handle_remove.clone()}
                                    >
                                        {"Remove"}
                                    </button>
                                </td>
                            </tr>
                        }
                    }).collect::<Html>()
                }
            </tbody>
        </table>
    }
}
