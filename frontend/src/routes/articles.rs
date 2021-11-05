use {
    crate::{
        components::{article::articles::Articles as ArticlesList, loader::Loader},
        entities::interfaces::{IArticle, Status},
        service::{articles::get_article_list, future::handle_future},
        store::store::BlogStore,
    },
    std::{collections::HashMap, rc::Rc},
    yew::{html, Callback, Properties},
    yew_functional::{function_component, use_context, use_effect_with_deps, use_state},
};

#[derive(Properties, Clone, PartialEq)]
pub struct ArticlesProps {
    pub dispatch_articles: Callback<HashMap<i32, IArticle>>,
    pub dispatch_error: Callback<bool>,
}

#[function_component(Articles)]
pub fn articles(
    ArticlesProps {
        dispatch_articles,
        dispatch_error,
    }: &ArticlesProps,
) -> Html {
    let (dispatch_articles, dispatch_error) = (dispatch_articles.clone(), dispatch_error.clone());
    let (is_loading, set_loading) = use_state(|| false);

    let context = use_context::<Rc<BlogStore>>().expect("No context found!");
    let articles = &context.articles;

    use_effect_with_deps(
        move |_| {
            set_loading(true);
            let future = async { get_article_list().await };
            handle_future(
                future,
                move |data: Result<HashMap<i32, IArticle>, Status>| {
                    match data {
                        Ok(articles) => dispatch_articles.emit(articles),
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
        <div style="display: flex; flex-direction: column; flex: 1;">
            <ArticlesList articles={articles} />
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
    }
}
