use {
    yew::{html, Properties},
    yew_functional::function_component,
};

#[derive(Properties, Clone, PartialEq)]
pub struct PageNotFoundProps {
    #[prop_or(String::from("Page not found."))]
    pub missed_route: String,
}

#[function_component(PageNotFound)]
pub fn page_not_found(PageNotFoundProps { missed_route }: &PageNotFoundProps) -> Html {
    // "Page '' not found.".len() + missed_route.len()
    let mut error_message = String::with_capacity(18 + missed_route.len());
    error_message.push_str("Page '");
    error_message.push_str(missed_route);
    error_message.push_str("' not found.");

    html! {
        <div style="display: flex; flex: 1; justify-content: center;">
            <p>{error_message}</p>
        </div>
    }
}
