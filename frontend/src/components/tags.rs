use {
    crate::{
        components::{
            loader::Loader,
            tag_label::TagLabel,
            text::{Text, TextVariant},
        },
        entities::interfaces::{ITag, Status},
        service::{future::handle_future, tags::get_tag_list},
        store::store::BlogStore,
    },
    std::rc::Rc,
    yew::{html, Callback, Properties},
    yew_functional::{function_component, use_context, use_effect_with_deps, use_state},
};

#[derive(Properties, Clone, PartialEq)]
pub struct TagsProps {
    pub dispatch_tags: Callback<Vec<ITag>>,
    pub dispatch_error: Callback<bool>,
}

#[function_component(Tags)]
pub fn tags(
    TagsProps {
        dispatch_tags,
        dispatch_error,
    }: &TagsProps,
) -> Html {
    let (is_loading, set_loading) = use_state(|| false);

    let context = use_context::<Rc<BlogStore>>().expect("No context found!");
    let tags = &context.tags;

    let dispatch_tags = dispatch_tags.clone();
    let dispatch_error = dispatch_error.clone();

    use_effect_with_deps(
        move |_| {
            set_loading(true);
            let future = async { get_tag_list().await };
            handle_future(future, move |data: Result<Vec<ITag>, Status>| {
                match data {
                    Ok(tags) => dispatch_tags.emit(tags),
                    Err(_) => dispatch_error.emit(true),
                };
                set_loading(false);
            });
            || {}
        },
        (),
    );

    html! {
        <div style="display: flex; justify-content: center; flex: 1;">
            <div style="flex: 1; max-width: 1024px;">
                <div style="align-items: center; display: flex; margin-bottom: 24px;">
                    <Text variant={TextVariant::Heading} value="/tags" />
                </div>
                <div>
                    {for tags.iter().map(move |tag| { html! {<TagLabel tag={&tag.label} />} }) }
                </div>
                {if *is_loading {
                    html! {
                        <div style="align-items: center; justify-content: center; display: flex; margin-top: 24px; margin-bottom: 24px;">
                            <Loader />
                        </div>
                    }
                } else {
                    html! {}
                }}
            </div>
        </div>
    }
}
