#[cfg(feature = "editable")]
use {
    crate::{components::tag_label::TagLabel, entities::interfaces::IProject},
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
                        <h3 style="font-weight: bold; font-size: 1rem;">{&project.title}</h3>
                    </div>
                    <div style="display: flex; flex: 1; margin-top: 4px;">
                        <div style="display: flex; flex: 1 1 20%; margin-right: 8px;">
                            <p style="white-space: pre-line;">{&project.description}</p>
                        </div>
                        <div style="display: flex; flex: 1 1 20%; flex-direction: column; margin-right: 8px;">
                            <div style="margin-bottom: 8px;">
                                <p>{"Features"}</p>
                            </div>
                            <p style="white-space: pre-line;">{&project.features}</p>
                        </div>
                        <div style="display: flex; flex: 1; flex-direction: column;">
                            <div style="margin-bottom: 8px;">
                                <p>{"Technologies"}</p>
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
