use dioxus::prelude::*;

use crate::ui::{AppShell, EmptyState, HomePage, LibraryPage, NavItem, Theme, UpdatesPage};

#[component]
pub fn App() -> Element {
    let mut theme = use_signal(|| Theme::Dark);
    let mut active = use_signal(|| NavItem::Library);

    let title = active.read().label();

    rsx! {
        link { rel: "stylesheet", href: asset!("/assets/app.css") }
        div {
            class: "ny-app",
            "data-theme": "{theme.read().as_attr()}",
            AppShell {
                theme: *theme.read(),
                active: *active.read(),
                title: title.to_string(),
                on_navigate: move |item| active.set(item),
                on_toggle_theme: move |_| {
                    let next = if *theme.read() == Theme::Dark { Theme::Light } else { Theme::Dark };
                    theme.set(next);
                },
                match *active.read() {
                    NavItem::Library => rsx! { LibraryPage {} },
                    NavItem::Updates => rsx! { UpdatesPage {} },
                    NavItem::History => rsx! {
                        EmptyState {
                            eyebrow: "HISTORY".to_string(),
                            title: "Reading history".to_string(),
                            description: "The history surface is ready for persisted reading activity, resume actions and search.".to_string(),
                        }
                    },
                    NavItem::Browse => rsx! { HomePage {} },
                    NavItem::More => rsx! {
                        EmptyState {
                            eyebrow: "MORE".to_string(),
                            title: "More library tools".to_string(),
                            description: "Settings, downloads, categories, tracking and backup will live behind this secondary destination.".to_string(),
                        }
                    },
                }
            }
        }
    }
}
