use {
    crate::{
        components::{loader::Loader, results::Results, text_input::TextInput},
        entities::{
            interfaces::{IArticle, IProject, SearchResults, Status},
            project_category::ProjectCategory,
        },
        hooks::use_effect_except_on_mount::use_effect_except_on_mount,
        service::{future::handle_future, search::get_results_for_query},
        store::store::BlogStore,
    },
    std::{collections::HashMap, rc::Rc},
    yew::{html, Callback, ChangeData, MouseEvent, Properties},
    yew_functional::{function_component, use_context, use_state},
};

#[derive(Properties, Clone, PartialEq)]
pub struct SearchProps {
    pub dispatch_search_results: Callback<(String, SearchResults)>,
    pub dispatch_error: Callback<bool>,
    pub on_click_result: Callback<MouseEvent>,
}

#[function_component(Search)]
pub fn search(
    SearchProps {
        dispatch_search_results,
        dispatch_error,
        on_click_result,
    }: &SearchProps,
) -> Html {
    let context = use_context::<Rc<BlogStore>>().expect("No context found!");

    let (is_loading, set_loading) = use_state(|| false);

    let search_query = context.search_query.clone();
    let (query, set_query) = use_state(move || search_query);

    // See if I could use references to the articles/projects states and pass them down the <Results/> component.
    let (articles, projects) = match &context.search_results.get(&*query) {
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

    let on_query_change: Callback<ChangeData> = Callback::from(move |event: ChangeData| {
        if let ChangeData::Value(query) = event {
            set_query(query);
        }
    });

    {
        let (dispatch_search_results, dispatch_error) =
            (dispatch_search_results.clone(), dispatch_error.clone());
        use_effect_except_on_mount(
            move |query| {
                set_loading(true);
                let (query, query_arg) = (query.clone(), query.clone());
                let future = async move { get_results_for_query(&query_arg).await };
                handle_future(future, move |data: Result<SearchResults, Status>| {
                    match data {
                        Ok(results) => dispatch_search_results.emit(((*query).clone(), results)),
                        Err(_) => dispatch_error.emit(true),
                    };
                    set_loading(false);
                });
            },
            query.clone(),
        );
    };

    html! {
        <div style="margin-top: 12px; display: flex; flex-direction: column;">
            <TextInput value={&*query} onchange={on_query_change} />
            <div style="margin-top:12px; display: flex; flex-direction: column; max-height: 32rem; overflow-y: auto;">
                {if query.is_empty() {
                    html! {}
                } else {
                    html! {
                        <Results
                            articles={articles}
                            articles_count={articles_count}
                            projects_by_category={projects_by_category}
                            projects_count={projects_count}
                            is_loading={*is_loading}
                            on_click_result={on_click_result}
                        />
                    }
                }}
                {if *is_loading {
                html! {
                    <div style="align-items:center; justify-content: center; display: flex; margin-top: 4px; margin-bottom: 4px;">
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
