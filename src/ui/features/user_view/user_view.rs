use yew::{function_component, html, use_state_eq, AttrValue, Callback, Html, Properties};

use super::user_edit_form::UserEditForm;
use super::user_table::UserTable;
use crate::ui::components::card::Card;
use crate::ui::data::UserConfig;
use log::info;

#[derive(Properties, PartialEq)]
pub struct UserViewProps {
    pub users: Vec<UserConfig>,

    pub onchange: Callback<Vec<UserConfig>>,
}

#[function_component]
pub fn UserView(props: &UserViewProps) -> Html {
    let current_user = use_state_eq(|| None::<usize>);
    let users = use_state_eq(|| props.users.clone());
    let handle_edit = {
        let current_user = current_user.clone();
        Callback::from(move |i: usize| current_user.set(Some(i)))
    };
    let handle_add = {
        let current_user = current_user.clone();
        let users = users.clone();
        Callback::from(move |_: ()| {
            current_user.set(Some(users.len()));
        })
    };
    let handle_close = {
        let current_user = current_user.clone();
        Callback::from(move |_: ()| current_user.set(None))
    };
    let handle_submit = {
        let current_user = current_user.clone();
        let users = users.clone();
        let onchange = props.onchange.clone();
        Callback::from(move |user: UserConfig| {
            let idx = (*current_user).clone().unwrap();
            let mut new_users = (*users).clone();
            if users.len() > idx {
                new_users[idx] = user;
            } else {
                new_users.push(user);
            }
            users.set(new_users);
            current_user.set(None);
            onchange.emit((*users).clone());
        })
    };
    let new_user = {
        let users = props.users.clone();
        move || match users.last() {
            Some(user) => UserConfig {
                id: user.id + 1,
                name: AttrValue::from(""),
            },
            _ => UserConfig {
                id: 0,
                name: AttrValue::from(""),
            },
        }
    };

    html! {
        <div>
            <Card>
                <UserTable
                    users={(*users).clone()}
                    onedit={handle_edit}
                    onadd={handle_add}
                />
                if let Some(idx) = (*current_user).clone() {
                    <UserEditForm
                        user={props.users.get(idx).cloned().unwrap_or(new_user())}
                        oncancel={handle_close}
                        onsubmit={handle_submit}
                    />
                }
            </Card>
        </div>
    }
}
