use dioxus::prelude::*;

use super::progress::Progress;

#[derive(Props, Clone, PartialEq)]
pub struct LibraryCardProps {
    pub title: String,
    pub kind: String,
    #[props(default = 0)]
    pub progress: u8,
    pub accent: String,
}

#[component]
pub fn LibraryCard(props: LibraryCardProps) -> Element {
    let progress = props.progress.min(100);

    rsx! {
        article { class: "ny-library-card",
            div { class: "ny-library-card__cover", "{props.accent}" }
            div { class: "ny-library-card__content",
                div { class: "ny-library-card__meta", "{props.kind}" }
                h3 { "{props.title}" }
                p { "Chapter progress · {progress}%" }
                Progress { value: progress }
            }
        }
    }
}
