use dioxus::prelude::*;

use super::{
    header::Header,
    nav::{NavItem, Sidebar},
    theme::Theme,
};

#[derive(Props, Clone, PartialEq)]
pub struct AppShellProps {
    pub theme: Theme,
    pub active: NavItem,
    pub on_navigate: EventHandler<NavItem>,
    pub on_toggle_theme: EventHandler<MouseEvent>,
    pub children: Element,
}

#[component]
pub fn AppShell(props: AppShellProps) -> Element {
    rsx! {
        div { class: "ny-shell",
            Sidebar {
                active: props.active,
                on_select: move |item| props.on_navigate.call(item),
            }
            div { class: "ny-main",
                Header {
                    theme: props.theme,
                    on_toggle_theme: props.on_toggle_theme,
                }
                main { class: "ny-main__content", {props.children} }
            }
        }
    }
}
