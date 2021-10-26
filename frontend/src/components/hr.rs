use {
    yew::{html, Properties},
    yew_functional::function_component,
};

#[derive(Properties, Clone, PartialEq)]
pub struct HrProps {
    #[prop_or("auto")]
    pub width: &'static str,
    #[prop_or("auto")]
    pub height: &'static str,
    #[prop_or("0")]
    pub border: &'static str,
    #[prop_or("1px solid rgb(41, 41, 41)")]
    pub border_top: &'static str,
}

#[function_component(Hr)]
pub fn hr(
    HrProps {
        width,
        height,
        border,
        border_top,
    }: &HrProps,
) -> Html {
    html! {
        <hr style={format!("width: {}; height: {}; border: {}; border-top: {};", width, height, border, border_top)} />
    }
}
