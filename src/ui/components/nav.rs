use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NavItem {
    Library,
    Discover,
    History,
    Categories,
}

impl NavItem {
    fn label(self) -> &'static str {
        match self {
            Self::Library => "Library",
            Self::Discover => "Discover",
            Self::History => "History",
            Self::Categories => "Categories",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SidebarProps {
    pub active: NavItem,
    pub on_select: EventHandler<NavItem>,
}

#[component]
pub fn Sidebar(props: SidebarProps) -> Element {
    let items = [
        (NavItem::Library, "01"),
        (NavItem::Discover, "02"),
        (NavItem::History, "03"),
        (NavItem::Categories, "04"),
    ];

    rsx! {
        aside { class: "ny-sidebar",
            div { class: "ny-brand",
                div { class: "ny-brand__mark", "H" }
                div {
                    strong { "NY-HON" }
                    span { "Reader" }
                }
            }
            nav { class: "ny-sidebar__nav", aria_label: "Primary navigation",
                for (item, index) in items {
                    button {
                        class: if item == props.active { "ny-nav-item is-active" } else { "ny-nav-item" },
                        aria_current: if item == props.active { "page" } else { "false" },
                        onclick: move |_| props.on_select.call(item),
                        span { class: "ny-nav-item__index", "{index}" }
                        span { "{item.label()}" }
                    }
                }
            }
            div { class: "ny-sidebar__footer",
                span { class: "ny-status-dot" }
                span { "Local library ready" }
            }
        }
    }
}
