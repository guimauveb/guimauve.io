use {
    crate::entities::interfaces::IChapter,
    yew::{html, Properties},
    yew_functional::function_component,
};

#[derive(Properties, Clone, PartialEq)]
pub struct ChapterListProps {
    pub chapters: Vec<IChapter>,
}

#[function_component(ChapterList)]
pub fn chapter_list(ChapterListProps { chapters }: &ChapterListProps) -> Html {
    html! {
        <div style="margin-bottom: 12px;">
            <h2 class="article-chapter" style="margin-bottom: 12px !important;">{"Chapters"}</h2>
            {for chapters.iter().enumerate().map(|(index, chapter)| {
                html! {
                    <div>
                        <a href="#".to_string() + &chapter.id.to_string()>{(index + 1).to_string() + " - " + &chapter.title}</a>
                    </div>
                }
            })
            }
        </div>
    }
}
