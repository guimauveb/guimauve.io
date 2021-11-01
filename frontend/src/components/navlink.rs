use {
    crate::components::text::Text,
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
        let (label, on_hover) = (label.clone(), on_hover.clone());
        move |hovered: bool| on_hover.emit((label, hovered))
    };
    html! {
        <Text
             white_space="nowrap"
             color={if *is_other_link_hovered {"rgb(110, 110, 110)"} else { "inherit" }}
             onmouseover={Callback::from(move |_| handle_on_hover(true))}
             onmouseout={
                 let handle_on_hover = handle_on_hover.clone();
                 Callback::from(move |_| handle_on_hover(false))
             }
             value={if *hovered { *label_on_hover } else { *label }}
        />
    }
}
