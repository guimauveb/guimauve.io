use {
    crate::{
        components::{
            article::article::Article as ArticleComponent, box_component::BoxComponent,
            loader::Loader,
        },
        entities::interfaces::{IArticle, Status},
        service::{articles::get_article, future::handle_future},
        store::store::BlogStore,
    },
    std::rc::Rc,
    web_sys::{window, ScrollToOptions},
    yew::{html, Callback, Properties},
    yew_functional::{function_component, use_context, use_effect_with_deps, use_state},
};

#[derive(Properties, Clone, PartialEq)]
pub struct ArticleProps {
    #[prop_or_default]
    pub id: i32,
    pub dispatch_article: Callback<IArticle>,
    pub dispatch_error: Callback<bool>,
}

#[function_component(Article)]
pub fn article(
    ArticleProps {
        id,
        dispatch_article,
        dispatch_error,
    }: &ArticleProps,
) -> Html {
    let (is_loading, set_loading) = use_state(|| false);

    let context = use_context::<Rc<BlogStore>>().expect("No context found!");
    let article_result = context.articles.get(id);

    {
        let (dispatch_article, dispatch_error, id) =
            (dispatch_article.clone(), dispatch_error.clone(), *id);
        use_effect_with_deps(
            move |_| {
                window()
                    .unwrap()
                    .scroll_to_with_scroll_to_options(ScrollToOptions::new().top(0 as f64));
                set_loading(true);
                let future = async move { get_article(&id).await };
                handle_future(future, move |data: Result<IArticle, Status>| {
                    match data {
                        Ok(article) => dispatch_article.emit(article),
                        Err(_) => dispatch_error.emit(true),
                    };
                    set_loading(false); // !!
                });
                || {}
            },
            (),
        );
    }

    html! {
        <BoxComponent display="flex" flex="1" flex_direction="column">
            {match article_result {
                    Some(article) => html! {
                        <ArticleComponent
                            article={article}
                            dispatch_article={dispatch_article}
                            dispatch_error={dispatch_error}
                        />
                    },
                    None => html! {}
                }
            }
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
