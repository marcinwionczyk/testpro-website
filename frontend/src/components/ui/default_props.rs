use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct DefaultProps {
    #[props(default)]
    pub class: Option<String>,
    pub children: Element,
}