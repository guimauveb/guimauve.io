use {
    yew::{html, Callback, Html, MouseEvent, Properties},
    yew_functional::function_component,
};

fn highlight_code_in_html(highlighted_code: &str) -> Html {
    let div = yew::utils::document().create_element("div").unwrap();
    div.set_inner_html(highlighted_code);

    Html::VRef(div.into())
}

#[derive(Properties, Clone, PartialEq)]
pub struct CodeProps {
    #[prop_or_default]
    pub highlighted_code: String,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

#[function_component(Code)]
pub fn code(
    CodeProps {
        highlighted_code,
        onclick,
    }: &CodeProps,
) -> Html {
    html! {
        <div style="flex: 1; overflow-x: hidden;" onclick=onclick>
            {highlight_code_in_html(highlighted_code)}
        </div>
    }
}
