use yew::{Properties, AttrValue, function_component, html, Html};

#[derive(Properties, PartialEq)]
pub struct AccordionContainerProps {
    pub children: Html,
}

#[function_component]
pub fn AccordionContainer(props: &AccordionContainerProps) -> Html {
    html! {
        <div class="accordion">
            {props.children}
        </div>
    }
}
