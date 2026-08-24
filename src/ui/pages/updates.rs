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
pub enum UpdatesFilter {
    All,
    Unread,
    Downloaded,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum UpdatesState {
    Ready,
    Loading,
    Empty,
    Error,
    Offline,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct UpdateItem {
    title: &'static str,
    source: &'static str,
    chapter: &'static str,
    released: &'static str,
    progress: u8,
    downloaded: bool,
    initials: &'static str,
}

const UPDATES: [UpdateItem; 5] = [
    UpdateItem { title: "Frieren", source: "MangaDex", chapter: "Chapter 142", released: "12 min ago", progress: 82, downloaded: false, initials: "FR" },
    UpdateItem { title: "Solo Leveling", source: "MangaFire", chapter: "Chapter 180", released: "48 min ago", progress: 91, downloaded: true, initials: "SL" },
    UpdateItem { title: "Blue Lock", source: "MangaDex", chapter: "Chapter 305", released: "2 hours ago", progress: 100, downloaded: false, initials: "BL" },
    UpdateItem { title: "The Beginning After the End", source: "Webtoon", chapter: "Chapter 234", released: "Yesterday", progress: 64, downloaded: true, initials: "TB" },
    UpdateItem { title: "Monstress", source: "Comic Source", chapter: "Issue 57", released: "Yesterday", progress: 12, downloaded: false, initials: "MO" },
];

fn matches_filter(item: UpdateItem, filter: UpdatesFilter, query: &str) -> bool {
    let matches_filter = match filter {
        UpdatesFilter::All => true,
        UpdatesFilter::Unread => item.progress < 100,
        UpdatesFilter::Downloaded => item.downloaded,
    };
    let query = query.trim().to_lowercase();
    let matches_query = query.is_empty()
        || item.title.to_lowercase().contains(&query)
        || item.source.to_lowercase().contains(&query)
        || item.chapter.to_lowercase().contains(&query);

    matches_filter && matches_query
}

#[derive(Props, Clone, PartialEq)]
pub struct UpdatesPageProps {
    #[props(default = UpdatesState::Ready)]
    pub state: UpdatesState,
}

#[component]
pub fn UpdatesPage(props: UpdatesPageProps) -> Element {
    let mut filter = use_signal(|| UpdatesFilter::All);
    let mut search = use_signal(String::new);
    let mut selection_mode = use_signal(|| false);
    let mut selected = use_signal(Vec::<&'static str>::new);
    let mut state = use_signal(|| props.state);

    let query = search.read().clone();
    let visible: Vec<UpdateItem> = UPDATES
        .iter()
        .copied()
        .filter(|item| matches_filter(*item, *filter.read(), &query))
        .collect();
    let selected_count = selected.read().len();
    let current_state = *state.read();

    rsx! {
        section { class: "ny-page ny-updates-page",
            SectionHeader {
                eyebrow: "UPDATES".to_string(),
                title: "New chapters".to_string(),
                description: Some("Review the latest releases, act on multiple chapters at once, and keep upcoming releases visible.".to_string()),
            }

            div { class: "ny-library-toolbar",
                div { class: "ny-filter-group",
                    button {
                        class: if *filter.read() == UpdatesFilter::All { "ny-filter is-active" } else { "ny-filter" },
                        onclick: move |_| filter.set(UpdatesFilter::All),
                        "All"
                    }
                    button {
                        class: if *filter.read() == UpdatesFilter::Unread { "ny-filter is-active" } else { "ny-filter" },
                        onclick: move |_| filter.set(UpdatesFilter::Unread),
                        "Unread"
                    }
                    button {
                        class: if *filter.read() == UpdatesFilter::Downloaded { "ny-filter is-active" } else { "ny-filter" },
                        onclick: move |_| filter.set(UpdatesFilter::Downloaded),
                        "Downloaded"
                    }
                }

                div { class: "ny-library-toolbar__actions",
                    div { class: "ny-search",
                        span { class: "ny-search__icon", "⌕" }
                        input {
                            value: "{search}",
                            placeholder: "Search updates…",
                            aria_label: "Search updates",
                            oninput: move |event| search.set(event.value()),
                        }
                    }
                    Button {
                        variant: ButtonVariant::Secondary,
                        label: if *selection_mode.read() { format!("Done ({selected_count})") } else { "Select".to_string() },
                        onclick: move |_| {
                            let next = !*selection_mode.read();
                            selection_mode.set(next);
                            if !next {
                                selected.set(Vec::new());
                            }
                        },
                    }
                    Button {
                        variant: ButtonVariant::Secondary,
                        label: "Refresh",
                        onclick: move |_| state.set(UpdatesState::Ready),
                    }
                }
            }

            if *selection_mode.read() && selected_count > 0 {
                div { class: "ny-library-toolbar",
                    div { class: "ny-filter-group",
                        span { class: "ny-eyebrow", "SELECTION" }
                        span { "{selected_count} chapters selected" }
                    }
                    div { class: "ny-library-toolbar__actions",
                        Button { variant: ButtonVariant::Secondary, label: "Download" }
                        Button { variant: ButtonVariant::Ghost, label: "Mark read" }
                        Button {
                            variant: ButtonVariant::Ghost,
                            label: "Clear",
                            onclick: move |_| selected.set(Vec::new()),
                        }
                    }
                }
            }

            match current_state {
                UpdatesState::Loading => rsx! {
                    EmptyState {
                        eyebrow: "LOADING".to_string(),
                        title: "Checking for new chapters".to_string(),
                        description: "The UI exposes the loading boundary while source discovery remains outside the GUI layer.".to_string(),
                    }
                },
                UpdatesState::Error => rsx! {
                    EmptyState {
                        eyebrow: "ERROR".to_string(),
                        title: "Updates could not be loaded".to_string(),
                        description: "Retrying stays at the UI boundary; source networking belongs to application services.".to_string(),
                        action_label: Some("Retry".to_string()),
                        onclick: Some(EventHandler::new(move |_| state.set(UpdatesState::Ready))),
                    }
                },
                UpdatesState::Offline => rsx! {
                    EmptyState {
                        eyebrow: "OFFLINE".to_string(),
                        title: "Using the last known updates".to_string(),
                        description: "The offline state remains explicit without making the UI responsible for synchronization.".to_string(),
                    }
                },
                UpdatesState::Empty => rsx! {
                    EmptyState {
                        eyebrow: "UP TO DATE".to_string(),
                        title: "No new chapters".to_string(),
                        description: "Your followed titles are currently up to date.".to_string(),
                    }
                },
                UpdatesState::Ready => {
                    if visible.is_empty() {
                        rsx! {
                            EmptyState {
                                eyebrow: "NO MATCHES".to_string(),
                                title: "Nothing matches this view".to_string(),
                                description: "Try another search or filter to find a recent release.".to_string(),
                            }
                        }
                    } else {
                        rsx! {
                            section { class: "ny-library-collection ny-library-collection--list",
                                for item in visible {
                                    {
                                        let is_selected = selected.read().contains(&item.title);
                                        let download_status = if item.downloaded { "Downloaded" } else { "Not downloaded" };
                                        rsx! {
                                            article {
                                                class: if is_selected { "ny-library-item is-selected" } else { "ny-library-item" },
                                                onclick: move |_| {
                                                    if !*selection_mode.read() {
                                                        return;
                                                    }
                                                    let mut items = selected.write();
                                                    if let Some(index) = items.iter().position(|title| *title == item.title) {
                                                        items.remove(index);
                                                    } else {
                                                        items.push(item.title);
                                                    }
                                                },
                                                Cover {
                                                    title: item.title.to_string(),
                                                    subtitle: Some(item.chapter.to_string()),
                                                    aspect: "ny-cover--compact".to_string(),
                                                }
                                                div { class: "ny-library-item__content",
                                                    div { class: "ny-library-item__meta",
                                                        Badge { label: item.source.to_string() }
                                                        span { "{item.released}" }
                                                    }
                                                    h3 { "{item.title}" }
                                                    p { "{item.chapter} · {download_status}" }
                                                    Progress { value: item.progress }
                                                    div { class: "ny-library-item__footer",
                                                        span { class: "ny-library-item__initials", "{item.initials}" }
                                                        if *selection_mode.read() {
                                                            span { class: "ny-eyebrow", if is_selected { "SELECTED" } else { "SELECT" } }
                                                        } else {
                                                            div { class: "ny-library-toolbar__actions",
                                                                Button { variant: ButtonVariant::Ghost, label: "Read" }
                                                                Button { variant: ButtonVariant::Secondary, label: if item.downloaded { "Downloaded".to_string() } else { "Download".to_string() } }
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
                },
            }

            section { class: "ny-card",
                h2 { class: "ny-card__title", "Upcoming releases" }
                div { class: "ny-card__body",
                    div { class: "ny-release-list",
                        div { class: "ny-release-row",
                            div {
                                strong { "Frieren" }
                                span { "Chapter 143" }
                            }
                            time { "Tomorrow" }
                        }
                        div { class: "ny-release-row",
                            div {
                                strong { "Blue Lock" }
                                span { "Chapter 306" }
                            }
                            time { "Thu, 27 Aug" }
                        }
                        div { class: "ny-release-row",
                            div {
                                strong { "Monstress" }
                                span { "Issue 58" }
                            }
                            time { "Fri, 28 Aug" }
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
    fn unread_filter_excludes_completed_updates() {
        let completed = UpdateItem { progress: 100, ..UPDATES[0] };
        assert!(!matches_filter(completed, UpdatesFilter::Unread, ""));
        assert!(matches_filter(UPDATES[0], UpdatesFilter::Unread, ""));
    }

    #[test]
    fn downloaded_filter_matches_only_downloaded_items() {
        assert!(matches_filter(UPDATES[1], UpdatesFilter::Downloaded, ""));
        assert!(!matches_filter(UPDATES[0], UpdatesFilter::Downloaded, ""));
    }

    #[test]
    fn search_matches_title_source_and_chapter() {
        assert!(matches_filter(UPDATES[0], UpdatesFilter::All, "frieren"));
        assert!(matches_filter(UPDATES[0], UpdatesFilter::All, "mangadex"));
        assert!(matches_filter(UPDATES[0], UpdatesFilter::All, "142"));
        assert!(!matches_filter(UPDATES[0], UpdatesFilter::All, "one piece"));
    }
}
