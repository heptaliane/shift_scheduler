use yew::{function_component, html, Html, Properties};

use super::data::UserTag;

#[derive(Properties, PartialEq)]
pub struct UserTagInputProps {
    pub tags: Vec<UserTag>,
}

#[function_component]
pub fn UserTagInput(props: &UserTagInputProps) -> Html {
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
                                />
                            </td>
                        </tr>
                    }).collect::<Html>()
                }
            </tbody>
        </table>
    }
}
