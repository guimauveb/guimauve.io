use {
    crate::{components::projects::Projects as ProjectsComponent, entities::interfaces::IProject},
    std::collections::HashMap,
    yew::{html, Callback, Properties},
    yew_functional::function_component,
};

#[derive(Properties, Clone, PartialEq)]
pub struct ProjectsProps {
    pub dispatch_projects: Callback<HashMap<i32, IProject>>,
    pub dispatch_error: Callback<bool>,
}

#[function_component(Projects)]
pub fn projects(
    ProjectsProps {
        dispatch_projects,
        dispatch_error,
    }: &ProjectsProps,
) -> Html {
    html! {
        <ProjectsComponent dispatch_projects={dispatch_projects} dispatch_error={dispatch_error} />
    }
}
