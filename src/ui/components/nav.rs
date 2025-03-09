use yew::{function_component, html, use_state_eq, AttrValue, ChildrenWithProps, Html, Properties};

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

    html! {
        <div>
            <ul class="nav nav-tabs">
            {
                tabs.iter().enumerate().map(|(i, name)| html! {
                    <li class="nav-item">
                        if i == *active {
                            <a class="nav-link active">
                                {name}
                            </a>
                        } else {
                            <a class="nav-link">
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
