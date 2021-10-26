#[cfg(feature = "editable")]
use {
    crate::{
        components::{box_component::BoxComponent, tag_label::TagLabel, text::Text},
        entities::interfaces::IProject,
    },
    yew::{html, Properties},
    yew_functional::function_component,
};

#[cfg(feature = "editable")]
#[derive(Properties, Clone, PartialEq)]
pub struct ResumeProjectProps {
    pub project: IProject,
}

#[cfg(feature = "editable")]
#[function_component(ResumeProject)]
pub fn resume_project(ResumeProjectProps { project }: &ResumeProjectProps) -> Html {
    html! {
        <BoxComponent display="flex" flex="1" font_size="0.8em" flex_direction="column" mb="12px">
            <BoxComponent display="flex" flex="1">
                <BoxComponent display="flex" flex="1" flex_direction="column">
                    <BoxComponent mb="4px">
                        <Text font_weight="bold" value={&project.title} as_element="h3" font_size="1.rem"/>
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
                        <BoxComponent display="flex" flex="1" flex_direction="column">
                            <BoxComponent mb="8px">
                                <Text value="Technologies" />
                            </BoxComponent>
                            <BoxComponent>
                                {for project.tags.iter().map(|tag| html! {<TagLabel tag={&tag.label}/>})}
                            </BoxComponent>
                        </BoxComponent>
                    </BoxComponent>
                </BoxComponent>
            </BoxComponent>
        </BoxComponent>
    }
}
