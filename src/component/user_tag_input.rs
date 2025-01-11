use web_sys::wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{function_component, html, Callback, Event, Html, Properties};

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

    html! {
        <table class="table">
            <thead>
                <tr>
                    <th scope="col">{"#"}</th>
                    <th scope="col">{"Tag"}</th>
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
                        </tr>
                    }).collect::<Html>()
                }
            </tbody>
        </table>
    }
}
