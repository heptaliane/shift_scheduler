use web_sys::wasm_bindgen::JsCast;
use web_sys::HtmlAnchorElement;
use yew::{
    function_component, html, use_state_eq, AttrValue, Callback, ChildrenWithProps, MouseEvent, Html,
    Properties,
};

#[derive(Properties, PartialEq)]
pub struct NavItemProps {
    pub name: AttrValue,
    pub children: Html,
}

#[function_component]
pub fn NavItem(props: &NavItemProps) -> Html {
    html! {
        <div>
            {props.children.clone()}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct NavProps {
    pub children: ChildrenWithProps<NavItem>,
}

#[function_component]
pub fn Nav(props: &NavProps) -> Html {
    let tabs = use_state_eq(|| {
        props
            .children
            .iter()
            .map(|item| item.props.name.clone())
            .collect::<Vec<AttrValue>>()
    });
    let active = use_state_eq(|| 0);
    let handle_click = {
        let active = active.clone();
        Callback::from(move |e: MouseEvent| {
            let a = e.target().unwrap().dyn_into::<HtmlAnchorElement>().unwrap();
            active.set(a.name().parse().unwrap());
        })
    };

    html! {
        <div>
            <ul class="nav nav-tabs">
            {
                tabs.iter().enumerate().map(|(i, name)| html! {
                    <li class="nav-item">
                        if i == *active {
                            <a
                                class="nav-link active"
                                name={i.to_string()}
                                onclick={handle_click.clone()}
                            >
                                {name}
                            </a>
                        } else {
                            <a
                                class="nav-link"
                                name={i.to_string()}
                                onclick={handle_click.clone()}
                            >
                                {name}
                            </a>
                        }
                    </li>
                }).collect::<Html>()
            }
            </ul>
            {
                props
                    .children
                    .iter()
                    .enumerate()
                    .filter_map(|(i, item)| match i == *active {
                        true => Some(item),
                        _ => None,
                    })
                    .collect::<Html>()
            }
        </div>
    }
}
