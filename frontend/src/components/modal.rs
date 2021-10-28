use {
    crate::components::button::Button,
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
              <div style="display: flex; flex: 0; justify-content: space-between; margin-bottom: 8px;">
                  {title.clone()}
                  <div style="height: 32px;">
                      <Button onclick={onclose} icon_name="fa fa-times" />
                  </div>
                </div>
                {body.clone()}
                <div style="display: flex; justify-content: flex-end; margin-top: 24px;">
                    <Button onclick={onclose} label="Close"/>
                </div>
            </div>
        </div>
    }
}
