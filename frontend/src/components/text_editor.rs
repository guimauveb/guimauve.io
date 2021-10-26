use {
    crate::components::text_area::TextArea,
    yew::{html, Callback, ChangeData, Properties},
    yew_functional::function_component,
};

// TODO - Create base components with default (TBoxProps ;))
#[derive(Properties, Clone, PartialEq)]
pub struct TextEditorProps {
    #[prop_or(8)]
    pub rows: i32,
    pub data: String,
    pub onchange: Callback<ChangeData>,
}

#[function_component(TextEditor)]
pub fn text_editor(
    TextEditorProps {
        rows,
        data,
        onchange,
    }: &TextEditorProps,
) -> Html {
    html! {
        <TextArea rows={rows} onchange={onchange} value={data} />
    }
}
