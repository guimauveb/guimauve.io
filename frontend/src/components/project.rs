use {
    crate::{
        components::{
            box_component::BoxComponent,
            button::*,
            hr::Hr,
            image::Image,
            modal::Modal,
            slideshow::Slideshow,
            tag_label::TagLabel,
            text::{Text, TextVariant},
        },
        entities::interfaces::IProject,
    },
    yew::{html, Callback, MouseEvent, Properties},
    yew_functional::{function_component, use_state},
};

#[derive(Properties, Clone, PartialEq)]
pub struct ProjectProps {
    pub project: IProject,
    #[prop_or_default]
    pub on_tag_clicked: Callback<MouseEvent>,
}

#[function_component(Project)]
pub fn project(
    ProjectProps {
        project,
        on_tag_clicked,
    }: &ProjectProps,
) -> Html {
    let (selected_image_index, set_selected_image) = use_state(|| 0_usize);

    let (is_gallery_modal_open, set_gallery_modal) = use_state(|| false);
    let open_gallery_modal: Callback<()> = {
        let set_gallery_modal = set_gallery_modal.clone();
        Callback::from(move |_| set_gallery_modal(true))
    };
    let close_gallery_modal: Callback<MouseEvent> = {
        let set_gallery_modal = set_gallery_modal;
        Callback::from(move |_| set_gallery_modal(false))
    };

    html! {
        <BoxComponent display="flex" flex="1" font_size="0.8em" flex_direction="column">
            <BoxComponent display="flex" flex="1">
                <BoxComponent mr="12px">
                    <Image width="8em" src={&project.image} />
                </BoxComponent>
                <BoxComponent display="flex" flex="1" flex_direction="column">
                    <BoxComponent mb="4px">
                        <Text value={&project.title} as_element="h3" font_weight="bold" font_size="1.rem" />
                    </BoxComponent>
                    <BoxComponent display="flex" flex="1" mt="4px">
                        <BoxComponent display="flex" flex="1 1 20%" mr="8px">
                            <Text white_space="pre-line" value={&project.description} />
                        </BoxComponent>
                        <BoxComponent display="flex" flex="1 1 20%" flex_direction="column" mr="8px">
                            <BoxComponent mb="8px">
                                <Text value="Features" />
                            </BoxComponent>
                            <Text white_space="pre-line" value={&project.features} />
                        </BoxComponent>
                        <BoxComponent display="flex" flex="1" flex_direction="column" word_break="break-all">
                            <BoxComponent mb="8px">
                                <Text value="Technologies" />
                            </BoxComponent>
                            <BoxComponent onclick={on_tag_clicked}>
                                {for project.tags.iter().map(|tag| html! {<TagLabel tag={&tag.label}/>})}
                            </BoxComponent>
                        </BoxComponent>
                    </BoxComponent>
                    <BoxComponent display="flex" flex="1" mt="12px">
                        {match &project.gallery {
                            Some(gallery) => {
                                if gallery.is_empty() {
                                    html! {}
                                } else {
                                    html! {
                                        <>
                                            <BoxComponent display="flex" flex="1" align_items="center">
                                                <BoxComponent mr="8px">
                                                    <Text value="Gallery" />
                                                </BoxComponent>
                                                <BoxComponent display="flex" flex="0">
                                                        {
                                                            for gallery.iter().enumerate().map(|(index, image)| {
                                                               html! {
                                                                   <BoxComponent
                                                                        position="relative"
                                                                        width="100%"
                                                                        br="6px"
                                                                        cursor="pointer"
                                                                        onclick={
                                                                           let (set_selected_image, open_gallery_modal) = (set_selected_image.clone(), open_gallery_modal.clone());
                                                                           Callback::from(move |_| {
                                                                               set_selected_image(index);
                                                                               open_gallery_modal.emit(())
                                                                           })
                                                                       }
                                                                    >
                                                                        <BoxComponent ml="8px" position="relative" br="2px" font_size="0" width="50px">
                                                                            <Image src={image} />
                                                                            <div class="project-gallery-img-container-middle"></div>
                                                                        </BoxComponent>
                                                                   </BoxComponent>
                                                               }
                                                            })
                                                        }
                                                </BoxComponent>
                                            </BoxComponent>
                                            {
                                                if *is_gallery_modal_open {
                                                    html! {
                                                        <Modal
                                                            title={html! {
                                                                <BoxComponent mb="8px">
                                                                    <Text variant={TextVariant::Heading} value={&project.title} />
                                                                </BoxComponent>
                                                            }}
                                                            body={html! {
                                                                <Slideshow selected_image_index={*selected_image_index}
                                                                           selected_image={gallery.get(*selected_image_index).expect("Invalid index!")}
                                                                           slideshow_length={gallery.len()}
                                                                           /* NOTE - Identified as a redudant closure by cargo clippy. But cloning the function
                                                                            * and passing it to the callback doesn't work */
                                                                           select_image={Callback::from(move |index: usize| set_selected_image(index) )}
                                                                />}}
                                                            onclose={close_gallery_modal}
                                                        />
                                                    }
                                                } else {
                                                    html! {}
                                                }
                                            }
                                        </>
                                    }}
                                },
                                None => html! {},
                        }}
                    </BoxComponent>
                </BoxComponent>
            </BoxComponent>
            <BoxComponent display="flex" flex="1" justify_content="flex-end" mt="12px">
                {match &project.git {
                    Some(git) => html!{
                        <a href={git} target="_blank">
                            <Button variant={ButtonVariant::Plain} icon_name="fa fa-code" label="Code" />
                        </a>
                    },
                    None => html! {
                            <Button variant={ButtonVariant::Plain} icon_name="fa fa-code" label="Private source" disabled={true} />
                        },
                }}
                {match &project.visit_link {
                    Some(link) => html! {
                        <a target="_blank" href={link}>
                            <Button variant={ButtonVariant::Plain} icon_name="fa fa-eye" label="Visit" />
                        </a>
                },
                    None => html! {},
                }}
                {match &project.live_link {
                    Some(link) => html! {
                        <a target="_blank" href={link}>
                            <Button variant={ButtonVariant::Plain} icon_name="fa fa-eye" label="Live version" />
                        </a>
                    },
                    None => html! {},
                }}
                {match &project.download_link {
                    Some(link) => html! {
                        <a target="_blank" href={link}>
                            <Button variant={ButtonVariant::Plain} icon_name="fa fa-download" label="Download" />
                        </a>
                    },
                    None => html! {},
                }}
            </BoxComponent>
            <BoxComponent mt="16px" mb="16px" flex="1">
                <Hr />
            </BoxComponent>
        </BoxComponent>
    }
}
