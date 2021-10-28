use {crate::components::text::Text, yew::html, yew_functional::function_component};

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
      <footer>
        <div style="flex: 0; display: flex; justify-content: center; margin-bottom: 12px;">
            <Text value="guimauve" color="rgba(221, 221, 221, .7)" user_select="none"/>
        </div>
      </footer>
    }
}
