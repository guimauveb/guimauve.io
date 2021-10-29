use {
    yew::{html, Callback, MouseEvent, Properties},
    yew_functional::function_component,
};

#[derive(Properties, Clone, PartialEq)]
pub struct ImageProps {
    pub src: String,
    #[prop_or("100%")]
    pub width: &'static str,
    #[prop_or("auto")]
    pub height: &'static str,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or("inherit")]
    pub object_fit: &'static str,
    #[prop_or_default]
    pub style: &'static str,
}

#[function_component(Image)]
pub fn image(
    ImageProps {
        src,
        width,
        height,
        onclick,
        object_fit,
        style,
    }: &ImageProps,
) -> Html {
    html! {
        <img onclick={onclick} src={&src} style={format!("width: {}; height: {}; object-fit: {}; {}", &width, &height, &object_fit, &style)} />
    }
}
