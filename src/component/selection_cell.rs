use std::rc::Rc;

use web_sys::HtmlSelectElement;
use yew::prelude::use_node_ref;
use yew::{function_component, html, AttrValue, Callback, Html, Properties};

const DEFAULT_WIDTH: usize = 60;
const DEFAULT_HEIGHT: usize = 60;
const DEFAULT_COLOR: &str = "white";

#[derive(Properties, PartialEq)]
pub struct SelectionCellProps {
    pub selection: Rc<Vec<String>>,

    #[prop_or(0)]
    pub initial_selected: usize,

    #[prop_or(DEFAULT_WIDTH)]
    pub width: usize,
    #[prop_or(DEFAULT_HEIGHT)]
    pub height: usize,

    #[prop_or(AttrValue::from(DEFAULT_COLOR))]
    pub color: AttrValue,

    pub on_change: Callback<usize>,
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
                on_change.emit(input.value().parse().unwrap());
            }
        })
    };

    html! {
        <select
            class="form-select"
            style={
                format!(
                    "max-width: {:?}px; max-height: {:?}px; background-color: {}",
                    props.width,
                    props.height,
                    props.color,
                )
            }
            onchange={handle_change}
            ref={selection_ref}
        >
            {
                props.selection.iter().enumerate().map(|(i, label)| {
                    html!{
                        <option
                            value={i.to_string()}
                            selected={i == props.initial_selected}
                        >
                            {label}
                        </option>
                    }
                }).collect::<Html>()
            }
        </select>
    }
}
