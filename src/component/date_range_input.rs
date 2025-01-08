use web_sys::HtmlInputElement;
use yew::{function_component, html, use_node_ref, AttrValue, Callback, Event, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct DateRangeInputProps {
    pub date_from: AttrValue,
    pub date_to: AttrValue,

    pub onchange: Callback<(AttrValue, AttrValue)>,
}

#[function_component]
pub fn DateRangeInput(props: &DateRangeInputProps) -> Html {
    let from_ref = use_node_ref();
    let to_ref = use_node_ref();
    let handle_change = {
        let onchange = props.onchange.clone();
        let from_ref = from_ref.clone();
        let to_ref = to_ref.clone();

        Callback::from(move |_: Event| {
            onchange.emit((
                AttrValue::from(from_ref.cast::<HtmlInputElement>().unwrap().value()),
                AttrValue::from(to_ref.cast::<HtmlInputElement>().unwrap().value()),
            ));
        })
    };

    html! {
        <table class="table">
            <tr>
                <th scope="row">{"Date from"}</th>
                <td>
                    <input
                        type="date"
                        ref={from_ref}
                        value={props.date_from.clone()}
                        onchange={handle_change.clone()}
                    />
                </td>
            </tr>
            <tr>
                <th scope="row">{"Date to"}</th>
                <td>
                    <input
                        type="date"
                        ref={to_ref}
                        value={props.date_to.clone()}
                        onchange={handle_change.clone()}
                    />
                </td>
            </tr>
        </table>
    }
}
