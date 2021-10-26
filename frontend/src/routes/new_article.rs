use {
    crate::{
        components::article::article::Article, entities::action::Action,
        entities::interfaces::IArticle, store::store::BlogStore,
    },
    std::rc::Rc,
    yew::{html, Callback, Properties},
    yew_functional::{function_component, use_context},
};

#[derive(Properties, Clone, PartialEq)]
pub struct NewArticleProps {
    pub dispatch_new_article: Callback<IArticle>,
    pub dispatch_error: Callback<bool>,
}

#[function_component(NewArticle)]
pub fn new_article(
    NewArticleProps {
        dispatch_new_article,
        dispatch_error,
    }: &NewArticleProps,
) -> Html {
    let context = use_context::<Rc<BlogStore>>().expect("No context found!");
    let new_article = &context.new_article;

    html! {
        <Article
            article={new_article}
            article_action={Action::Add}
            dispatch_article={dispatch_new_article}
            dispatch_error={dispatch_error}
        />
    }
}
