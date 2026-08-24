use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BadgeProps {
    pub label: String,
}

#[component]
pub fn Badge(props: BadgeProps) -> Element {
    rsx! {
        span { class: "ny-badge", "{props.label}" }
    }
}
