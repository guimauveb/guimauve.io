use {
    crate::{
        components::{
            article::articles::Articles as ArticlesList, box_component::BoxComponent,
            loader::Loader,
        },
        entities::interfaces::{IArticle, Status},
        service::{articles::get_article_list, future::handle_future},
        store::store::BlogStore,
    },
    std::{collections::HashMap, rc::Rc},
    yew::{html, Callback, Properties},
    yew_functional::{function_component, use_context, use_effect_with_deps, use_state},
};

#[derive(Properties, Clone, PartialEq)]
pub struct HomeProps {
    pub dispatch_articles: Callback<HashMap<i32, IArticle>>,
    pub dispatch_error: Callback<bool>,
}

#[function_component(Home)]
pub fn home(
    HomeProps {
        dispatch_articles,
        dispatch_error,
    }: &HomeProps,
) -> Html {
    let dispatch_articles = dispatch_articles.clone();
    let dispatch_error = dispatch_error.clone();
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
        <BoxComponent display="flex" flex_direction="column" flex="1">
            <ArticlesList articles={articles} />
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
    }
}
