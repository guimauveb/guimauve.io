use {
    crate::components::box_component::BoxComponent,
    yew::{html, Callback, MouseEvent, Properties},
    yew_functional::function_component,
};

#[derive(Clone, PartialEq)]
pub enum ButtonVariant {
    Normal,
    Plain,
    Warning,
    Danger,
}

#[derive(Properties, Clone, PartialEq)]
pub struct ButtonProps {
    #[prop_or(ButtonVariant::Normal)]
    pub variant: ButtonVariant,
    #[prop_or_default]
    pub icon_name: String,
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub disabled: bool,
}

#[function_component(Button)]
pub fn button(
    ButtonProps {
        variant,
        icon_name,
        label,
        onclick,
        disabled,
    }: &ButtonProps,
) -> Html {
    let variant = match &variant {
        ButtonVariant::Normal => "",
        ButtonVariant::Plain => "background: rgb(221, 221, 221); color: #0d0d0d",
        ButtonVariant::Warning => "background: yellow",
        ButtonVariant::Danger => "background: #b30000",
    };

    html! {
        <button style={variant} onclick=onclick class="button" disabled=*disabled>
            <BoxComponent display="flex" flex="1" justify_content="center" align_items="center">
                {if !icon_name.is_empty() {
                    html! {
                        <BoxComponent mr="4px">
                            <i class={icon_name} aria-hidden="true"></i>
                        </BoxComponent>
                    }
                } else {
                    html! {}
                }}
                {label}
            </BoxComponent>
        </button>
    }
}
