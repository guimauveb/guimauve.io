use {
    super::article_card::ArticleCard,
    crate::entities::interfaces::IArticle,
    std::collections::HashMap,
    yew::{html, Properties},
    yew_functional::function_component,
};

#[cfg(feature = "editable")]
use {crate::routes::AppRoute, yew_router::components::RouterAnchor};

#[derive(Properties, Clone, PartialEq)]
pub struct ArticlesProps {
    pub articles: HashMap<i32, IArticle>,
}

#[cfg(not(feature = "editable"))]
#[function_component(Articles)]
pub fn articles(ArticlesProps { articles }: &ArticlesProps) -> Html {
    html! {
        <div style="display: flex; justify-content: center; flex: 0;">
            <div style="flex: 1; max-width: 1024px;">
                <div style="align-items: center; display: flex; margin-bottom: 24px;">
                    <h1 class="heading">{"/articles"}</h1>
                </div>
                <div>
                    {for articles.iter().map(move |(_, article)| {
                         html! {
                            <div style="margin-bottom: 12px;">
                                <div style="align-items: center;">
                                    <ArticleCard article={article} />
                                </div>
                                <div style="margin-top: 12px;">
                                    <hr style="border: 0; border-top: 1px solid rgb(41, 41, 41);"/>
                                </div>
                            </div>
                            }
                        }
                    )
                }
                </div>
            </div>
        </div>
    }
}

#[cfg(feature = "editable")]
#[function_component(Articles)]
pub fn articles(ArticlesProps { articles }: &ArticlesProps) -> Html {
    html! {
        <div style="display: flex; justify-content: center; flex: 0;">
            <div style="flex: 1; max-width: 1024px;">
                <div style="align-items: center; display: flex; margin-bottom: 24px;">
                    <h1 class="heading">{"/articles"}</h1>
                    <RouterAnchor<AppRoute> route=AppRoute::NewArticle>
                        <div style="align-items: center; position: relative; display: flex;">
                            <div style="width: 42px; height: 42px; display: flex; justify-content: center; margin-left: 8px; align-items: center; cursor: pointer;">
                                <i style="font-size: 1rem" class="fa fa-plus"></i>
                            </div>
                        </div>
                    </RouterAnchor<AppRoute>>
                </div>
                {for articles.iter().map(move |(_, article)| {
                    html! {
                        <div style="margin-bottom: 12px;">
                            <div style="align-items: center;">
                                <ArticleCard article={article} />
                            </div>
                            <div style="margin-top: 12px;">
                                <hr style="border: 0; border-top: 1px solid rgb(41, 41, 41);"/>
                            </div>
                        </div>
                          }
                       }
                   )
                }
            </div>
        </div>
    }
}
