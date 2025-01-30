use std::collections::HashSet;

use yew::{function_component, html, AttrValue, Callback, Html, MouseEvent, Properties};

use crate::ui::components::table::Table;
use crate::ui::data::UserConfig;

const HEADERS: [&str; 3] = ["#", "name", "edit"];

#[derive(Properties, PartialEq)]
pub struct UserTableProps {
    pub users: Vec<UserConfig>,

    pub onedit: Callback<usize>,
    pub onadd: Callback<()>,
}

#[function_component]
pub fn UserTable(props: &UserTableProps) -> Html {
    let elems: Vec<Vec<Html>> = props
        .users
        .iter()
        .enumerate()
        .map(|(i, user)| {
            vec![html! {user.id}, html! {user.name.clone()}, {
                let onedit = props.onedit.clone();
                html! {
                    <button
                        type="button"
                        class="btn btn-primary"
                        onclick={Callback::from(move |_: MouseEvent| {
                            onedit.emit(i);
                        })}
                    >
                    {"Edit"}
                    </button>
                }
            }]
        })
        .collect();
    let headers: Vec<AttrValue> = HEADERS
        .iter()
        .map(|h| AttrValue::from(h.to_string()))
        .collect();

    html! {
        <div class="d-grid gap-2">
            <Table
                headers={headers}
                cell_elements={elems}
                primary_column={HashSet::from_iter([0])}
            />
            <button
                type="button"
                class="btn btn-primary"
                onclick={
                    let onadd = props.onadd.clone();
                    Callback::from(move |_: MouseEvent| {
                        onadd.emit(());
                    })
                }
            >
                {"+"}
            </button>
        </div>
    }
}
