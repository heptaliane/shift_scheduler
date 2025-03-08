use yew::{function_component, html, Html, Properties};

use crate::ui::components::table::{
    Table, TableBody, TableCell, TableHeader, TableHeaderCell, TableRow,
};
use crate::ui::data::DateConfig;

#[derive(Properties, PartialEq)]
pub struct DateTableProps {
    pub dates: Vec<DateConfig>,
}

#[function_component]
pub fn DateTable(props: &DateTableProps) -> Html {
    html! {
        <Table>
            <TableHeader>
                <TableRow>
                    <TableHeaderCell>{"Date"}</TableHeaderCell>
                </TableRow>
            </TableHeader>
            <TableBody>
            {
                props.dates.iter().map(|date| html! {
                    <TableRow>
                        <TableCell>{date.date.to_string()}</TableCell>
                    </TableRow>
                }).collect::<Html>()
            }
            </TableBody>
        </Table>
    }
}
