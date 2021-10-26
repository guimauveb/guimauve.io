use {
    crate::{
        components::{
            box_component::BoxComponent,
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
        <BoxComponent display="flex" justify_content="center" flex="1">
            <BoxComponent flex="1" max_width="1024px">
                <BoxComponent>
                    <Text variant={TextVariant::Heading} value="/projects" />
                    <BoxComponent mb="24px" mt="24px">
                        <Text value="I believe building projects is the best way to learn a new technology. Here are some of mine." />
                    </BoxComponent>
                    <BoxComponent mb="16px">
                        <Hr />
                    </BoxComponent>
                    {for projects_by_category.iter().map(move |(category, projects): (&ProjectCategory, &Vec<IProject>)| {
                        html! {
                            <>
                                <BoxComponent mt="4px" mb="16px">
                                    <Text as_element="h2" value={category.to_string()} font_weight="bold" />
                                </BoxComponent>
                                {for projects.iter().map(move |project| { html! { <Project project={project} /> } })}
                            </>
                        }
                    })}
                    {if *is_loading {
                    html! {
                        <BoxComponent align_items="center" justify_content="center" display="flex" mt="24px" mb="24px">
                            <Loader />
                        </BoxComponent>
                    }
                    } else {
                        html! {}
                    }}
                    <BoxComponent>
                        <BoxComponent display="flex" justify_content="center" mt="12px" mb="12px">
                            <a target="_blank" href="https://www.github.com/guimauveb/">
                                <BoxComponent display="flex" font_size="1.4em" mt="24px" mb="48px" align_items="center">
                                    <Text value="See more projects on " /><i class="fa fa-github"/>
                                </BoxComponent>
                            </a>
                        </BoxComponent>
                    </BoxComponent>
                </BoxComponent>
            </BoxComponent>
        </BoxComponent>
    }
}
