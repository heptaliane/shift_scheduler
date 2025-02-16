use std::collections::HashSet;

use web_sys::wasm_bindgen::JsCast;
use web_sys::{HtmlButtonElement, HtmlInputElement};
use yew::{
    function_component, html, use_state_eq, AttrValue, Callback, Event, Html, MouseEvent,
    Properties,
};

use crate::ui::components::form_container::FormContainer;
use crate::ui::components::table::{
    Table, TableBody, TableCell, TableHeader, TableHeaderCell, TableRow,
};

#[derive(Properties, PartialEq)]
pub struct EditableTableProps<const N: usize> {
    pub data: Vec<(usize, [AttrValue; N])>,
    pub headers: [AttrValue; N],
    pub default: [AttrValue; N],

    pub onchange: Callback<Vec<(usize, [AttrValue; N])>>,
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
    let handle_add = {
        let row_idx = row_idx.clone();
        let row_item = row_item.clone();
        let default_item = props.default.clone();
        let data = data.clone();
        Callback::from(move |_: MouseEvent| {
            row_idx.set(Some(data.len()));
            row_item.set(default_item.clone());
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
    let handle_close = {
        let row_idx = row_idx.clone();
        Callback::from(move |_: ()| row_idx.set(None))
    };
    let handle_submit = {
        let row_idx = row_idx.clone();
        let row_item = row_item.clone();
        let data = data.clone();
        let onchange = props.onchange.clone();
        Callback::from(move |_: ()| {
            let idx = (*row_idx).clone().unwrap();
            let mut new_data = (*data).clone();
            if new_data.len() > idx {
                new_data[idx].1 = (*row_item).clone();
            } else {
                let id = match new_data.last() {
                    Some(&(i, _)) => i + 1,
                    _ => 0,
                };
                new_data.push((id, (*row_item).clone()));
            }
            onchange.emit(new_data.clone());
            data.set(new_data);
            row_idx.set(None);
        })
    };

    html! {
        <div>
            <Table>
                <TableHeader>
                    <TableRow>
                        <TableHeaderCell>{"#"}</TableHeaderCell>
                        {
                            props.headers.iter().map(|txt| html! {
                                <TableHeaderCell>{txt}</TableHeaderCell>
                            }).collect::<Html>()
                        }
                        <TableHeaderCell>{"Edit"}</TableHeaderCell>
                    </TableRow>
                </TableHeader>
                <TableBody>
                {
                    data.iter().enumerate().map(|(i, (id, items))| html! {
                        <TableRow>
                            <TableCell header={true}>
                                {id}
                            </TableCell>
                            {
                                items.iter().map(|item| html! {
                                    <TableCell>{item}</TableCell>
                                }).collect::<Html>()
                            }
                            <TableCell>
                                <button
                                    type="button"
                                    class="btn btn-primary"
                                    name={i.to_string()}
                                    onclick={handle_edit.clone()}
                                >
                                    {"Edit"}
                                </button>
                            </TableCell>
                        </TableRow>
                    }).collect::<Html>()
                }
                </TableBody>
            </Table>
            <div class="d-grid">
                <button
                    type="button"
                    class="btn btn-primary"
                    onclick={handle_add}
                >
                    {"+"}
                </button>
            </div>
            if row_idx.is_some() {
                {
                    (0..N).map(|i| {
                        html! {
                            <FormContainer
                                oncancel={handle_close.clone()}
                                onsubmit={handle_submit.clone()}
                            >
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
                            </FormContainer>
                        }
                    }).collect::<Html>()
                }
            }
        </div>
    }
}
