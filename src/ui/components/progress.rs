use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ProgressProps {
    #[props(default = 0)]
    pub value: u8,
}

#[component]
pub fn Progress(props: ProgressProps) -> Element {
    let value = props.value.min(100);
    rsx! {
        div { class: "ny-progress", role: "progressbar", aria_valuemin: "0", aria_valuemax: "100", aria_valuenow: "{value}",
            div { class: "ny-progress__fill", style: "width: {value}%" }
        }
    }
}
