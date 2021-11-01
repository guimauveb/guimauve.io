use {
    crate::components::button::Button,
    yew::{html, Callback, Properties},
    yew_functional::function_component,
};

#[derive(Properties, Clone, PartialEq)]
pub struct SlideshowProps {
    pub selected_image: String,
    pub slideshow_length: usize,
    pub selected_image_index: usize,
    pub select_image: Callback<usize>,
}

#[function_component(Slideshow)]
pub fn slideshow(
    SlideshowProps {
        slideshow_length,
        selected_image,
        selected_image_index,
        select_image,
    }: &SlideshowProps,
) -> Html {
    let justify_content = if *slideshow_length > 1 {
        "space-between"
    } else {
        "center"
    };

    html! {
        <div style={format!("display: flex; justify-content: {}; align-items: center; max-height: 32rem;", justify_content)}>
            {if *slideshow_length > 1 {
                html! {
                    <div>
                        <Button
                            icon_name="fa fa-chevron-left"
                            onclick={
                                let (slideshow_length, selected_image_index, select_image) =
                                    (*slideshow_length, *selected_image_index, select_image.clone());
                                Callback::from(move |_| {
                                    if selected_image_index == 0 {
                                        select_image.emit(slideshow_length - 1)
                                    } else {
                                        select_image.emit(selected_image_index - 1)
                                    }
                                })
                            }
                        />
                    </div>
                }
            } else {
                html! {}
            }}
            <img src={selected_image} style="width: 75%; object-fit: contain; align-self: stretch;"/>
            {if *slideshow_length > 1 {
                html! {
                    <div>
                        <Button
                            icon_name="fa fa-chevron-right"
                            onclick={
                                 let (slideshow_length, selected_image_index, select_image) =
                                     (*slideshow_length, *selected_image_index, select_image.clone());
                                 Callback::from(move |_| {
                                     if selected_image_index == slideshow_length - 1 {
                                         select_image.emit(0)
                                     } else {
                                         select_image.emit(selected_image_index + 1)
                                     }
                                 })
                            }
                        />
                    </div>
                }
            } else {
                html! {}
            }}
        </div>
    }
}
