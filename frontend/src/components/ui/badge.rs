use dioxus::prelude::*;
use tw_merge::*;

#[derive(TwClass)]
#[tw(class = "inline-flex items-center rounded-full border px-2.5 py-0.5 text-xs font-semibold transition-colors focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2")]
struct Badge {
    variant: BadgeVariant,
}

#[derive(TwVariant, PartialEq)]
pub enum BadgeVariant {
    #[tw(default)]
    #[tw(class = "border-transparent bg-primary text-primary-foreground hover:bg-primary/80")]
    Default,
    #[tw(class = "border-transparent bg-secondary text-secondary-foreground hover:bg-secondary/80")]
    Secondary,
    #[tw(class = "border-transparent bg-destructive text-destructive-foreground hover:bg-destructive/80")]
    Destructive,
    #[tw(class = "text-foreground")]
    Outline,
}

#[derive(Props, PartialEq)]
#[derive(Clone)]
pub struct BadgeProps {
    #[props(default)]
    pub variant: BadgeVariant,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub id: Option<String>,
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    #[props(default)]
    pub children: Element,
}

#[component]
pub fn DxsBadge(props: BadgeProps) -> Element {
    let badge = Badge { variant: props.variant };
    let class_name = tw_merge!(badge.to_class(), props.class);
    rsx! {
        div {
            class: "{class_name}",
            id: props.id,
            onclick: move |evt| {
                if let Some(handler) = &props.onclick {
                    handler.call(evt);
                }
            },
            {props.children}
        }
    }
}
