use {
    crate::store::store::{reducer, Action, StoreModel},
    yew::{html, Callback, MouseEvent},
    yew_functional::function_component, use_effect_with_deps, use_reducer_with_init, use_state},
};

#[function_component(Index)]
pub fn index() -> Html {
    //let initial_state = StoreModel {};
    //let (store, dispatch) =
    //    use_reducer_with_init(reducer, initial_state, |initial_state: StoreModel| {
    //        initial_state
    //    });
    html! {
        "/"
    }
}
