use std::collections::HashSet;

use yew::{function_component, html, AttrValue, Html, Properties};

use crate::ui::components::table::Table;

#[derive(Properties, PartialEq)]
pub struct EditableTableProps<const N: usize> {
    pub data: Vec<[AttrValue; N]>,
    pub headers: [AttrValue; N],
}

#[function_component]
pub fn EditableTable<const N: usize>(props: &EditableTableProps<N>) -> Html {
    let elems: Vec<Vec<Html>> = props
        .data
        .iter()
        .map(|items| items.iter().map(|item| html! { item }).collect())
        .collect();

    html! {
        <div>
            <Table
                headers={props.headers.to_vec()}
                cell_elements={elems}
                primary_column={HashSet::from_iter([0])}
            />
        </div>
    }
}
