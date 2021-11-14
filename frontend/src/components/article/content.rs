use {
    crate::{
        components::code::Code,
        entities::{
            action::Action,
            content_type::ContentType,
            interfaces::{IArticle, IContent},
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
            select::Select,
            text_area::TextArea,
        },
        entities::{
            content_type::CONTENT_TYPES,
            interfaces::Status,
            language::{Language, LANGUAGES},
        },
        service::{
            articles::{add_content, delete_content, get_article, update_content},
            future::handle_future,
        },
        store::store::BlogStore,
        API_URL,
    },
    yew::{ChangeData, MouseEvent},
    yew_functional::{use_context, use_effect_with_deps, use_state},
};

#[derive(Properties, Clone, PartialEq)]
pub struct ContentProps {
    pub content: Rc<IContent>,
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
#[function_component(Content)]
pub fn content(ContentProps { content, .. }: &ContentProps) -> Html {
    html! {
        <div style="align-items: center; position: relative; display: flex; margin-bottom: 24px;">
            {match &content.content_type {
                ContentType::Text => html! {
                    <p style="white-space: break-spaces;">{&content.content}</p>
                },
                ContentType::Comment => html! {
                    <p style="white-space: break-spaces; font-style: italic">{&content.content}</p>
                },
                ContentType::Link => html! {
                    <a target="_blank" href={match &content.url {
                        Some(url) => url,
                        None => ""
                    }}>
                        <p style="white-space: break-spaces;">{&content.content}</p>
                    </a>
                },
                ContentType::Code => html! {
                    <div style="max-width: 100vw; display: flex; flex: 1; overflow-x: auto;">
                        <Code highlighted_code={match &content.highlighted_code {
                            Some(code) => code,
                            None => &content.content,
                        }}/>
                    </div>
                },
                ContentType::Image => html! {
                    <div style="display: flex; justify-content: center;">
                        <img src={&content.content} style="width: 75%; height: 100%; object-fit: contain;" />
                    </div>
                    }
                }
            }
        </div>
    }
}

#[cfg(feature = "editable")]
#[function_component(Content)]
pub fn content(
    ContentProps {
        action,
        article_action,
        on_edit,
        edited,
        content,
        dispatch_article,
        dispatch_error,
    }: &ContentProps,
) -> Html {
    let content = content.clone();

    let (is_loading, set_loading) = use_state(|| false);
    let (form, update_form) = {
        let content = (*content).clone();
        use_state(move || content)
    };

    let on_edit_content: Callback<MouseEvent> = {
        let (content_index, on_edit) = (content.index, on_edit.clone());
        Callback::from(move |_| on_edit.emit((content_index, true)))
    };

    // Content type
    let on_change_content_type: Callback<ContentType> = {
        let (form, update_form) = (form.clone(), update_form.clone());
        Callback::from(move |content_type: ContentType| {
            update_form(IContent {
                content_type,
                ..(*form).clone()
            });
        })
    };

    //Content
    let on_change_content_content: Callback<ChangeData> = {
        let (form, update_form) = (form.clone(), update_form.clone());
        Callback::from(move |event: ChangeData| match event {
            ChangeData::Value(content) => update_form(IContent {
                content,
                ..(*form).clone()
            }),
            _ => (),
        })
    };
    // Language
    let on_change_content_language: Callback<Language> = {
        let (form, update_form) = (form.clone(), update_form.clone());
        Callback::from(move |language| {
            update_form(IContent {
                language: Some(language),
                ..(*form).clone()
            });
        })
    };
    // URL
    let on_change_content_url: Callback<ChangeData> = {
        let (form, update_form) = (form.clone(), update_form.clone());
        Callback::from(move |event: ChangeData| match event {
            ChangeData::Value(url) => update_form(IContent {
                url: Some(url),
                ..(*form).clone()
            }),
            _ => (),
        })
    };

    // Update form on content update.
    {
        let update_form = update_form.clone();
        use_effect_with_deps(
            move |content| {
                update_form(<IContent>::clone(content));
                || {}
            },
            content.clone(),
        );
    };

    let on_cancel_edit: Callback<MouseEvent> = {
        let (on_edit, content_index) = (on_edit.clone(), content.index);
        Callback::from(move |_| {
            on_edit.emit((content_index, false));
        })
    };
    let on_delete_content: Callback<MouseEvent> = {
        let (
            article_id,
            content_id,
            content_index,
            chapter_id,
            article_action,
            set_loading,
            dispatch_article,
            dispatch_error,
        ) = (
            content.article_id,
            content.id,
            content.index,
            content.chapter_id,
            article_action.clone(),
            set_loading.clone(),
            dispatch_article.clone(),
            dispatch_error.clone(),
        );
        let context = use_context::<Rc<BlogStore>>().expect("Could not find context!");
        match article_action {
            // Existing article
            Action::Edit => Callback::from(move |_| {
                set_loading(true);

                let (set_loading, dispatch_article, dispatch_error) = (
                    set_loading.clone(),
                    dispatch_article.clone(),
                    dispatch_error.clone(),
                );
                let future = async move { delete_content(&content_id).await };
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
            }),
            // Article being created
            Action::Add => Callback::from(move |_| {
                let mut article = context.new_article.clone();
                let chapter = article
                    .chapters
                    .iter_mut()
                    .find(|c| c.id == chapter_id)
                    .expect("Could not find chapter!");

                for cont in &mut chapter.contents {
                    if cont.index > content_index {
                        cont.index = cont.index - 1;
                    }
                }
                let content_index = chapter
                    .contents
                    .iter()
                    .position(|c| c.id == content_id)
                    .expect("Could not find content");
                chapter.contents.remove(content_index);
                dispatch_article.emit(article);
            }),
        }
    };

    let on_save_content: Callback<MouseEvent> = {
        let (
            content_action,
            form,
            article_action,
            set_loading,
            dispatch_article,
            dispatch_error,
            on_edit,
        ) = (
            action.clone(),
            form.clone(),
            article_action.clone(),
            set_loading.clone(),
            dispatch_article.clone(),
            dispatch_error.clone(),
            on_edit.clone(),
        );
        let context = use_context::<Rc<BlogStore>>().expect("Could not find context!");
        Callback::from(move |_| {
            let (form, content_action, dispatch_article, dispatch_error, set_loading, on_edit) = (
                IContent {
                    content: match form.content_type {
                        // Remove the API URL from the image path
                        ContentType::Image => (&form.content[API_URL.len()..]).to_owned(),
                        _ => form.content.clone(),
                    },
                    language: match form.content_type {
                        ContentType::Code => match &form.language {
                            None => Some(Language::Bash),
                            _ => form.language.clone(),
                        },
                        _ => None,
                    },
                    ..(*form).clone()
                },
                content_action.clone(),
                dispatch_article.clone(),
                dispatch_error.clone(),
                set_loading.clone(),
                on_edit.clone(),
            );
            match article_action {
                // Existing article
                Action::Edit => {
                    set_loading(true);
                    let (content_index, set_loading, dispatch_article, dispatch_error) = (
                        form.index,
                        set_loading.clone(),
                        dispatch_article.clone(),
                        dispatch_error.clone(),
                    );
                    match content_action {
                        Action::Add => {
                            let future = async move { add_content(&IContent { ..form }).await };
                            handle_future(future, move |response: Result<IArticle, Status>| {
                                match response {
                                    Ok(article) => dispatch_article.emit(article),
                                    Err(_) => dispatch_error.emit(true),
                                };
                                set_loading(false);
                            });
                        }
                        Action::Edit => {
                            let future = async move { update_content(&IContent { ..form }).await };
                            handle_future(future, move |response: Result<IArticle, Status>| {
                                match response {
                                    Ok(article) => dispatch_article.emit(article),
                                    Err(_) => dispatch_error.emit(true),
                                };
                                set_loading(false);
                                on_edit.emit((content_index, false));
                            });
                        }
                    };
                }
                // Article being created
                Action::Add => {
                    set_loading(true);
                    let mut article = context.new_article.clone();
                    let chapter = article
                        .chapters
                        .iter_mut()
                        .find(|c| c.id == form.chapter_id)
                        .expect("Could not find chapter!");

                    match content_action {
                        Action::Add => {
                            for cont in &mut chapter.contents {
                                if cont.index >= form.index {
                                    cont.index = cont.index + 1;
                                }
                            }
                            chapter.contents.insert(form.index as usize, form);
                            dispatch_article.emit(article);
                            set_loading(false);
                        }
                        Action::Edit => {
                            let content_index = chapter
                                .contents
                                .iter()
                                .position(|c| c.id == form.id)
                                .expect("Could not find content");
                            chapter.contents[content_index] = form;
                            dispatch_article.emit(article);
                            set_loading(false);
                            on_edit.emit((content_index as i32, false));
                        }
                    };
                }
            }
        })
    };

    html! {
        <>
            {match form.content_type {
                ContentType::Text => match *edited {
                    true => html! {
                        <div>
                            <Select<ContentType> selected={&form.content_type} options={&CONTENT_TYPES} onchange={on_change_content_type} />
                            <TextArea rows={8} value={&form.content} onchange={on_change_content_content} />
                            <div style="display: flex; margin-top: 4px; margin-bottom: 4px; justify-content: flex-end; font-size:.8em;">
                                {match action {
                                    Action::Edit => html! {<Button variant={ButtonVariant::Danger} onclick={&on_delete_content} label="Delete"/>},
                                    _ => html! {}
                                }}
                                <Button onclick={on_cancel_edit} label="Cancel" />
                                <Button onclick={on_save_content} label="Save" />
                            </div>
                        </div>
                    },
                    false => html! {
                        <div style="align-items:center; position: relative; display: flex; margin-top: 8px; margin-bottom: 8px;">
                            <div style="display: flex; justify-content: center; align-items: center;">
                                <div onclick={on_edit_content} style="position: absolute; right: -64px; cursor: pointer;">
                                    <i class="fa fa-edit"/>
                                </div>
                            </div>
                            <p style="white-space: break-spaces;">{&content.content}</p>
                        </div>
                        },
                },
                ContentType::Comment => match *edited {
                    true => html! {
                        <div>
                            <Select<ContentType> selected={&form.content_type} options={&CONTENT_TYPES} onchange={on_change_content_type} />
                            <TextArea rows={8} value={&form.content} onchange={on_change_content_content} />
                            <div style="display: flex; margin-top: 4px; margin-bottom: 4px; justify-content: flex-end; font-size:.8em;">
                                {match action {
                                    Action::Edit => html! {<Button variant={ButtonVariant::Danger} onclick={&on_delete_content} label="Delete"/>},
                                    _ => html! {}
                                    }
                                }
                                <Button onclick={on_cancel_edit} label="Cancel" />
                                <Button onclick={on_save_content} label="Save" />
                            </div>
                        </div>
                    },
                    false => html! {
                           <div style="align-items: center; position: relative; display: flex; margin-top: 8px; margin-bottom: 8px;">
                                <div style="display: flex; justify-content: center; align-items: center;">
                                   <div onclick={on_edit_content} style="position: absolute; right: -64px; cursor: pointer;">
                                        <i class="fa fa-edit"/>
                                    </div>
                                </div>
                                <p style="white-space: break-spaces; font-style: italic;">{&content.content}</p>
                            </div>
                    },
                },
                ContentType::Link => match *edited {
                    true => html! {
                        <div>
                            <Select<ContentType> selected={&form.content_type} options={&CONTENT_TYPES} onchange={on_change_content_type} />
                            <TextArea rows={8} value={&form.content} onchange={on_change_content_content} />
                            <TextArea rows={1} value={match &form.url {
                                    Some(url) => url,
                                    None => "URL...",
                                }}
                                onchange={on_change_content_url}
                            />
                            <div style="display: flex; margin-top: 4px; margin-bottom: 4px; justify-content: flex-end; font-size:.8em;">
                               {match action {
                                    Action::Edit => html! {<Button variant={ButtonVariant::Danger} onclick={&on_delete_content} label="Delete"/>},
                                    _ => html! {}
                                }}
                                <Button onclick={on_cancel_edit} label="Cancel" />
                                <Button onclick={on_save_content} label="Save" />
                            </div>
                        </div>
                    },
                    false => html! {
                        <div style="align-items: center; position: relative; display: flex; margin-top: 8px; margin-bottom: 8px;">
                            <div style="display: flex; justify-content: center; align-items: center;">
                                <div onclick={on_edit_content} style="position: absolute; right: -64px; cursor: pointer;">
                                    <i class="fa fa-edit"/>
                                </div>
                            </div>
                            <a target="_blank" href={match &form.url {
                                Some(url) => url,
                                None => ""
                            }}>
                            <p style="white-space: break-spaces;">{&content.content}</p>
                            </a>
                        </div>
                    },
                },
                ContentType::Code => match *edited {
                    true => html! {
                        <div>
                            <div style="display: flex;">
                                <Select<ContentType> selected={&form.content_type} options={&CONTENT_TYPES} onchange={on_change_content_type} />
                                <div style="margin-left: 8px;">
                                    <Select<Language>
                                        selected={match &form.language {
                                            Some(language) => language,
                                            None => &Language::Bash,
                                        }}
                                        options={LANGUAGES}
                                        onchange={on_change_content_language}
                                    />
                                </div>
                            </div>
                            <TextArea rows={8} value={&form.content} onchange={on_change_content_content} />
                            <div style="display: flex; margin-top: 4px; margin-bottom: 4px; justify-content: flex-end; font-size:.8em;">
                               {match action {
                                    Action::Edit => html! {<Button variant={ButtonVariant::Danger} onclick={&on_delete_content} label="Delete"/>},
                                    _ => html! {}
                                }}
                                <Button onclick={on_cancel_edit} label="Cancel" />
                                <Button onclick={on_save_content} label="Save" />
                            </div>
                        </div>
                    },
                    false => html! {
                        <div style="align-items: center; position: relative; display: flex; margin-top: 8px; margin-bottom: 8px;">
                           <div style="display: flex; justify-content: center; align-items: center;">
                                <div onclick={on_edit_content} style="position: absolute; right: -64px; cursor: pointer;">
                                    <i class="fa fa-edit"/>
                                </div>
                            </div>
                            <Code
                                highlighted_code={match &content.highlighted_code.clone() {
                                    Some(code) => code,
                                    None => &content.content,
                                }}
                            />
                        </div>
                    },
                },
                ContentType::Image => match *edited {
                    true => html! {
                        <div>
                            <Select<ContentType> selected={&form.content_type} options={&CONTENT_TYPES} onchange={on_change_content_type} />
                            <TextArea rows={1} value={&form.content} onchange={on_change_content_content} />
                            <div style="display: flex; margin-top: 4px; margin-bottom: 4px; justify-content: flex-end; font-size:.8em;">
                                {match action {
                                    Action::Edit => html! {<Button variant={ButtonVariant::Danger} onclick={&on_delete_content} label="Delete"/>},
                                    _ => html! {}
                                }}
                                <Button onclick={on_cancel_edit} label="Cancel" />
                                <Button onclick={on_save_content} label="Save" />
                            </div>
                        </div>
                    },
                    false => {
                        html! {
                            <div style="align-items: center; position: relative; display: flex; margin-top: 8px; margin-bottom: 8px;">
                                <div style="display: flex; justify-content: center; align-items: center;">
                                    <div onclick={on_edit_content} style="position: absolute; right: -64px; cursor: pointer;">
                                        <i class="fa fa-edit"/>
                                    </div>
                                </div>
                                <img src={&content.content} style="width: 75%; height: 100%; object-fit: contain;" />
                            </div>
                        }
                    }
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
        </>
    }
}
