use {
    super::article_card::ArticleCard,
    crate::{
        components::{
            box_component::BoxComponent,
            hr::Hr,
            text::{Text, TextVariant},
        },
        entities::interfaces::IArticle,
    },
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
        <BoxComponent display="flex" justify_content="center" flex="0">
            <BoxComponent flex="1" max_width="1024px">
                <BoxComponent align_items="center" display="flex" mb="24px">
                    <Text variant={TextVariant::Heading} value="/articles" />
                </BoxComponent>
                <BoxComponent>
                    {for articles.iter().map(move |(_, article)| {
                         html! {
                            <BoxComponent mb="12px">
                                <BoxComponent align_items="center">
                                    <ArticleCard article={article} />
                                </BoxComponent>
                                <BoxComponent mt="12px">
                                    <Hr />
                                </BoxComponent>
                            </BoxComponent>
                            }
                        }
                    )
                }
                </BoxComponent>
            </BoxComponent>
        </BoxComponent>
    }
}

#[cfg(feature = "editable")]
#[function_component(Articles)]
pub fn articles(ArticlesProps { articles }: &ArticlesProps) -> Html {
    html! {
        <BoxComponent display="flex" justify_content="center" flex="0">
            <BoxComponent flex="1" max_width="1024px">
                <BoxComponent align_items="center" flex="1" display="flex" mb="24px">
                    <Text variant={TextVariant::Heading} value="/articles" />
                    <RouterAnchor<AppRoute> route=AppRoute::NewArticle>
                        <BoxComponent align_items="center" position="relative" display="flex">
                            <BoxComponent width="42px" height="42px" display="flex" justify_content="center" ml="8px" align_items="center" cursor="pointer">
                                <i style="font-size: 1rem" class="fa fa-plus"></i>
                            </BoxComponent>
                        </BoxComponent>
                    </RouterAnchor<AppRoute>>
                </BoxComponent>
                {for articles.iter().map(move |(_, article)| {
                    html! {
                        <BoxComponent mb="12px">
                            <BoxComponent align_items="center">
                                <ArticleCard article={article} />
                            </BoxComponent>
                            <BoxComponent mt="12px">
                                 <Hr />
                            </BoxComponent>
                        </BoxComponent>
                          }
                       }
                   )
                }
            </BoxComponent>
        </BoxComponent>
    }
}
