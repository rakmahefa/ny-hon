use dioxus::prelude::*;

use crate::ui::{
    AppShell, EmptyState, HomePage, LibraryPage, MangaDetailPage, NavItem, Theme, UpdatesPage,
};

#[component]
pub fn App() -> Element {
    let mut theme = use_signal(|| Theme::Dark);
    let mut active = use_signal(|| NavItem::Library);
    let mut detail_title = use_signal(|| None::<String>);

    let title = if detail_title.read().is_some() {
        "Manga detail".to_string()
    } else {
        active.read().label().to_string()
    };

    rsx! {
        link { rel: "stylesheet", href: asset!("/assets/app.css") }
        link { rel: "stylesheet", href: asset!("/assets/manga-detail.css") }
        div {
            class: "ny-app",
            "data-theme": "{theme.read().as_attr()}",
            AppShell {
                theme: *theme.read(),
                active: *active.read(),
                title: title,
                on_navigate: move |item| {
                    detail_title.set(None);
                    active.set(item);
                },
                on_toggle_theme: move |_| {
                    let next = if *theme.read() == Theme::Dark { Theme::Light } else { Theme::Dark };
                    theme.set(next);
                },
                if let Some(selected_title) = detail_title.read().clone() {
                    rsx! {
                        MangaDetailPage {
                            title: selected_title,
                            on_back: move |_| detail_title.set(None),
                            on_open_reader: move |_| {},
                            on_download: move |_| {},
                            on_toggle_library: move |_| {},
                            on_refresh: move |_| {},
                        }
                    }
                } else {
                    match *active.read() {
                        NavItem::Library => rsx! {
                            LibraryPage {
                                on_open: Some(EventHandler::new(move |item: String| detail_title.set(Some(item)))),
                            }
                        },
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
}
