use std::rc::Rc;

use web_sys::HtmlSelectElement;
use yew::prelude::use_node_ref;
use yew::{function_component, html, AttrValue, Callback, Html, Properties};

const DEFAULT_COLOR: &str = "white";

const UNSELECTED_LABEL: &str = "-";

#[derive(Properties, PartialEq)]
pub struct SelectionCellProps {
    pub selection: Rc<Vec<String>>,

    #[prop_or(None)]
    pub selected: Option<usize>,

    #[prop_or(None)]
    pub color: Option<AttrValue>,

    pub on_change: Callback<Option<usize>>,
}

#[function_component]
pub fn SelectionCell(props: &SelectionCellProps) -> Html {
    let selection_ref = use_node_ref();
    let handle_change = {
        let selection_ref = selection_ref.clone();
        let on_change = props.on_change.clone();
        Callback::from(move |_| {
            let input = selection_ref.cast::<HtmlSelectElement>();
            if let Some(input) = input {
                on_change.emit(match input.value().parse() {
                    Ok(v) => Some(v),
                    _ => None,
                });
            }
        })
    };

    let color = props
        .color
        .clone()
        .unwrap_or(AttrValue::from(DEFAULT_COLOR));

    html! {
        <select
            class="form-select"
            style={format!("background-color: {}", color)}
            onchange={handle_change}
            ref={selection_ref}
        >
            <option
                value=""
                selected={props.selected == None}
            >
                {UNSELECTED_LABEL}
            </option>
            {
                props.selection.iter().enumerate().map(|(i, label)| {
                    html!{
                        <option
                            value={i.to_string()}
                            selected={props.selected == Some(i)}
                        >
                            {label}
                        </option>
                    }
                }).collect::<Html>()
            }
        </select>
    }
}
