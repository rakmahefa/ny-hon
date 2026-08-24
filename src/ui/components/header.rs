use dioxus::prelude::*;

use super::theme::Theme;

#[derive(Props, Clone, PartialEq)]
pub struct HeaderProps {
    pub theme: Theme,
    pub on_toggle_theme: EventHandler<MouseEvent>,
}

#[component]
pub fn Header(props: HeaderProps) -> Element {
    rsx! {
        header { class: "ny-header",
            div { class: "ny-header__title",
                span { class: "ny-eyebrow", "PERSONAL READER" }
                h1 { "Library" }
            }
            div { class: "ny-header__actions",
                label { class: "ny-search",
                    span { class: "ny-search__icon", "⌕" }
                    input { r#type: "search", placeholder: "Search your library" }
                }
                button {
                    class: "ny-theme-toggle",
                    title: "Switch theme",
                    aria_label: "Switch theme",
                    onclick: move |event| props.on_toggle_theme.call(event),
                    span { class: "ny-theme-toggle__glyph", if props.theme == Theme::Dark { "☼" } else { "◐" } }
                    span { "{props.theme.label()}" }
                }
            }
        }
    }
}
