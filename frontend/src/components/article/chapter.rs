use {
    super::contents::Contents,
    crate::{
        components::text::{Text, TextVariant},
        entities::{
            action::Action,
            interfaces::{IArticle, IChapter},
        },
    },
    std::rc::Rc,
    yew::{html, Callback, Properties},
    yew_functional::function_component,
};

#[cfg(feature = "editable")]
use {
    crate::{
        components::{
            button::{Button, ButtonVariant},
            loader::Loader,
            text_area::TextArea,
        },
        entities::interfaces::Status,
        service::{
            articles::{add_chapter, delete_chapter, get_article, update_chapter},
            future::handle_future,
        },
        store::store::BlogStore,
    },
    yew::{ChangeData, MouseEvent},
    yew_functional::{use_context, use_effect_with_deps, use_state},
};

#[derive(Properties, Clone, PartialEq)]
pub struct ChapterProps {
    pub chapter: Rc<IChapter>,
    #[prop_or(Action::Edit)]
    pub action: Action,
    #[prop_or(Action::Edit)]
    pub article_action: Action,
    #[prop_or_default]
    pub edited: bool,
    #[prop_or_default]
    pub on_edit: Callback<(i32, bool)>,
    #[prop_or_default]
    pub dispatch_article: Callback<IArticle>,
    #[prop_or_default]
    pub dispatch_error: Callback<bool>,
}

#[cfg(not(feature = "editable"))]
#[function_component(Chapter)]
pub fn chapter(ChapterProps { chapter, .. }: &ChapterProps) -> Html {
    html! {
        <>
            <div style="align-items: center; position: relative; display: flex; margin-top: 8px; margin-bottom: 8px;">
                <Text variant={TextVariant::ArticleChapter} as_element="h2" value={&chapter.title} />
            </div>
            <Contents contents={Rc::new(chapter.contents.clone())} />
        </>
    }
}

#[cfg(feature = "editable")]
#[function_component(Chapter)]
pub fn chapter(
    ChapterProps {
        chapter,
        action,
        article_action,
        on_edit,
        edited,
        dispatch_article,
        dispatch_error,
    }: &ChapterProps,
) -> Html {
    let contents = &chapter.contents;
    let (is_loading, set_loading) = use_state(|| false);

    // Chapter form (which only consists of a title)
    let (chapter_title, set_chapter_title) = {
        let chapter_title = chapter.title.clone();
        use_state(move || chapter_title)
    };

    {
        let set_chapter_title = set_chapter_title.clone();
        use_effect_with_deps(
            move |chapter| {
                set_chapter_title(chapter.title.clone());
                || {}
            },
            chapter.clone(),
        );
    };
    let on_change_chapter_title: Callback<ChangeData> = {
        let set_chapter_title = set_chapter_title.clone();
        Callback::from(move |event: ChangeData| match event {
            ChangeData::Value(updated_chapter_title) => set_chapter_title(updated_chapter_title),
            _ => (),
        })
    };

    let on_edit_chapter: Callback<MouseEvent> = {
        let (chapter_index, on_edit) = (chapter.index, on_edit.clone());
        Callback::from(move |_| on_edit.emit((chapter_index, true)))
    };
    let on_cancel_edit: Callback<MouseEvent> = {
        let (chapter_index, on_edit) = (chapter.index, on_edit.clone());
        Callback::from(move |_| on_edit.emit((chapter_index, false)))
    };

    let on_delete_chapter: Callback<MouseEvent> = {
        let (
            article_action,
            article_id,
            chapter_index,
            chapter_id,
            set_loading,
            dispatch_article,
            dispatch_error,
        ) = (
            article_action.clone(),
            chapter.article_id,
            chapter.index,
            chapter.id,
            set_loading.clone(),
            dispatch_article.clone(),
            dispatch_error.clone(),
        );
        let context = use_context::<Rc<BlogStore>>().expect("Could not find context!");

        let article = match article_action {
            Action::Add => context.new_article.clone(),
            Action::Edit => context
                .articles
                .get(&article_id)
                .expect("Could not find article!")
                .clone(),
        };

        Callback::from(move |_| match article_action {
            Action::Edit => {
                set_loading(true);
                let (set_loading, dispatch_article, dispatch_error) = (
                    set_loading.clone(),
                    dispatch_article.clone(),
                    dispatch_error.clone(),
                );
                let future = async move { delete_chapter(&chapter_id).await };
                handle_future(
                    future,
                    move |response: Result<Status, Status>| match response {
                        Ok(_) => {
                            let (set_loading, dispatch_article, dispatch_error) = (
                                set_loading.clone(),
                                dispatch_article.clone(),
                                dispatch_error.clone(),
                            );
                            let future = async move { get_article(&article_id).await };
                            handle_future(future, move |data: Result<IArticle, Status>| {
                                match data {
                                    Ok(article) => dispatch_article.emit(article),
                                    Err(_) => dispatch_error.emit(true),
                                };
                                set_loading(false);
                            })
                        }
                        Err(_) => dispatch_error.emit(true),
                    },
                );
            }
            Action::Add => {
                let mut article = article.clone();
                for chap in &mut article.chapters {
                    if chap.index > chapter_index {
                        chap.index = chap.index - 1;
                    }
                }
                let index = article
                    .chapters
                    .iter()
                    .position(|c| c.id == chapter_id)
                    .unwrap();
                article.chapters.remove(index);
                dispatch_article.emit(article);
            }
        })
    };

    let on_save_chapter: Callback<MouseEvent> = {
        let (
            article_id,
            article_action,
            chapter_index,
            chapter,
            chapter_action,
            set_loading,
            dispatch_article,
            dispatch_error,
            on_edit,
        ) = (
            chapter.article_id,
            article_action.clone(),
            chapter.index,
            Rc::new(IChapter {
                id: chapter.id,
                article_id: chapter.article_id,
                title: (*chapter_title).clone(),
                index: chapter.index,
                contents: vec![],
            }),
            action.clone(),
            set_loading.clone(),
            dispatch_article.clone(),
            dispatch_error.clone(),
            on_edit.clone(),
        );
        let context = use_context::<Rc<BlogStore>>().expect("Could not find context!");
        let article = match article_action {
            Action::Add => context.new_article.clone(),
            Action::Edit => context
                .articles
                .get(&article_id)
                .expect("Could not find article!")
                .clone(),
        };

        Callback::from(move |_| {
            let (chapter, set_loading, dispatch_article, dispatch_error, on_edit) = (
                chapter.clone(),
                set_loading.clone(),
                dispatch_article.clone(),
                dispatch_error.clone(),
                on_edit.clone(),
            );

            // Existing article
            match article_action {
                Action::Edit => {
                    set_loading(true);
                    match chapter_action {
                        Action::Add => {
                            let future = async move { add_chapter(&chapter).await };
                            handle_future(future, move |data: Result<IArticle, Status>| {
                                let set_loading = set_loading.clone();
                                match data {
                                    Ok(article) => dispatch_article.emit(article),
                                    Err(_) => dispatch_error.emit(true),
                                };
                                set_loading(false);
                            });
                        }
                        Action::Edit => {
                            let future = async move { update_chapter(&chapter).await };
                            handle_future(future, move |data: Result<IArticle, Status>| {
                                match data {
                                    Ok(article) => dispatch_article.emit(article),
                                    Err(_) => dispatch_error.emit(true),
                                };
                                set_loading(false);
                                on_edit.emit((chapter_index, false));
                            });
                        }
                    }
                }
                // Article being created
                Action::Add => {
                    set_loading(true);
                    let mut article = article.clone();
                    match chapter_action {
                        Action::Add => {
                            for chap in &mut article.chapters {
                                if chap.index >= chapter.index {
                                    chap.index = chap.index + 1;
                                }
                            }
                            article
                                .chapters
                                .insert(chapter.index as usize, (*chapter).clone());
                            dispatch_article.emit(article);
                            set_loading(false);
                        }
                        Action::Edit => {
                            let index = article
                                .chapters
                                .iter()
                                .position(|c| c.id == chapter.id)
                                .unwrap();
                            article.chapters[index].title = chapter.title.clone();
                            dispatch_article.emit(article);
                            set_loading(false);
                            on_edit.emit((index as i32, false));
                        }
                    }
                }
            }
        })
    };

    html! {
        <div>
            {match *edited {
                true => html! {
                    <div>
                        <TextArea rows={2} value={&*chapter_title} onchange={on_change_chapter_title} />
                        <div style="display: flex; margin-top: 4px; margin-bottom: 4px; justify-content: flex-end; font-size: .8em;">
                        {match action {
                            Action::Edit => html! {
                                <Button variant={ButtonVariant::Danger} onclick={&on_delete_chapter} label="Delete"/>
                            },
                            _ => html! {}
                        }}
                            <Button onclick={on_cancel_edit} label="Cancel"/>
                            <Button onclick={on_save_chapter} label="Save"/>
                        </div>
                    </div>
                },
                false => html! {
                    <div style="align-items: center; position: relative; display: flex; margin-top: 8px; margin-bottom: 8px;">
                        <div
                            onclick={on_edit_chapter}
                            style="width:42px; height:42px; display: flex; justify-content: center; align-items: center; position: absolute; right:-78px; cursor: pointer;">
                            <i class="fa fa-edit"></i>
                        </div>
                        <Text variant={TextVariant::ArticleChapter} as_element="h2" value={&chapter.title} />
                    </div>
                    }
            }}
            {if *is_loading {
                html! {
                    <div style="align-items: center; justify-content: center; position: relative; display: flex;">
                        <Loader />
                    </div>
                }
            } else {
                html! {}
            }}
            <Contents
                contents={Rc::new(contents.clone())}
                article_action={article_action}
                chapter_action={action}
                article_id={chapter.article_id}
                chapter_id={chapter.id}
                dispatch_article={dispatch_article}
                dispatch_error={dispatch_error}
            />
        </div>
    }
}
