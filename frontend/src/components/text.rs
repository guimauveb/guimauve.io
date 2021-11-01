// TODO - Improve
use {
    yew::{html, Callback, MouseEvent, Properties},
    yew_functional::function_component,
};

#[derive(Clone, PartialEq)]
pub enum TextVariant {
    Normal,
    Heading,
    ArticleChapter,
    Comment,
    Caption,
}

#[derive(Properties, Clone, PartialEq)]
pub struct TextProps {
    #[prop_or(TextVariant::Normal)]
    pub variant: TextVariant,
    #[prop_or("p")]
    pub as_element: &'static str,
    #[prop_or_default]
    pub value: String,
    #[prop_or("normal")]
    pub font_weight: &'static str,
    #[prop_or("inherit")]
    pub font_size: &'static str,
    #[prop_or("pre-wrap")]
    pub white_space: &'static str,
    #[prop_or("auto")]
    pub user_select: &'static str,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub onmouseover: Callback<MouseEvent>,
    #[prop_or_default]
    pub onmouseout: Callback<MouseEvent>,
    #[prop_or("inherit")]
    pub color: &'static str,
}

#[function_component(Text)]
pub fn text(
    TextProps {
        variant,
        as_element,
        value,
        white_space,
        user_select,
        onclick,
        onmouseover,
        onmouseout,
        color,
        font_size,
        font_weight,
        ..
    }: &TextProps,
) -> Html {
    let mut style = String::with_capacity(
        13 + white_space.len()
            + 9
            + color.len()
            + 15
            + user_select.len()
            + 13
            + font_size.len()
            + 15
            + font_weight.len(),
    );
    style.push_str("white-space: ");
    style.push_str(&white_space);
    style.push_str("; color: ");
    style.push_str(&color);
    style.push_str("; user-select: ");
    style.push_str(&user_select);
    style.push_str("; font-size: ");
    style.push_str(&font_size);
    style.push_str("; font-weight: ");
    style.push_str(&font_weight);

    let variant = match &variant {
        TextVariant::Normal => "",
        TextVariant::Heading => "heading",
        TextVariant::ArticleChapter => "article-chapter",
        TextVariant::Comment => "comment",
        TextVariant::Caption => "caption",
    };

    match as_element as &str {
        "h1" => {
            html! { <h1 onmouseover={onmouseover} onmouseout={onmouseout} onclick={onclick} class={variant} style={&style}>{&value}</h1> }
        }
        "h2" => {
            html! { <h2 onmouseover={onmouseover} onmouseout={onmouseout} onclick={onclick} class={variant} style={&style}>{&value}</h2> }
        }
        "h3" => {
            html! { <h3 onmouseover={onmouseover} onmouseout={onmouseout} onclick={onclick} class={variant} style={&style}>{&value}</h3> }
        }
        "h4" => {
            html! { <h4 onmouseover={onmouseover} onmouseout={onmouseout} onclick={onclick} class={variant} style={&style}>{&value}</h4> }
        }
        _ => {
            html! {
                <p class={variant} style={style} onmouseover={onmouseover} onmouseout={onmouseout} onclick={onclick}>{value}</p>
            }
        }
    }
}
