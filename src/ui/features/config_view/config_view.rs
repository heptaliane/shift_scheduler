use yew::{function_component, html, AttrValue, Callback, Html, Properties};

use crate::ui::components::accordion::{AccordionContainer, AccordionItem};
use crate::ui::data::{DateConfig, UserConfig, UserTag};
use crate::ui::features::date_view::date_view::DateView;
use crate::ui::features::tag_view::tag_view::TagView;
use crate::ui::features::user_view::user_view::UserView;

#[derive(Properties, PartialEq)]
pub struct ConfigViewProps {
    pub users: Vec<UserConfig>,
    pub tags: Vec<UserTag>,
    pub dates: Vec<DateConfig>,

    pub on_users_change: Callback<Vec<UserConfig>>,
    pub on_tags_change: Callback<Vec<UserTag>>,
    pub on_dates_change: Callback<Vec<DateConfig>>,
}

#[function_component]
pub fn ConfigView(props: &ConfigViewProps) -> Html {
    let handle_user_change = {
        let onchange = props.on_users_change.clone();
        Callback::from(move |users: Vec<UserConfig>| {
            onchange.emit(users);
        })
    };
    let handle_tag_change = {
        let onchange = props.on_tags_change.clone();
        Callback::from(move |tags: Vec<UserTag>| {
            onchange.emit(tags);
        })
    };
    let handle_date_change = {
        let onchange = props.on_dates_change.clone();
        Callback::from(move |dates: Vec<DateConfig>| {
            onchange.emit(dates);
        })
    };
    html! {
        <AccordionContainer>
            <AccordionItem header={AttrValue::from("Users")}>
                <UserView
                    users={props.users.clone()}
                    user_tags={props.tags.clone()}
                    onchange={handle_user_change}
                />
            </AccordionItem>
            <AccordionItem header={AttrValue::from("Tags")}>
                <TagView
                    tags={props.tags.clone()}
                    onchange={handle_tag_change}
                />
            </AccordionItem>
            <AccordionItem header={AttrValue::from("Dates")}>
                <DateView
                    dates={props.dates.clone()}
                    onchange={handle_date_change}
                />
            </AccordionItem>
        </AccordionContainer>
    }
}
