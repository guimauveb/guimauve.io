use {
    crate::components::{box_component::BoxComponent, text::Text},
    yew::{html, Properties},
    yew_functional::function_component,
};

#[derive(Properties, Clone, PartialEq)]
pub struct PageNotFoundProps {
    #[prop_or(String::from("Page not found."))]
    pub page_name: String,
}

#[function_component(PageNotFound)]
pub fn page_not_found(PageNotFoundProps { page_name }: &PageNotFoundProps) -> Html {
    html! {
        <BoxComponent display="flex" flex="1" justify_content="center">
            <Text value={page_name} />
        </BoxComponent>
    }
}
