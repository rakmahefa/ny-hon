use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct EmptyStateProps {
    pub eyebrow: String,
    pub title: String,
    pub description: String,
    #[props(default)]
    pub action_label: Option<String>,
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
}

#[component]
pub fn EmptyState(props: EmptyStateProps) -> Element {
    rsx! {
        section { class: "ny-empty-state",
            div { class: "ny-empty-state__mark", "—" }
            span { class: "ny-eyebrow", "{props.eyebrow}" }
            h2 { "{props.title}" }
            p { "{props.description}" }
            if let Some(label) = props.action_label.clone() {
                button {
                    class: "ny-button ny-button--primary",
                    onclick: move |event| {
                        if let Some(handler) = &props.onclick {
                            handler.call(event);
                        }
                    },
                    "{label}"
                }
            }
        }
    }
}
