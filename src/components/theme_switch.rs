use crate::hooks::use_theme::use_theme;
use yew::prelude::*;

#[function_component(ThemeSwitch)]
pub fn theme_switch() -> Html {
    let (_theme, toggle_theme) = use_theme();

    html! {
        <button
            class="theme-switch"
            onclick={Callback::from(move |_| toggle_theme.emit(()))}
            aria-label="Toggle theme"
        >
            <span class="theme-icon sun">{"☀️"}</span>
            <span class="theme-icon moon">{"🌙"}</span>
            <span class="theme-toggle"></span>
        </button>
    }
}
