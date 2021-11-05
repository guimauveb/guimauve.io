use {
    super::content::Content,
    crate::entities::{
        action::Action,
        interfaces::{IArticle, IContent},
    },
    std::rc::Rc,
    yew::{html, Callback, Properties},
    yew_functional::function_component,
};

#[cfg(feature = "editable")]
use {
    std::collections::HashMap,
    yew_functional::{use_effect_with_deps, use_state},
};

#[derive(Properties, Clone, PartialEq)]
pub struct ContentsProps {
    pub contents: Rc<Vec<IContent>>,
    #[prop_or_default]
    pub article_id: i32,
    #[prop_or_default]
    pub chapter_id: i32,
    #[prop_or(Action::Edit)]
    pub chapter_action: Action,
    #[prop_or(Action::Edit)]
    pub article_action: Action,
    #[prop_or_default]
    pub dispatch_article: Callback<IArticle>,
    #[prop_or_default]
    pub dispatch_error: Callback<bool>,
}

#[cfg(not(feature = "editable"))]
#[function_component(Contents)]
pub fn contents(ContentsProps { contents, .. }: &ContentsProps) -> Html {
    html! {
        {for contents.iter().map(|content| html! {<Content content={Rc::new(content.clone())} />})}
    }
}

#[cfg(feature = "editable")]
#[function_component(Contents)]
pub fn contents(
    ContentsProps {
        contents,
        article_action,
        article_id,
        chapter_id,
        chapter_action,
        dispatch_article,
        dispatch_error,
    }: &ContentsProps,
) -> Html {
    let contents_length = contents.len() as i32;
    let contents = contents.clone();

    /* Existing contents */
    let (edited_contents, set_edited_contents) = {
        let contents = contents
            .clone()
            .iter()
            .map(|content: &IContent| (content.index, false))
            .collect::<HashMap<i32, bool>>();

        use_state(move || contents)
    };
    let on_edit_existing_content: Callback<(i32, bool)> = {
        let (edited_contents, set_edited_contents) =
            (edited_contents.clone(), set_edited_contents.clone());
        Callback::from(move |(content_index, edited): (i32, bool)| {
            let mut edited_contents = (*edited_contents).clone();
            let set_edited_contents = set_edited_contents.clone();
            *edited_contents.get_mut(&content_index).unwrap() = edited;
            set_edited_contents(edited_contents)
        })
    };
    {
        let set_edited_contents = set_edited_contents.clone();
        use_effect_with_deps(
            move |contents| {
                set_edited_contents(
                    contents
                        .iter()
                        .map(|content: &IContent| (content.index, false))
                        .collect::<HashMap<i32, bool>>(),
                );

                || {}
            },
            contents.clone(),
        );
    }

    /* New contents */
    let (new_contents_displayed, set_new_contents_displayed) = {
        let (contents, last_index) = (contents.clone(), contents_length);
        // TODO - Check if it is possible to map and insert without making it mutable.
        let mut new_contents = contents
            .iter()
            .map(|content: &IContent| (content.index, false))
            .collect::<HashMap<i32, bool>>();
        new_contents.insert(last_index, false);

        use_state(move || new_contents)
    };
    let on_display_new_content: Callback<(i32, bool)> = {
        let (new_contents_displayed, set_new_contents_displayed) = (
            new_contents_displayed.clone(),
            set_new_contents_displayed.clone(),
        );
        Callback::from(move |(content_index, displayed)| {
            let mut new_contents_displayed = (*new_contents_displayed).clone();
            *new_contents_displayed.get_mut(&content_index).unwrap() = displayed;
            set_new_contents_displayed(new_contents_displayed)
        })
    };

    // Update contents and new contents hashmaps when contents are updated.
    // TODO - Keep previous states
    {
        let set_new_contents_displayed = set_new_contents_displayed.clone();
        use_effect_with_deps(
            move |contents| {
                // TODO - Check if it is possible to map and insert without making it mutable.
                let mut new_contents = contents
                    .iter()
                    .map(|content: &IContent| (content.index, false))
                    .collect::<HashMap<i32, bool>>();
                new_contents.insert(contents.len() as i32, false);

                set_new_contents_displayed(new_contents);
                || {}
            },
            contents.clone(),
        );
    }

    html! {
        <>
            {for contents.iter().map(|content| {
                html! {
                    <>
                        {match *&new_contents_displayed.get(&content.index) {
                            Some(true) => {
                                html! {
                                   <Content
                                        action={Action::Add}
                                        article_action={article_action}
                                        content={
                                            Rc::new(IContent {
                                                id: contents_length + 1 as i32,
                                                article_id: *article_id,
                                                chapter_id: *chapter_id,
                                                index: content.index,
                                                ..IContent::default()
                                            })
                                        }
                                        edited={true}
                                        on_edit={&on_display_new_content}
                                        dispatch_article={dispatch_article}
                                        dispatch_error={dispatch_error}
                                    />
                                }
                            },
                            Some(false) => {
                                html! {
                                    <div
                                        style="flex: 1; display: flex; justify-content: flex-end; align-items:center;">
                                        <div
                                            onclick={
                                                let (content_index, on_display_new_content) = (content.index, on_display_new_content.clone());
                                                Callback::from(move|_| on_display_new_content.emit((content_index, true)))
                                            }
                                            style="display: flex; align-items:center;cursor: pointer;position: relative; right: -64px; height: 42px">
                                            <i class="fa fa-plus"></i>
                                            <div style="margin-left: 8px;">
                                                <p>{"Content"}</p>
                                            </div>
                                        </div>
                                    </div>
                               }
                           },
                            None => html! {}
                        }}
                        <Content
                            content={Rc::new(content.clone())}
                            article_action={article_action}
                            edited={
                                match *&edited_contents.get(&content.index) {
                                    Some(true) => true,
                                    _ => false,
                                }
                            }
                            on_edit={&on_edit_existing_content}
                            dispatch_article={dispatch_article}
                            dispatch_error={dispatch_error}
                        />
                    </>
                }
            })}
            {if *chapter_action != Action::Add {
                match *&new_contents_displayed.get(&(contents_length)) {
                    Some(true) => {
                        html! {
                            <Content
                                action={Action::Add}
                                article_action={article_action}
                                content={
                                    Rc::new(IContent {
                                        id: contents_length + 1 as i32,
                                        article_id: *article_id,
                                        chapter_id: *chapter_id,
                                        index: contents_length,
                                        ..IContent::default()
                                   })
                                }
                                edited={true}
                                on_edit={&on_display_new_content}
                                dispatch_article={dispatch_article}
                                dispatch_error={dispatch_error}
                            />
                        }
                    },
                    Some(false) => {
                        html! {
                            <div
                                style="flex: 1; display: flex; justify-content: flex-end; align-items:center;">
                                <div
                                    onclick={
                                        let on_display_new_content = on_display_new_content.clone();
                                        Callback::from(move|_| on_display_new_content.emit((contents_length, true)))
                                    }
                                    style="display: flex; align-items:center; cursor: pointer; position: relative; right: -64px; height: 42px;">
                                    <i class="fa fa-plus"></i>
                                    <div style="margin-left: 8px;">
                                        <p>{"Content"}</p>
                                    </div>
                                </div>
                            </div>
                        }
                    },
                    None => html! {}
                }
            } else {
                html! {}
            }}
        </>
    }
}
