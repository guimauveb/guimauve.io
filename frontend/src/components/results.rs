use {
    crate::{
        components::{article::article_card::ArticleCard, project::Project},
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
    articles_count_header.push_str(articles_count_str);
    articles_count_header.push_str(" articles found.");

    let projects_count_str = &projects_count.to_string();
    let mut projects_count_header = String::with_capacity(projects_count_str.len() + 16);
    projects_count_header.push_str(projects_count_str);
    projects_count_header.push_str(" projects found.");

    html! {
        <div>
            <div style="margin-bottom: 16px;">
                {if *is_loading {
                    html! {
                        <p>{"- articles found."}</p>
                    }
                } else if *articles_count == 0 {
                         html! {
                             <p>{"No article found."}</p>
                         }
                } else {
                    html! {
                        <>
                            {if *articles_count == 1 {
                                html! {
                                    <p>{"1 article found."}</p>
                                }
                            } else {
                                html! {
                                    <p>{articles_count_header}</p>
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
                            <hr style="border: 0; border-top: 1px solid rgb(41, 41, 41);"/>
                        </div>
                    </div>
                    }
                })
            }
            <div style="margin-top: 16px;">
                {if *is_loading {
                    html! {
                        <p>{"- projects found."}</p>
                    }
                } else if *projects_count == 0 {
                    html! {
                        <p>{"No project found."}</p>
                    }
                } else {
                    html! {
                        <>
                            {if *projects_count == 1 {
                                html! {
                                    <p>{"1 project found."}</p>
                                }
                            } else {
                                html! {
                                    <p>{projects_count_header}</p>
                                }
                            }}
                        </>
                    }
                }}
            </div>
            {for projects_by_category.iter().map(|(category, projects): (&ProjectCategory, &Vec<IProject>)| {
                if projects.is_empty() {
                    html! {}
                } else {
                    html! {
                        <>
                            <div style="margin-top: 16px; margin-bottom: 16px;">
                                <h3>{category.to_string()}</h3>
                            </div>
                            {for projects.iter().map(|project| html! {<Project project={project} on_tag_clicked={on_click_result} />})}
                        </>
                    }
                }}
            )}
        </div>
    }
}
