use yew::{function_component, html, use_state_eq, AttrValue, Callback, Html, Properties};

use crate::ui::components::card::Card;
use crate::ui::components::editable_table::EditableTable;
use crate::ui::data::UserConfig;

#[derive(Properties, PartialEq)]
pub struct UserViewProps {
    pub users: Vec<UserConfig>,

    pub onchange: Callback<Vec<UserConfig>>,
}

#[function_component]
pub fn UserView(props: &UserViewProps) -> Html {
    let handle_change = {
        let onchange = props.onchange.clone();
        Callback::from(move |data: Vec<(usize, [AttrValue; 1])>| {
            onchange.emit(
                data.iter()
                    .map(|(id, arr)| UserConfig {
                        id: id.clone(),
                        name: arr[0].clone(),
                    })
                    .collect(),
            );
        })
    };

    html! {
        <Card>
            <EditableTable<1>
                data={
                    props
                        .users
                        .iter()
                        .map(|user| (user.id, [user.name.clone()]))
                        .collect::<Vec<(usize, [AttrValue; 1])>>()
                }
                headers={[AttrValue::from("Name")]}
                default={[AttrValue::from("")]}
                onchange={handle_change}
            />
        </Card>
    }
}
