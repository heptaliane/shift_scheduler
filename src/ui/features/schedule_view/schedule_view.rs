use std::collections::HashMap;

use yew::{function_component, html, use_state_eq, AttrValue, Callback, Html, Properties};

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
    let handle_change = { Callback::from(move |schedule: ScheduleMap| {}) };
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
            </div>
        </div>
    }
}
