use {
    crate::routes::AppRoute,
    yew::{html, Properties},
    yew_functional::function_component,
    yew_router::components::RouterAnchor,
};

#[derive(Properties, Clone, PartialEq)]
pub struct TagLabelProps {
    pub tag: String,
}

#[function_component(TagLabel)]
pub fn tag_label(TagLabelProps { tag }: &TagLabelProps) -> Html {
    let base_class = "tag-link ";
    let mut class_name = String::with_capacity(base_class.len() + tag.len());
    class_name.push_str(base_class);
    class_name.push_str(tag);

    html! {
        <RouterAnchor<AppRoute> route={AppRoute::Tag { tag: tag.clone() }}>
            <span class={&class_name}>{&tag}</span>
        </RouterAnchor<AppRoute>>
    }
}
