use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Ghost,
}

impl ButtonVariant {
    fn class(self) -> &'static str {
        match self {
            Self::Primary => "ny-button ny-button--primary",
            Self::Secondary => "ny-button ny-button--secondary",
            Self::Ghost => "ny-button ny-button--ghost",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    #[props(default)]
    pub variant: ButtonVariant,
    #[props(default)]
    pub disabled: bool,
    pub label: String,
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    rsx! {
        button {
            class: props.variant.class(),
            disabled: props.disabled,
            aria_label: props.label.clone(),
            onclick: move |event| {
                if let Some(handler) = &props.onclick {
                    handler.call(event);
                }
            },
            "{props.label}"
        }
    }
}
