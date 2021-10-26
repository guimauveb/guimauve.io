use {
    super::checkbox::CheckBox,
    yew::{html, Callback, Properties},
    yew_functional::function_component,
};

#[derive(Properties, Clone, PartialEq)]
pub struct SwitchProps {
    pub on: bool,
    pub onchange: Callback<bool>,
}

#[function_component(Switch)]
pub fn switch(SwitchProps { on, onchange }: &SwitchProps) -> Html {
    html! {
        <label class="switch">
            <CheckBox checked={*on} onchange={onchange} />
            <span class="slider"/>
        </label>
    }
}
