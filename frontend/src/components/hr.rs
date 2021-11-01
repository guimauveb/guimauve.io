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
    let mut style = String::with_capacity(
        7 + width.len() + 10 + height.len() + 10 + border.len() + 14 + border_top.len(),
    );
    style.push_str("width: ");
    style.push_str(&width);
    style.push_str("; height: ");
    style.push_str(&height);
    style.push_str("; border: ");
    style.push_str(&border);
    style.push_str("; border-top: ");
    style.push_str(&border_top);

    html! {
        <hr style={style} />
    }
}
