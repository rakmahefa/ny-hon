use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct LibraryCardProps {
    pub title: String,
    pub kind: String,
    pub progress: String,
    pub accent: String,
}

#[component]
pub fn LibraryCard(props: LibraryCardProps) -> Element {
    rsx! {
        article { class: "ny-library-card",
            div { class: "ny-library-card__cover", "{props.accent}" }
            div { class: "ny-library-card__content",
                div { class: "ny-library-card__meta", "{props.kind}" }
                h3 { "{props.title}" }
                p { "{props.progress}" }
                button { class: "ny-text-button", "Continue reading" }
            }
        }
    }
}
