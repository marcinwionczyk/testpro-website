use dioxus::prelude::*;
use tw_merge::*;
use crate::components::ui::default_props::DefaultProps;

#[component]
pub fn DxsCard(props: DefaultProps) -> Element {
    let class_name = tw_merge!("rounded-lg border bg-card text-card-foreground shadow-sm", props.class);
    rsx! {
        div { class: "{class_name}", {props.children} }
    }
}

#[component]
pub fn DxsCardHeader(props: DefaultProps) -> Element {
    let class_name = tw_merge!("flex flex-col space-y-1.5 p-6", props.class);
    rsx! {
        div { class: "{class_name}", {props.children} }
    }
}

#[component]
pub fn DxsCardTitle(props: DefaultProps) -> Element {
    let class_name = tw_merge!("text-2xl font-semibold leading-none tracking-tight", props.class);
    rsx! {
        h3 { class: "{class_name}", {props.children} }
    }
}

#[component]
pub fn DxsCardDescription(props: DefaultProps) -> Element {
    let class_name = tw_merge!("text-sm text-black", props.class);
    rsx! {
        p { class: "{class_name}", {props.children} }
    }
}

#[component]
pub fn DxsCardContent(props: DefaultProps) -> Element {
    let class_name = tw_merge!("p-6 pt-0", props.class);
    rsx! {
        div { class: "{class_name}", {props.children} }
    }
}

#[component]
pub fn DxsCardFooter(props: DefaultProps) -> Element {
    let class_name = tw_merge!("flex items-center p-6 pt-0", props.class);
    rsx! {
        div { class: "{class_name}", {props.children} }
    }
}