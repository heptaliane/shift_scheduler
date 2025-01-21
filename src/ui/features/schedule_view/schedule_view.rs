use yew::{function_component, AttrValue, Callback, Properties, html, Html};

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
    html! {
        <div class="card">
            <div class="card-body">
            </div>
            <div class="card-footer">
            </div>
        </div>
    }
}
