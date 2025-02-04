use std::collections::HashSet;

use web_sys::wasm_bindgen::JsCast;
use web_sys::{HtmlButtonElement, HtmlInputElement};
use yew::{
    function_component, html, use_state_eq, AttrValue, Callback, Event, Html, MouseEvent,
    Properties,
};

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

    let handle_edit = {
        let row_idx = row_idx.clone();
        let row_item = row_item.clone();
        let data = data.clone();
        Callback::from(move |e: MouseEvent| {
            let btn = e.target().unwrap().dyn_into::<HtmlButtonElement>().unwrap();
            let idx: usize = btn.name().parse().unwrap();
            row_idx.set(Some(idx));
            let (_, item) = data[idx].clone();
            row_item.set(item);
        })
    };
    let handle_change = {
        let row_item = row_item.clone();
        Callback::from(move |e: Event| {
            let input = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
            let idx: usize = input.name().parse().unwrap();
            let mut new_item = (*row_item).clone();
            new_item[idx] = AttrValue::from(input.value());
            row_item.set(new_item);
        })
    };

    let elems: Vec<Vec<Html>> = data
        .iter()
        .enumerate()
        .map(|(i, (_, items))| {
            vec![
                vec![html! {"#"}],
                items.iter().map(|item| html! {item}).collect(),
                vec![html! {
                    <button
                        type="button"
                        name={i.to_string()}
                        class="btn btn-primary"
                        onclick={handle_edit.clone()}
                    >
                        {"Edit"}
                    </button>
                }],
            ]
            .concat()
        })
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
                                        name={i.to_string()}
                                        value={(*row_item)[i].clone()}
                                        onchange={handle_change.clone()}
                                    />
                                </div>
                            }
                        }).collect::<Html>()
                    }
                }
        </div>
    }
}
