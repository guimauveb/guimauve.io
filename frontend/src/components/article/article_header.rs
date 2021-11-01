use {
    crate::{
        components::{
            tag_label::TagLabel,
            text::{Text, TextVariant},
        },
        entities::{
            action::Action,
            interfaces::{IArticle, IArticleHeader},
        },
        utils::date::format_date,
    },
    std::rc::Rc,
    yew::{html, Callback, Properties},
    yew_functional::function_component,
};

#[cfg(feature = "editable")]
use {
    crate::{
        components::{button::Button, loader::Loader, text_area::TextArea},
        entities::interfaces::Status,
        service::{articles::update_article_header, future::handle_future},
        store::store::BlogStore,
        utils::date::get_current_readable_date,
        API_URL,
    },
    yew::{ChangeData, MouseEvent},
    yew_functional::{use_context, use_state},
};

#[derive(Properties, Clone, PartialEq)]
pub struct ArticleHeaderProps {
    pub article_header: Rc<IArticleHeader>,
    #[prop_or(Action::Edit)]
    pub article_action: Action,
    #[prop_or(false)]
    pub edited: bool,
    #[prop_or_default]
    pub dispatch_article: Callback<IArticle>,
    #[prop_or_default]
    pub dispatch_error: Callback<bool>,
}

#[cfg(not(feature = "editable"))]
#[function_component(ArticleHeader)]
pub fn article_header(ArticleHeaderProps { article_header, .. }: &ArticleHeaderProps) -> Html {
    html! {
        <>
            <div style="margin-bottom: 8px;">
                    <Text as_element="h1" variant={TextVariant::Heading} value={&article_header.title} />
            </div>
            <div style="margin-top: 8px; margin-bottom: 8px;">
                {for article_header.tags.iter().map(move |tag| html! { <TagLabel tag={&tag.label} /> })}
            </div>
            <div style="margin-top: 12px; margin-bottom: 12px;">
                {match format_date(&article_header.pub_date) {
                    Ok(date) => html! {<Text value={&date}/>},
                    Err(_) => html! {<Text value="An error occured!"/>},
                }}
            </div>
            <div style="margin-top: 8px; margin-bottom: 12px;">
                <div style="display: flex; flex: 1; flex-direction: column;">
                    <img src={&article_header.image} style="width: 100%; object-fit: contain;"/>
                    {match &article_header.image_credits {
                        Some(credits) => html! {
                            <p style="font-style: italic; font-size: .8rem; margin-top: 4px;">
                                {credits}
                            </p>
                        },
                        None => html! {}
                    }}
                </div>
            </div>
            <div style="margin-top: 8px; margin-bottom: 8px;">
                <Text as_element="h3" value={&article_header.headline} variant={TextVariant::Comment} />
            </div>
        </>
    }
}

#[cfg(feature = "editable")]
#[function_component(ArticleHeader)]
pub fn article_header(
    ArticleHeaderProps {
        article_header,
        article_action,
        edited,
        dispatch_article,
        dispatch_error,
    }: &ArticleHeaderProps,
) -> Html {
    let edited = edited.clone();

    let (is_title_edited, set_title_edited) = use_state(move || edited.clone());
    let (is_headline_edited, set_headline_edited) = use_state(move || edited.clone());
    let (is_image_edited, set_image_edited) = use_state(move || edited.clone());

    let (is_loading, set_loading) = use_state(|| false);

    let (form, update_form) = {
        let article_header = article_header.clone();
        use_state(move || (*article_header).clone())
    };

    // Title
    let on_edit_title: Callback<MouseEvent> = {
        let (set_title_edited, update_form, article_header) = (
            set_title_edited.clone(),
            update_form.clone(),
            article_header.clone(),
        );
        Callback::from(move |_| {
            update_form((*article_header).clone()); // ?
            set_title_edited(true)
        })
    };
    let on_change_title: Callback<ChangeData> = {
        let (update_form, form) = (update_form.clone(), form.clone());
        Callback::from(move |event: ChangeData| match event {
            ChangeData::Value(title) => update_form(IArticleHeader {
                title,
                ..(*form).clone()
            }),
            _ => (),
        })
    };
    let on_cancel_edit_title: Callback<MouseEvent> = {
        let set_title_edited = set_title_edited.clone();
        Callback::from(move |_| set_title_edited(false))
    };

    let set_edited = {
        let (set_title_edited, set_image_edited, set_headline_edited) = (
            set_title_edited.clone(),
            set_image_edited.clone(),
            set_headline_edited.clone(),
        );
        move |edited| {
            set_title_edited(edited);
            set_image_edited(edited);
            set_headline_edited(edited);
        }
    };

    // TODO - Tags -> Use a multiselect
    //let on_edit_tags: Callback<MouseEvent> = {
    //    let set_tags_edited = set_tags_edited.clone();
    //    let update_form = update_form.clone();
    //    let tags = ITag { tags: None };
    //    Callback::from(move |_| {
    //        update_form(article.clone());
    //        set_image_edited(true)
    //    })
    //};

    // Preview
    let on_edit_headline: Callback<MouseEvent> = {
        let (set_headline_edited, update_form, article_header) = (
            set_headline_edited.clone(),
            update_form.clone(),
            article_header.clone(),
        );
        Callback::from(move |_| {
            update_form((*article_header).clone()); // ?
            set_headline_edited(true)
        })
    };
    let on_change_headline: Callback<ChangeData> = {
        let (update_form, form) = (update_form.clone(), form.clone());
        Callback::from(move |event: ChangeData| match event {
            ChangeData::Value(headline) => update_form(IArticleHeader {
                headline,
                ..(*form).clone()
            }),
            _ => (),
        })
    };
    let on_cancel_edit_headline: Callback<MouseEvent> = {
        let set_headline_edited = set_headline_edited.clone();
        Callback::from(move |_| set_headline_edited(false))
    };

    // Image
    let on_edit_image: Callback<MouseEvent> = {
        let (set_image_edited, update_form, article_header) = (
            set_image_edited.clone(),
            update_form.clone(),
            article_header.clone(),
        );
        Callback::from(move |_| {
            update_form((*article_header).clone());
            set_image_edited(true)
        })
    };
    let on_change_image: Callback<ChangeData> = {
        let (update_form, form) = (update_form.clone(), form.clone());
        Callback::from(move |event: ChangeData| match event {
            ChangeData::Value(image) => update_form(IArticleHeader {
                image,
                ..(*form).clone()
            }),
            _ => (),
        })
    };
    let on_cancel_edit_image: Callback<MouseEvent> = {
        let set_image_edited = set_image_edited.clone();
        Callback::from(move |_| set_image_edited(false))
    };

    let on_save_article_header: Callback<MouseEvent> = {
        let (form, article_action, dispatch_article, dispatch_error, set_loading, set_edited) = (
            form.clone(),
            article_action.clone(),
            dispatch_article.clone(),
            dispatch_error.clone(),
            set_loading.clone(),
            set_edited.clone(),
        );
        let context = use_context::<Rc<BlogStore>>().expect("Could not find context!");
        let article = match article_action {
            Action::Add => context.new_article.clone(),
            Action::Edit => context
                .articles
                .get(&article_header.article_id)
                .expect("Could not find article!")
                .clone(),
        };
        Callback::from(move |_| {
            set_loading(true);

            let (form, article, set_loading, dispatch_article, dispatch_error, set_edited) = (
                form.clone(),
                article.clone(),
                set_loading.clone(),
                dispatch_article.clone(),
                dispatch_error.clone(),
                set_edited.clone(),
            );

            match article_action {
                Action::Add => {
                    dispatch_article.emit(IArticle {
                        // TODO - dispatch_article_header
                        id: form.article_id,
                        title: form.title.clone(),
                        pub_date: form.pub_date.clone(),
                        published: form.published,
                        headline: form.headline.clone(),
                        image: form.image.clone(),
                        tags: form.tags.clone(), // TODO
                        ..article
                    });
                    set_edited(false);
                    set_loading(false);
                }
                Action::Edit => {
                    let future = async move {
                        update_article_header(&IArticleHeader {
                            image: (&form.image[API_URL.len()..]).to_owned(),
                            ..(*form).clone()
                        })
                        .await
                    };
                    handle_future(future, move |response: Result<IArticle, Status>| {
                        match response {
                            Ok(article) => dispatch_article.emit(article),
                            Err(_) => dispatch_error.emit(true),
                        };
                        set_edited(false);
                        set_loading(false);
                    });
                }
            };
        })
    };

    let readable_date = match article_action {
        Action::Edit => match format_date(&article_header.pub_date) {
            Ok(date) => date,
            Err(_) => "An error has occured.".to_owned(),
        },
        Action::Add => match get_current_readable_date() {
            Ok(current_date) => current_date,
            Err(_) => "An errror has occured.".to_owned(),
        },
    };

    html! {
        <>
            {match *is_title_edited {
                true => html! {
                    <>
                        <TextArea rows={2} value={&form.title} onchange={on_change_title} />
                        <div style="display: flex; margin-top: 4px; margin-bottom: 4px; justify-content: flex-end; font-size: .8em;">
                            <>
                                <Button onclick={&on_cancel_edit_title} label="Cancel"/>
                                <Button onclick={&on_save_article_header} label="Save"/>
                            </>
                        </div>
                    </>
                    },
                    false => html! {
                        <div style="align-items: center; position: relative; display: flex; margin-top: 8px; margin-bottom: 8px;">
                            <div
                                onclick={&on_edit_title}
                                style="width:42px; height:42px; display: flex; justify-content: center; align-items:center; position: absolute; right: -78px; cursor: pointer;">
                                <i class="fa fa-edit"></i>
                            </div>
                            <Text as_element="h1" variant={TextVariant::Heading} value={&article_header.title} />
                        </div>
                        }
                }
            }
            <div style="margin-top: 4px; margin-bottom: 4px;">
                {for article_header.tags.iter().map(move |tag| {
                    html! { <TagLabel tag={&tag.label} /> }
                })}
            </div>
            <div style="display: flex; margin-top: 8px; margin-bottom: 12px;">
                <Text value={readable_date}/>
            </div>
            <div>
                {match *is_image_edited {
                    true => html! {
                        <>
                            <TextArea rows={1} value={&form.image} onchange={on_change_image} />
                            <div style="display: flex; margin-top: 4px; margin-bottom: 4px; justify-content: flex-end; font-size: .8em;">
                            <>
                                <Button onclick={&on_cancel_edit_image} label="Cancel"/>
                                <Button onclick={&on_save_article_header} label="Save"/>
                            </>
                            </div>
                        </>
                    },
                    false => {
                        html! {
                            <div style="align-items: center; position: relative; display: flex; margin-top: 8px; margin-bottom: 12px;">
                               <div
                                    onclick={on_edit_image}
                                    style="width:42px; height:42px; display: flex; justify-content: center; align-items:center; position: absolute; right: -78px; cursor: pointer;">
                                    <i class="fa fa-edit"></i>
                                </div>
                                <div style="display: flex; flex: 1;">
                                    <img src={&article_header.image} style="width: 100%;" />
                                    {match &article_header.image_credits {
                                        Some(credits) => html! {
                                            <p style="transform: rotate(180deg); writing-mode: tb-rl; font-style: italic; font-size: .8rem; margin-left: 4px;">
                                                {credits}
                                            </p>
                                        },
                                        None => html! {}
                                    }}
                                </div>
                            </div>
                        }
                    }
                }}
            </div>
            <div>
                {match *is_headline_edited {
                    true => html! {
                        <>
                            <TextArea rows={2} value={&form.headline} onchange={&on_change_headline} />
                            <div style="display: flex; margin-top: 4px; margin-bottom: 4px; justify-content: flex-end; font-size: .8em;">
                            <>
                                <Button onclick={&on_cancel_edit_headline} label="Cancel"/>
                                <Button onclick={&on_save_article_header} label="Save"/>
                            </>
                            </div>
                        </>
                    },
                    false => html! {
                        <div style="align-items: center; position: relative; display: flex; margin-top: 8px; margin-bottom: 8px;">
                            <div
                                onclick={on_edit_headline}
                                style="width:42px; height:42px; display: flex; justify-content: center; align-items:center; position: absolute; right: -78px; cursor: pointer;">
                                <i class="fa fa-edit"></i>
                            </div>
                        <Text as_element="h3" value={&article_header.headline} variant={TextVariant::Comment} />
                        </div>
                        }
                }}
            </div>
            {if *is_loading {
                html! {
                    <div style="align-items: center; justify-content: center; display: flex; margin-top: 48px;">
                        <Loader />
                    </div>
                }
            } else {
                html! {}
            }}
        </>
    }
}
