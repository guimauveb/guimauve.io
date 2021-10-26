use {
    yew::{html, Callback, Children, MouseEvent, Properties},
    yew_functional::function_component,
};

#[derive(Properties, Clone, PartialEq)]
pub struct BoxProps {
    #[prop_or("unset")]
    pub top: &'static str,
    #[prop_or("unset")]
    pub left: &'static str,
    #[prop_or("unset")]
    pub right: &'static str,
    #[prop_or("inherit")]
    pub font_size: &'static str,
    #[prop_or("unset")]
    pub max_height: &'static str,
    #[prop_or("inherit")]
    pub max_width: &'static str,
    #[prop_or("unset")]
    pub width: &'static str,
    #[prop_or("unset")]
    pub height: &'static str,
    #[prop_or("static")]
    pub position: &'static str,
    #[prop_or("unset")]
    pub mt: &'static str,
    #[prop_or("unset")]
    pub mb: &'static str,
    #[prop_or("unset")]
    pub ml: &'static str,
    #[prop_or("unset")]
    pub mr: &'static str,
    #[prop_or("unset")]
    pub p: &'static str,
    #[prop_or("unset")]
    pub br: &'static str,
    #[prop_or("block")]
    pub display: &'static str,
    #[prop_or("unset")]
    pub flex: &'static str,
    #[prop_or("row")]
    pub flex_direction: &'static str,
    #[prop_or("unset")]
    pub flex_wrap: &'static str,
    #[prop_or("normal")]
    pub justify_content: &'static str,
    #[prop_or("normal")]
    pub align_items: &'static str,
    #[prop_or_default]
    pub children: Children,
    #[prop_or("unset")]
    pub overflow_x: &'static str,
    #[prop_or("unset")]
    pub overflow_y: &'static str,
    #[prop_or("")]
    pub style: &'static str,
    #[prop_or("unset")]
    pub cursor: &'static str,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or("inherit")]
    pub word_break: &'static str,
}

#[function_component(BoxComponent)]
pub fn box_component(props: &BoxProps) -> Html {
    let style = format!(
        "display: {};
                position: {};
                top: {};
                left: {};
                right: {};
                max-width: {};
                width: {};
                max-height: {};
                height: {};
                margin-top: {};
                margin-bottom: {};
                margin-left: {};
                margin-right: {};
                padding: {};
                flex: {};
                flex-direction: {};
                flex-wrap: {};
                justify-content: {};
                align-items: {};
                font-size: {};
                border-radius: {};
                overflow-x: {};
                overflow-y: {};
                cursor: {};
                word-break: {};
                {};",
        &props.display,
        &props.position,
        &props.top,
        &props.left,
        &props.right,
        &props.max_width,
        &props.width,
        &props.max_height,
        &props.height,
        &props.mt,
        &props.mb,
        &props.ml,
        &props.mr,
        &props.p,
        &props.flex,
        &props.flex_direction,
        &props.flex_wrap,
        &props.justify_content,
        &props.align_items,
        &props.font_size,
        &props.br,
        &props.overflow_x,
        &props.overflow_y,
        &props.cursor,
        &props.word_break,
        &props.style
    );

    html! {
        <div style={style} onclick={&props.onclick}>
            {props.children.clone()}
        </div>
    }
}
