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
pub enum LibraryView {
    Grid,
    List,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LibraryFilter {
    All,
    Reading,
    Completed,
    Unread,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LibraryState {
    Ready,
    Loading,
    Empty,
    Error,
    Offline,
}

#[derive(Props, Clone, PartialEq)]
pub struct LibraryPageProps {
    #[props(default = LibraryView::Grid)]
    pub view: LibraryView,
    #[props(default)]
    pub on_open: Option<EventHandler<String>>,
}

#[component]
pub fn LibraryPage(props: LibraryPageProps) -> Element {
    let mut view = use_signal(|| props.view);
    let mut filter = use_signal(|| LibraryFilter::All);
    let mut search = use_signal(String::new);
    let mut selection_mode = use_signal(|| false);
    let mut selected = use_signal(Vec::<&'static str>::new);
    let mut state = use_signal(|| LibraryState::Ready);

    let items = [
        ("Frieren", "Manga", "Chapter 141", 82_u8, "FR"),
        ("The Beginning After the End", "Webtoon", "Chapter 233", 64_u8, "TB"),
        ("Solo Leveling", "Manga", "Chapter 179", 91_u8, "SL"),
        ("The Sandman", "Comic", "Issue 18", 37_u8, "SM"),
        ("Blue Lock", "Manga", "Chapter 304", 100_u8, "BL"),
        ("Monstress", "Comic", "Issue 56", 12_u8, "MO"),
    ];

    let query = search.read().trim().to_lowercase();
    let visible: Vec<_> = items
        .iter()
        .copied()
        .filter(|(title, _, _, progress, _)| {
            let matches_filter = match *filter.read() {
                LibraryFilter::All => true,
                LibraryFilter::Reading => *progress > 0 && *progress < 100,
                LibraryFilter::Completed => *progress == 100,
                LibraryFilter::Unread => *progress == 0,
            };
            let matches_search = query.is_empty() || title.to_lowercase().contains(&query);
            matches_filter && matches_search
        })
        .collect();

    let current_state = *state.read();
    let selected_count = selected.read().len();

    rsx! {
        section { class: "ny-page ny-library-page",
            SectionHeader {
                eyebrow: "LIBRARY".to_string(),
                title: "Your collection".to_string(),
                description: Some("Manage everything you are reading, completed, or keeping for later.".to_string()),
            }

            div { class: "ny-library-toolbar",
                div { class: "ny-filter-group",
                    button {
                        class: if *filter.read() == LibraryFilter::All { "ny-filter is-active" } else { "ny-filter" },
                        onclick: move |_| filter.set(LibraryFilter::All),
                        "All"
                    }
                    button {
                        class: if *filter.read() == LibraryFilter::Reading { "ny-filter is-active" } else { "ny-filter" },
                        onclick: move |_| filter.set(LibraryFilter::Reading),
                        "Reading"
                    }
                    button {
                        class: if *filter.read() == LibraryFilter::Completed { "ny-filter is-active" } else { "ny-filter" },
                        onclick: move |_| filter.set(LibraryFilter::Completed),
                        "Completed"
                    }
                    button {
                        class: if *filter.read() == LibraryFilter::Unread { "ny-filter is-active" } else { "ny-filter" },
                        onclick: move |_| filter.set(LibraryFilter::Unread),
                        "Unread"
                    }
                }

                div { class: "ny-library-toolbar__actions",
                    div { class: "ny-search",
                        span { class: "ny-search__icon", "⌕" }
                        input {
                            value: "{search}",
                            placeholder: "Search library…",
                            aria_label: "Search library",
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
                    Button { variant: ButtonVariant::Secondary, label: "Categories" }
                    Button {
                        variant: ButtonVariant::Secondary,
                        label: "Refresh",
                        onclick: move |_| state.set(LibraryState::Ready),
                    }
                    button {
                        class: "ny-view-toggle",
                        aria_label: "Toggle library layout",
                        onclick: move |_| {
                            let next = if *view.read() == LibraryView::Grid { LibraryView::List } else { LibraryView::Grid };
                            view.set(next);
                        },
                        if *view.read() == LibraryView::Grid { "List view" } else { "Grid view" }
                    }
                }
            }

            if *selection_mode.read() && selected_count > 0 {
                div { class: "ny-library-toolbar",
                    span { class: "ny-eyebrow", "SELECTION" }
                    span { "{selected_count} items selected" }
                    Button {
                        variant: ButtonVariant::Ghost,
                        label: "Clear selection",
                        onclick: move |_| selected.set(Vec::new()),
                    }
                }
            }

            match current_state {
                LibraryState::Loading => rsx! {
                    EmptyState {
                        eyebrow: "LOADING".to_string(),
                        title: "Loading your library".to_string(),
                        description: "The library state is ready for asynchronous domain data.".to_string(),
                    }
                },
                LibraryState::Error => rsx! {
                    EmptyState {
                        eyebrow: "ERROR".to_string(),
                        title: "Library could not be loaded".to_string(),
                        description: "The UI exposes a retry boundary without owning persistence or networking.".to_string(),
                        action_label: Some("Retry".to_string()),
                        onclick: Some(EventHandler::new(move |_| state.set(LibraryState::Ready))),
                    }
                },
                LibraryState::Offline => rsx! {
                    EmptyState {
                        eyebrow: "OFFLINE".to_string(),
                        title: "Local library only".to_string(),
                        description: "Browsing the local collection remains available while remote sources are unavailable.".to_string(),
                    }
                },
                LibraryState::Empty => rsx! {
                    EmptyState {
                        eyebrow: "EMPTY".to_string(),
                        title: "Your library is empty".to_string(),
                        description: "Add manga, webtoons or comics from Browse to build your collection.".to_string(),
                    }
                },
                LibraryState::Ready => {
                    if visible.is_empty() {
                        rsx! {
                            EmptyState {
                                eyebrow: "NO MATCHES".to_string(),
                                title: "Nothing here yet".to_string(),
                                description: "Try another search or filter. The empty state is ready for real library persistence later.".to_string(),
                            }
                        }
                    } else {
                        rsx! {
                            section { class: if *view.read() == LibraryView::Grid { "ny-library-collection ny-library-collection--grid" } else { "ny-library-collection ny-library-collection--list" },
                                for (title, kind, chapter, progress, initials) in visible {
                                    {
                                        let is_selected = selected.read().contains(&title);
                                        let on_open = props.on_open.clone();
                                        rsx! {
                                            article {
                                                class: if is_selected { "ny-library-item is-selected" } else { "ny-library-item" },
                                                onclick: move |_| {
                                                    if !*selection_mode.read() {
                                                        return;
                                                    }
                                                    let mut items = selected.write();
                                                    if let Some(index) = items.iter().position(|item| *item == title) {
                                                        items.remove(index);
                                                    } else {
                                                        items.push(title);
                                                    }
                                                },
                                                Cover {
                                                    title: title.to_string(),
                                                    subtitle: Some(chapter.to_string()),
                                                    aspect: if *view.read() == LibraryView::Grid { "ny-cover--portrait".to_string() } else { "ny-cover--compact".to_string() },
                                                }
                                                div { class: "ny-library-item__content",
                                                    div { class: "ny-library-item__meta",
                                                        Badge { label: kind.to_string() }
                                                        span { "{chapter}" }
                                                    }
                                                    h3 { "{title}" }
                                                    p { "Local progress · {progress}%" }
                                                    Progress { value: progress }
                                                    div { class: "ny-library-item__footer",
                                                        span { class: "ny-library-item__initials", "{initials}" }
                                                        if *selection_mode.read() {
                                                            span { class: "ny-eyebrow", if is_selected { "SELECTED" } else { "SELECT" } }
                                                        } else {
                                                            Button {
                                                                variant: ButtonVariant::Ghost,
                                                                label: "Open",
                                                                onclick: move |_| {
                                                                    if let Some(handler) = &on_open {
                                                                        handler.call(title.to_string());
                                                                    }
                                                                },
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
        }
    }
}
