use {
    super::chapter::Chapter,
    crate::entities::{
        action::Action,
        interfaces::{IArticle, IChapter},
    },
    std::rc::Rc,
    yew::{html, Callback, Properties},
    yew_functional::function_component,
};

#[cfg(feature = "editable")]
use {
    crate::components::{box_component::BoxComponent, text::Text},
    std::collections::HashMap,
    yew_functional::{use_effect_with_deps, use_state},
};

#[derive(Properties, Clone, PartialEq)]
pub struct ChaptersProps {
    pub chapters: Rc<Vec<IChapter>>,
    #[prop_or_default]
    pub article_id: i32,
    #[prop_or(Action::Edit)]
    pub article_action: Action,
    #[prop_or_default]
    pub dispatch_article: Callback<IArticle>,
    #[prop_or_default]
    pub dispatch_error: Callback<bool>,
}

#[cfg(not(feature = "editable"))]
#[function_component(Chapters)]
pub fn chapters(ChaptersProps { chapters, .. }: &ChaptersProps) -> Html {
    html! { {for chapters.iter().map(|chapter| html! {<Chapter chapter={Rc::new(chapter.clone())} />})} }
}

#[cfg(feature = "editable")]
#[function_component(Chapters)]
pub fn chapters(
    ChaptersProps {
        chapters,
        article_action,
        article_id,
        dispatch_article,
        dispatch_error,
    }: &ChaptersProps,
) -> Html {
    let chapters_length = chapters.len() as i32;
    let chapters = chapters.clone();

    /* Existing chapters */
    let (edited_chapters, set_edited_chapters) = {
        let chapters = chapters
            .clone()
            .iter()
            .map(|chapter: &IChapter| (chapter.index, false))
            .collect::<HashMap<i32, bool>>();

        use_state(move || chapters)
    };

    let on_edit_existing_chapter: Callback<(i32, bool)> = {
        let (edited_chapters, set_edited_chapters) =
            (edited_chapters.clone(), set_edited_chapters.clone());
        Callback::from(move |(chapter_index, edited): (i32, bool)| {
            let (mut edited_chapters, set_edited_chapters) =
                (edited_chapters.clone(), set_edited_chapters.clone());
            /* Here, edited_chapters Rc's inner data is cloned since there are other Rc pointers to the same location:
             * Rc `edited_data` (at line 23) and this one. Hence for this Rc, we'll get a strong_count of 1.
             * Therefore it does not really makes sense to borrow `edited_chapter` as mut and create a mutable reference, since the data
             * it points to will be cloned anyway. */
            *Rc::make_mut(&mut edited_chapters)
                .get_mut(&chapter_index)
                .unwrap() = edited;

            set_edited_chapters((*edited_chapters).clone());
        })
    };
    // Update edited chapters hashmaps when chapters are updated. TODO - Keep previous states
    {
        let set_edited_chapters = set_edited_chapters.clone();
        use_effect_with_deps(
            move |chapters| {
                set_edited_chapters(
                    chapters
                        .iter()
                        .map(|chapter: &IChapter| (chapter.index, false))
                        .collect::<HashMap<i32, bool>>(),
                );
                || {}
            },
            chapters.clone(),
        );
    }

    /* New chapters */
    let (new_chapters_displayed, set_new_chapters_displayed) = {
        let (chapters, last_index) = (chapters.clone(), chapters_length);
        let mut new_chapters = chapters
            .iter()
            .map(|chapter: &IChapter| (chapter.index, false))
            .collect::<HashMap<i32, bool>>();
        new_chapters.insert(last_index, false);

        use_state(move || new_chapters)
    };
    let on_display_new_chapter: Callback<(i32, bool)> = {
        let (new_chapters_displayed, set_new_chapters_displayed) = (
            new_chapters_displayed.clone(),
            set_new_chapters_displayed.clone(),
        );
        Callback::from(move |(chapter_index, displayed)| {
            let mut new_chapters_displayed = (*new_chapters_displayed).clone();
            *new_chapters_displayed.get_mut(&chapter_index).unwrap() = displayed;
            set_new_chapters_displayed(new_chapters_displayed)
        })
    };
    // Update new chapters hashmaps when chapters are updated. TODO - Keep previous edit states
    {
        let set_new_chapters_displayed = set_new_chapters_displayed.clone();
        use_effect_with_deps(
            move |chapters| {
                let mut new_chapters = chapters
                    .clone()
                    .iter()
                    .map(|chapter: &IChapter| (chapter.index, false))
                    .collect::<HashMap<i32, bool>>();
                new_chapters.insert(chapters.len() as i32, false);
                set_new_chapters_displayed(new_chapters);
                || {}
            },
            chapters.clone(),
        );
    }

    html! {
        <>
            {for chapters.iter().map(|chapter| {
                html! {
                    <>
                        {match *&new_chapters_displayed.get(&chapter.index) {
                            Some(true) => {
                                html! {
                                    <Chapter
                                        action={Action::Add}
                                        article_action={article_action}
                                        chapter={
                                            Rc::new(IChapter {
                                                id: chapters_length + 1 as i32,
                                                article_id: *article_id,
                                                index: chapter.index,
                                                title: "New chapter...".to_owned(),
                                                ..IChapter::default()
                                            })
                                        }
                                        edited={true}
                                        on_edit={&on_display_new_chapter}
                                        dispatch_article={dispatch_article.clone()}
                                        dispatch_error={dispatch_error.clone()}

                                    />
                                }
                            },
                            Some(false) => {
                                html! {
                                    <BoxComponent
                                        flex="1"
                                        display="flex"
                                        justify_content="flex-end"
                                        align_items="center"
                                    >
                                        <BoxComponent
                                            onclick={
                                                let (chapter_index, on_display_new_chapter) = (chapter.index, on_display_new_chapter.clone());
                                                Callback::from(move|_| on_display_new_chapter.emit((chapter_index, true)))
                                            }
                                            display="flex"
                                            align_items="center"
                                            cursor="pointer"
                                            position="relative"
                                            right="-64px"
                                            height="42px"
                                        >
                                            <i class="fa fa-plus"></i>
                                            <BoxComponent ml="8px">
                                                <Text value="Chapter" />
                                            </BoxComponent>
                                        </BoxComponent>
                                    </BoxComponent>
                                }
                            }
                            None => html! {}
                            }
                        }
                        <Chapter
                            chapter={Rc::new(chapter.clone())}
                            article_action={article_action}
                            edited={
                                match *&edited_chapters.get(&chapter.index) {
                                    Some(true) => true,
                                    _ => false,
                                }
                            }
                            on_edit={&on_edit_existing_chapter}
                            dispatch_article={dispatch_article.clone()}
                            dispatch_error={dispatch_error.clone()}
                        />
                    </>
                }
            })}
            {match *&new_chapters_displayed.get(&chapters_length) {
                Some(true) => {
                    html! {
                        <Chapter
                            article_action={article_action}
                            action={Action::Add}
                            chapter={
                                Rc::new(IChapter {
                                    id: chapters_length + 1 as i32,
                                    article_id: *article_id,
                                    index: chapters_length,
                                    title: "New chapter...".to_owned(),
                                    ..IChapter::default()
                               })
                            }
                            edited={true}
                            on_edit={&on_display_new_chapter}
                            dispatch_article={dispatch_article.clone()}
                            dispatch_error={dispatch_error.clone()}
                        />
                    }
                },
                Some(false) => {
                    html! {
                        <BoxComponent
                            flex="1"
                            display="flex"
                            justify_content="flex-end"
                            align_items="center"
                            >
                                <BoxComponent
                                    onclick={
                                        let on_display_new_chapter = on_display_new_chapter.clone();
                                        Callback::from(move|_| on_display_new_chapter.emit((chapters_length, true)))
                                    }
                                    display="flex"
                                    align_items="center"
                                    cursor="pointer"
                                    position="relative"
                                    right="-64px"
                                    height="42px"
                                >
                                    <i class="fa fa-plus"></i>
                                    <BoxComponent ml="8px">
                                        <Text value="Chapter" />
                                    </BoxComponent>
                                </BoxComponent>
                            </BoxComponent>
                    }
                },
                None => html! {}
            }}
        </>
    }
}
