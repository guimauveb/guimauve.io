use {
    crate::{components::navbar::Navbar, entities::interfaces::SearchResults},
    yew::{html, Callback, Properties},
    yew_functional::function_component,
};

#[derive(Properties, Clone, PartialEq)]
pub struct HeaderProps {
    pub dispatch_search_results: Callback<(String, SearchResults)>,
    pub dispatch_error: Callback<bool>,
}

#[function_component(Header)]
pub fn header(
    HeaderProps {
        dispatch_search_results,
        dispatch_error,
    }: &HeaderProps,
) -> Html {
    html! {
        <div style="display: flex; margin-bottom: 36px;">
            <Navbar dispatch_search_results={dispatch_search_results} dispatch_error={dispatch_error} />
        </div>
    }
}
