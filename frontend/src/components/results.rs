use {
    crate::{
        components::{article::article_card::ArticleCard, hr::Hr, project::Project, text::Text},
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
    let articles_count_str = &articles_count.to_string();
    let mut articles_count_header = String::with_capacity(articles_count_str.len() + 16);
    articles_count_header.push_str(&articles_count_str);
    articles_count_header.push_str(" articles found.");

    let projects_count_str = &projects_count.to_string();
    let mut projects_count_header = String::with_capacity(projects_count_str.len() + 16);
    projects_count_header.push_str(&projects_count_str);
    projects_count_header.push_str(" projects found.");

    html! {
        <div>
            <div style="margin-bottom: 16px;">
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
                                    <Text value={articles_count_header} />
                                }
                            }}
                        </>
                    }
                }}
            </div>
            {for articles.iter().map(move |(_, article)| {
                html! {
                    <div style="margin-bottom: 12px;">
                        <div style="align-items: center;" onclick={on_click_result}>
                            <ArticleCard article={article} />
                        </div>
                        <div style="margin-top: 12px;">
                            <Hr />
                        </div>
                    </div>
                    }
                })
            }
            <div style="margin-top: 16px;">
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
                                    <Text value={projects_count_header} />
                                }
                            }}
                        </>
                    }
                }}
            </div>
            {for projects_by_category.iter().map(move |(category, projects): (&ProjectCategory, &Vec<IProject>)| {
                if projects.is_empty() {
                    html! {}
                } else {
                    html! {
                        <>
                            <div style="margin-top: 16px; margin-bottom: 16px;">
                                <Text as_element="h3" value={category.to_string()} />
                            </div>
                            {for projects.iter().map(move |project| html! {<Project project={project} on_tag_clicked={on_click_result} />})}
                        </>
                    }
                }}
            )}
        </div>
    }
}
