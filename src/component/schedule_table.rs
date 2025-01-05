use std::collections::HashMap;
use std::rc::Rc;

use yew::{function_component, html, Callback, Html, Properties};

use super::selection_cell::SelectionCell;

#[derive(Properties, PartialEq)]
pub struct ScheduleTableProp {
    pub psedo_schedule: Rc<HashMap<(usize, usize), usize>>,
    pub worker_names: Rc<Vec<String>>,
    pub date_labels: Rc<Vec<String>>,
    pub state_map: Rc<HashMap<usize, String>>,

    pub on_change: Callback<(usize, usize, Option<usize>)>,
}

#[function_component]
pub fn ScheduleTable(props: &ScheduleTableProp) -> Html {
    let n_dates = props.date_labels.len();
    let mut selection = props.state_map.iter().collect::<Vec<(&usize, &String)>>();
    selection.sort_by(|(a, _), (b, _)| a.cmp(b));
    let selection: Rc<Vec<String>> =
        Rc::new(selection.into_iter().map(|(_, v)| v.clone()).collect());

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
                                let handle_change = Callback::from(move |v: usize|{
                                    on_change.emit((i.clone(), j.clone(), Some(v)));
                                });
                                html! {
                                    <td>
                                        <SelectionCell
                                            selection={selection.clone()}
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
