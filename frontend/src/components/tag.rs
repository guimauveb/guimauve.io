use {
    crate::{
        components::{loader::Loader, results::Results},
        entities::{
            interfaces::{IArticle, IProject, SearchResults, Status},
            project_category::ProjectCategory,
        },
        service::{future::handle_future, tags::get_results_for_tag},
        store::store::BlogStore,
    },
    std::{collections::HashMap, rc::Rc},
    web_sys::{window, ScrollToOptions},
    yew::{html, Callback, Properties},
    yew_functional::{function_component, use_context, use_effect_with_deps, use_state},
};

#[derive(Properties, Clone, PartialEq)]
pub struct TagProps {
    pub tag: String,
    pub dispatch_tag_results: Callback<(String, SearchResults)>,
    pub dispatch_error: Callback<bool>,
}

#[function_component(Tag)]
pub fn tag(
    TagProps {
        tag,
        dispatch_tag_results,
        dispatch_error,
    }: &TagProps,
) -> Html {
    let context = use_context::<Rc<BlogStore>>().expect("No context found!");

    let (is_loading, set_loading) = use_state(|| false);
    let dispatch_tag_results = dispatch_tag_results.clone();

    // See if I could use references to the articles/projects states and pass them down the <Results/> component.
    let (articles, projects) = match &context.tag_results.get(tag) {
        Some(results) => (
            results
                .articles_ids
                .iter()
                .map(|id| {
                    (
                        *id,
                        context
                            .articles
                            .get(id)
                            .expect("Article not found!")
                            .clone(),
                    )
                })
                .collect::<HashMap<i32, IArticle>>(),
            results
                .projects_ids
                .iter()
                .map(|id| (*id, context.projects.get(id).expect("Project not found!")))
                .collect::<HashMap<i32, &IProject>>(),
        ),
        None => (HashMap::new(), HashMap::new()),
    };

    let (articles_count, projects_count) = (articles.len(), projects.len());

    let projects_by_category = projects.iter().fold(
        HashMap::new(),
        |mut acc: HashMap<ProjectCategory, Vec<IProject>>, (_, project)| {
            acc.insert(
                project.category.clone(),
                match acc.get(&project.category) {
                    Some(results) => {
                        let mut results = results.clone();
                        results.insert(0, (*project).clone());
                        results
                    }
                    None => vec![(*project).clone()],
                },
            );
            acc
        },
    );

    {
        let dispatch_error = dispatch_error.clone();
        use_effect_with_deps(
            move |tag| {
                window()
                    .unwrap()
                    .scroll_to_with_scroll_to_options(ScrollToOptions::new().top(0 as f64));
                set_loading(true);
                let (tag, tag_arg) = (tag.clone(), tag.clone());
                let future = async move { get_results_for_tag(&tag_arg).await };
                handle_future(future, move |data: Result<SearchResults, Status>| {
                    match data {
                        Ok(results) => dispatch_tag_results.emit((tag.clone(), results)),
                        Err(_) => dispatch_error.emit(true),
                    };
                    set_loading(false);
                });
                || {}
            },
            tag.clone(),
        );
    };

    let tag_header_base = "/tags/";
    let mut tag_header = String::with_capacity(tag_header_base.len() + tag.len());
    tag_header.push_str(tag_header_base);
    tag_header.push_str(tag);

    html! {
        <div style="display: flex; justify-content: center; flex: 1;">
            <div style="flex: 1; max-width: 1024px;">
                <div>
                    <div style="align-items: center; display: flex; margin-bottom: 24px;">
                        <h1 class="heading">{tag_header}</h1>
                    </div>
                    <Results
                        articles={articles}
                        articles_count={articles_count}
                        projects_by_category={projects_by_category}
                        projects_count={projects_count}
                        is_loading={*is_loading}
                    />
                </div>
                {if *is_loading {
                    html! {
                        <div style="align-items: center; justify-content: center; display: flex; margin-top: 24px; margin-bottom: 24px;">
                            <Loader />
                        </div>
                    }
                } else {
                    html! {}
                }}
            </div>
        </div>
    }
}
