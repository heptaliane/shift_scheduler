use web_sys::{wasm_bindgen::JsCast, HtmlInputElement};
use yew::{function_component, html, Callback, Event, Html, MouseEvent, Properties};

use super::data::WorkType;

const DEFAULT_WORKTYPE_NAME: &str = "";
const DEFAULT_WORKTYPE_COLOR: &str = "#ffffff";

#[derive(Properties, PartialEq)]
pub struct WorkTypeInputProps {
    pub worktypes: Vec<WorkType>,

    pub onchange: Callback<Vec<WorkType>>,
}

#[function_component]
pub fn WorkTypeInput(props: &WorkTypeInputProps) -> Html {
    let handle_text_change = {
        let worktypes = props.worktypes.clone();
        let onchange = props.onchange.clone();

        Callback::from(move |e: Event| {
            let elem = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();

            let idx: usize = elem.name().parse().unwrap();
            let mut worktypes = worktypes.clone();
            worktypes[idx].name = elem.value();
            onchange.emit(worktypes);
        })
    };

    let handle_color_change = {
        let worktypes = props.worktypes.clone();
        let onchange = props.onchange.clone();

        Callback::from(move |e: Event| {
            let elem = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();

            let idx: usize = elem.name().parse().unwrap();
            let mut worktypes = worktypes.clone();
            worktypes[idx].color = elem.value();
            onchange.emit(worktypes);
        })
    };

    let handle_worktype_add = {
        let worktypes = props.worktypes.clone();
        let onchange = props.onchange.clone();

        Callback::from(move |_: MouseEvent| {
            let mut worktypes = worktypes.clone();
            worktypes.push(WorkType::new(
                worktypes.len(),
                DEFAULT_WORKTYPE_NAME,
                DEFAULT_WORKTYPE_COLOR,
            ));
            onchange.emit(worktypes);
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
                <tr>
                    <td colspan="3">
                        <div class="d-grid">
                            <button
                                type="button"
                                class="btn btn-primary"
                                onclick={handle_worktype_add.clone()}
                            >
                                {"+"}
                            </button>
                        </div>
                    </td>
                </tr>
            </tbody>
        </table>
    }
}
