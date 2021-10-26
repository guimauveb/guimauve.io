use {
    crate::components::{box_component::BoxComponent, text::Text},
    yew::html,
    yew_functional::function_component,
};

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
      <footer>
        <BoxComponent flex="0" display="flex" justify_content="center" mb="12px">
            <Text value="guimauve" color="rgba(221, 221, 221, .7)" user_select="none"/>
        </BoxComponent>
      </footer>
    }
}
