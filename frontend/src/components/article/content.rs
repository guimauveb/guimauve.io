use {
    crate::{
        components::{
            box_component::BoxComponent,
            code::Code,
            image::Image,
            text::{Text, TextVariant},
        },
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
            text_editor::TextEditor,
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
       {match &content.content_type {
           ContentType::Text => html! {
                <BoxComponent align_items="center" position="relative" display="flex" mb="24px">
                   <Text value={&content.content} />
                </BoxComponent>
           },
           ContentType::Comment => html! {
                <BoxComponent align_items="center" justify_content="center" position="relative" display="flex" mb="24px">
                   <Text variant={TextVariant::Comment} value={&content.content} />
                </BoxComponent>
           },
           ContentType::Link => html! {
                <BoxComponent align_items="center" position="relative" display="flex" mb="24px">
                    <a target="_blank" href={match &content.url {
                        Some(url) => url,
                        None => ""
                    }}>
                        <Text value={&content.content} />
                    </a>
                </BoxComponent>
           },
           ContentType::Code => html! {
                <BoxComponent max_width="100vw" align_items="center" position="relative" display="flex" mb="24px">
                       <Code highlighted_code={match &content.highlighted_code {
                            Some(code) => code,
                            None => &content.content,
                       }}/>
                </BoxComponent>
               },
           ContentType::Image => html! {
                <BoxComponent align_items="center" justify_content="center" display="flex" mb="24px">
                    <Image src={&content.content} width="75%" />
                </BoxComponent>
                }
           }
       }
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
        let article = match article_action {
            Action::Add => context.new_article.clone(),
            Action::Edit => context
                .articles
                .get(&article_id)
                .expect("Could not find article!")
                .clone(),
        };
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
                let mut article = article.clone();
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
                dispatch_article.emit(article)
            }),
        }
    };

    let on_save_content: Callback<MouseEvent> = {
        let (
            article_id,
            content_action,
            form,
            article_action,
            set_loading,
            dispatch_article,
            dispatch_error,
            on_edit,
        ) = (
            content.article_id,
            action.clone(),
            form.clone(),
            article_action.clone(),
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
            let (form, content_action, dispatch_article, dispatch_error, set_loading, on_edit) = (
                // Trim the API URL from the image path
                IContent {
                    content: match form.content_type {
                        ContentType::Image => (&form.content[API_URL.len()..]).to_owned(),
                        _ => form.content.clone(),
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
                    let mut article = article.clone();
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
                        <BoxComponent>
                            <Select<ContentType> selected={&form.content_type} options={&CONTENT_TYPES} onchange={on_change_content_type} />
                            <TextEditor rows={8} data={&form.content} onchange={on_change_content_content} />
                            <BoxComponent display="flex" mt="4px" mb="4px" justify_content="flex-end" font_size=".8em">
                                {match action {
                                    Action::Edit => html! {<Button variant={ButtonVariant::Danger} onclick={&on_delete_content} label="Delete"/>},
                                    _ => html! {}
                                }}
                                <Button onclick={on_cancel_edit} label="Cancel" />
                                <Button onclick={on_save_content} label="Save" />
                            </BoxComponent>
                        </BoxComponent>
                    },
                    false => html! {
                        <BoxComponent align_items="center" position="relative" display="flex" mt="8px" mb="8px">
                            <BoxComponent display="flex" justify_content="center" align_items="center">
                                <BoxComponent onclick={on_edit_content} position="absolute" right="-64px" cursor="pointer">
                                    <i class="fa fa-edit"/>
                                </BoxComponent>
                            </BoxComponent>
                            <Text value={&content.content} />
                        </BoxComponent>
                        },
                },
                ContentType::Comment => match *edited {
                    true => html! {
                        <BoxComponent>
                            <Select<ContentType> selected={&form.content_type} options={&CONTENT_TYPES} onchange={on_change_content_type} />
                            <TextEditor rows={8} data={&form.content} onchange={on_change_content_content} />
                            <BoxComponent display="flex" mt="4px" mb="4px" justify_content="flex-end" font_size=".8em">
                                {match action {
                                    Action::Edit => html! {<Button variant={ButtonVariant::Danger} onclick={&on_delete_content} label="Delete"/>},
                                    _ => html! {}
                                    }
                                }
                                <Button onclick={on_cancel_edit} label="Cancel" />
                                <Button onclick={on_save_content} label="Save" />
                            </BoxComponent>
                        </BoxComponent>
                    },
                    false => html! {
                           <BoxComponent align_items="center" position="relative" display="flex" mt="8px" mb="8px">
                                <BoxComponent display="flex" justify_content="center" align_items="center">
                                   <BoxComponent onclick={on_edit_content} position="absolute" right="-64px" cursor="pointer">
                                        <i class="fa fa-edit"/>
                                    </BoxComponent>
                                </BoxComponent>
                                <Text variant={TextVariant::Comment} value={&content.content} />
                            </BoxComponent>
                    },
                },
                ContentType::Link => match *edited {
                    true => html! {
                        <BoxComponent>
                            <Select<ContentType> selected={&form.content_type} options={&CONTENT_TYPES} onchange={on_change_content_type} />
                            <TextEditor rows={8} data={&form.content} onchange={on_change_content_content} />
                            <TextEditor rows={1} data={match &form.url {
                                    Some(url) => url,
                                    None => "URL...",
                                }}
                                onchange={on_change_content_url}
                            />
                            <BoxComponent display="flex" mt="4px" mb="4px" justify_content="flex-end" font_size=".8em">
                               {match action {
                                    Action::Edit => html! {<Button variant={ButtonVariant::Danger} onclick={&on_delete_content} label="Delete"/>},
                                    _ => html! {}
                                }}
                                <Button onclick={on_cancel_edit} label="Cancel" />
                                <Button onclick={on_save_content} label="Save" />
                            </BoxComponent>
                        </BoxComponent>
                    },
                    false => html! {
                        <BoxComponent align_items="center" position="relative" display="flex" mt="8px" mb="8px">
                            <BoxComponent display="flex" justify_content="center" align_items="center">
                                <BoxComponent onclick={on_edit_content} position="absolute" right="-64px" cursor="pointer">
                                    <i class="fa fa-edit"/>
                                </BoxComponent>
                            </BoxComponent>
                            <a target="_blank" href={match &form.url {
                                Some(url) => url,
                                None => ""
                            }}>
                                <Text value={&content.content} />
                            </a>
                        </BoxComponent>
                    },
                },
                ContentType::Code => match *edited {
                    true => html! {
                        <BoxComponent>
                            <BoxComponent display="flex">
                                <Select<ContentType> selected={&form.content_type} options={&CONTENT_TYPES} onchange={on_change_content_type} />
                                <BoxComponent ml="8px">
                                    <Select<Language> selected={match &form.language {
                                                                Some(language) => language,
                                                                None => &Language::Bash,
                                    }} options={&LANGUAGES} onchange={on_change_content_language} />
                                </BoxComponent>
                            </BoxComponent>
                            <TextEditor rows={8} data={&form.content} onchange={on_change_content_content} />
                            <BoxComponent display="flex" mt="4px" mb="4px" justify_content="flex-end" font_size=".8em">
                               {match action {
                                    Action::Edit => html! {<Button variant={ButtonVariant::Danger} onclick={&on_delete_content} label="Delete"/>},
                                    _ => html! {}
                                }}
                                <Button onclick={on_cancel_edit} label="Cancel" />
                                <Button onclick={on_save_content} label="Save" />
                            </BoxComponent>
                        </BoxComponent>
                    },
                    false => html! {
                       <BoxComponent align_items="center" position="relative" display="flex" mt="8px" mb="8px" max_width="100vw">
                            <BoxComponent display="flex" justify_content="center" align_items="center">
                                <BoxComponent onclick={on_edit_content} position="absolute" right="-64px" cursor="pointer">
                                    <i class="fa fa-edit"/>
                                </BoxComponent>
                            </BoxComponent>
                            <Code highlighted_code={match &content.highlighted_code.clone() {
                                                                Some(code) => code,
                                                                None => &content.content,
                                                            }}
                            />
                        </BoxComponent>
                    },
                },
                ContentType::Image => match *edited {
                    true => html! {
                        <BoxComponent>
                            <Select<ContentType> selected={&form.content_type} options={&CONTENT_TYPES} onchange={on_change_content_type} />
                            <TextEditor rows={1} data={&form.content} onchange={on_change_content_content} />
                            <BoxComponent display="flex" mt="4px" mb="4px" justify_content="flex-end" font_size=".8em">
                                {match action {
                                    Action::Edit => html! {<Button variant={ButtonVariant::Danger} onclick={&on_delete_content} label="Delete"/>},
                                    _ => html! {}
                                }}
                                <Button onclick={on_cancel_edit} label="Cancel" />
                                <Button onclick={on_save_content} label="Save" />
                            </BoxComponent>
                        </BoxComponent>
                    },
                    false => {
                        html! {
                            <BoxComponent justify_content="center" align_items="center" position="relative" display="flex" mt="8px" mb="8px">
                                <BoxComponent display="flex" justify_content="center" align_items="center">
                                    <BoxComponent onclick={on_edit_content} position="absolute" right="-64px" cursor="pointer">
                                        <i class="fa fa-edit"/>
                                    </BoxComponent>
                                </BoxComponent>
                                <Image src={&content.content} width="75%" />
                            </BoxComponent>
                        }
                    }
                }
            }}
            {if *is_loading {
                html! {
                    <BoxComponent align_items="center" justify_content="center" position="relative" display="flex">
                        <Loader />
                    </BoxComponent>
                }
            } else {
                html! {}
            }}
        </>
    }
}
