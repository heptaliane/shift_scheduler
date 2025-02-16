use std::collections::HashMap;

use yew::{function_component, html, use_state_eq, AttrValue, Callback, Html, Properties};

use crate::ui::components::badge::Badge;
use crate::ui::components::card::Card;
use crate::ui::components::table::{
    Table, TableBody, TableCell, TableHeader, TableHeaderCell, TableRow,
};
use crate::ui::data::{UserConfig, UserTag};

#[derive(Properties, PartialEq)]
pub struct UserViewProps {
    pub users: Vec<UserConfig>,
    pub user_tags: Vec<UserTag>,

    pub onchange: Callback<Vec<UserConfig>>,
}

#[function_component]
pub fn UserView(props: &UserViewProps) -> Html {
    let users = use_state_eq(|| props.users.clone());
    let tags_lut: HashMap<usize, UserTag> = props
        .user_tags
        .iter()
        .map(|tags| (tags.id, tags.clone()))
        .collect();
    let handle_change = {
        let onchange = props.onchange.clone();
        Callback::from(move |data: Vec<(usize, [AttrValue; 1])>| {
            onchange.emit(
                data.iter()
                    .map(|(id, arr)| UserConfig {
                        id: id.clone(),
                        name: arr[0].clone(),
                        tags: Vec::new(),
                    })
                    .collect(),
            );
        })
    };

    html! {
        <Card>
            <Table>
                <TableHeader>
                    <TableRow>
                        <TableHeaderCell>{"#"}</TableHeaderCell>
                        <TableHeaderCell>{"Name"}</TableHeaderCell>
                        <TableHeaderCell>{"Tags"}</TableHeaderCell>
                        <TableHeaderCell>{"Edit"}</TableHeaderCell>
                    </TableRow>
                </TableHeader>
                <TableBody>
                {
                    users.iter().map(|user| html!{
                        <TableRow>
                            <TableCell header={true}>
                                {user.id}
                            </TableCell>
                            <TableCell>
                                {user.name.clone()}
                            </TableCell>
                            <TableCell>
                                <div class="d-grid">
                                {
                                    user.tags.iter().map(|i| html! {
                                        <Badge text={tags_lut[i].label.clone()} />
                                    }).collect::<Html>()
                                }
                                </div>
                            </TableCell>
                        </TableRow>
                    }).collect::<Html>()
                }
                </TableBody>
            </Table>
        </Card>
    }
}
