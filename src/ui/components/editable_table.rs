use std::collections::HashSet;

use yew::{function_component, html, use_state_eq, AttrValue, Html, Properties};

use crate::ui::components::table::Table;

#[derive(Properties, PartialEq)]
pub struct EditableTableProps<const N: usize> {
    pub data: Vec<(usize, [AttrValue; N])>,
    pub headers: [AttrValue; N],
    pub default: [AttrValue; N],
}

#[function_component]
pub fn EditableTable<const N: usize>(props: &EditableTableProps<N>) -> Html {
    let row_idx = use_state_eq(|| None::<usize>);
    let row_item = use_state_eq(|| props.default.clone());
    let data = use_state_eq(|| props.data.clone());

    let elems: Vec<Vec<Html>> = data
        .iter()
        .map(|(_, items)| items.iter().map(|item| html! { item }).collect())
        .collect();

    html! {
        <div>
            <Table
                headers={props.headers.to_vec()}
                cell_elements={elems}
                primary_column={HashSet::from_iter([0])}
            />
                if let Some(idx) = (*row_idx).clone() {
                    {
                        (0..N).map(|i| {
                            html! {
                                <div>
                                    <label class="form-label">
                                        {props.headers[i].clone()}
                                    </label>
                                    <input
                                        type="text"
                                        class="form-control"
                                        value={(*row_item)[i].clone()}
                                    />
                                </div>
                            }
                        }).collect::<Html>()
                    }
                }
        </div>
    }
}
