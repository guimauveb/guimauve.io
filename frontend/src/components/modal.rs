use wasm_bindgen::{JsCast, JsValue};
use {
    crate::components::button::Button,
    web_sys::HtmlElement,
    yew::{html, Callback, Html, MouseEvent, Properties},
    yew_functional::function_component,
};

#[derive(Properties, Clone, PartialEq)]
pub struct ModalProps {
    #[prop_or_default]
    pub id: String,
    #[prop_or(html! {})]
    pub title: Html,
    #[prop_or(html! {})]
    pub body: Html,
    #[prop_or_default]
    pub height: String,
    #[prop_or_default]
    pub onclose: Callback<()>,
}

#[function_component(Modal)]
pub fn modal(
    ModalProps {
        title,
        body,
        id,
        onclose,
        ..
    }: &ModalProps,
) -> Html {
    let onclose = onclose.clone();
    let on_click_outside: Callback<MouseEvent> = {
        let (onclose, id) = (onclose.clone(), id.clone());
        Callback::from(move |event: MouseEvent| {
            if !id.is_empty() {
                let target = event.target().unwrap();
                let js_value = target.as_ref() as &JsValue;
                let element = js_value.clone().dyn_into::<HtmlElement>().unwrap().id();
                if element == id {
                    onclose.emit(());
                }
            }
        })
    };

    html! {
        <div id={id} onclick={Callback::from(move |event| on_click_outside.emit(event))} class="modal-container" style="--m-background: hsla(0, 0%, 0%, .4);">
            <div class="modal">
                <div style="display: flex; flex: 0; justify-content: space-between; margin-bottom: 8px;">
                    {title.clone()}
                    <div style="height: 32px;">
                        <Button
                            onclick={
                                let onclose = onclose.clone();
                                Callback::from(move |_| onclose.emit(()))
                            }
                            icon_name="fa fa-times"
                        />
                    </div>
                </div>
                {body.clone()}
                <div style="display: flex; justify-content: flex-end; margin-top: 24px;">
                    <Button onclick={Callback::from(move |_| onclose.emit(()))} label="Close"/>
                </div>
            </div>
        </div>
    }
}
