use {
    crate::components::button::Button,
    yew::{html, Callback, Properties},
    yew_functional::function_component,
};

#[derive(Properties, Clone, PartialEq)]
pub struct SlideshowProps {
    pub selected_image_url: String,
    pub slideshow_length: usize,
    pub selected_image_index: usize,
    pub select_image: Callback<usize>,
}

#[function_component(Slideshow)]
pub fn slideshow(
    SlideshowProps {
        slideshow_length,
        selected_image_url,
        selected_image_index,
        select_image,
    }: &SlideshowProps,
) -> Html {
    let (slideshow_length, selected_image_index) = (*slideshow_length, *selected_image_index);

    let on_previous_image_clicked = {
        let select_image = select_image.clone();
        Callback::from(move |_| {
            if selected_image_index == 0 {
                select_image.emit(slideshow_length - 1);
            } else {
                select_image.emit(selected_image_index - 1);
            }
        })
    };
    let on_next_image_clicked = {
        let select_image = select_image.clone();
        Callback::from(move |_| {
            if selected_image_index == slideshow_length - 1 {
                select_image.emit(0);
            } else {
                select_image.emit(selected_image_index + 1);
            }
        })
    };

    let justify_content = if slideshow_length > 1 {
        "space-between"
    } else {
        "center"
    };

    let mut container_style = String::with_capacity(32 + 13 + 41);
    container_style.push_str("display: flex; justify-content: ");
    container_style.push_str(justify_content);
    container_style.push_str("; align-items: center; max-height: 28rem;");

    html! {
        <div style={container_style}>
            {if slideshow_length > 1 {
                html! { <Button icon_name="fa fa-chevron-left" onclick={on_previous_image_clicked} /> }
            } else { html! {} }}
            <img src={selected_image_url} style="width: 75%; object-fit: contain; align-self: stretch;"/>
            {if slideshow_length > 1 {
                html! { <Button icon_name="fa fa-chevron-right" onclick={on_next_image_clicked} /> }
            } else { html! {} }}
        </div>
    }
}
