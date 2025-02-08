use yew::{function_component, html, AttrValue, Callback, Html, Properties};

use crate::ui::components::card::Card;
use crate::ui::components::editable_table::EditableTable;
use crate::ui::data::UserTag;

#[derive(Properties, PartialEq)]
pub struct TagViewProps {
    pub tags: Vec<UserTag>,

    pub onchange: Callback<Vec<UserTag>>,
}

#[function_component]
pub fn TagView(props: &TagViewProps) -> Html {
    let handle_change = {
        let onchange = props.onchange.clone();
        Callback::from(move |data: Vec<(usize, [AttrValue; 1])>| {
            onchange.emit(
                data.iter()
                    .map(|(id, arr)| UserTag {
                        id: id.clone(),
                        label: arr[0].clone(),
                    })
                    .collect(),
            )
        })
    };

    html! {
            <Card>
                <EditableTable<1>
                    data={
                        props
                            .tags
                            .iter()
                            .map(|tag| (tag.id, [tag.label.clone()]))
                            .collect::<Vec<(usize, [AttrValue; 1])>>()
                    }
                    headers={[AttrValue::from("Label")]}
                    default={[AttrValue::from("")]}
                    onchange={handle_change}
                />
            </Card>
    }
}
