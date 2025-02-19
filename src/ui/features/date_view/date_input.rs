use chrono::NaiveDate;
use web_sys::wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{function_component, html, Callback, Event, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct DateInputProps {
    pub value: NaiveDate,

    pub onchange: Callback<NaiveDate>,
}

#[function_component]
pub fn DateInput(props: &DateInputProps) -> Html {
    let handle_change = {
        let onchange = props.onchange.clone();
        Callback::from(move |e: Event| {
            let input = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
            onchange.emit(NaiveDate::parse_from_str(&input.value(), "%Y-%m-%d").unwrap());
        })
    };
    html! {
        <input
            type="date"
            class="form-control"
            value={props.value.to_string()}
            onchange={handle_change}
        />
    }
}
