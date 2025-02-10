use std::collections::{HashMap, HashSet};

use web_sys::wasm_bindgen::JsCast;
use web_sys::HtmlButtonElement;
use yew::{
    function_component, html, use_state_eq, AttrValue, Callback, Html, MouseEvent, Properties,
};

use crate::ui::components::form_container::FormContainer;
use crate::ui::components::table::Table;
use crate::ui::data::UserTag;

#[derive(Properties, PartialEq)]
pub struct TagPickerProps {
    pub tags: Vec<UserTag>,
    pub data: Vec<usize>,

    pub onsubmit: Callback<Vec<UserTag>>,
}

#[function_component]
pub fn TagPicker(props: &TagPickerProps) -> Html {
    let tags_lut: HashMap<usize, UserTag> =
        HashMap::from_iter(props.tags.iter().map(|tag| (tag.id, tag.clone())));
    let data = use_state_eq(|| props.data.clone());
    let show = use_state_eq(|| false);
    let handle_delete = {
        let data = data.clone();
        Callback::from(move |e: MouseEvent| {
            let btn = e.target().unwrap().dyn_into::<HtmlButtonElement>().unwrap();
            let idx: usize = btn.name().parse().unwrap();
            let mut new_data = (*data).clone();
            new_data.remove(idx);
            data.set(new_data);
        })
    };
    let handle_show = {
        let show = show.clone();
        Callback::from(move |_: MouseEvent| show.set(true))
    };
    let handle_hide = {
        let show = show.clone();
        Callback::from(move |_: ()| show.set(false))
    };
    let handle_submit = {
        let show = show.clone();
        let data = data.clone();
        let onsubmit = props.onsubmit.clone();
        let tags_lut = tags_lut.clone();
        Callback::from(move |_: ()| {
            show.set(false);
            onsubmit.emit(data.iter().map(|id| tags_lut[id].clone()).collect());
        })
    };

    html! {
        <div>
            <button
                type="button"
                class="btn btn-primary"
                onclick={handle_show.clone()}
            >
                {"Edit"}
            </button>
            <FormContainer
                oncancel={handle_hide.clone()}
                onsubmit={handle_submit.clone()}
            >
                <Table
                    headers={vec![AttrValue::from("Tag name")]}
                    cell_elements={
                        data
                            .iter()
                            .enumerate()
                            .map(|(i, id)| vec![
                                html! {tags_lut[id].label.clone()},
                                html! {
                                    <button
                                        type="button"
                                        class="btn btn-primary"
                                        name={i.to_string()}
                                        onclick={handle_delete.clone()}
                                    >
                                        {"Delete"}
                                    </button>
                                },
                            ]).collect::<Vec<Vec<Html>>>()
                    }
                    primary_column={HashSet::new()}
                />
                <div class="d-grid">
                    <button
                        type="button"
                        class="btn btn-primary"
                    >
                        {"+"}
                    </button>
                </div>
            </FormContainer>
        </div>
    }
}
