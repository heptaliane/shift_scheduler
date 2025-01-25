use std::collections::HashSet;

use yew::{function_component, html, AttrValue, Html, Properties};

use crate::ui::components::table::Table;
use crate::ui::data::UserConfig;

const HEADERS: [&str; 2] = ["#", "name"];

#[derive(Properties, PartialEq)]
pub struct UserTableProps {
    pub users: Vec<UserConfig>,
}

#[function_component]
pub fn UserTable(props: &UserTableProps) -> Html {
    let elems: Vec<Vec<Html>> = props
        .users
        .iter()
        .map(|user| vec![html! {user.id}])
        .collect();
    let headers: Vec<AttrValue> = HEADERS.iter().map(|h| AttrValue::from(h.clone())).collect();
    html! {
        <Table
            headers={headers}
            cell_elements={elems}
            primary_column={HashSet::from_iter([0])}
        />
    }
}
