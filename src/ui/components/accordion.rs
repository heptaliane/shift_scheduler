use yew::{
    function_component, html, use_state_eq, AttrValue, Callback, Html, MouseEvent, Properties,
};

#[derive(Properties, PartialEq)]
pub struct AccordionContainerProps {
    pub children: Html,
}

#[function_component]
pub fn AccordionContainer(props: &AccordionContainerProps) -> Html {
    html! {
        <div class="accordion">
            {props.children.clone()}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct AccordionItemProps {
    pub header: AttrValue,
    pub children: Html,

    #[prop_or(true)]
    pub initial_show: bool,
}

#[function_component]
pub fn AccordionItem(props: &AccordionItemProps) -> Html {
    let show = use_state_eq(|| props.initial_show);
    let handle_show = {
        let show = show.clone();
        Callback::from(move |_: MouseEvent| show.set(true))
    };
    let handle_hide = {
        let show = show.clone();
        Callback::from(move |_: MouseEvent| show.set(false))
    };

    html! {
        <div class="accordion-item">
            <h2 class="accordion-header">
                if *show {
                    <button
                        class="accordion-button"
                        type="button"
                        onclick={handle_hide}
                    >
                        {props.header.clone()}
                    </button>
                } else {
                    <button
                        class="accordion-button collapsed"
                        type="button"
                        onclick={handle_show}
                    >
                        {props.header.clone()}
                    </button>
                }
            </h2>
            if *show {
                <div class="accordion-collapse collapse show">
                    <div class="accordion-body">
                        {props.children.clone()}
                    </div>
                </div>
            }
        </div>
    }
}
