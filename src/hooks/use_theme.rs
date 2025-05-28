use web_sys::window;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum Theme {
    Light,
    Dark,
}

#[hook]
pub fn use_theme() -> (Theme, Callback<()>) {
    let theme = use_state(|| {
        let window = window().unwrap();
        let document = window.document().unwrap();
        let storage = window.local_storage().unwrap().unwrap();

        let stored_theme = storage.get_item("theme").unwrap_or(None);
        let initial_theme = match stored_theme.as_deref() {
            Some("dark") => Theme::Dark,
            _ => Theme::Light,
        };

        let root = document.document_element().unwrap();
        root.set_attribute(
            "data-theme",
            match initial_theme {
                Theme::Light => "light",
                Theme::Dark => "dark",
            },
        )
        .unwrap();

        initial_theme
    });

    let toggle_theme = {
        let theme = theme.clone();
        Callback::from(move |_| {
            let new_theme = match *theme {
                Theme::Light => Theme::Dark,
                Theme::Dark => Theme::Light,
            };

            let window = window().unwrap();
            let document = window.document().unwrap();
            let storage = window.local_storage().unwrap().unwrap();

            let root = document.document_element().unwrap();
            let theme_str = match new_theme {
                Theme::Light => "light",
                Theme::Dark => "dark",
            };

            root.set_attribute("data-theme", theme_str).unwrap();
            storage.set_item("theme", theme_str).unwrap();

            theme.set(new_theme);
        })
    };

    ((*theme).clone(), toggle_theme)
}
