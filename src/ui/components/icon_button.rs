use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct IconButtonProps {
    pub label: String,
    #[props(default)]
    pub disabled: bool,
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    pub children: Element,
}

#[component]
pub fn IconButton(props: IconButtonProps) -> Element {
    rsx! {
        button {
            class: "ny-icon-button",
            disabled: props.disabled,
            aria_label: props.label.clone(),
            title: props.label.clone(),
            onclick: move |event| {
                if let Some(handler) = &props.onclick {
                    handler.call(event);
                }
            },
            {props.children}
        }
    }
}
