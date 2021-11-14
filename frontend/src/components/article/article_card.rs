use {
    crate::{
        components::tag_label::TagLabel, entities::interfaces::IArticle, routes::AppRoute,
        utils::date::format_date,
    },
    yew::{html, Callback, MouseEvent, Properties},
    yew_functional::function_component,
    yew_router::components::RouterAnchor,
};

#[derive(Properties, Clone, PartialEq)]
pub struct ArticleCardProps {
    pub article: IArticle,
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,
}

#[cfg(not(feature = "editable"))]
#[function_component(ArticleCard)]
pub fn article_card(ArticleCardProps { article, on_click }: &ArticleCardProps) -> Html {
    html! {
        <div onclick={on_click}>
            <RouterAnchor<AppRoute> route=AppRoute::Article{id: article.id}>
                <div style="margin-bottom: 8px;">
                    <h1 class="heading">{&article.title}</h1>
                </div>
                <div style="margin-top: 8px; margin-bottom: 8px;">
                    {for article.tags.iter().map(|tag| { html! { <TagLabel tag={&tag.label} /> } })}
                </div>
                <div style="display: flex; margin-top: 12px; margin-bottom: 8px;">
                    <p>{format_date(&article.pub_date).unwrap_or_else(|_|"An error occured.".to_string())}</p>
                    {match &article.updated {
                        Some(update_date) => {
                            html! {
                                <div style="display: flex; margin-left: 16px; font-style: italic;">
                                    <p>{"Updated:"}</p>
                                    <p style="margin-left: 8px;">
                                        {format_date(update_date).unwrap_or_else(|_|"An error occured.".to_string())}
                                    </p>
                                </div>
                            }
                        },
                        _ => html! {}
                    }}
                </div>
                <div style="margin-top: 8px; margin-bottom: 8px;">
                    <img src={&article.image} style="object-fit: cover; height: 16em; width: 100%;" />
                </div>
                <div style="margin-top: 8px; margin-bottom: 8px;">
                    <p>{&article.headline}</p>
                </div>
            </RouterAnchor<AppRoute>>
        </div>
    }
}

#[cfg(feature = "editable")]
#[function_component(ArticleCard)]
pub fn article_card(ArticleCardProps { article, on_click }: &ArticleCardProps) -> Html {
    html! {
        <div onclick={on_click}>
            <RouterAnchor<AppRoute> route=AppRoute::Article{id: article.id}>
                <div style="margin-bottom: 8px;">
                    <h1 class="heading">{&article.title}</h1>
                </div>
                <div style="margin-top: 8px; margin-bottom: 8px;">
                    {for article.tags.iter().map(|tag| { html! { <TagLabel tag={&tag.label} /> } })}
                </div>
                <div style="margin-top: 12px; margin-bottom: 8px; display: flex; justify-content: space-between;">
                    <div style="display: flex;">
                        <p>{format_date(&article.pub_date).unwrap_or_else(|_|"An error occured.".to_string())}</p>
                        {match &article.updated {
                            Some(update_date) => {
                                html! {
                                    <div style="display: flex; margin-left: 16px; font-style: italic;">
                                        <p>{"Updated:"}</p>
                                        <p style="margin-left: 8px;">
                                            {format_date(update_date).unwrap_or_else(|_|"An error occured.".to_string())}
                                        </p>
                                    </div>
                                }
                            },
                            _ => html! {}
                        }}
                    </div>
                    <div sytle="font-size: 1.2rem;">
                        {if article.published {
                            html! {
                                <i class="fa fa-eye"/>
                            }
                        } else {
                            html! {
                                <i class="fa fa-eye-slash"/>
                            }
                        }}
                    </div>
                </div>
                <div style="margin-top: 8px; margin-bottom: 8px;">
                    <img src={&article.image} style="object-fit: cover; height: 16em; width: 100%;" />
                </div>
                <div style="margin-top: 8px; margin-bottom: 8px;">
                    <p>{&article.headline}</p>
                </div>
            </RouterAnchor<AppRoute>>
        </div>
    }
}
