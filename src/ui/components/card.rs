use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CardProps {
    #[props(default)]
    pub title: Option<String>,
    #[props(default = String::new())]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn Card(props: CardProps) -> Element {
    let class_name = if props.class.is_empty() {
        "ny-card".to_string()
    } else {
        format!("ny-card {}", props.class)
    };

    rsx! {
        section { class: class_name,
            if let Some(title) = props.title.clone() {
                h2 { class: "ny-card__title", "{title}" }
            }
            div { class: "ny-card__body", {props.children} }
        }
    }
}
