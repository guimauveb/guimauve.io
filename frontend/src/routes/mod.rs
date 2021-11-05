use {yew_router::prelude::Switch, yew_router::switch::Permissive};

pub mod about;
pub mod article;
pub mod articles;
pub mod new_article;
pub mod projects;
pub mod tag;
pub mod tags;

#[derive(Switch, Clone, PartialEq)]
pub enum AppRoute {
    #[cfg(feature = "editable")]
    #[to = "/articles/new!"]
    NewArticle,
    #[to = "/articles/{id}"]
    Article { id: i32 },
    #[to = "/articles!"]
    Articles,
    #[to = "/tags/{tag}"]
    Tag { tag: String },
    #[to = "/tags!"]
    Tags,
    #[to = "/projects!"]
    Projects,
    #[to = "/about!"]
    About,
    #[cfg(feature = "editable")]
    #[to = "/live-resume"]
    LiveResume,
    #[to = "/!"]
    Home,
    #[to = "/404"]
    PageNotFound(Permissive<String>),
}
