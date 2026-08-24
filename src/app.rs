use dioxus::prelude::*;

use crate::ui::{HomePage, Theme};

#[component]
pub fn App() -> Element {
    let mut theme = use_signal(|| Theme::Dark);

    rsx! {
        link { rel: "stylesheet", href: asset!("/assets/app.css") }
        div {
            class: "ny-app",
            "data-theme": "{theme.read().as_attr()}",
            HomePage {
                theme: *theme.read(),
                on_toggle_theme: move |_| {
                    let next = if *theme.read() == Theme::Dark { Theme::Light } else { Theme::Dark };
                    theme.set(next);
                },
            }
        }
    }
}
