use {
    crate::{
        components::{
            hr::Hr,
            loader::Loader,
            project::Project,
            text::{Text, TextVariant},
        },
        entities::{
            interfaces::{IProject, Status},
            project_category::ProjectCategory,
        },
        service::{future::handle_future, projects::get_all_projects},
        store::store::BlogStore,
    },
    std::{collections::HashMap, rc::Rc},
    web_sys::window,
    yew::{html, Callback, Properties},
    yew_functional::{function_component, use_context, use_effect_with_deps, use_state},
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
    let dispatch = dispatch_projects.clone();
    let dispatch_error = dispatch_error.clone();
    let (is_loading, set_loading) = use_state(|| false);

    let context = use_context::<Rc<BlogStore>>().expect("No context found!");
    let projects = &context.projects;

    let projects_by_category = projects.iter().fold(
        HashMap::new(),
        |mut acc: HashMap<ProjectCategory, Vec<IProject>>, (_, project)| {
            acc.insert(
                project.category.clone(),
                match acc.get(&project.category) {
                    Some(results) => {
                        let mut results = results.clone();
                        results.insert(0, project.clone());
                        results
                    }
                    None => vec![project.clone()],
                },
            );
            acc
        },
    );

    use_effect_with_deps(
        move |_| {
            window().unwrap().scroll_to();
            set_loading(true);
            let future = async { get_all_projects().await };
            handle_future(
                future,
                move |data: Result<HashMap<i32, IProject>, Status>| {
                    match data {
                        Ok(projects) => dispatch.emit(projects),
                        Err(_) => dispatch_error.emit(true),
                    };
                    set_loading(false);
                },
            );
            || {}
        },
        (),
    );

    html! {
        <div style="display: flex; justify-content: center; flex: 1;">
            <div style="flex: 1; max-width: 1024px;">
                <div>
                    <Text variant={TextVariant::Heading} value="/projects" />
                    <div style="margin-bottom: 24px; margin-top: 24px;">
                        <Text value="I believe building projects is the best way to learn a new technology. Here are some of mine." />
                    </div>
                    <div style="margin-bottom: 16px;">
                        <Hr />
                    </div>
                    {for projects_by_category.iter().map(move |(category, projects): (&ProjectCategory, &Vec<IProject>)| {
                        html! {
                            <>
                                <div style="margin-top: 4px; margin-bottom: 16px;">
                                    <Text as_element="h2" value={category.to_string()} font_weight="bold" />
                                </div>
                                {for projects.iter().map(move |project| { html! { <Project project={project} /> } })}
                            </>
                        }
                    })}
                    {if *is_loading {
                    html! {
                        <div style="align-items: center; justify-content: center; display: flex; margin-top: 24px; margin-bottom: 24px;">
                            <Loader />
                        </div>
                    }
                    } else {
                        html! {}
                    }}
                    <div>
                        <div style="display: flex; justify-content: center; margin-top: 12px; margin-bottom: 12px;">
                            <a target="_blank" href="https://www.github.com/guimauveb/">
                                <div style="display: flex; font_size: 1.4em; margin-top: 24px; margin-bottom: 48px; align-items: center;">
                                    <Text value="See more projects on " /><i class="fa fa-github"/>
                                </div>
                            </a>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
