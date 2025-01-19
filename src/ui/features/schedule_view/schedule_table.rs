use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use yew::{function_component, html, AttrValue, Callback, Html, Properties};

use crate::ui::components::select::Select;
use crate::ui::components::table::Table;
use crate::ui::data::{UserConfig, WorkType};

type ScheduleMap = HashMap<(usize, usize), usize>;

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
    let selection_lut: Rc<HashMap<usize, usize>> = Rc::new(
        props
            .worktypes
            .iter()
            .enumerate()
            .map(|(i, w)| (i, w.id))
            .collect(),
    );
    let mut headers = vec![AttrValue::from("#")];
    headers.extend(props.col_labels.clone());
    let handle_change = {
        let schedule = props.schedule.clone();
        Callback::from(move |(user_id, col_id, value): (usize, usize, Option<usize>)| {})
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
                        selection_cell(
                            user.clone(),
                            i,
                            props.schedule.get(&(user.id, i)).copied(),
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
        <Table
            headers={headers}
            cell_elements={cells}
            primary_column={HashSet::from([0])}
        />
    }
}
