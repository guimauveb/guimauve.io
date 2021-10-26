use {
    yew::{html, Callback, ChangeData, Properties},
    yew_functional::function_component,
};

#[derive(Properties, Clone, PartialEq)]
pub struct TextInputProps {
    pub value: String,
    pub onchange: Callback<ChangeData>,
}

#[function_component(TextInput)]
pub fn text_input(TextInputProps { value, onchange }: &TextInputProps) -> Html {
    html! {
        <input
            style="font-family: inherit; color: rgb(217, 225, 242); background: inherit; width: 100%; border-radius: 4px; padding: 8px; border: 1px solid; font-size: 1rem; resize: vertical;"
            onchange={onchange}
            value={value}
        />
    }
}
