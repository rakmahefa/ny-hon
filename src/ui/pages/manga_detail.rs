use dioxus::prelude::*;

use crate::ui::components::{
    badge::Badge,
    button::{Button, ButtonVariant},
    card::Card,
    cover::Cover,
    empty_state::EmptyState,
    progress::Progress,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MangaDetailState {
    Ready,
    Loading,
    Error,
    Offline,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ChapterFilter {
    All,
    Unread,
    Downloaded,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Chapter {
    number: &'static str,
    title: &'static str,
    released: &'static str,
    read: bool,
    downloaded: bool,
}

const CHAPTERS: [Chapter; 6] = [
    Chapter { number: "141", title: "The Northern Arc", released: "Aug 21", read: false, downloaded: false },
    Chapter { number: "140", title: "A Quiet Crossing", released: "Aug 14", read: false, downloaded: true },
    Chapter { number: "139", title: "Ashes and Snow", released: "Aug 07", read: true, downloaded: true },
    Chapter { number: "138", title: "The Long Road", released: "Jul 31", read: true, downloaded: false },
    Chapter { number: "137", title: "Beyond the Wall", released: "Jul 24", read: true, downloaded: false },
    Chapter { number: "136", title: "A New Morning", released: "Jul 17", read: true, downloaded: true },
];

fn matches_filter(chapter: Chapter, filter: ChapterFilter) -> bool {
    match filter {
        ChapterFilter::All => true,
        ChapterFilter::Unread => !chapter.read,
        ChapterFilter::Downloaded => chapter.downloaded,
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct MangaDetailPageProps {
    pub title: String,
    #[props(default = MangaDetailState::Ready)]
    pub state: MangaDetailState,
    pub on_back: EventHandler<()>,
    pub on_open_reader: EventHandler<()>,
    pub on_download: EventHandler<()>,
    pub on_toggle_library: EventHandler<()>,
    pub on_refresh: EventHandler<()>,
}

#[component]
pub fn MangaDetailPage(props: MangaDetailPageProps) -> Element {
    let mut filter = use_signal(|| ChapterFilter::All);
    let mut library_added = use_signal(|| true);
    let mut search = use_signal(String::new);

    let chapter_query = search.read().trim().to_lowercase();
    let visible: Vec<Chapter> = CHAPTERS
        .iter()
        .copied()
        .filter(|chapter| matches_filter(*chapter, *filter.read()))
        .filter(|chapter| {
            chapter_query.is_empty()
                || chapter.number.contains(&chapter_query)
                || chapter.title.to_lowercase().contains(&chapter_query)
        })
        .collect();

    match props.state {
        MangaDetailState::Loading => return rsx! {
            section { class: "ny-page ny-detail-page",
                EmptyState {
                    eyebrow: "MANGA DETAIL".to_string(),
                    title: "Loading title information".to_string(),
                    description: "The detail surface exposes loading without taking ownership of source or persistence logic.".to_string(),
                }
            }
        },
        MangaDetailState::Error => return rsx! {
            section { class: "ny-page ny-detail-page",
                EmptyState {
                    eyebrow: "MANGA DETAIL".to_string(),
                    title: "Title information unavailable".to_string(),
                    description: "Retry remains an application-level concern while the UI keeps a deterministic error boundary.".to_string(),
                    action_label: Some("Retry".to_string()),
                    onclick: Some(EventHandler::new(move |_| props.on_refresh.call(()))),
                }
            }
        },
        MangaDetailState::Offline => return rsx! {
            section { class: "ny-page ny-detail-page",
                EmptyState {
                    eyebrow: "OFFLINE".to_string(),
                    title: "Showing cached title information".to_string(),
                    description: "The detail view remains navigable while remote metadata is unavailable.".to_string(),
                }
            }
        },
        MangaDetailState::Ready => {}
    }

    let unread_count = CHAPTERS.iter().filter(|chapter| !chapter.read).count();
    let downloaded_count = CHAPTERS.iter().filter(|chapter| chapter.downloaded).count();

    rsx! {
        section { class: "ny-page ny-detail-page",
            div { class: "ny-detail-toolbar",
                Button {
                    variant: ButtonVariant::Ghost,
                    label: "← Back to library",
                    onclick: move |_| props.on_back.call(()),
                }
                div { class: "ny-detail-toolbar__actions",
                    Badge { label: "MANGA".to_string() }
                    Badge { label: "MangaDex".to_string() }
                }
            }

            section { class: "ny-detail-hero",
                div { class: "ny-detail-hero__cover",
                    Cover {
                        title: props.title.clone(),
                        subtitle: Some("Series cover".to_string()),
                        aspect: "ny-cover--detail".to_string(),
                    }
                }
                div { class: "ny-detail-hero__content",
                    span { class: "ny-eyebrow", "MANGA DETAIL" }
                    h1 { "{props.title}" }
                    p { class: "ny-detail-hero__description", "Frieren follows an elven mage after the end of a hero's journey, focusing on memory, time, and the quiet lives left behind." }
                    div { class: "ny-detail-metadata",
                        div { strong { "Author" } span { "Kanehito Yamada" } }
                        div { strong { "Artist" } span { "Tsukasa Abe" } }
                        div { strong { "Status" } span { "Ongoing" } }
                        div { strong { "Genres" } span { "Adventure · Fantasy · Drama" } }
                    }
                    div { class: "ny-detail-actions",
                        Button {
                            variant: ButtonVariant::Primary,
                            label: "Continue reading",
                            onclick: move |_| props.on_open_reader.call(()),
                        }
                        Button {
                            variant: ButtonVariant::Secondary,
                            label: if *library_added.read() { "In library".to_string() } else { "Add to library".to_string() },
                            onclick: move |_| {
                                let next = !*library_added.read();
                                library_added.set(next);
                                props.on_toggle_library.call(());
                            },
                        }
                        Button {
                            variant: ButtonVariant::Secondary,
                            label: "Download latest",
                            onclick: move |_| props.on_download.call(()),
                        }
                    }
                }
                div { class: "ny-detail-progress-card",
                    span { class: "ny-eyebrow", "READING PROGRESS" }
                    strong { "82%" }
                    Progress { value: 82 }
                    span { "Chapter 141 · {unread_count} unread" }
                }
            }

            div { class: "ny-detail-grid",
                Card { title: Some("Library status".to_string()), class: "ny-detail-card".to_string(),
                    div { class: "ny-detail-stat-grid",
                        div { strong { "{unread_count}" } span { "Unread chapters" } }
                        div { strong { "{downloaded_count}" } span { "Downloaded" } }
                        div { strong { "24 Aug" } span { "Last updated" } }
                        div { strong { "4" } span { "Categories" } }
                    }
                }
                Card { title: Some("Tracking".to_string()), class: "ny-detail-card".to_string(),
                    div { class: "ny-tracking-card",
                        div {
                            strong { "AniList" }
                            span { "Not linked" }
                        }
                        Button { variant: ButtonVariant::Secondary, label: "Manage" }
                        div {
                            strong { "MyAnimeList" }
                            span { "Not linked" }
                        }
                        Button { variant: ButtonVariant::Secondary, label: "Manage" }
                    }
                }
            }

            Card { title: Some("Chapters".to_string()), class: "ny-detail-chapters".to_string(),
                div { class: "ny-detail-chapter-toolbar",
                    div { class: "ny-filter-group",
                        button {
                            class: if *filter.read() == ChapterFilter::All { "ny-filter is-active" } else { "ny-filter" },
                            onclick: move |_| filter.set(ChapterFilter::All),
                            "All"
                        }
                        button {
                            class: if *filter.read() == ChapterFilter::Unread { "ny-filter is-active" } else { "ny-filter" },
                            onclick: move |_| filter.set(ChapterFilter::Unread),
                            "Unread ({unread_count})"
                        }
                        button {
                            class: if *filter.read() == ChapterFilter::Downloaded { "ny-filter is-active" } else { "ny-filter" },
                            onclick: move |_| filter.set(ChapterFilter::Downloaded),
                            "Downloaded ({downloaded_count})"
                        }
                    }
                    div { class: "ny-search",
                        span { class: "ny-search__icon", "⌕" }
                        input {
                            value: "{search}",
                            placeholder: "Search chapters…",
                            aria_label: "Search chapters",
                            oninput: move |event| search.set(event.value()),
                        }
                    }
                }
                if visible.is_empty() {
                    EmptyState {
                        eyebrow: "NO CHAPTERS".to_string(),
                        title: "No chapters match".to_string(),
                        description: "Try another filter or chapter search.".to_string(),
                    }
                } else {
                    div { class: "ny-detail-chapter-list",
                        for chapter in visible {
                            div { class: "ny-detail-chapter-row",
                                div { class: "ny-detail-chapter-number", "{chapter.number}" }
                                div { class: "ny-detail-chapter-copy",
                                    strong { "{chapter.title}" }
                                    span { "Released {chapter.released}" }
                                }
                                div { class: "ny-detail-chapter-status",
                                    if chapter.read { Badge { label: "Read".to_string() } } else { Badge { label: "Unread".to_string() } }
                                    if chapter.downloaded { Badge { label: "Downloaded".to_string() } }
                                }
                                div { class: "ny-library-toolbar__actions",
                                    Button { variant: ButtonVariant::Ghost, label: "Read", onclick: move |_| props.on_open_reader.call(()) }
                                    Button { variant: ButtonVariant::Secondary, label: if chapter.downloaded { "Downloaded".to_string() } else { "Download".to_string() }, onclick: move |_| props.on_download.call(()) }
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
    fn unread_filter_excludes_read_chapters() {
        assert!(matches_filter(CHAPTERS[0], ChapterFilter::Unread));
        assert!(!matches_filter(CHAPTERS[2], ChapterFilter::Unread));
    }

    #[test]
    fn downloaded_filter_matches_downloaded_chapters() {
        assert!(matches_filter(CHAPTERS[1], ChapterFilter::Downloaded));
        assert!(!matches_filter(CHAPTERS[0], ChapterFilter::Downloaded));
    }
}