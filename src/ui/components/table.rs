use std::collections::{HashMap, HashSet};

use yew::{function_component, html, AttrValue, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct TableProp {
    pub headers: Vec<AttrValue>,
    pub cell_elements: Vec<Vec<Html>>,

    #[prop_or(HashSet::from([0]))]
    pub primary_column: HashSet<usize>,
}

#[function_component]
pub fn Table(props: &TableProp) -> Html {
    html! {
        <table class="table">
            <thead>
                <tr>
                {
                    props.headers.iter().map(|label| html! {
                        <th scope="col">
                        {label}
                        </th>
                    }).collect::<Html>()
                }
                </tr>
            </thead>
            <tbody>
            {
                props.cell_elements.iter().map(|elems| html! {
                    <tr>
                    {
                        elems.iter().enumerate().map(|(i, elem)| html! {
                            if props.primary_column.contains(&i) {
                                <th scope="row">
                                {elem.clone()}
                                </th>
                            } else {
                                <td>
                                {elem.clone()}
                                </td>
                            }
                        }).collect::<Html>()
                    }
                    </tr>
                }).collect::<Html>()
            }
            </tbody>
        </table>
    }
}
