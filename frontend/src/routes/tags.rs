use {
    crate::{components::tags::Tags as TagsComponent, entities::interfaces::ITag},
    yew::{html, Callback, Properties},
    yew_functional::function_component,
};

#[derive(Properties, Clone, PartialEq)]
pub struct TagsProps {
    pub dispatch_tags: Callback<Vec<ITag>>,
    pub dispatch_error: Callback<bool>,
}

#[function_component(Tags)]
pub fn tags(
    TagsProps {
        dispatch_tags,
        dispatch_error,
    }: &TagsProps,
) -> Html {
    html! {
        <TagsComponent dispatch_tags={dispatch_tags} dispatch_error={dispatch_error} />
    }
}
