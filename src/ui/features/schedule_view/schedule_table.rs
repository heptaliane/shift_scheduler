use std::collections::{HashMap, HashSet};

use yew::{function_component, html, AttrValue, Callback, Html, Properties};

use crate::ui::components::select::Select;
use crate::ui::components::table::{
    Table, TableBody, TableCell, TableHeader, TableHeaderCell, TableRow,
};
use crate::ui::data::{UserConfig, WorkType};

pub type ScheduleMap = HashMap<(usize, usize), usize>;

#[derive(Properties, PartialEq)]
pub struct ScheduleTableProp {
    pub schedule: ScheduleMap,

    pub users: Vec<UserConfig>,
    pub col_labels: Vec<AttrValue>,

    pub worktypes: Vec<WorkType>,

    pub onchange: Callback<HashMap<(usize, usize), usize>>,
}

fn selection_cell(
    user: UserConfig,
    col_id: usize,
    selected: Option<usize>,
    selection: Vec<AttrValue>,
    onchange: Callback<(usize, usize, Option<usize>)>,
) -> Html {
    let handle_change = Callback::from(move |selected: Option<usize>| {
        onchange.emit((user.id, col_id, selected));
    });

    html! {
        <Select
            selected={selected}
            selection={selection}
            onchange={handle_change}
        />
    }
}

#[function_component]
pub fn ScheduleTable(props: &ScheduleTableProp) -> Html {
    let selection: Vec<AttrValue> = props.worktypes.iter().map(|w| w.name.clone()).collect();
    let selection_lut: HashMap<Option<usize>, usize> = props
        .worktypes
        .iter()
        .enumerate()
        .map(|(i, w)| (Some(i), w.id))
        .collect();
    let mut headers = vec![AttrValue::from("#")];
    headers.extend(props.col_labels.clone());
    let handle_change = {
        let schedule = props.schedule.clone();
        let onchange = props.onchange.clone();
        Callback::from(
            move |(user_id, col_id, value): (usize, usize, Option<usize>)| {
                let mut schedule = schedule.clone();
                if let Some(v) = value {
                    schedule.insert((user_id, col_id), v);
                } else {
                    schedule.remove(&(user_id, col_id));
                }
                onchange.emit(schedule);
            },
        )
    };

    let cells = props
        .users
        .iter()
        .map(|user| {
            let mut cols: Vec<Html> = vec![html! {<p>{user.id}</p>}];
            cols.extend(
                props
                    .col_labels
                    .iter()
                    .enumerate()
                    .map(|(i, _)| {
                        let worktype_id = props.schedule.get(&(user.id, i)).cloned();
                        let selected = selection_lut.get(&worktype_id);
                        selection_cell(
                            user.clone(),
                            i,
                            selected.copied(),
                            selection.clone(),
                            handle_change.clone(),
                        )
                    })
                    .collect::<Vec<Html>>(),
            );
            cols
        })
        .collect::<Vec<Vec<Html>>>();

    html! {
        <Table>
            <TableHeader>
                <TableRow>
                    <TableHeaderCell>
                        {"#"}
                    </TableHeaderCell>
                    {
                        props.col_labels.iter().map(|label| html! {
                            <TableHeaderCell>
                                {label}
                            </TableHeaderCell>
                        }).collect::<Html>()
                    }
                </TableRow>
            </TableHeader>
            <TableBody>
                {
                    props.users.iter().map(|user| html! {
                        <TableRow>
                            <TableCell>
                                {user.name.clone()}
                            </TableCell>
                            {
                                props.col_labels.iter().enumerate().map(|(i, _)| {
                                    let worktype_id = props.schedule.get(&(user.id, i)).cloned();
                                    let selected = selection_lut.get(&worktype_id);
                                    selection_cell(
                                        user.clone(),
                                        i,
                                        selected.copied(),
                                        selection.clone(),
                                        handle_change.clone()
                                    )
                                }).collect::<Html>()
                            }
                        </TableRow>
                    }).collect::<Html>()
                }
            </TableBody>
        </Table>
    }
}
