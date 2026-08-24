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
pub enum DownloadsFilter {
    All,
    Queued,
    Active,
    Completed,
    Failed,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DownloadsState {
    Ready,
    Loading,
    Empty,
    Error,
    Offline,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum DownloadStatus {
    Queued,
    Active,
    Completed,
    Failed,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct DownloadItem {
    title: &'static str,
    chapter: &'static str,
    source: &'static str,
    status: DownloadStatus,
    progress: u8,
    size: &'static str,
    initials: &'static str,
}

const DOWNLOADS: [DownloadItem; 6] = [
    DownloadItem { title: "Frieren", chapter: "Chapter 142", source: "MangaDex", status: DownloadStatus::Active, progress: 68, size: "31.4 MB", initials: "FR" },
    DownloadItem { title: "Solo Leveling", chapter: "Chapter 180", source: "MangaFire", status: DownloadStatus::Completed, progress: 100, size: "28.7 MB", initials: "SL" },
    DownloadItem { title: "Blue Lock", chapter: "Chapter 305", source: "MangaDex", status: DownloadStatus::Queued, progress: 0, size: "34.1 MB", initials: "BL" },
    DownloadItem { title: "The Beginning After the End", chapter: "Chapter 234", source: "Webtoon", status: DownloadStatus::Active, progress: 42, size: "22.6 MB", initials: "TB" },
    DownloadItem { title: "Monstress", chapter: "Issue 57", source: "Comic Source", status: DownloadStatus::Failed, progress: 17, size: "41.8 MB", initials: "MO" },
    DownloadItem { title: "The Sandman", chapter: "Issue 18", source: "Local Source", status: DownloadStatus::Queued, progress: 0, size: "19.3 MB", initials: "SM" },
];

fn status_label(status: DownloadStatus) -> &'static str {
    match status {
        DownloadStatus::Queued => "Queued",
        DownloadStatus::Active => "Downloading",
        DownloadStatus::Completed => "Completed",
        DownloadStatus::Failed => "Failed",
    }
}

fn matches_filter(item: DownloadItem, filter: DownloadsFilter, query: &str) -> bool {
    let matches_filter = match filter {
        DownloadsFilter::All => true,
        DownloadsFilter::Queued => item.status == DownloadStatus::Queued,
        DownloadsFilter::Active => item.status == DownloadStatus::Active,
        DownloadsFilter::Completed => item.status == DownloadStatus::Completed,
        DownloadsFilter::Failed => item.status == DownloadStatus::Failed,
    };
    let query = query.trim().to_lowercase();
    let matches_query = query.is_empty()
        || item.title.to_lowercase().contains(&query)
        || item.chapter.to_lowercase().contains(&query)
        || item.source.to_lowercase().contains(&query);

    matches_filter && matches_query
}

#[derive(Props, Clone, PartialEq)]
pub struct DownloadsPageProps {
    #[props(default = DownloadsState::Ready)]
    pub state: DownloadsState,
}

#[component]
pub fn DownloadsPage(props: DownloadsPageProps) -> Element {
    let mut filter = use_signal(|| DownloadsFilter::All);
    let mut search = use_signal(String::new);
    let mut selection_mode = use_signal(|| false);
    let mut selected = use_signal(Vec::<&'static str>::new);
    let mut state = use_signal(|| props.state);

    let query = search.read().clone();
    let visible: Vec<DownloadItem> = DOWNLOADS
        .iter()
        .copied()
        .filter(|item| matches_filter(*item, *filter.read(), &query))
        .collect();
    let selected_count = selected.read().len();
    let current_state = *state.read();

    rsx! {
        section { class: "ny-page ny-downloads-page",
            SectionHeader {
                eyebrow: "DOWNLOADS".to_string(),
                title: "Download queue".to_string(),
                description: Some("Track queued chapters, active transfers, completed files and recoverable failures in one place.".to_string()),
            }

            div { class: "ny-library-toolbar",
                div { class: "ny-filter-group",
                    button {
                        class: if *filter.read() == DownloadsFilter::All { "ny-filter is-active" } else { "ny-filter" },
                        onclick: move |_| filter.set(DownloadsFilter::All),
                        "All"
                    }
                    button {
                        class: if *filter.read() == DownloadsFilter::Queued { "ny-filter is-active" } else { "ny-filter" },
                        onclick: move |_| filter.set(DownloadsFilter::Queued),
                        "Queued"
                    }
                    button {
                        class: if *filter.read() == DownloadsFilter::Active { "ny-filter is-active" } else { "ny-filter" },
                        onclick: move |_| filter.set(DownloadsFilter::Active),
                        "Active"
                    }
                    button {
                        class: if *filter.read() == DownloadsFilter::Completed { "ny-filter is-active" } else { "ny-filter" },
                        onclick: move |_| filter.set(DownloadsFilter::Completed),
                        "Completed"
                    }
                    button {
                        class: if *filter.read() == DownloadsFilter::Failed { "ny-filter is-active" } else { "ny-filter" },
                        onclick: move |_| filter.set(DownloadsFilter::Failed),
                        "Failed"
                    }
                }

                div { class: "ny-library-toolbar__actions",
                    div { class: "ny-search",
                        span { class: "ny-search__icon", "⌕" }
                        input {
                            value: "{search}",
                            placeholder: "Search downloads…",
                            aria_label: "Search downloads",
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
                    Button { variant: ButtonVariant::Secondary, label: "Pause all" }
                    Button {
                        variant: ButtonVariant::Secondary,
                        label: "Refresh",
                        onclick: move |_| state.set(DownloadsState::Ready),
                    }
                }
            }

            if *selection_mode.read() && selected_count > 0 {
                div { class: "ny-library-toolbar",
                    div { class: "ny-filter-group",
                        span { class: "ny-eyebrow", "SELECTION" }
                        span { "{selected_count} downloads selected" }
                    }
                    div { class: "ny-library-toolbar__actions",
                        Button { variant: ButtonVariant::Secondary, label: "Resume" }
                        Button { variant: ButtonVariant::Ghost, label: "Pause" }
                        Button { variant: ButtonVariant::Ghost, label: "Retry" }
                        Button {
                            variant: ButtonVariant::Ghost,
                            label: "Clear",
                            onclick: move |_| selected.set(Vec::new()),
                        }
                    }
                }
            }

            match current_state {
                DownloadsState::Loading => rsx! {
                    EmptyState {
                        eyebrow: "LOADING".to_string(),
                        title: "Loading download queue".to_string(),
                        description: "The queue state is explicit while transfer management remains outside the GUI layer.".to_string(),
                    }
                },
                DownloadsState::Error => rsx! {
                    EmptyState {
                        eyebrow: "ERROR".to_string(),
                        title: "Downloads could not be loaded".to_string(),
                        description: "Retry stays at the UI boundary; queue persistence and transfer recovery belong to application services.".to_string(),
                        action_label: Some("Retry".to_string()),
                        onclick: Some(EventHandler::new(move |_| state.set(DownloadsState::Ready))),
                    }
                },
                DownloadsState::Offline => rsx! {
                    EmptyState {
                        eyebrow: "OFFLINE".to_string(),
                        title: "Download queue is offline".to_string(),
                        description: "The last known queue remains representable without allowing the UI to perform synchronization.".to_string(),
                    }
                },
                DownloadsState::Empty => rsx! {
                    EmptyState {
                        eyebrow: "QUEUE EMPTY".to_string(),
                        title: "Nothing is downloading".to_string(),
                        description: "New chapter downloads will appear here when the application service schedules them.".to_string(),
                    }
                },
                DownloadsState::Ready => {
                    if visible.is_empty() {
                        rsx! {
                            EmptyState {
                                eyebrow: "NO MATCHES".to_string(),
                                title: "No downloads match this view".to_string(),
                                description: "Try another search or download state.".to_string(),
                            }
                        }
                    } else {
                        rsx! {
                            section { class: "ny-library-collection ny-library-collection--list",
                                for item in visible {
                                    {
                                        let is_selected = selected.read().contains(&item.title);
                                        let action_label = match item.status {
                                            DownloadStatus::Queued => "Start",
                                            DownloadStatus::Active => "Pause",
                                            DownloadStatus::Completed => "Open",
                                            DownloadStatus::Failed => "Retry",
                                        };
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
                                                        Badge { label: status_label(item.status).to_string() }
                                                        span { "{item.source}" }
                                                        span { "{item.size}" }
                                                    }
                                                    h3 { "{item.title}" }
                                                    p { "{item.chapter}" }
                                                    Progress { value: item.progress }
                                                    div { class: "ny-library-item__footer",
                                                        span { class: "ny-library-item__initials", "{item.initials}" }
                                                        if *selection_mode.read() {
                                                            span { class: "ny-eyebrow", if is_selected { "SELECTED" } else { "SELECT" } }
                                                        } else {
                                                            Button {
                                                                variant: if item.status == DownloadStatus::Failed { ButtonVariant::Primary } else { ButtonVariant::Ghost },
                                                                label: action_label.to_string(),
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn download_filter_matches_status_and_query() {
        assert!(matches_filter(DOWNLOADS[0], DownloadsFilter::Active, "frieren"));
        assert!(matches_filter(DOWNLOADS[2], DownloadsFilter::Queued, "mangadex"));
        assert!(matches_filter(DOWNLOADS[4], DownloadsFilter::Failed, "issue 57"));
        assert!(!matches_filter(DOWNLOADS[1], DownloadsFilter::Active, "solo"));
        assert!(!matches_filter(DOWNLOADS[0], DownloadsFilter::Failed, "frieren"));
    }
}
