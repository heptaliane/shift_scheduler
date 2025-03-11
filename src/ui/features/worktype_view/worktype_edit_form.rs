use web_sys::wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{function_component, html, use_state_eq, AttrValue, Callback, Event, Html, Properties};

use crate::ui::components::form_container::FormContainer;
use crate::ui::data::WorkType;

#[derive(Properties, PartialEq)]
pub struct WorktypeEditFormProp {
    pub worktype: WorkType,

    pub onsubmit: Callback<WorkType>,
    pub oncancel: Callback<()>,
}

#[function_component]
pub fn WorktypeEditForm(props: &WorktypeEditFormProp) -> Html {
    let worktype = use_state_eq(|| props.worktype.clone());
    let handle_submit = {
        let worktype = worktype.clone();
        let onsubmit = props.onsubmit.clone();
        Callback::from(move |_: ()| {
            onsubmit.emit((*worktype).clone());
        })
    };
    let handle_name_change = {
        let worktype = worktype.clone();
        Callback::from(move |e: Event| {
            let mut new_worktype = (*worktype).clone();
            let inp = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
            new_worktype.name = AttrValue::from(inp.value());
            worktype.set(new_worktype);
        })
    };
    let handle_color_change = {
        let worktype = worktype.clone();
        Callback::from(move |e: Event| {
            let mut new_worktype = (*worktype).clone();
            let inp = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
            new_worktype.color = AttrValue::from(inp.value());
            worktype.set(new_worktype);
        })
    };

    html! {
        <FormContainer
            onsubmit={handle_submit}
            oncancel={props.oncancel.clone()}
        >
            <label class="form-label">
                {"Name"}
            </label>
            <input
                type="text"
                class="form-control"
                value={(*worktype).name.clone()}
                onchange={handle_name_change}
            />
            <label class="form-label">
                {"Color"}
            </label>
            <input
                type="color"
                class="form-control"
                value={(*worktype).color.clone()}
                onchange={handle_color_change}
            />
        </FormContainer>
    }
}
