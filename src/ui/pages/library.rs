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

#[derive(Props, Clone, PartialEq)]
pub struct LibraryPageProps {
    #[props(default = LibraryView::Grid)]
    pub view: LibraryView,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum LibraryFilter {
    All,
    Reading,
    Completed,
    Unread,
}

#[component]
pub fn LibraryPage(props: LibraryPageProps) -> Element {
    let mut view = use_signal(|| props.view);
    let mut filter = use_signal(|| LibraryFilter::All);

    let items = [
        ("Frieren", "Manga", "Chapter 141", 82_u8, "FR"),
        ("The Beginning After the End", "Webtoon", "Chapter 233", 64_u8, "TB"),
        ("Solo Leveling", "Manga", "Chapter 179", 91_u8, "SL"),
        ("The Sandman", "Comic", "Issue 18", 37_u8, "SM"),
        ("Blue Lock", "Manga", "Chapter 304", 100_u8, "BL"),
        ("Monstress", "Comic", "Issue 56", 12_u8, "MO"),
    ];

    let visible = items.iter().copied().filter(|(_, _, _, progress, _)| match *filter.read() {
        LibraryFilter::All => true,
        LibraryFilter::Reading => *progress > 0 && *progress < 100,
        LibraryFilter::Completed => *progress == 100,
        LibraryFilter::Unread => *progress == 0,
    });

    rsx! {
        section { class: "ny-page ny-library-page",
            SectionHeader {
                eyebrow: "LIBRARY".to_string(),
                title: "Your collection".to_string(),
                description: Some("A focused home for everything you are reading, completed, or keeping for later.".to_string()),
            }

            div { class: "ny-library-toolbar",
                div { class: "ny-filter-group",
                    for (label, value) in [("All", LibraryFilter::All), ("Reading", LibraryFilter::Reading), ("Completed", LibraryFilter::Completed), ("Unread", LibraryFilter::Unread)] {
                        button {
                            class: if *filter.read() == value { "ny-filter is-active" } else { "ny-filter" },
                            onclick: move |_| filter.set(value),
                            "{label}"
                        }
                    }
                }
                div { class: "ny-library-toolbar__actions",
                    Button { variant: ButtonVariant::Secondary, label: "Categories" }
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

            if visible.clone().count() == 0 {
                EmptyState {
                    eyebrow: "NO MATCHES".to_string(),
                    title: "Nothing here yet".to_string(),
                    description: "This library state is ready for real persistence later.".to_string(),
                }
            } else {
                section { class: if *view.read() == LibraryView::Grid { "ny-library-collection ny-library-collection--grid" } else { "ny-library-collection ny-library-collection--list" },
                    for (title, kind, chapter, progress, initials) in visible {
                        article { class: "ny-library-item",
                            Cover { title: title.to_string(), subtitle: Some(chapter.to_string()), aspect: if *view.read() == LibraryView::Grid { "ny-cover--portrait".to_string() } else { "ny-cover--compact".to_string() } }
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
                                    Button { variant: ButtonVariant::Ghost, label: "Open" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
