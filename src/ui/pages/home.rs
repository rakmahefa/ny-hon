use dioxus::prelude::*;

use crate::ui::{
    components::{app_shell::AppShell, card::Card, header::HeaderProps, library_card::LibraryCard},
    Theme,
};

#[derive(Props, Clone, PartialEq)]
pub struct HomePageProps {
    pub theme: Theme,
    pub on_toggle_theme: EventHandler<MouseEvent>,
}

#[component]
pub fn HomePage(props: HomePageProps) -> Element {
    let featured = [
        ("Frieren", "Manga", "Chapter 141 · 82%", "FR"),
        ("The Beginning After the End", "Webtoon", "Chapter 233 · 64%", "TB"),
        ("Solo Leveling", "Manga", "Chapter 179 · 91%", "SL"),
        ("The Sandman", "Comic", "Issue 18 · 37%", "SM"),
    ];

    let new_updates = [
        ("Frieren", "Chapter 141", "12 min ago"),
        ("The Beginning After the End", "Chapter 233", "1 h ago"),
        ("Blue Lock", "Chapter 304", "4 h ago"),
    ];

    rsx! {
        AppShell {
            theme: props.theme,
            on_toggle_theme: props.on_toggle_theme,
            section { class: "ny-home",
                section { class: "ny-hero",
                    div { class: "ny-hero__copy",
                        span { class: "ny-eyebrow", "WELCOME BACK" }
                        h2 { "Your next chapter is already here." }
                        p { "A focused Linux reader for manga, webtoons and comics — without hosting the content." }
                        button { class: "ny-primary-button", "Open Library" }
                    }
                    div { class: "ny-hero__stat-grid",
                        div { class: "ny-stat",
                            strong { "128" }
                            span { "Library items" }
                        }
                        div { class: "ny-stat",
                            strong { "07" }
                            span { "New chapters" }
                        }
                    }
                }

                div { class: "ny-section-heading",
                    div {
                        span { class: "ny-eyebrow", "CONTINUE" }
                        h2 { "Pick up where you left off" }
                    }
                    button { class: "ny-text-button", "View all" }
                }

                section { class: "ny-library-grid",
                    for (title, kind, progress, accent) in featured {
                        LibraryCard {
                            title: title.to_string(),
                            kind: kind.to_string(),
                            progress: progress.to_string(),
                            accent: accent.to_string(),
                        }
                    }
                }

                section { class: "ny-dashboard-grid",
                    Card { title: Some("Latest releases".to_string()), class: "ny-updates-card".to_string(),
                        div { class: "ny-release-list",
                            for (title, chapter, time) in new_updates {
                                div { class: "ny-release-row",
                                    div {
                                        strong { "{title}" }
                                        span { "{chapter}" }
                                    }
                                    time { "{time}" }
                                }
                            }
                        }
                    }
                    Card { title: Some("Reading profile".to_string()), class: "ny-profile-card".to_string(),
                        div { class: "ny-profile",
                            div { class: "ny-profile__ring", "72%" }
                            div {
                                strong { "Reading streak" }
                                p { "12 days · 4h 18m this week" }
                                button { class: "ny-secondary-button", "Open history" }
                            }
                        }
                    }
                }
            }
        }
    }
}
