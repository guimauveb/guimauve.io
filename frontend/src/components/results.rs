use {
    crate::{
        components::{
            article::article_card::ArticleCard, box_component::BoxComponent, hr::Hr,
            project::Project, text::Text,
        },
        entities::{
            interfaces::{IArticle, IProject},
            project_category::ProjectCategory,
        },
    },
    std::collections::HashMap,
    yew::{html, Callback, MouseEvent, Properties},
    yew_functional::function_component,
};

#[derive(Properties, Clone, PartialEq)]
pub struct ResultsProps {
    pub articles: HashMap<i32, IArticle>,
    pub articles_count: usize,
    pub projects_by_category: HashMap<ProjectCategory, Vec<IProject>>,
    pub projects_count: usize,
    pub is_loading: bool,
    #[prop_or_default]
    pub on_click_result: Callback<MouseEvent>,
}

#[function_component(Results)]
pub fn results(
    ResultsProps {
        articles,
        articles_count,
        projects_by_category,
        projects_count,
        is_loading,
        on_click_result,
    }: &ResultsProps,
) -> Html {
    html! {
        <BoxComponent>
            <BoxComponent mb="16px">
                {if *is_loading {
                    html! {
                        <Text value="- articles found." />
                    }
                } else if *articles_count == 0 {
                         html! {
                             <Text value="No article found." />
                         }
                } else {
                    html! {
                        <>
                            {if *articles_count == 1 {
                                html! {
                                    <Text value="1 article found." />
                                }
                            } else {
                                html! {
                                    <Text value={format!("{} articles found.", *articles_count)} />
                                }
                            }}
                        </>
                    }
                }}
            </BoxComponent>
            {for articles.iter().map(move |(_, article)| {
                html! {
                    <BoxComponent mb="12px">
                        <BoxComponent align_items="center" onclick={on_click_result}>
                            <ArticleCard article={article} />
                        </BoxComponent>
                        <BoxComponent mt="12px">
                            <Hr />
                        </BoxComponent>
                    </BoxComponent>
                    }
                })
            }
            <BoxComponent mt="16px">
                {if *is_loading {
                    html! {
                        <Text value="- projects found." />
                    }
                } else if *projects_count == 0 {
                    html! {
                        <Text value="No project found." />
                    }
                } else {
                    html! {
                        <>
                            {if *projects_count == 1 {
                                html! {
                                    <Text value="1 project found." />
                                }
                            } else {
                                html! {
                                    <Text value={format!("{} projects found.", *projects_count)} />
                                }
                            }}
                        </>
                    }
                }}
            </BoxComponent>
            {for projects_by_category.iter().map(move |(category, projects): (&ProjectCategory, &Vec<IProject>)| {
                if projects.is_empty() {
                    html! {}
                } else {
                    html! {
                        <>
                            <BoxComponent mt="12px" mb="16px">
                                <Text as_element="h3" value={category.to_string()} />
                            </BoxComponent>
                            {for projects.iter().map(move |project| html! {<Project project={project} on_tag_clicked={on_click_result} />})}
                        </>
                    }
                }}
            )}
        </BoxComponent>
    }
}
