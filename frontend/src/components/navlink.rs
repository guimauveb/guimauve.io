use {
    crate::components::text::Text,
    yew::{html, Callback, Properties},
    yew_functional::function_component,
};

#[derive(Properties, Clone, PartialEq)]
pub struct NavLinkProps<'a> {
    pub label: &'a str,
    #[prop_or_default]
    pub on_hover: Callback<(&'a str, bool)>,
    #[prop_or_default]
    pub on_hover_label: &'a str,
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
        on_hover_label,
        is_other_link_hovered,
    }: &NavLinkProps<'static>,
) -> Html {
    html! {
        <Text
             white_space="nowrap"
             color={if *is_other_link_hovered {"rgb(110, 110, 110)"} else { "inherit" }}
             onmouseover={
                        let (label, on_hover) = (*label, on_hover.clone());
                        Callback::from(move |_| on_hover.emit((label, true)))
              }
             onmouseout={
                       let (label, on_hover) = (*label, on_hover.clone());
                       Callback::from(move |_| on_hover.emit((label, false)))
             }
             value={if *hovered { *on_hover_label } else { *label }}
        />
    }
}
