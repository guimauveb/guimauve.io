use {
    super::{article_header::ArticleHeader, chapters::Chapters},
    crate::entities::{
        action::Action,
        interfaces::{IArticle, IArticleHeader},
    },
    std::rc::Rc,
    yew::{html, Callback, Properties},
    yew_functional::function_component,
};

#[cfg(feature = "editable")]
use {
    crate::{
        components::{button::Button, loader::Loader, switch::Switch},
        entities::interfaces::{IPublishArticle, Status},
        hooks::use_effect_except_on_mount::use_effect_except_on_mount,
        routes::AppRoute,
        service::{
            articles::{add_article, delete_article, publish_article},
            future::handle_future,
        },
        API_URL,
    },
    yew::MouseEvent,
    yew_functional::use_state,
    yew_router::agent::{RouteAgentDispatcher, RouteRequest},
};

#[derive(Properties, Clone, PartialEq)]
pub struct ArticleProps {
    #[prop_or(Action::Edit)]
    pub article_action: Action,
    #[prop_or_default]
    pub article: IArticle,
    #[prop_or_default]
    pub dispatch_article: Callback<IArticle>,
    #[prop_or_default]
    pub dispatch_error: Callback<bool>,
}

#[cfg(not(feature = "editable"))]
#[function_component(Article)]
pub fn article(ArticleProps { article, .. }: &ArticleProps) -> Html {
    html! {
        <div style="display: flex; flex: 1;">
            <div style="flex: 1" />
            <div style="max-width: 1024px; flex: 1 0 100%;">
                <div style="flex: 1; flex-direction: column;">
                    <ArticleHeader
                        article_header={
                            Rc::new( // TODO - Use IArticleHeader inside IArticle
                                IArticleHeader {
                                    id: article.id,
                                    title: article.title.clone(),
                                    pub_date: article.pub_date.clone(),
                                    published: article.published,
                                    headline: article.headline.clone(),
                                    image: article.image.clone(),
                                    image_credits: article.image_credits.clone(),
                                    tags: article.tags.clone(),
                                    updated: article.updated.clone(),
                                }
                            )
                        }
                    />
                    <div>
                        <Chapters
                            chapters={Rc::new(article.chapters.clone())}
                            article_id={article.id} />
                    </div>
                </div>
            </div>
            <div style="flex: 1;">
                // TODO - Chapters 'pop up' component!
            </div>
        </div>
    }
}

#[cfg(feature = "editable")]
#[function_component(Article)]
pub fn article(
    ArticleProps {
        article_action,
        article,
        dispatch_article,
        dispatch_error,
    }: &ArticleProps,
) -> Html {
    let (is_loading, set_loading) = use_state(|| false);

    let on_cancel: Callback<MouseEvent> = {
        let article_action = article_action.clone();
        Callback::from(move |_| {
            match article_action {
                // Redirect to /articles when cancelling new article creation.
                Action::Add => {
                    RouteAgentDispatcher::<()>::new()
                        .send(RouteRequest::ChangeRoute(AppRoute::Articles.into()));
                }
                Action::Edit => (),
            };
        })
    };
    let on_submit_article: Callback<MouseEvent> = {
        let (article, article_action, set_loading, dispatch_article, dispatch_error) = (
            article.clone(),
            article_action.clone(),
            set_loading.clone(),
            dispatch_article.clone(),
            dispatch_error.clone(),
        );
        Callback::from(move |_| {
            let (article, set_loading, dispatch_article, dispatch_error) = (
                IArticle {
                    image: (&article.image[API_URL.len()..]).to_owned(),
                    ..article.clone()
                },
                set_loading.clone(),
                dispatch_article.clone(),
                dispatch_error.clone(),
            );
            if article_action == Action::Add {
                let future = async move { add_article(&article).await };
                handle_future(future, move |response: Result<IArticle, Status>| {
                    match response {
                        Ok(article) => {
                            dispatch_article.emit(IArticle::default());
                            RouteAgentDispatcher::<()>::new().send(RouteRequest::ChangeRoute(
                                (AppRoute::Article { id: article.id }).into(),
                            ));
                        }
                        Err(_) => dispatch_error.emit(true),
                    };
                    set_loading(false);
                });
            };
        })
    };
    let on_delete_article: Callback<MouseEvent> = {
        let (article_id, set_loading, dispatch_error) =
            (article.id, set_loading.clone(), dispatch_error.clone());
        Callback::from(move |_| {
            set_loading(true);
            let (set_loading, dispatch_error) = (set_loading.clone(), dispatch_error.clone());
            let future = async move { delete_article(&article_id).await };
            handle_future(future, move |response: Result<Status, Status>| {
                set_loading(false);
                if response.is_ok() {
                    RouteAgentDispatcher::<()>::new()
                        .send(RouteRequest::ChangeRoute(AppRoute::Articles.into()));
                } else {
                    dispatch_error.emit(true);
                }
            });
        })
    };

    let (published, set_published) = {
        let published = article.published;
        use_state(move || published)
    };
    let on_publish_article: Callback<bool> = Callback::from(move |on| set_published(on));

    {
        let (article_id, dispatch_article, dispatch_error) =
            (article.id, dispatch_article.clone(), dispatch_error.clone());
        use_effect_except_on_mount(
            move |published| {
                set_loading(true);
                let payload = IPublishArticle {
                    published: *published,
                };
                let future = async move { publish_article(&article_id, &payload).await };
                handle_future(future, move |data: Result<IArticle, Status>| {
                    match data {
                        Ok(article) => dispatch_article.emit(article),
                        Err(_) => dispatch_error.emit(true),
                    };
                    set_loading(false);
                });
            },
            *published,
        );
    };

    html! {
        <div style="display: flex; flex: 1;">
            <div style="flex: 1;" />
            <div style="max-width: 1024px; flex: 1 0 100%;">
                <div style="flex: 1; flex-direction: column;">
                    <div style="align-items: center; position: relative; display: flex; margin-top: 8px; margin-bottom: 24px;">
                        {match article_action {
                            Action::Edit => html! {
                                <div
                                    onclick={on_delete_article}
                                    style="width: 42px; height: 42px; display: flex; justify-content: center; align-items:center; position: absolute; right: -48px; cursor: pointer;">
                                        <i class="fa fa-trash"></i>
                                    <div style="margin-left: 8px;">
                                        <p>{"Delete"}</p>
                                    </div>
                                </div>
                            },
                            Action::Add => html! {
                                <div style="display: flex; justify-content: center; align-items: center; font-size: .8em; position: absolute; right: -64px;">
                                    <Button onclick={on_cancel} label="Cancel"/>
                                    <Button onclick={on_submit_article} label="Save"/>
                                </div>
                            }
                        }}
                        <div style="display: flex; align-items: center;">
                            <div style="margin-right: 12px;">
                                <p>{"Publish"}</p>
                            </div>
                            <Switch
                                on={*published}
                                onchange={on_publish_article}
                            />
                        </div>
                    </div>
                    <ArticleHeader
                        article_action={article_action}
                        article_header={
                            Rc::new(
                                IArticleHeader {
                                    id: article.id,
                                    title: article.title.clone(),
                                    pub_date: article.pub_date.clone(),
                                    published: article.published,
                                    headline: article.headline.clone(),
                                    image: article.image.clone(),
                                    image_credits: article.image_credits.clone(),
                                    tags: article.tags.clone(),
                                    updated: article.updated.clone(),
                                }
                            )
                        }
                        dispatch_article={dispatch_article}
                        dispatch_error={dispatch_error}
                    />
                    <div>
                        <Chapters
                            chapters={Rc::new(article.chapters.clone())}
                            article_action={article_action}
                            article_id={article.id}
                            dispatch_article={dispatch_article}
                            dispatch_error={dispatch_error}
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
            <div style="flex: 1;">
                // TODO - Chapters 'pop up' component!
            </div>
        </div>
    }
}
