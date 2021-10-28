use {
    crate::{
        components::{
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
        <div style="display: flex; flex: 1; font-size: 0.8em; flex-direction: column;">
            <div style="display: flex; flex: 1">
                <div style="margin-right: 12px">
                    <Image width="8em" src={&project.image} />
                </div>
                <div style="display: flex; flex: 1; flex-direction: column;">
                    <div style="margin-bottom: 4px;">
                        <Text value={&project.title} as_element="h3" font_weight="bold" font_size="1.rem" />
                    </div>
                    <div style="display: flex; flex: 1; margin-top: 4px;">
                        <div style="display: flex; flex: 1 1 20%; margin-right: 8px;">
                            <Text white_space="pre-line" value={&project.description} />
                        </div>
                        <div style="display: flex; flex: 1 1 20%; flex-direction: column; margin-right: 8px;">
                            <div style="margin-bottom: 8px;">
                                <Text value="Features" />
                            </div>
                            <Text white_space="pre-line" value={&project.features} />
                        </div>
                        <div style="display: flex; flex: 1; flex-direction: column; word-break: break-all;">
                            <div style="margin-bottom: 8px;">
                                <Text value="Technologies" />
                            </div>
                            <div onclick={on_tag_clicked}>
                                {for project.tags.iter().map(|tag| html! {<TagLabel tag={&tag.label}/>})}
                            </div>
                        </div>
                    </div>
                    <div style="display: flex; flex: 1; margin-top: 12px;">
                        {match &project.gallery {
                            Some(gallery) => {
                                if gallery.is_empty() {
                                    html! {}
                                } else {
                                    html! {
                                        <>
                                            <div style="display: flex; flex: 1; align-items: center;">
                                                <div style="margin-right: 8px;">
                                                    <Text value="Gallery" />
                                                </div>
                                                <div style="display: flex; flex: 0;">
                                                        {
                                                            for gallery.iter().enumerate().map(|(index, image)| {
                                                               html! {
                                                                   <div
                                                                       style="position: relative; width: 100%; br: 6px; cursor: pointer;"
                                                                        onclick={
                                                                           let (set_selected_image, open_gallery_modal) = (set_selected_image.clone(), open_gallery_modal.clone());
                                                                           Callback::from(move |_| {
                                                                               set_selected_image(index);
                                                                               open_gallery_modal.emit(())
                                                                           })
                                                                       }
                                                                    >
                                                                        <div style="margin-left: 8px; position: relative; br: 2px; font-size: 0; width: 50px;">
                                                                            <Image src={image} />
                                                                            <div class="project-gallery-img-container-middle"></div>
                                                                        </div>
                                                                   </div>
                                                               }
                                                            })
                                                        }
                                                </div>
                                            </div>
                                            {
                                                if *is_gallery_modal_open {
                                                    html! {
                                                        <Modal
                                                            title={html! {
                                                                <div style="margin-bottom: 8px;">
                                                                    <Text variant={TextVariant::Heading} value={&project.title} />
                                                                </div>
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
                    </div>
                </div>
            </div>
            <div style="display: flex; flex: 1; justify-content: flex-end; margin-top: 12px;">
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
            </div>
            <div style="margin-top: 16px; margin-bottom: 16px; flex: 1">
                <Hr />
            </div>
        </div>
    }
}
