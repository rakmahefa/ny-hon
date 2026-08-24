use dioxus::prelude::*;

use crate::ui::components::{
    badge::Badge,
    button::{Button, ButtonVariant},
    cover::Cover,
    empty_state::EmptyState,
    progress::Progress,
    section_header::SectionHeader,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum HistoryFilter {
    All,
    InProgress,
    Completed,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum HistoryState {
    Ready,
    Loading,
    Empty,
    Error,
    Offline,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct HistoryItem {
    title: &'static str,
    kind: &'static str,
    chapter: &'static str,
    read_at: &'static str,
    progress: u8,
    initials: &'static str,
}

const HISTORY: [HistoryItem; 6] = [
    HistoryItem {
        title: "Frieren",
        kind: "Manga",
        chapter: "Chapter 141",
        read_at: "12 min ago",
        progress: 82,
        initials: "FR",
    },
    HistoryItem {
        title: "Solo Leveling",
        kind: "Manga",
        chapter: "Chapter 179",
        read_at: "48 min ago",
        progress: 91,
        initials: "SL",
    },
    HistoryItem {
        title: "Blue Lock",
        kind: "Manga",
        chapter: "Chapter 304",
        read_at: "2 hours ago",
        progress: 100,
        initials: "BL",
    },
    HistoryItem {
        title: "The Sandman",
        kind: "Comic",
        chapter: "Issue 18",
        read_at: "Yesterday",
        progress: 37,
        initials: "SM",
    },
    HistoryItem {
        title: "Monstress",
        kind: "Comic",
        chapter: "Issue 56",
        read_at: "Yesterday",
        progress: 12,
        initials: "MO",
    },
    HistoryItem {
        title: "The Beginning After the End",
        kind: "Webtoon",
        chapter: "Chapter 233",
        read_at: "2 days ago",
        progress: 64,
        initials: "TB",
    },
];

fn matches_filter(item: HistoryItem, filter: HistoryFilter, query: &str) -> bool {
    let matches_filter = match filter {
        HistoryFilter::All => true,
        HistoryFilter::InProgress => item.progress > 0 && item.progress < 100,
        HistoryFilter::Completed => item.progress == 100,
    };

    let query = query.trim().to_lowercase();
    let matches_query = query.is_empty()
        || item.title.to_lowercase().contains(&query)
        || item.kind.to_lowercase().contains(&query)
        || item.chapter.to_lowercase().contains(&query);

    matches_filter && matches_query
}

#[derive(Props, Clone, PartialEq)]
pub struct HistoryPageProps {
    #[props(default = HistoryState::Ready)]
    pub state: HistoryState,
}

#[component]
pub fn HistoryPage(props: HistoryPageProps) -> Element {
    let mut filter = use_signal(|| HistoryFilter::All);
    let mut search = use_signal(String::new);
    let mut state = use_signal(|| props.state);
    let mut clear_requested = use_signal(|| false);

    let query = search.read().clone();
    let visible: Vec<HistoryItem> = HISTORY
        .iter()
        .copied()
        .filter(|item| matches_filter(*item, *filter.read(), &query))
        .collect();
    let current_state = *state.read();

    if *clear_requested.read() {
        return rsx! {
            section { class: "ny-page ny-history-page",
                EmptyState {
                    eyebrow: "HISTORY CLEARED".to_string(),
                    title: "Your reading history is empty".to_string(),
                    description: "The clear action is kept at the UI boundary until history persistence is provided by the application layer.".to_string(),
                }
            }
        };
    }

    rsx! {
        section { class: "ny-page ny-history-page",
            SectionHeader {
                eyebrow: "HISTORY".to_string(),
                title: "Keep reading where you left off".to_string(),
                description: Some("Review recent reading activity, resume unfinished chapters, and manage your history without coupling the UI to persistence.".to_string()),
            }

            div { class: "ny-library-toolbar",
                div { class: "ny-filter-group",
                    button {
                        class: if *filter.read() == HistoryFilter::All { "ny-filter is-active" } else { "ny-filter" },
                        onclick: move |_| filter.set(HistoryFilter::All),
                        "All"
                    }
                    button {
                        class: if *filter.read() == HistoryFilter::InProgress { "ny-filter is-active" } else { "ny-filter" },
                        onclick: move |_| filter.set(HistoryFilter::InProgress),
                        "In progress"
                    }
                    button {
                        class: if *filter.read() == HistoryFilter::Completed { "ny-filter is-active" } else { "ny-filter" },
                        onclick: move |_| filter.set(HistoryFilter::Completed),
                        "Completed"
                    }
                }

                div { class: "ny-library-toolbar__actions",
                    div { class: "ny-search",
                        span { class: "ny-search__icon", "⌕" }
                        input {
                            value: "{search}",
                            placeholder: "Search history…",
                            aria_label: "Search reading history",
                            oninput: move |event| search.set(event.value()),
                        }
                    }
                    Button {
                        variant: ButtonVariant::Secondary,
                        label: "Clear history",
                        onclick: move |_| clear_requested.set(true),
                    }
                    Button {
                        variant: ButtonVariant::Secondary,
                        label: "Refresh",
                        onclick: move |_| state.set(HistoryState::Ready),
                    }
                }
            }

            match current_state {
                HistoryState::Loading => rsx! {
                    EmptyState {
                        eyebrow: "LOADING".to_string(),
                        title: "Loading reading history".to_string(),
                        description: "The history state is ready for asynchronous application data.".to_string(),
                    }
                },
                HistoryState::Error => rsx! {
                    EmptyState {
                        eyebrow: "ERROR".to_string(),
                        title: "History could not be loaded".to_string(),
                        description: "Retry behavior stays at the UI boundary while persistence belongs to application services.".to_string(),
                        action_label: Some("Retry".to_string()),
                        onclick: Some(EventHandler::new(move |_| state.set(HistoryState::Ready))),
                    }
                },
                HistoryState::Offline => rsx! {
                    EmptyState {
                        eyebrow: "OFFLINE".to_string(),
                        title: "Using locally available history".to_string(),
                        description: "Recent reading activity remains visible without requiring remote source access.".to_string(),
                    }
                },
                HistoryState::Empty => rsx! {
                    EmptyState {
                        eyebrow: "EMPTY".to_string(),
                        title: "No reading history yet".to_string(),
                        description: "Open a chapter from your library and Ny Hon will expose the resume workflow here.".to_string(),
                    }
                },
                HistoryState::Ready => {
                    if visible.is_empty() {
                        rsx! {
                            EmptyState {
                                eyebrow: "NO MATCHES".to_string(),
                                title: "Nothing matches this view".to_string(),
                                description: "Try another search or history filter.".to_string(),
                            }
                        }
                    } else {
                        rsx! {
                            section { class: "ny-library-collection ny-library-collection--list",
                                for item in visible {
                                    {
                                        rsx! {
                                            article { class: "ny-library-item",
                                                Cover {
                                                    title: item.title.to_string(),
                                                    subtitle: Some(item.chapter.to_string()),
                                                    aspect: "ny-cover--compact".to_string(),
                                                }
                                                div { class: "ny-library-item__content",
                                                    div { class: "ny-library-item__meta",
                                                        Badge { label: item.kind.to_string() }
                                                        span { "Read {item.read_at}" }
                                                    }
                                                    h3 { "{item.title}" }
                                                    p { "{item.chapter} · {item.progress}% complete" }
                                                    Progress { value: item.progress }
                                                    div { class: "ny-library-item__footer",
                                                        span { class: "ny-library-item__initials", "{item.initials}" }
                                                        div { class: "ny-library-toolbar__actions",
                                                            Button { variant: ButtonVariant::Ghost, label: "Remove" }
                                                            Button { variant: ButtonVariant::Primary, label: "Resume" }
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
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn history_filter_matches_expected_progress_states() {
        assert!(matches_filter(HISTORY[0], HistoryFilter::InProgress, "frieren"));
        assert!(matches_filter(HISTORY[2], HistoryFilter::Completed, "blue"));
        assert!(!matches_filter(HISTORY[0], HistoryFilter::Completed, "frieren"));
        assert!(!matches_filter(HISTORY[0], HistoryFilter::All, "japanese"));
    }
}
