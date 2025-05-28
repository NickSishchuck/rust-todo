use crate::components::theme_switch::ThemeSwitch;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub on_add_click: Callback<()>,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    let on_add_click = props.on_add_click.clone();

    html! {
        <header class="header">
            <h1 class="logo">{"TODO"}</h1>
            <ThemeSwitch />
            <button
                class="add-button"
                onclick={Callback::from(move |_| on_add_click.emit(()))}
            >
                <span class="add-icon">{"+"}</span>
                <span class="add-text">{"New"}</span>
            </button>
        </header>
    }
}
