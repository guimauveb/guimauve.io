use {
    crate::{
        components::{
            box_component::BoxComponent, modal::Modal, navlinks::NavLinks, search::Search,
            text::Text,
        },
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
            <BoxComponent display="flex" flex="1" justify_content="center" align_items="center" mt="16px">
                <BoxComponent display="flex" max_width="1024px" flex="1">
                    <NavLinks />
                    <BoxComponent display="flex" flex="0" justify_content="flex-end">
                        <a onclick=open_search_modal><i class="fa fa-search" ></i></a>
                    </BoxComponent>
                </BoxComponent>
            {
                if *is_search_modal_open {
                    html! {
                        <Modal
                            title={html! {
                                    <BoxComponent display="flex" align_items="center">
                                        <Text value="Search" />
                                    </BoxComponent>
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
       </BoxComponent>
    }
}
