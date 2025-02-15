use yew::{function_component, functional, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct TableProps {
    pub children: Html,
}

#[function_component]
pub fn Table(props: &TableProps) -> Html {
    html! {
        <table class="table">
            {props.children.clone()}
        </table>
    }
}

#[derive(Properties, PartialEq)]
pub struct TableHeaderProps {
    pub children: Html,
}

#[function_component]
pub fn TableHeader(props: &TableHeaderProps) -> Html {
    html! {
        <thead>
            {props.children.clone()}
        </thead>
    }
}

#[derive(Properties, PartialEq)]
pub struct TableBodyProps {
    pub children: Html,
}

#[function_component]
pub fn TableBody(props: &TableBodyProps) -> Html {
    html! {
        <tbody>
            {props.children.clone()}
        </tbody>
    }
}

#[derive(Properties, PartialEq)]
pub struct TableRowProps {
    pub children: Html,
}

#[function_component]
pub fn TableRow(props: &TableRowProps) -> Html {
    html! {
        <tr>
            {props.children.clone()}
        </tr>
    }
}

#[derive(Properties, PartialEq)]
pub struct TableHeaderCellProps {
    pub children: Html,
}

#[function_component]
pub fn TableHeaderCell(props: &TableHeaderCellProps) -> Html {
    html! {
        <th>
            {props.children.clone()}
        </th>
    }
}

#[derive(Properties, PartialEq)]
pub struct TableCellProps {
    pub children: Html,

    #[prop_or(false)]
    pub header: bool,
}

#[function_component]
pub fn TableCell(props: &TableCellProps) -> Html {
    if props.header {
        html! {
            <th scope="row">
                {props.children.clone()}
            </th>
        }
    } else {
        html! {
            <td>
                {props.children.clone()}
            </td>
        }
    }
}
