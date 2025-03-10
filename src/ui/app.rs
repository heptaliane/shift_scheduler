use yew::prelude::{function_component, html, use_state_eq, AttrValue, Callback, Html};

use super::components::nav::{Nav, NavItem};
use super::data::{DateConfig, UserConfig, WorkType};
use super::features::schedule_view::schedule_view::ScheduleView;

#[function_component(App)]
pub fn app() -> Html {
    let users = use_state_eq(|| Vec::<UserConfig>::new());
    let dates = use_state_eq(|| Vec::<DateConfig>::new());
    let worktypes = use_state_eq(|| Vec::<WorkType>::new());
    let handle_schedule_submit = {
        Callback::from(move |schedule: Vec<Vec<Option<usize>>>| {
            // TODO: implememnt ga
        })
    };

    html! {
        <Nav>
            <NavItem name={AttrValue::from("schedule")}>
                <ScheduleView
                    users={(*users).clone()}
                    col_labels={
                        dates
                            .iter()
                            .map(|d| AttrValue::from(d.date.to_string()))
                            .collect::<Vec<AttrValue>>()
                    }
                    worktypes={(*worktypes).clone()}
                    onsubmit={handle_schedule_submit.clone()}
                />
            </NavItem>
        </Nav>
    }
}
