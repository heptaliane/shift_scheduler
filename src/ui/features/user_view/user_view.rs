use yew::{function_component, html, use_state_eq, AttrValue, Callback, Html, Properties};

use crate::ui::components::card::Card;
use crate::ui::components::table::Table;
use crate::ui::data::{UserConfig, UserTag};

#[derive(Properties, PartialEq)]
pub struct UserViewProps {
    pub users: Vec<UserConfig>,
    pub user_tags: Vec<UserTag>,

    pub onchange: Callback<Vec<UserConfig>>,
}

#[function_component]
pub fn UserView(props: &UserViewProps) -> Html {
    let users = use_state_eq(|| props.users.clone());
    let handle_change = {
        let onchange = props.onchange.clone();
        Callback::from(move |data: Vec<(usize, [AttrValue; 1])>| {
            onchange.emit(
                data.iter()
                    .map(|(id, arr)| UserConfig {
                        id: id.clone(),
                        name: arr[0].clone(),
                        tags: Vec::new(),
                    })
                    .collect(),
            );
        })
    };

    html! {
        <Card>
            <Table
                headers={vec![
                    AttrValue::from("#"),
                    AttrValue::from("Name"),
                    AttrValue::from("Tags"),
                    AttrValue::from("Edit"),
                ]}
            cell_elements={
                users.iter().map(|user| vec![
                    html!{user.id},
                    html!{user.name.clone()},
                    html!{
                        <div class="d-grid">
                        {
                            user.tags.iter().map(|&i| html! {
                                <span class="badge text-bg-primary">
                                    {props.user_tags[i].label.clone()}
                                </span>
                            }).collect::<Html>()
                        }
                        </div>
                    }
                ]).collect::<Vec<Vec<Html>>>()
            }
            />
        </Card>
    }
}
