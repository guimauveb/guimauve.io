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
pub fn tag_tag(TagLabelProps { tag }: &TagLabelProps) -> Html {
    html! {
        <RouterAnchor<AppRoute> route={AppRoute::Tag { tag: tag.clone() }}>
            <span class={"tag-link ".to_owned() + tag}>{&tag}</span>
        </RouterAnchor<AppRoute>>
    }
}
