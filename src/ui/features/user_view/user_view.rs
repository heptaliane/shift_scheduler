use yew::{function_component, html, use_state_eq, Callback, Html, Properties};

use super::user_edit_form::UserEditForm;
use super::user_table::UserTable;
use crate::ui::components::card::Card;
use crate::ui::data::UserConfig;

#[derive(Properties, PartialEq)]
pub struct UserViewProps {
    pub users: Vec<UserConfig>,

    pub onchange: Callback<Vec<UserConfig>>,
}

#[function_component]
pub fn UserView(props: &UserViewProps) -> Html {
    let current_user = use_state_eq(|| None::<UserConfig>);
    let handle_edit = { Callback::from(move |i: usize| {}) };
    let handle_add = { Callback::from(move |_: ()| {}) };
    let handle_close = {
        let current_user = current_user.clone();
        Callback::from(move |_: ()| current_user.set(None))
    };
    let handle_submit = {
        let current_user = current_user.clone();
        let onchange = props.onchange.clone();
        Callback::from(move |user: UserConfig| {
            current_user.set(None);
        })
    };

    html! {
        <div>
            <Card>
                <UserTable
                    users={props.users.clone()}
                    onedit={handle_edit}
                    onadd={handle_add}
                />
                if let Some(user) = (*current_user).clone() {
                    <UserEditForm
                        id={user.id}
                        oncancel={handle_close}
                        onsubmit={handle_submit}
                    />
                }
            </Card>
        </div>
    }
}
