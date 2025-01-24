use std::collections::HashMap;

use yew::{
    function_component, html, use_state_eq, AttrValue, Callback, Html, MouseEvent, Properties,
};

use super::schedule_table::{ScheduleMap, ScheduleTable};
use crate::ui::data::{UserConfig, WorkType};

#[derive(Properties, PartialEq)]
pub struct ScheduleViewProp {
    pub users: Vec<UserConfig>,
    pub col_labels: Vec<AttrValue>,
    pub worktypes: Vec<WorkType>,

    pub onsubmit: Callback<Vec<Vec<Option<usize>>>>,
}

#[function_component]
pub fn ScheduleView(props: &ScheduleViewProp) -> Html {
    let schedule = use_state_eq(|| ScheduleMap::new());
    let handle_change = {
        let schedule = schedule.clone();
        Callback::from(move |new_schedule: ScheduleMap| {
            schedule.set(new_schedule);
        })
    };
    let handle_submit = {
        let schedule = schedule.clone();
        let onsubmit = props.onsubmit.clone();
        let users = props.users.clone();
        let n_cols = props.col_labels.len();
        Callback::from(move |_: MouseEvent| {
            onsubmit.emit(
                users
                    .iter()
                    .map(|user| {
                        (0..n_cols)
                            .map(|i| schedule.get(&(user.id, i)).cloned())
                            .collect()
                    })
                    .collect(),
            );
        })
    };
    html! {
        <div class="card">
            <div class="card-body">
                <ScheduleTable
                    schedule={(*schedule).clone()}
                    users={props.users.clone()}
                    col_labels={props.col_labels.clone()}
                    worktypes={props.worktypes.clone()}
                    onchange={handle_change.clone()}
                />
            </div>
            <div class="card-footer">
                <div class="d-grid">
                    <button
                        type="button"
                        class="btn btn-primary"
                    >
                        {"Submit"}
                    </button>
                </div>
            </div>
        </div>
    }
}
