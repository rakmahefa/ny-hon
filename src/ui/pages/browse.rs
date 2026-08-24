use dioxus::prelude::*;

use crate::ui::components::{
    badge::Badge,
    button::{Button, ButtonVariant},
    card::Card,
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
struct Source {
    name: &'static str,
    kind: &'static str,
    language: &'static str,
    installed: bool,
    enabled: bool,
}

const SOURCES: [Source; 6] = [
    Source { name: "MangaDex", kind: "Manga", language: "English", installed: true, enabled: true },
    Source { name: "Webtoon", kind: "Webtoon", language: "English", installed: true, enabled: true },
    Source { name: "MangaFire", kind: "Manga", language: "English", installed: true, enabled: true },
    Source { name: "Comick", kind: "Manga", language: "English", installed: false, enabled: false },
    Source { name: "Aniyomi Extensions", kind: "Extensions", language: "Multiple", installed: false, enabled: false },
    Source { name: "Local Files", kind: "Local", language: "Filesystem", installed: true, enabled: true },
];

#[derive(Props, Clone, PartialEq)]
pub struct BrowsePageProps {
    #[props(default = BrowseState::Ready)]
    pub state: BrowseState,
}

fn source_matches(source: Source, query: &str) -> bool {
    let query = query.trim().to_lowercase();
    query.is_empty()
        || source.name.to_lowercase().contains(&query)
        || source.kind.to_lowercase().contains(&query)
        || source.language.to_lowercase().contains(&query)
}

#[component]
pub fn BrowsePage(props: BrowsePageProps) -> Element {
    let mut search = use_signal(String::new);
    let mut global_search = use_signal(String::new);
    let mut state = use_signal(|| props.state);
    let mut enabled = use_signal(|| {
        SOURCES
            .iter()
            .filter(|source| source.enabled)
            .map(|source| source.name)
            .collect::<Vec<_>>()
    });

    let query = search.read().clone();
    let visible: Vec<Source> = SOURCES
        .iter()
        .copied()
        .filter(|source| source_matches(*source, &query))
        .collect();
    let enabled_count = enabled.read().len();
    let has_global_query = !global_search.read().trim().is_empty();

    match *state.read() {
        BrowseState::Loading => return rsx! {
            section { class: "ny-page ny-browse-page",
                EmptyState {
                    eyebrow: "BROWSE".to_string(),
                    title: "Loading sources".to_string(),
                    description: "Source discovery is represented by an explicit UI state; the actual source registry belongs outside the GUI layer.".to_string(),
                }
            }
        },
        BrowseState::Error => return rsx! {
            section { class: "ny-page ny-browse-page",
                EmptyState {
                    eyebrow: "ERROR".to_string(),
                    title: "Sources could not be loaded".to_string(),
                    description: "The page keeps retry behavior at its event boundary without owning networking or extensions.".to_string(),
                    action_label: Some("Retry".to_string()),
                    onclick: Some(EventHandler::new(move |_| state.set(BrowseState::Ready))),
                }
            }
        },
        BrowseState::Offline => return rsx! {
            section { class: "ny-page ny-browse-page",
                EmptyState {
                    eyebrow: "OFFLINE".to_string(),
                    title: "Browse is offline".to_string(),
                    description: "Installed local sources remain visible while remote discovery is unavailable.".to_string(),
                }
            }
        },
        BrowseState::Empty => return rsx! {
            section { class: "ny-page ny-browse-page",
                EmptyState {
                    eyebrow: "NO SOURCES".to_string(),
                    title: "No sources installed".to_string(),
                    description: "Install a source extension to begin global manga, webtoon or comic discovery.".to_string(),
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
                description: Some("Search across enabled sources, manage extensions, and prepare titles for library migration.".to_string()),
            }

            div { class: "ny-library-toolbar",
                div { class: "ny-search",
                    span { class: "ny-search__icon", "⌕" }
                    input {
                        value: "{search}",
                        placeholder: "Search sources…",
                        aria_label: "Search sources",
                        oninput: move |event| search.set(event.value()),
                    }
                }
                div { class: "ny-library-toolbar__actions",
                    Button {
                        variant: ButtonVariant::Secondary,
                        label: format!("Refresh · {enabled_count} enabled"),
                        onclick: move |_| state.set(BrowseState::Ready),
                    }
                    Button { variant: ButtonVariant::Secondary, label: "Extensions" }
                }
            }

            section { class: "ny-dashboard-grid",
                Card { title: Some("Global search".to_string()), class: "ny-browse-global-search".to_string(),
                    p { class: "ny-browse-lead", "Search the enabled source registry and inspect results before adding a title to the library." }
                    div { class: "ny-search ny-browse-search",
                        span { class: "ny-search__icon", "⌕" }
                        input {
                            value: "{global_search}",
                            placeholder: "Search manga, webtoons, comics…",
                            aria_label: "Global search",
                            oninput: move |event| global_search.set(event.value()),
                        }
                        Button { variant: ButtonVariant::Primary, label: "Search" }
                    }
                    if has_global_query {
                        div { class: "ny-browse-search-status",
                            span { class: "ny-eyebrow", "READY" }
                            span { "Search request prepared for the source/application layer." }
                        }
                    }
                    div { class: "ny-filter-group ny-browse-tags",
                        Badge { label: "Manga" }
                        Badge { label: "Webtoon" }
                        Badge { label: "Comic" }
                        Badge { label: "Multiple languages" }
                    }
                }
                Card { title: Some("Migration".to_string()), class: "ny-browse-migration".to_string(),
                    strong { "Move a title between sources" }
                    p { "The migration workflow will compare matching titles and expose a source-safe transfer action." }
                    Button { variant: ButtonVariant::Ghost, label: "Open migration" }
                }
            }

            if visible.is_empty() {
                EmptyState {
                    eyebrow: "NO MATCHES".to_string(),
                    title: "No source matches".to_string(),
                    description: "Try a different source, language, or extension name.".to_string(),
                }
            } else {
                section { class: "ny-source-grid",
                    for source in visible {
                        {
                            let is_enabled = enabled.read().contains(&source.name);
                            rsx! {
                                article { class: "ny-card ny-source-card",
                                    div { class: "ny-source-card__head",
                                        div {
                                            span { class: "ny-source-card__mark", "{source.name.chars().next().unwrap_or('N')}" }
                                            div {
                                                h3 { "{source.name}" }
                                                p { "{source.kind} · {source.language}" }
                                            }
                                        }
                                        Badge { label: if is_enabled { "Enabled" } else if source.installed { "Installed" } else { "Available" } }
                                    }
                                    div { class: "ny-source-card__body",
                                        div { class: "ny-source-card__meta",
                                            span { "{source.kind}" }
                                            span { "{source.language}" }
                                        }
                                        div { class: "ny-source-card__actions",
                                            if source.installed {
                                                Button {
                                                    variant: if is_enabled { ButtonVariant::Secondary } else { ButtonVariant::Ghost },
                                                    label: if is_enabled { "Disable".to_string() } else { "Enable".to_string() },
                                                    onclick: move |_| {
                                                        let mut sources = enabled.write();
                                                        if let Some(index) = sources.iter().position(|name| *name == source.name) {
                                                            sources.remove(index);
                                                        } else {
                                                            sources.push(source.name);
                                                        }
                                                    },
                                                }
                                            } else {
                                                Button { variant: ButtonVariant::Primary, label: "Install" }
                                            }
                                            Button { variant: ButtonVariant::Ghost, label: "Browse" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_search_matches_name_kind_and_language() {
        assert!(source_matches(SOURCES[0], "mangadex"));
        assert!(source_matches(SOURCES[0], "manga"));
        assert!(source_matches(SOURCES[0], "english"));
        assert!(!source_matches(SOURCES[0], "japanese"));
    }
}
