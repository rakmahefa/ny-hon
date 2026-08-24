use dioxus::prelude::*;

use crate::ui::components::{
    badge::Badge,
    button::{Button, ButtonVariant},
    card::Card,
    cover::Cover,
    empty_state::EmptyState,
    section_header::SectionHeader,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BrowseState {
    Ready,
    Loading,
    Empty,
    Error,
    Offline,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum BrowseTab {
    Sources,
    Extensions,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct SourceItem {
    name: &'static str,
    kind: &'static str,
    status: &'static str,
    title_count: u32,
    enabled: bool,
}

const SOURCES: [SourceItem; 5] = [
    SourceItem { name: "MangaDex", kind: "Manga", status: "Ready", title_count: 42000, enabled: true },
    SourceItem { name: "Webtoon", kind: "Webtoon", status: "Ready", title_count: 12500, enabled: true },
    SourceItem { name: "MangaFire", kind: "Manga", status: "Ready", title_count: 18400, enabled: true },
    SourceItem { name: "Comic Source", kind: "Comic", status: "Limited", title_count: 3200, enabled: false },
    SourceItem { name: "Local", kind: "Local", status: "Offline-safe", title_count: 128, enabled: true },
];

fn matches_source(source: SourceItem, query: &str) -> bool {
    let query = query.trim().to_lowercase();
    query.is_empty()
        || source.name.to_lowercase().contains(&query)
        || source.kind.to_lowercase().contains(&query)
        || source.status.to_lowercase().contains(&query)
}

#[derive(Props, Clone, PartialEq)]
pub struct BrowsePageProps {
    #[props(default = BrowseState::Ready)]
    pub state: BrowseState,
    #[props(default)]
    pub on_open_source: Option<EventHandler<String>>,
    #[props(default)]
    pub on_global_search: Option<EventHandler<String>>,
}

#[component]
pub fn BrowsePage(props: BrowsePageProps) -> Element {
    let mut tab = use_signal(|| BrowseTab::Sources);
    let mut search = use_signal(String::new);
    let mut source_query = use_signal(String::new);

    let visible_sources: Vec<SourceItem> = SOURCES
        .iter()
        .copied()
        .filter(|source| matches_source(*source, &source_query.read()))
        .collect();

    let global_search = search.read().clone();
    let on_open_source = props.on_open_source.clone();
    let on_global_search = props.on_global_search.clone();

    match props.state {
        BrowseState::Loading => return rsx! {
            section { class: "ny-page ny-browse-page",
                EmptyState {
                    eyebrow: "BROWSE".to_string(),
                    title: "Loading sources".to_string(),
                    description: "Source discovery remains outside the GUI layer.".to_string(),
                }
            }
        },
        BrowseState::Error => return rsx! {
            section { class: "ny-page ny-browse-page",
                EmptyState {
                    eyebrow: "BROWSE".to_string(),
                    title: "Sources unavailable".to_string(),
                    description: "Retry is exposed through the UI boundary while source services own the actual refresh.".to_string(),
                }
            }
        },
        BrowseState::Offline => return rsx! {
            section { class: "ny-page ny-browse-page",
                EmptyState {
                    eyebrow: "OFFLINE".to_string(),
                    title: "Local sources only".to_string(),
                    description: "Remote source discovery is unavailable; local content remains accessible.".to_string(),
                }
            }
        },
        BrowseState::Empty => return rsx! {
            section { class: "ny-page ny-browse-page",
                EmptyState {
                    eyebrow: "BROWSE".to_string(),
                    title: "No sources configured".to_string(),
                    description: "Configure source adapters through the application layer before exposing them here.".to_string(),
                }
            }
        },
        BrowseState::Ready => {}
    }

    rsx! {
        section { class: "ny-page ny-browse-page",
            SectionHeader {
                eyebrow: "BROWSE".to_string(),
                title: "Discover your next series".to_string(),
                description: Some("Browse configured sources, inspect extensions, search globally, and keep source management outside the presentation layer.".to_string()),
            }

            Card { class: "ny-browse-search-card".to_string(),
                div { class: "ny-browse-search",
                    span { class: "ny-search__icon", "⌕" }
                    input {
                        value: "{search}",
                        placeholder: "Search across sources…",
                        aria_label: "Global source search",
                        oninput: move |event| search.set(event.value()),
                    }
                    Button {
                        variant: ButtonVariant::Primary,
                        label: "Search",
                        onclick: move |_| {
                            if let Some(handler) = &on_global_search {
                                handler.call(global_search.clone());
                            }
                        },
                    }
                }
            }

            div { class: "ny-browse-tabs",
                button {
                    class: if *tab.read() == BrowseTab::Sources { "ny-filter is-active" } else { "ny-filter" },
                    onclick: move |_| tab.set(BrowseTab::Sources),
                    "Sources"
                }
                button {
                    class: if *tab.read() == BrowseTab::Extensions { "ny-filter is-active" } else { "ny-filter" },
                    onclick: move |_| tab.set(BrowseTab::Extensions),
                    "Extensions"
                }
            }

            match *tab.read() {
                BrowseTab::Sources => rsx! {
                    div { class: "ny-browse-toolbar",
                        div { class: "ny-filter-group",
                            Badge { label: "Sources".to_string() }
                            span { class: "ny-eyebrow", "{visible_sources.len()} available" }
                        }
                        div { class: "ny-search",
                            span { class: "ny-search__icon", "⌕" }
                            input {
                                value: "{source_query}",
                                placeholder: "Filter sources…",
                                aria_label: "Filter sources",
                                oninput: move |event| source_query.set(event.value()),
                            }
                        }
                    }
                    if visible_sources.is_empty() {
                        EmptyState {
                            eyebrow: "NO MATCHES".to_string(),
                            title: "No source matches".to_string(),
                            description: "Try another source name or type.".to_string(),
                        }
                    } else {
                        section { class: "ny-browse-source-grid",
                            for source in visible_sources {
                                Card { class: "ny-source-card".to_string(),
                                    div { class: "ny-source-card__top",
                                        Cover {
                                            title: source.name.to_string(),
                                            subtitle: Some(source.kind.to_string()),
                                            aspect: "ny-cover--compact".to_string(),
                                        }
                                        div { class: "ny-source-card__copy",
                                            div { class: "ny-library-item__meta",
                                                Badge { label: source.kind.to_string() }
                                                Badge { label: source.status.to_string() }
                                            }
                                            h3 { "{source.name}" }
                                            p { "{source.title_count} titles" }
                                        }
                                    }
                                    div { class: "ny-source-card__footer",
                                        span { class: "ny-eyebrow", if source.enabled { "ENABLED" } else { "DISABLED" } }
                                        Button {
                                            variant: if source.enabled { ButtonVariant::Secondary } else { ButtonVariant::Ghost },
                                            label: if source.enabled { "Browse".to_string() } else { "Enable".to_string() },
                                            onclick: {
                                                let handler = on_open_source.clone();
                                                let name = source.name.to_string();
                                                move |_| {
                                                    if let Some(handler) = &handler {
                                                        handler.call(name.clone());
                                                    }
                                                }
                                            },
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                BrowseTab::Extensions => rsx! {
                    section { class: "ny-browse-source-grid",
                        Card { class: "ny-extension-card".to_string(),
                            h2 { "Installed extensions" }
                            p { "Extensions are represented as deterministic UI contracts; installation and updates belong to application services." }
                            div { class: "ny-extension-row",
                                strong { "Core manga adapters" }
                                Badge { label: "3 installed".to_string() }
                            }
                            div { class: "ny-extension-row",
                                strong { "Community adapters" }
                                Badge { label: "Coming later".to_string() }
                            }
                        }
                        Card { class: "ny-extension-card".to_string(),
                            h2 { "Source migration" }
                            p { "Migration workflows can later map library titles between source adapters without coupling the GUI to source implementations." }
                            Button { variant: ButtonVariant::Secondary, label: "Open migration" }
                        }
                    }
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_search_matches_name_and_kind() {
        assert!(matches_source(SOURCES[0], "mangadex"));
        assert!(matches_source(SOURCES[1], "webtoon"));
        assert!(!matches_source(SOURCES[0], "comic"));
    }

    #[test]
    fn empty_query_keeps_all_sources() {
        assert_eq!(
            SOURCES
                .iter()
                .filter(|source| matches_source(**source, ""))
                .count(),
            SOURCES.len()
        );
    }
}
