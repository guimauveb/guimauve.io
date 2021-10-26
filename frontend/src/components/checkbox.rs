use {
    yew::{html, Callback, ChangeData, Properties},
    yew_functional::function_component,
};

#[derive(Properties, Clone, PartialEq)]
pub struct CheckBoxProps {
    pub checked: bool,
    pub onchange: Callback<bool>,
}

#[function_component(CheckBox)]
pub fn checkbox(CheckBoxProps { checked, onchange }: &CheckBoxProps) -> Html {
    let (checked, onchange) = (*checked, onchange.clone());
    let on_checkbox_state_change: Callback<ChangeData> =
        Callback::from(move |event: ChangeData| {
            if let ChangeData::Value(_) = event {
                onchange.emit(!checked)
            }
        });

    html! {
        <input
            class="checkbox"
            checked={checked}
            onchange={on_checkbox_state_change}
            type="checkbox"
        />
    }
}
