use web_sys::wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;
use yew::{function_component, html, AttrValue, Callback, Event, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct SelectProp {
    pub selection: Vec<AttrValue>,

    #[prop_or(None)]
    pub selected: Option<usize>,

    pub onchange: Callback<Option<usize>>,
}

#[function_component]
pub fn Select(props: &SelectProp) -> Html {
    let handle_change = {
        let onchange = props.onchange.clone();
        Callback::from(move |e: Event| {
            let elem = e.target().unwrap().dyn_into::<HtmlSelectElement>().unwrap();
            onchange.emit(match elem.value().parse() {
                Ok(v) => Some(v),
                _ => None,
            });
        })
    };

    html! {
        <select
            class="form-select"
            onchange={handle_change}
        >
            <option selected={props.selected == None} />
            {
                props.selection.iter().enumerate().map(|(i, label)| html! {
                    <option
                        value={i.to_string()}
                        selected={props.selected == Some(i)}
                    >
                    {label}
                    </option>
                }).collect::<Html>()
            }
        </select>
    }
}
