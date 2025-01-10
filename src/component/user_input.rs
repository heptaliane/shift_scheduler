use yew::{function_component, html, AttrValue, Callback, Html, Properties};

use super::data::UserConfig;

#[derive(Properties, PartialEq)]
pub struct UserInputProps {
    pub users: Vec<UserConfig>,
    pub tags: Vec<AttrValue>,
}

#[function_component]
pub fn UserInput(props: &UserInputProps) -> Html {
    html! {
        <table class="table">
            <thead>
                <tr>
                    <th scope="col">{"#"}</th>
                    <th scope="col">{"Name"}</th>
                    <th scope="col">{"Tags"}</th>
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
                                            let tags = props.tags.clone();
                                            html!{
                                                <span
                                                    class="badge text-bg-primary"
                                                    style="margin: 2px;"
                                                >
                                                    {tags[j].clone()}
                                                </span>
                                            }
                                        }).collect::<Html>()
                                    }
                                </td>
                            </tr>
                        }
                    }).collect::<Html>()
                }
            </tbody>
        </table>
    }
}
