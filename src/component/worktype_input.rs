use web_sys::{wasm_bindgen::JsCast, HtmlInputElement};
use yew::{function_component, html, Callback, Event, Html, Properties};

use super::data::WorkType;

#[derive(Properties, PartialEq)]
pub struct WorkTypeInputProps {
    pub worktypes: Vec<WorkType>,

    pub onchange: Callback<(usize, WorkType)>,
}

#[function_component]
pub fn WorkTypeInput(props: &WorkTypeInputProps) -> Html {
    let handle_text_change = {
        let worktypes = props.worktypes.clone();
        let onchange = props.onchange.clone();

        Callback::from(move |e: Event| {
            let elem = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();

            let idx: usize = elem.name().parse().unwrap();
            let worktype = worktypes[idx].clone();
            let value = elem.value();

            let new_worktype = WorkType::new(worktype.id, &value, &worktype.color);
            onchange.emit((idx, new_worktype));
        })
    };

    let handle_color_change = {
        let worktypes = props.worktypes.clone();
        let onchange = props.onchange.clone();

        Callback::from(move |e: Event| {
            let elem = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();

            let idx: usize = elem.name().parse().unwrap();
            let worktype = worktypes[idx].clone();
            let value = elem.value();

            let new_worktype = WorkType::new(worktype.id, &worktype.name, &value);
            onchange.emit((idx, new_worktype));
        })
    };

    html! {
        <table class="table">
            <thead>
                <tr>
                    <th scope="col">{"#"}</th>
                    <th scope="col">{"Name"}</th>
                    <th scope="col">{"Color"}</th>
                </tr>
            </thead>
            <tbody>
                {
                    props.worktypes.iter().enumerate().map(|(i, w)| {
                        let worktype = w.clone();
                        html!{
                            <tr>
                                <th scope="row">{worktype.id}</th>
                                <td>
                                    <input
                                        type="text"
                                        name={i.to_string()}
                                        value={worktype.name}
                                        onchange={handle_text_change.clone()}
                                    />
                                </td>
                                <td>
                                    <input
                                        type="color"
                                        name={i.to_string()}
                                        value={worktype.color}
                                        onchange={handle_color_change.clone()}
                                    />
                                </td>
                            </tr>
                        }
                    }).collect::<Html>()
                }
            </tbody>
        </table>
    }
}
