use dioxus::prelude::*;

use super::{badge::Badge, cover::Cover, progress::Progress};

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
            Cover {
                title: props.title.clone(),
                subtitle: props.kind.clone(),
                aspect: "".to_string(),
            }
            div { class: "ny-library-card__content",
                div { class: "ny-library-card__meta",
                    Badge { label: props.kind.clone() }
                }
                h3 { "{props.title}" }
                p { "Chapter progress · {progress}%" }
                Progress { value: progress }
            }
        }
    }
}
