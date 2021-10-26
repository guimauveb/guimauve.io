use {
    crate::entities::interfaces::{IArticle, IProject, ITag, ResultsIds, SearchResults},
    std::collections::HashMap,
};

#[derive(Clone, PartialEq, Default)]
pub struct BlogStore {
    pub articles: HashMap<i32, IArticle>,
    pub new_article: IArticle,
    pub projects: HashMap<i32, IProject>,
    pub tags: Vec<ITag>,
    pub selected_tag: ITag,
    pub tag_results: HashMap<String, ResultsIds>,
    pub current_search_query: String,
    pub search_results: HashMap<String, ResultsIds>,
    pub is_error: bool,
}

#[derive(Clone)]
pub enum Action {
    SetArticles(HashMap<i32, IArticle>),
    SetArticle(IArticle),
    SetNewArticle(IArticle),
    SetProjects(HashMap<i32, IProject>),
    SetTags(Vec<ITag>),
    SetTag(ITag),
    SetTagResults((String, SearchResults)),
    SetSearchResults((String, SearchResults)),
    SetError(bool),
}

#[derive(Clone)]
pub struct StoreDispatch(pub std::rc::Rc<dyn std::ops::Fn(Action) -> ()>);

impl StoreDispatch {
    pub fn emit(&self, action: Action) {
        (self.0)(action);
    }
}

fn update_results(
    search_query: &str,
    results: &SearchResults,
    articles_state: &mut HashMap<i32, IArticle>,
    projects_state: &mut HashMap<i32, IProject>,
    search_state: &mut HashMap<String, ResultsIds>,
) {
    // Update the entities states
    for (id, article) in results.articles.iter() {
        articles_state.insert(*id, article.clone());
    }
    for (id, project) in results.projects.iter() {
        projects_state.insert(*id, project.clone());
    }

    /* The searches / tags results states only contain the ids of the matching objects, so that we don't store them twice.
     * Meaning that when we need to access the results, we use their ids to fetch them from their own state. */
    let (articles_ids, projects_ids): (Vec<i32>, Vec<i32>) = (
        results.articles.iter().map(|(id, _)| *id).collect(),
        results.projects.iter().map(|(id, _)| *id).collect(),
    );

    search_state.insert(
        search_query.to_owned(),
        ResultsIds {
            articles_ids,
            projects_ids,
        },
    );
}

pub fn reducer(prev: std::rc::Rc<BlogStore>, action: Action) -> BlogStore {
    let BlogStore {
        mut articles,
        mut projects,
        mut tag_results,
        mut search_results,
        ..
    } = (&*prev).clone();

    match action {
        Action::SetArticles(articles) => BlogStore {
            articles,
            is_error: false,
            ..(*prev).clone()
        },
        Action::SetArticle(article) => {
            articles.insert(article.id, article);
            BlogStore {
                articles,
                is_error: false,
                ..(*prev).clone()
            }
        }
        Action::SetNewArticle(new_article) => BlogStore {
            new_article,
            ..(*prev).clone()
        },
        Action::SetProjects(projects) => BlogStore {
            projects,
            is_error: false,
            ..(*prev).clone()
        },
        Action::SetTags(tags) => BlogStore {
            tags,
            is_error: false,
            ..(*prev).clone()
        },
        Action::SetTag(selected_tag) => BlogStore {
            selected_tag,
            is_error: false,
            ..(*prev).clone()
        },
        Action::SetSearchResults((search_query, results)) => {
            update_results(
                &search_query,
                &results,
                &mut articles,
                &mut projects,
                &mut search_results,
            );
            BlogStore {
                articles,
                projects,
                search_results,
                current_search_query: search_query,
                is_error: false,
                ..(*prev).clone()
            }
        }
        Action::SetTagResults((tag, results)) => {
            update_results(
                &tag,
                &results,
                &mut articles,
                &mut projects,
                &mut tag_results,
            );
            BlogStore {
                articles,
                projects,
                tag_results,
                is_error: false,
                ..(*prev).clone()
            }
        }
        Action::SetError(is_error) => BlogStore {
            is_error,
            ..(*prev).clone()
        },
    }
}
