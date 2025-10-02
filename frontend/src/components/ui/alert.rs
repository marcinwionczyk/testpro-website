use dioxus::prelude::*;
use tw_merge::*;

#[derive(TwVariant, PartialEq)]
pub enum AlertVariant {
    #[tw(default)]
    #[tw(class = "bg-background text-foreground")]
    Default,
    #[tw(class = "border-destructive/50 text-destructive dark:border-destructive [&>svg]:text-destructive")]
    Destructive,
}
#[derive(TwClass, Clone, PartialEq)]
#[tw(class = "relative w-full rounded-lg border p-4 [&>svg~*]:pl-7 [&>svg+div]:translate-y-[-3px] [&>svg]:absolute [&>svg]:left-4 [&>svg]:top-4 [&>svg]:text-foreground")]
pub struct Alert {
    variant: AlertVariant
}

#[derive(Props, PartialEq, Clone)]
pub struct AlertProps {
    #[props(default)]
    pub variant: AlertVariant,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub children: Element,
}

#[derive(Props, PartialEq, Clone)]
pub struct AlertTitleProps {
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub children: Element,
}

#[derive(Props, PartialEq, Clone)]
pub struct AlertDescriptionProps {
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub children: Element,
}


#[component]
pub fn DxsAlert(props: AlertProps) -> Element {
    let alert = Alert { variant: props.variant };
    let class_name = tw_merge!(alert.to_class(), props.class);
    rsx! {
        div { class: "{class_name}", role: "alert", {props.children} }
    }
}

#[component]
pub fn DxsAlertTitle(props: AlertTitleProps) -> Element {
    rsx! {
        h5 { class: tw_merge!("mb-1 font-medium leading-none tracking-tight", props.class),
            {props.children}
        }
    }
}

#[component]
pub fn DxsAlertDescription(props: AlertDescriptionProps) -> Element {
    rsx! {
        p { class: tw_merge!("text-sm [&_p]:leading-relaxed", props.class) }
    }
}