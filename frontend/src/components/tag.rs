use {
    crate::{
        components::{
            box_component::BoxComponent,
            loader::Loader,
            results::Results,
            text::{Text, TextVariant},
        },
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
                .map(|id| {
                    (
                        *id,
                        context
                            .projects
                            .get(id)
                            .expect("Project not found!")
                            .clone(),
                    )
                })
                .collect::<HashMap<i32, IProject>>(),
        ),
        None => (HashMap::new(), HashMap::new()),
    };

    let articles_count = articles.len();
    let projects_count = projects.len();

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

    html! {
        <BoxComponent display="flex" justify_content="center" flex="1">
            <BoxComponent flex="1" max_width="1024px">
                <BoxComponent>
                    <BoxComponent align_items="center" display="flex" mb="24px">
                        <Text variant={TextVariant::Heading} value={"/tags/".to_owned() + tag}/>
                    </BoxComponent>
                    <Results
                        articles={articles}
                        articles_count={articles_count}
                        projects_by_category={projects_by_category}
                        projects_count={projects_count}
                        is_loading={*is_loading}
                    />
                </BoxComponent>
                {if *is_loading {
                    html! {
                        <BoxComponent align_items="center" justify_content="center" display="flex" mt="24px" mb="24px">
                            <Loader />
                        </BoxComponent>
                    }
                } else {
                    html! {}
                }}
            </BoxComponent>
        </BoxComponent>
    }
}
