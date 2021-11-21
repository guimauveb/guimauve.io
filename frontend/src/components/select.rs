// Const generics - (<T;const N: usize>) https://blog.rust-lang.org/2021/02/26/const-generics-mvp-beta.html
use {
    std::fmt::Display,
    yew::{html, Callback, ChangeData, Properties},
    yew_functional::{function_component, use_effect_with_deps, use_state},
};

#[derive(Properties, Clone, PartialEq)]
pub struct Props<T>
where
    T: Clone + PartialEq + Display + 'static,
{
    pub selected: Option<T>,
    pub options: &'static [T],
    pub onchange: Callback<T>,
    #[prop_or(false)]
    pub disabled: bool,
}

#[function_component(Select)]
pub fn select<T>(
    Props {
        selected,
        options,
        onchange,
        disabled,
    }: &Props<T>,
) -> Html
where
    T: Clone + PartialEq + Display + 'static,
{
    let onchange = onchange.clone();
    let selected = selected.clone();

    let (selected_index, set_selected_index) = {
        let options = <&[T]>::clone(options);
        let selected = selected.clone();
        use_state(move || options.iter().position(|i| Some(i) == selected.as_ref()))
    };

    let view_option = |value: &T| {
        let flag = selected.as_ref() == Some(value);
        html! {
            <option selected=flag>{value}</option>
        }
    };

    use_effect_with_deps(
        move |(selected_index, options)| {
            if let Some(idx) = selected_index.as_ref() {
                let item = options.get(*idx).cloned();
                if let Some(value) = item {
                    onchange.emit(value);
                }
            }
            || {}
        },
        (selected_index, <&[T]>::clone(options)),
    );

    html! {
        <select
           style="background: inherit; color: inherit; border-radius: 0.12em; margin-bottom: 8px; padding: 2px; font-family: inherit;"
           disabled=*disabled
           onchange={Callback::from(move |event| {
               match event {
                   ChangeData::Select(elem) => {
                       let index = elem.selected_index() as usize;
                       set_selected_index(Some(index));
                   }
                   _ => {
                       unreachable!();
                   }
               }
            })
           }
        >
            {for options.iter().map(view_option)}
        </select>
    }
}
