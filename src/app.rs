use dioxus::prelude::*;

use crate::ui::{AppShell, HomePage, LibraryPage, NavItem, Theme};
use crate::ui::components::empty_state::EmptyState;

#[component]
pub fn App() -> Element {
    let mut theme = use_signal(|| Theme::Dark);
    let mut active = use_signal(|| NavItem::Library);

    let title = match *active.read() {
        NavItem::Library => "Library",
        NavItem::Discover => "Discover",
        NavItem::History => "History",
        NavItem::Categories => "Categories",
    };

    rsx! {
        link { rel: "stylesheet", href: asset!("/assets/app.css") }
        div {
            class: "ny-app",
            "data-theme": "{theme.read().as_attr()}",
            AppShell {
                theme: *theme.read(),
                active: *active.read(),
                title: title.to_string(),
                on_navigate: move |item| active.set(item),
                on_toggle_theme: move |_| {
                    let next = if *theme.read() == Theme::Dark { Theme::Light } else { Theme::Dark };
                    theme.set(next);
                },
                match *active.read() {
                    NavItem::Library => rsx! { LibraryPage {} },
                    NavItem::Discover => rsx! { HomePage {} },
                    NavItem::History => rsx! {
                        EmptyState {
                            eyebrow: "HISTORY".to_string(),
                            title: "Reading history".to_string(),
                            description: "The standalone page boundary is ready; persistence will connect here later.".to_string(),
                        }
                    },
                    NavItem::Categories => rsx! {
                        EmptyState {
                            eyebrow: "CATEGORIES".to_string(),
                            title: "Organize your library".to_string(),
                            description: "Category management will be backed by the library domain, not the UI layer.".to_string(),
                        }
                    },
                }
            }
        }
    }
}
