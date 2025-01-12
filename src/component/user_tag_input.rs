use web_sys::wasm_bindgen::JsCast;
use web_sys::{HtmlButtonElement, HtmlInputElement};
use yew::{function_component, html, Callback, Event, Html, MouseEvent, Properties};

use super::data::UserTag;

#[derive(Properties, PartialEq)]
pub struct UserTagInputProps {
    pub tags: Vec<UserTag>,
    pub onchange: Callback<Vec<UserTag>>,
}

#[function_component]
pub fn UserTagInput(props: &UserTagInputProps) -> Html {
    let handle_change = {
        let tags = props.tags.clone();
        let onchange = props.onchange.clone();

        Callback::from(move |e: Event| {
            let elem = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
            let idx: usize = elem.name().parse().unwrap();
            let mut tags = tags.clone();
            tags[idx].label = elem.value();
            onchange.emit(tags);
        })
    };
    let handle_remove = {
        let tags = props.tags.clone();
        let onchange = props.onchange.clone();

        Callback::from(move |e: MouseEvent| {
            let elem = e.target().unwrap().dyn_into::<HtmlButtonElement>().unwrap();
            let idx: usize = elem.name().parse().unwrap();
            let mut tags = tags.clone();
            tags.remove(idx);
            onchange.emit(tags);
        })
    };
    let handle_add = {
        let tags = props.tags.clone();
        let onchange = props.onchange.clone();

        Callback::from(move |e: MouseEvent| {
            let mut tags = tags.clone();
            let id = match tags.last() {
                Some(tag) => tag.id + 1,
                _ => 0,
            };
            tags.push(UserTag::new(id, ""));
            onchange.emit(tags);
        })
    };

    html! {
        <table class="table">
            <thead>
                <tr>
                    <th scope="col">{"#"}</th>
                    <th scope="col">{"Tag"}</th>
                    <th scope="col"></th>
                </tr>
            </thead>
            <tbody>
                {
                    props.tags.iter().enumerate().map(|(i, t)| html! {
                        <tr>
                            <th scope="row">{t.id}</th>
                            <td>
                                <input
                                    type="text"
                                    class="form-control"
                                    name={i.to_string()}
                                    value={t.label.clone()}
                                    onchange={handle_change.clone()}
                                />
                            </td>
                            <td>
                                <button
                                    type="button"
                                    name={i.to_string()}
                                    class="btn btn-primary"
                                    onclick={handle_remove.clone()}
                                >
                                    {"Remove"}
                                </button>
                            </td>
                        </tr>
                    }).collect::<Html>()
                }
                <tr>
                    <td colspan="3">
                        <div class="d-grid">
                            <button
                                type="button"
                                class="btn btn-primary"
                                onclick={handle_add.clone()}
                            >
                                {"+"}
                            </button>
                        </div>
                    </td>
                </tr>
            </tbody>
        </table>
    }
}
