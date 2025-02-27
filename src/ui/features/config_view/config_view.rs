use yew::{function_component, html, Callback, Html, Properties};

use crate::ui::components::card::Card;
use crate::ui::data::{DateConfig, UserConfig, UserTag};
use crate::ui::features::date_view::date_view::DateView;
use crate::ui::features::tag_view::tag_view::TagView;
use crate::ui::features::user_view::user_view::UserView;

#[derive(Properties, PartialEq)]
pub struct ConfigViewProps {
    pub users: Vec<UserConfig>,
    pub tags: Vec<UserTag>,
    pub dates: Vec<DateConfig>,
}

#[function_component]
pub fn ConfigView(props: &ConfigViewProps) -> Html {
    let handle_user_change = {
        Callback::from(move |users: Vec<UserConfig>| {
            // TODO: Implement Callback
        })
    };
    let handle_tag_change = {
        Callback::from(move |tags: Vec<UserTag>| {
            // TODO: Implement Callback
        })
    };
    let handle_date_change = {
        Callback::from(move |dates: Vec<DateConfig>| {
            // TODO: Implement Callback
        })
    };
    html! {
        <Card>
            <UserView
                users={props.users.clone()}
                user_tags={props.tags.clone()}
                onchange={handle_user_change}
            />
            <TagView
                tags={props.tags.clone()}
                onchange={handle_tag_change}
            />
            <DateView
                onchange={handle_date_change}
            />
        </Card>
    }
}
