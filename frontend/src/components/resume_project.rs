#[cfg(feature = "editable")]
use {
    crate::{
        components::{tag_label::TagLabel, text::Text},
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
        <div style="display: flex; flex: 1; font-size= 0.8em; flex-direction: column; margin-bottom: 12px;">
            <div style="display: flex; flex: 1;">
                <div style="display: flex; flex: 1; flex-direction: column;">
                    <div style="margin-bottom: 4px;">
                        <Text font_weight="bold" value={&project.title} as_element="h3" font_size="1.rem"/>
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
                        <div style="display: flex; flex: 1; flex-direction: column;">
                            <div style="margin-bottom: 8px;">
                                <Text value="Technologies" />
                            </div>
                            <div>
                                {for project.tags.iter().map(|tag| html! {<TagLabel tag={&tag.label}/>})}
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
