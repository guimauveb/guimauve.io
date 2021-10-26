use {
    crate::components::{box_component::BoxComponent, button::Button, image::Image},
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
    html! {
        <BoxComponent display="flex" justify_content="center" max_height="32rem">
            <Image src={selected_image} width="75%" object_fit="contain"/>
            {if *slideshow_length > 1 {
                html! {
                    <>
                        <BoxComponent position="absolute" top="50%" left="2em">
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
                        </BoxComponent>
                        <BoxComponent position="absolute" top="50%" right="2em">
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
                        </BoxComponent>
                    </>
                }
            } else {
                html! {}
            }}
        </BoxComponent>
    }
}
