use {
    crate::{
        components::{modal::Modal, navlinks::NavLinks, search::Search, text::Text},
        entities::interfaces::SearchResults,
    },
    yew::{html, Callback, MouseEvent, Properties},
    yew_functional::{function_component, use_state},
};

#[derive(Properties, Clone, PartialEq)]
pub struct NavBarProps {
    pub dispatch_search_results: Callback<(String, SearchResults)>,
    pub dispatch_error: Callback<bool>,
}

#[function_component(Navbar)]
pub fn navbar(
    NavBarProps {
        dispatch_search_results,
        dispatch_error,
    }: &NavBarProps,
) -> Html {
    let (is_search_modal_open, sert_search_modal_open) = use_state(|| false);

    let close_search_modal: Callback<MouseEvent> = {
        let sert_search_modal_open = sert_search_modal_open.clone();
        Callback::from(move |_| sert_search_modal_open(false))
    };
    let open_search_modal: Callback<MouseEvent> =
        Callback::from(move |_| sert_search_modal_open(true));

    let on_click_result: Callback<MouseEvent> = close_search_modal.clone();

    html! {
            <div style="display: flex; flex: 1; justify-content: center; align-items: center; margin-top: 16px;">
                <div style="display: flex; max-width: 1024px; flex: 1;">
                    <NavLinks />
                    <div style="display: flex; flex: 0; justify-content: flex-end;">
                        <a onclick=open_search_modal><i class="fa fa-search" ></i></a>
                    </div>
                </div>
            {
                if *is_search_modal_open {
                    html! {
                        <Modal
                            title={html! {
                                    <div style="display: flex; align-items: center;">
                                        <Text value="Search" />
                                    </div>
                                }
                            }
                            body={
                                html!{
                                    <Search
                                        dispatch_search_results={dispatch_search_results}
                                        dispatch_error={dispatch_error}
                                        on_click_result={on_click_result}
                                    />
                                }
                            }
                            onclose={close_search_modal}
                        />
                    }
                } else {
                    html! {}
                }
            }
       </div>
    }
}
