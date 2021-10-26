use {
    yew::{html, Callback, ChangeData, Properties},
    yew_functional::function_component,
};

#[derive(Properties, Clone, PartialEq)]
pub struct TextAreaProps {
    pub value: String,
    pub onchange: Callback<ChangeData>,
    #[prop_or_default]
    pub rows: i32,
}

#[function_component(TextArea)]
pub fn text_area(
    TextAreaProps {
        value,
        onchange,
        rows,
    }: &TextAreaProps,
) -> Html {
    html! {
        <textarea
            style="color: rgb(217, 225, 242); background: inherit; width: 100%; border-radius: 4px; padding: 8px; border: 1px solid; font-size: 1rem; resize: vertical;"
            onchange=onchange
            rows={rows} class="text-area"
            value={value}
        />
    }
}
