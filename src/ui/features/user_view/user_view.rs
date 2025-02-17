use std::collections::HashMap;

use web_sys::wasm_bindgen::JsCast;
use web_sys::HtmlButtonElement;
use yew::{
    function_component, html, use_state_eq, AttrValue, Callback, Html, MouseEvent, Properties,
};

use crate::ui::components::badge::Badge;
use crate::ui::components::card::Card;
use crate::ui::components::table::{
    Table, TableBody, TableCell, TableHeader, TableHeaderCell, TableRow,
};
use crate::ui::data::{UserConfig, UserTag};

use super::user_edit_form::UserEditForm;

#[derive(Properties, PartialEq)]
pub struct UserViewProps {
    pub users: Vec<UserConfig>,
    pub user_tags: Vec<UserTag>,

    pub onchange: Callback<Vec<UserConfig>>,
}

#[function_component]
pub fn UserView(props: &UserViewProps) -> Html {
    let users = use_state_eq(|| props.users.clone());
    let current_user = use_state_eq(|| None::<(usize, UserConfig)>);
    let tags_lut: HashMap<usize, UserTag> = props
        .user_tags
        .iter()
        .map(|tags| (tags.id, tags.clone()))
        .collect();
    let handle_change = {
        let current_user = current_user.clone();
        let users = users.clone();
        Callback::from(move |e: MouseEvent| {
            let btn = e.target().unwrap().dyn_into::<HtmlButtonElement>().unwrap();
            let idx: usize = btn.name().parse().unwrap();
            current_user.set(Some((idx, users[idx].clone())));
        })
    };
    let handle_add = {
        let current_user = current_user.clone();
        let users = users.clone();
        Callback::from(move |_: MouseEvent| {
            let id = match users.last() {
                Some(user) => user.id + 1,
                _ => 0,
            };
            current_user.set(Some((
                users.len(),
                UserConfig {
                    id,
                    name: AttrValue::from(""),
                    tags: Vec::new(),
                },
            )))
        })
    };
    let handle_submit = {
        let current_user = current_user.clone();
        let users = users.clone();
        Callback::from(move |new_user: UserConfig| {
            let (idx, _) = (*current_user).clone().unwrap();
            let mut new_users = (*users).clone();
            if new_users.len() > idx {
                new_users[idx] = new_user;
            } else {
                new_users.push(new_user);
            }
            users.set(new_users);
            current_user.set(None);
        })
    };
    let handle_cancel = {
        let current_user = current_user.clone();
        Callback::from(move |_: ()| {
            current_user.set(None);
        })
    };

    html! {
        <div>
            <Card>
                <Table>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell>{"#"}</TableHeaderCell>
                            <TableHeaderCell>{"Name"}</TableHeaderCell>
                            <TableHeaderCell>{"Tags"}</TableHeaderCell>
                            <TableHeaderCell>{"Edit"}</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                    {
                        users.iter().enumerate().map(|(i, user)| html!{
                            <TableRow>
                                <TableCell header={true}>
                                    {user.id}
                                </TableCell>
                                <TableCell>
                                    {user.name.clone()}
                                </TableCell>
                                <TableCell>
                                    <div class="d-grid">
                                    {
                                        user.tags.iter().map(|i| html! {
                                            <Badge text={tags_lut[i].label.clone()} />
                                        }).collect::<Html>()
                                    }
                                    </div>
                                </TableCell>
                                <TableCell>
                                    <button
                                        type="button"
                                        class="btn btn-primary"
                                        name={i.to_string()}
                                        onclick={handle_change.clone()}
                                    >
                                        {"Edit"}
                                    </button>
                                </TableCell>
                            </TableRow>
                        }).collect::<Html>()
                    }
                    </TableBody>
                </Table>
            </Card>
            if let Some((id, user)) = (*current_user).clone() {
                <UserEditForm
                    user={user}
                    tags={props.user_tags.clone()}
                    onsubmit={handle_submit}
                    oncancel={handle_cancel}
                />
            }
        </div>
    }
}
