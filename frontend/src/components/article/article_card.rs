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
    let readable_date = format_date(&article.pub_date);
    html! {
        <div onclick={on_click}>
            <RouterAnchor<AppRoute> route=AppRoute::Article{id: article.id}>
                <div style="margin-bottom: 8px;">
                    <h1 class="heading">{&article.title}</h1>
                </div>
                <div style="margin-top: 8px; margin-bottom: 8px;">
                    {for article.tags.iter().map(|tag| { html! { <TagLabel tag={&tag.label} /> } })}
                </div>
                <div style="margin-top: 12px; margin-bottom: 8px;">
                    {match readable_date {
                        Ok(date) => html! {<p>{date}</p>},
                        Err(_) => html! {<p>{"An error occured!"}</p>}
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
    let readable_date = format_date(&article.pub_date);
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
                    {match readable_date {
                        Ok(date) => html! {<p>{date}</p>},
                        Err(_) => html! {<p>{"An error occured!"}</p>}
                    }}
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
