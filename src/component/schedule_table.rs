use std::collections::HashMap;
use std::rc::Rc;

use yew::{function_component, html, AttrValue, Callback, Html, Properties};

use super::data::WorkType;
use super::selection_cell::SelectionCell;

#[derive(Properties, PartialEq)]
pub struct ScheduleTableProp {
    pub schedule: Rc<HashMap<(usize, usize), usize>>,
    pub worker_names: Rc<Vec<String>>,
    pub date_labels: Rc<Vec<String>>,
    pub worktypes: Vec<WorkType>,

    pub on_change: Callback<(usize, usize, Option<usize>)>,
}

#[function_component]
pub fn ScheduleTable(props: &ScheduleTableProp) -> Html {
    let n_dates = props.date_labels.len();

    let worktypes: Vec<Rc<WorkType>> = props.worktypes.iter().map(|w| Rc::new(w.clone())).collect();
    let selection: Rc<Vec<String>> = Rc::new(worktypes.iter().map(|w| w.name.clone()).collect());

    html! {
        <table class="table">
            <thead>
                <tr>
                    <th scope="col">{"#"}</th>
                    {
                        props.date_labels.iter().map(|l| html! {
                            <th scope="col">{l}</th>
                        }).collect::<Html>()
                    }
                </tr>
            </thead>
            <tbody>
            {
                props.worker_names.iter().enumerate().map(|(i, name)| html! {
                    <tr>
                        <th scope="row">{name}</th>
                        {
                            (0..n_dates).map(|j| {
                                let on_change = props.on_change.clone();
                                let handle_change = Callback::from(move |v| {
                                    on_change.emit((i, j, v));
                                });
                                let (selected, color) = match props.schedule.get(&(i, j)) {
                                    Some(&v) => {
                                        let worktype = worktypes[v].clone();
                                        (Some(worktype.id), Some(worktype.color.clone()))
                                    },
                                    _ => (None, None),
                                };

                                html! {
                                    <td>
                                        <SelectionCell
                                            selection={selection.clone()}
                                            selected={selected}
                                            color={color}
                                            on_change={handle_change}
                                        />
                                    </td>
                                }
                            }).collect::<Html>()
                        }
                    </tr>
                }).collect::<Html>()
            }
            </tbody>
        </table>
    }
}
