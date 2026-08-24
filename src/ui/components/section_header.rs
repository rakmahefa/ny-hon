use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SectionHeaderProps {
    pub eyebrow: String,
    pub title: String,
    #[props(default)]
    pub description: Option<String>,
    #[props(default)]
    pub action_label: Option<String>,
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
}

#[component]
pub fn SectionHeader(props: SectionHeaderProps) -> Element {
    rsx! {
        header { class: "ny-section-header",
            div { class: "ny-section-header__copy",
                span { class: "ny-eyebrow", "{props.eyebrow}" }
                h2 { "{props.title}" }
                if let Some(description) = props.description.clone() {
                    p { "{description}" }
                }
            }
            if let Some(label) = props.action_label.clone() {
                button {
                    class: "ny-text-button",
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
