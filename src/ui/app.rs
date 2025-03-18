use yew::prelude::{function_component, html, use_state_eq, AttrValue, Callback, Html};

use super::components::nav::{Nav, NavItem};
use super::data::{DateConfig, UserConfig, WorkType, UserTag};
use super::features::schedule_view::schedule_view::ScheduleView;
use super::features::config_view::config_view::ConfigView;

#[function_component(App)]
pub fn app() -> Html {
    let users = use_state_eq(|| Vec::<UserConfig>::new());
    let dates = use_state_eq(|| Vec::<DateConfig>::new());
    let tags = use_state_eq(|| Vec::<UserTag>::new());
    let worktypes = use_state_eq(|| Vec::<WorkType>::new());
    let handle_schedule_submit = {
        Callback::from(move |schedule: Vec<Vec<Option<usize>>>| {
            // TODO: implememnt ga
        })
    };
    let handle_users_change = {
        let users = users.clone();
        Callback::from(move |new_users: Vec<UserConfig>| {
            users.set(new_users);
        })
    };
    let handle_dates_change = {
        let dates = dates.clone();
        Callback::from(move |new_dates: Vec<DateConfig>| {
            dates.set(new_dates);
        })
    };
    let handle_tags_change = {
        let tags = tags.clone();
        Callback::from(move |new_tags: Vec<UserTag>| {
            tags.set(new_tags);
        })
    };
    let handle_worktypes_change = {
        let worktypes = worktypes.clone();
        Callback::from(move |new_worktypes: Vec<WorkType>| {
            worktypes.set(new_worktypes);
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
            <NavItem name={AttrValue::from("Config")}>
                <ConfigView
                    users={(*users).clone()}
                    dates={(*dates).clone()}
                    tags={(*tags).clone()}
                    worktypes={(*worktypes).clone()}
                    on_users_change={handle_users_change}
                    on_dates_change={handle_dates_change}
                    on_tags_change={handle_tags_change}
                    on_worktypes_change={handle_worktypes_change}
                />
            </NavItem>
        </Nav>
    }
}
