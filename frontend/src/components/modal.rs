use {
    crate::components::{box_component::BoxComponent, button::Button},
    yew::{html, Callback, Html, MouseEvent, Properties},
    yew_functional::function_component,
};

#[derive(Properties, Clone, PartialEq)]
pub struct ModalProps {
    #[prop_or(html! {})]
    pub title: Html,
    pub body: Html,
    #[prop_or_default]
    pub height: String,
    pub onclose: Callback<MouseEvent>,
}

#[function_component(Modal)]
pub fn modal(
    ModalProps {
        title,
        body,
        onclose,
        ..
    }: &ModalProps,
) -> Html {
    html! {
      <div class="modal-container" style="--m-background: hsla(0, 0%, 0%, .4);">
          <div class="modal">
              <BoxComponent display="flex" flex="0" justify_content="space-between" mb="8px" >
                  {title.clone()}
                  <BoxComponent height="32px">
                      <Button onclick={onclose} icon_name="fa fa-times" />
                  </BoxComponent>
                </BoxComponent>
                {body.clone()}
                <BoxComponent display="flex" justify_content="flex-end" mt="24px">
                    <Button onclick={onclose} label="Close"/>
                </BoxComponent>
            </div>
        </div>
    }
}
