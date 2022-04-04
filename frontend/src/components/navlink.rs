use {
    std::*,
    yew::{html, Callback, Properties},
    yew_functional::function_component,
};

#[derive(Properties, Clone, PartialEq)]
pub struct NavLinkProps {
    pub label: &'static str,
    #[prop_or_default]
    pub on_hover: Callback<(&'static str, bool)>,
    #[prop_or_default]
    pub label_on_hover: &'static str,
    #[prop_or_default]
    pub hovered: bool,
    #[prop_or_default]
    pub is_other_link_hovered: bool,
}

#[function_component(NavLink)]
pub fn navlink(
    NavLinkProps {
        label,
        on_hover,
        hovered,
        label_on_hover,
        is_other_link_hovered,
    }: &NavLinkProps,
) -> Html {
    let handle_on_hover = {
        let (label, on_hover) = (<&str>::clone(label), on_hover.clone());
        move |hovered: bool| on_hover.emit((label, hovered))
    };

    let white_space = "white-space: nowrap; ";
    let color = if *is_other_link_hovered {
        "color: rgb(110, 110, 110);"
    } else {
        "color: inherit;"
    };
    let mut style = String::with_capacity(white_space.len() + color.len());
    style.push_str(white_space);
    style.push_str(color);

    html! {
        <p
            style={style}
            onmouseover={Callback::from(move |_| handle_on_hover(true))}
            onmouseout={
                let handle_on_hover = handle_on_hover.clone();
                Callback::from(move |_| handle_on_hover(false))
            }
        >
            {if *hovered { *label_on_hover } else { *label }}
        </p>
    }
}
