use {
    crate::{
        components::{footer::Footer, header::Header, page_not_found::PageNotFound},
        entities::interfaces::{IArticle, IProject, ITag, SearchResults},
        routes::{
            about::About, article::Article, articles::Articles, home::Home, projects::Projects,
            tag::Tag, tags::Tags, AppRoute,
        },
        store::store::{reducer, Action, BlogStore},
    },
    std::{collections::HashMap, rc::Rc},
    yew::{html, Callback},
    yew_functional::{function_component, use_reducer_with_init, ContextProvider},
    yew_router::{prelude::Route, router::Router, switch::Permissive},
};

#[cfg(feature = "editable")]
use crate::{components::live_resume::LiveResume, routes::new_article::NewArticle};

#[function_component(App)]
pub fn app() -> Html {
    let initial_state = BlogStore::default();

    let (store, dispatch) =
        use_reducer_with_init(reducer, initial_state, |initial_state: BlogStore| {
            initial_state
        });

    // Dispatchers
    let dispatch_articles: Callback<HashMap<i32, IArticle>> = {
        let dispatch = dispatch.clone();
        Callback::from(move |values: HashMap<i32, IArticle>| {
            dispatch(Action::SetArticles(values));
        })
    };
    let dispatch_projects: Callback<HashMap<i32, IProject>> = {
        let dispatch = dispatch.clone();
        Callback::from(move |values: HashMap<i32, IProject>| {
            dispatch(Action::SetProjects(values));
        })
    };
    let dispatch_article: Callback<IArticle> = {
        let dispatch = dispatch.clone();
        Callback::from(move |value: IArticle| {
            dispatch(Action::SetArticle(value));
        })
    };
    #[cfg(feature = "editable")]
    let dispatch_new_article: Callback<IArticle> = {
        let dispatch = dispatch.clone();
        Callback::from(move |value: IArticle| {
            dispatch(Action::SetNewArticle(value));
        })
    };
    let dispatch_tag_results: Callback<(String, SearchResults)> = {
        let dispatch = dispatch.clone();
        Callback::from(move |values: (String, SearchResults)| {
            dispatch(Action::SetTagResults(values));
        })
    };
    let dispatch_search_results: Callback<(String, SearchResults)> = {
        let dispatch = dispatch.clone();
        Callback::from(move |values: (String, SearchResults)| {
            dispatch(Action::SetSearchResults(values));
        })
    };
    let dispatch_tags: Callback<Vec<ITag>> = {
        let dispatch = dispatch.clone();
        Callback::from(move |values: Vec<ITag>| {
            dispatch(Action::SetTags(values));
        })
    };
    let dispatch_error: Callback<bool> = {
        Callback::from(move |value: bool| {
            dispatch(Action::SetError(value));
        })
    };

    type BlogStoreContextProvider = ContextProvider<Rc<BlogStore>>;

    html! {
        <BlogStoreContextProvider context=store>
            <div style="display: flex; flex: 1; flex-direction: column;">
                <Header dispatch_search_results={dispatch_search_results} dispatch_error={&dispatch_error} />
                <div style="display: flex; flex: 1;">
                    <Router<AppRoute, ()>
                        render = Router::render(move |route: AppRoute| {
                            match route {
                                AppRoute::Home => html! {
                                    <Home
                                        dispatch_articles={dispatch_articles.clone()}
                                        dispatch_error={dispatch_error.clone()}
                                    />
                                },
                                #[cfg(feature = "editable")]
                                AppRoute::NewArticle => html! {
                                    <NewArticle
                                        dispatch_new_article={dispatch_new_article.clone()}
                                        dispatch_error={dispatch_error.clone()}
                                    />
                                },
                                AppRoute::Article { id } => html! {
                                    <Article
                                        id={id}
                                        dispatch_article={dispatch_article.clone()}
                                        dispatch_error={dispatch_error.clone()}
                                    />
                                },
                                AppRoute::Articles => html! {
                                    <Articles
                                        dispatch_articles={dispatch_articles.clone()}
                                        dispatch_error={dispatch_error.clone()}
                                    />
                                },
                                AppRoute::Tags => html! {
                                    <Tags
                                        dispatch_tags={dispatch_tags.clone()}
                                        dispatch_error={dispatch_error.clone()}
                                    />
                                },
                                AppRoute::Tag { tag } => html! {
                                    <Tag
                                        tag={tag}
                                        dispatch_tag_results={dispatch_tag_results.clone()}
                                        dispatch_error={dispatch_error.clone()}
                                    />
                                },
                                AppRoute::Projects => html! {
                                    <Projects
                                        dispatch_projects={dispatch_projects.clone()}
                                        dispatch_error={dispatch_error.clone()}
                                    />
                                },
                                AppRoute::About => html! {<About />},
                                #[cfg(feature = "editable")]
                                AppRoute::LiveResume => html! {<LiveResume />},
                                AppRoute::PageNotFound(Permissive(None)) => html! {<PageNotFound />},
                                AppRoute::PageNotFound(Permissive(Some(missed_route))) => {
                                    html! {
                                        <PageNotFound missed_route={&missed_route} />
                                    }
                                }
                            }
                        })
                        redirect = Router::redirect(|route: Route<()>| {
                            AppRoute::PageNotFound(Permissive(Some(route.route)))
                        })
                    />
                </div>
                <Footer />
            </div>
        </BlogStoreContextProvider>
    }
}
