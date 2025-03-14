use web_sys::wasm_bindgen::JsCast;
use web_sys::HtmlButtonElement;
use yew::{
    function_component, html, use_state_eq, AttrValue, Callback, Html, MouseEvent, Properties,
};

use super::worktype_edit_form::WorktypeEditForm;
use crate::ui::components::card::Card;
use crate::ui::components::table::{
    Table, TableBody, TableCell, TableHeader, TableHeaderCell, TableRow,
};
use crate::ui::data::WorkType;

#[derive(Properties, PartialEq)]
pub struct WorktypeViewProps {
    pub worktypes: Vec<WorkType>,

    pub onchange: Callback<Vec<WorkType>>,
}

#[function_component]
pub fn WorktypeView(props: &WorktypeViewProps) -> Html {
    let worktypes = use_state_eq(|| props.worktypes.clone());
    let current_worktype = use_state_eq(|| None::<(usize, WorkType)>);
    let handle_change = {
        let worktypes = worktypes.clone();
        let current_worktype = current_worktype.clone();
        Callback::from(move |e: MouseEvent| {
            let btn = e.target().unwrap().dyn_into::<HtmlButtonElement>().unwrap();
            let idx: usize = btn.name().parse().unwrap();
            current_worktype.set(Some((idx, worktypes[idx].clone())));
        })
    };
    let handle_add = {
        let worktypes = worktypes.clone();
        let current_worktype = current_worktype.clone();
        Callback::from(move |_: MouseEvent| {
            let id = match worktypes.last() {
                Some(worktype) => worktype.id + 1,
                _ => 0,
            };
            current_worktype.set(Some((
                worktypes.len(),
                WorkType {
                    id,
                    name: AttrValue::from(""),
                    color: AttrValue::from("white"),
                },
            )))
        })
    };
    let handle_submit = {
        let onchange = props.onchange.clone();
        let current_worktype = current_worktype.clone();
        let worktypes = worktypes.clone();
        Callback::from(move |new_worktype: WorkType| {
            let (idx, _) = (*current_worktype).clone().unwrap();
            let mut new_worktypes = (*worktypes).clone();
            if new_worktypes.len() > idx {
                new_worktypes[idx] = new_worktype;
            } else {
                new_worktypes.push(new_worktype);
            }
            onchange.emit(new_worktypes.clone());
            worktypes.set(new_worktypes);
            current_worktype.set(None);
        })
    };
    let handle_cancel = {
        let current_worktype = current_worktype.clone();
        Callback::from(move |_: ()| {
            current_worktype.set(None);
        })
    };

    html! {
        <div>
            <Card>
                <Table>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell>{"#"}</TableHeaderCell>
                            <TableHeaderCell>{"Name"}</TableHeaderCell>
                            <TableHeaderCell>{"Color"}</TableHeaderCell>
                            <TableHeaderCell>{"Edit"}</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                    {
                        worktypes.iter().enumerate().map(|(i, worktype)| html! {
                            <TableRow>
                                <TableCell header={true}>
                                    {worktype.id}
                                </TableCell>
                                <TableCell>
                                    {worktype.name.clone()}
                                </TableCell>
                                <TableCell>
                                    {worktype.color.clone()}
                                </TableCell>
                                <TableCell>
                                    <button
                                        type="button"
                                        class="btn btn-primary"
                                        name={i.to_string()}
                                        onclick={handle_change.clone()}
                                    >
                                        {"Edit"}
                                    </button>
                                </TableCell>
                            </TableRow>
                        }).collect::<Html>()
                    }
                        <TableRow>
                            <TableCell span={Some(4)}>
                                <div class="d-grid">
                                    <button
                                        type="button"
                                        class="btn btn-primary"
                                        onclick={handle_add}
                                    >
                                        {"+"}
                                    </button>
                                </div>
                            </TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </Card>
            if let Some((_, worktype)) = (*current_worktype).clone() {
                <WorktypeEditForm
                    worktype={worktype}
                    onsubmit={handle_submit}
                    oncancel={handle_cancel}
                />
            }
        </div>
    }
}
