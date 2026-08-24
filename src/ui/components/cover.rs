use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CoverProps {
    pub title: String,
    #[props(default)]
    pub subtitle: Option<String>,
    #[props(default)]
    pub aspect: String,
}

#[component]
pub fn Cover(props: CoverProps) -> Element {
    let class_name = if props.aspect.is_empty() {
        "ny-cover".to_string()
    } else {
        format!("ny-cover {}", props.aspect)
    };

    rsx! {
        div {
            class: class_name,
            role: "img",
            aria_label: props.title.clone(),
            div { class: "ny-cover__initials", "{props.title.chars().take(2).collect::<String>().to_uppercase()}" }
            div { class: "ny-cover__shade" }
            div { class: "ny-cover__caption",
                strong { "{props.title}" }
                if let Some(subtitle) = props.subtitle.clone() {
                    span { "{subtitle}" }
                }
            }
        }
    }
}
