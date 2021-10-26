use {yew::html, yew_functional::function_component};

#[function_component(Loader)]
pub fn loader() -> Html {
    html! {
      <div class="lds-dual-ring"></div>
    }
}
