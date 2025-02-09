use std::collections::{HashMap, HashSet};

use yew::{function_component, html, AttrValue, Html, Properties};

use crate::ui::components::table::Table;
use crate::ui::data::UserTag;

#[derive(Properties, PartialEq)]
pub struct TagPickerProps {
    pub tags: Vec<UserTag>,
    pub data: Vec<usize>,
}

#[function_component]
pub fn TagPicker(props: &TagPickerProps) -> Html {
    let tags_lut: HashMap<usize, UserTag> =
        HashMap::from_iter(props.tags.iter().map(|tag| (tag.id, tag.clone())));
    let cells: Vec<Vec<Html>> = props
        .data
        .iter()
        .map(|id| vec![html! {tags_lut[id].label.clone()}])
        .collect();

    html! {
        <Table
            headers={vec![AttrValue::from("Tag name")]}
            cell_elements={cells}
            primary_column={HashSet::new()}
        />
    }
}
