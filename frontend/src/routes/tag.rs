use {
    crate::{components::tag::Tag as TagComponent, entities::interfaces::SearchResults},
    yew::{html, Callback, Properties},
    yew_functional::function_component,
};

#[derive(Properties, Clone, PartialEq)]
pub struct TagProps {
    pub tag: String,
    pub dispatch_tag_results: Callback<(String, SearchResults)>,
    pub dispatch_error: Callback<bool>,
}

#[function_component(Tag)]
pub fn tag(
    TagProps {
        tag,
        dispatch_tag_results,
        dispatch_error,
    }: &TagProps,
) -> Html {
    html! {
        <TagComponent tag={tag} dispatch_tag_results={dispatch_tag_results} dispatch_error={dispatch_error} />
    }
}
