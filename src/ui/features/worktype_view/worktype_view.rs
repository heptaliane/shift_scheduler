use yew::{function_component, html, use_state_eq, Callback, Html, Properties};

use crate::ui::components::card::Card;
use crate::ui::components::table::{
    Table, TableBody, TableCell, TableHeader, TableHeaderCell, TableRow,
};
use crate::ui::data::WorkType;

#[derive(Properties, PartialEq)]
pub struct WorktypeViewProps {
    pub worktypes: Vec<WorkType>,

    pub onchange: Callback<Vec<WorkType>>,
}

#[function_component]
pub fn WorktypeView(props: &WorktypeViewProps) -> Html {
    let worktypes = use_state_eq(|| props.worktypes.clone());
    html! {
        <div>
            <Card>
                <Table>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell>{"#"}</TableHeaderCell>
                            <TableHeaderCell>{"Name"}</TableHeaderCell>
                            <TableHeaderCell>{"Color"}</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                    {
                        worktypes.iter().map(|worktype| html! {
                            <TableRow>
                                <TableCell header={true}>
                                    {worktype.id}
                                </TableCell>
                                <TableCell>
                                    {worktype.name.clone()}
                                </TableCell>
                                <TableCell>
                                    {worktype.color.clone()}
                                </TableCell>
                            </TableRow>
                        }).collect::<Html>()
                    }
                    </TableBody>
                </Table>
            </Card>
        </div>
    }
}
