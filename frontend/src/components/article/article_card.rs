use {
    crate::{
        components::{
            box_component::BoxComponent,
            image::Image,
            tag_label::TagLabel,
            text::{Text, TextVariant},
        },
        entities::interfaces::IArticle,
        routes::AppRoute,
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
        <BoxComponent onclick={on_click}>
            <RouterAnchor<AppRoute> route=AppRoute::Article{id: article.id}>
                <BoxComponent mb="8px">
                    <Text as_element="h1" variant={TextVariant::Heading} value={&article.title} />
                </BoxComponent>
                <BoxComponent mt="8px" mb="8px">
                    {for article.tags.iter().map(move |tag| { html! { <TagLabel tag={&tag.label} /> } })}
                </BoxComponent>
                <BoxComponent mt="12px" mb="8px">
                    {match readable_date {
                        Ok(date) => html! {<Text value={&date}/>},
                        Err(_) => html! {<Text value="An error occured!"/>},
                    }}
                </BoxComponent>
                <BoxComponent mt="8px" mb="8px">
                    <Image src={&article.image} object_fit="cover" height="16em" />
                </BoxComponent>
                <BoxComponent mt="8px" mb="8px">
                    <Text value={&article.preview} />
                </BoxComponent>
            </RouterAnchor<AppRoute>>
        </BoxComponent>
    }
}

#[cfg(feature = "editable")]
#[function_component(ArticleCard)]
pub fn article_card(ArticleCardProps { article, on_click }: &ArticleCardProps) -> Html {
    let readable_date = format_date(&article.pub_date);
    html! {
        <BoxComponent onclick={on_click}>
            <RouterAnchor<AppRoute> route=AppRoute::Article{id: article.id}>
                <BoxComponent mb="8px">
                    <Text as_element="h1" variant={TextVariant::Heading} value={&article.title} />
                </BoxComponent>
                <BoxComponent mt="8px" mb="8px">
                    {for article.tags.iter().map(move |tag| { html! { <TagLabel tag={&tag.label} /> } })}
                </BoxComponent>
                <BoxComponent mt="12px" mb="8px" display="flex" justify_content="space-between">
                    {match readable_date {
                        Ok(date) => html! {<Text value={&date}/>},
                        Err(_) => html! {<Text value="An error occured!"/>},
                    }}
                    <BoxComponent font_size="1.2rem">
                        {if article.published {
                            html! {
                                <i class="fa fa-eye"/>
                            }
                        } else {
                            html! {
                                <i class="fa fa-eye-slash"/>
                            }
                        }}
                    </BoxComponent>
                </BoxComponent>
                <BoxComponent mt="8px" mb="8px">
                    <Image src={&article.image} object_fit="cover" height="16em" />
                </BoxComponent>
                <BoxComponent mt="8px" mb="8px">
                    <Text value={&article.preview} />
                </BoxComponent>
            </RouterAnchor<AppRoute>>
        </BoxComponent>
    }
}
