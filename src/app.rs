use dioxus::prelude::*;

use crate::ui::{LibraryPage, Theme};

#[component]
pub fn App() -> Element {
    let mut theme = use_signal(|| Theme::Dark);

    rsx! {
        link { rel: "stylesheet", href: asset!("/assets/app.css") }
        div {
            class: "ny-app",
            "data-theme": "{theme.read().as_attr()}",
            LibraryPage {}
        }
    }
}
