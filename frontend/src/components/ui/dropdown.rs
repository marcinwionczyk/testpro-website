use dioxus::prelude::*;
use tw_merge::*;

// Root dropdown context
#[derive(Clone, Copy)]
pub struct DropdownContext {
    pub is_open: Signal<bool>,
    pub toggle: Callback<()>,
    pub close: Callback<()>,
}

// Root dropdown component
#[derive(Props, PartialEq, Clone)]
pub struct DropdownMenuProps {
    #[props(default)]
    pub open: Option<bool>,
    #[props(default)]
    pub default_open: Option<bool>,
    #[props(default)]
    pub on_open_change: Option<EventHandler<bool>>,
    #[props(default)]
    pub children: Element,
}

#[component]
pub fn DropdownMenu(props: DropdownMenuProps) -> Element {
    let mut is_open = use_signal(|| props.default_open.unwrap_or(false));

    // Handle controlled state
    if let Some(open) = props.open {
        is_open.set(open);
    }

    let toggle = use_callback(move |_| {
        let new_state = !is_open();
        is_open.set(new_state);
        if let Some(handler) = &props.on_open_change {
            handler.call(new_state);
        }
    });

    let close = use_callback(move |_| {
        is_open.set(false);
        if let Some(handler) = &props.on_open_change {
            handler.call(false);
        }
    });

    let context = DropdownContext { is_open, toggle, close };
    use_context_provider(|| context);

    rsx! {
        div { class: "relative inline-block text-left", {props.children} }
    }
}

// Trigger component
#[derive(Props, PartialEq, Clone)]
pub struct DropdownMenuTriggerProps {
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub disabled: Option<bool>,
    #[props(default)]
    pub children: Element,
}

#[component]
pub fn DropdownMenuTrigger(props: DropdownMenuTriggerProps) -> Element {
    let context = use_context::<DropdownContext>();

    let class_name = tw_merge!(
        "inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50",
        props.class
    );

    rsx! {
        button {
            class: "{class_name}",
            disabled: props.disabled.unwrap_or(false),
            onclick: move |_| context.toggle.call(()),
            aria_expanded: context.is_open,
            aria_haspopup: "menu",
            {props.children}
        }
    }
}

// Content component
#[derive(Props, PartialEq, Clone)]
pub struct DropdownMenuContentProps {
    #[props(default)]
    pub class: Option<String>,
    #[props(default = 4)]
    pub side_offset: i32,
    #[props(default)]
    pub align: Option<String>, // "start" | "center" | "end"
    #[props(default)]
    pub children: Element,
}

#[component]
pub fn DropdownMenuContent(props: DropdownMenuContentProps) -> Element {
    let context = use_context::<DropdownContext>();

    let align_class = match props.align.as_deref() {
        Some("start") => "left-0",
        Some("end") => "right-0",
        _ => "left-1/2 -translate-x-1/2", // center (default)
    };

    let class_name = tw_merge!(
        "absolute z-50 min-w-[8rem] overflow-hidden rounded-md border bg-white p-1 text-gray-900 shadow-md",
        "data-[state=open]:animate-in data-[state=closed]:animate-out",
        "data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0",
        "data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95",
        align_class,
        props.class
    );

    // Close on outside click
    let close_on_outside_click = use_callback(move |_| {
        context.close.call(());
    });

    rsx! {
        if context.is_open.read().clone() {
            div { class: "fixed inset-0 z-40", onclick: close_on_outside_click }
            div {
                class: "{class_name}",
                style: format!("top: calc(100% + {}px)", props.side_offset),
                role: "menu",
                onclick: |e| e.stop_propagation(), // Prevent closing when clicking inside
                {props.children}
            }
        }
    }
}

// Menu item component
#[derive(Props, PartialEq, Clone)]
pub struct DropdownMenuItemProps {
    #[props(default)]
    pub class: Option<String>,
    #[props(default = false)]
    pub inset: bool,
    #[props(default)]
    pub disabled: Option<bool>,
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    #[props(default)]
    pub children: Element,
}

#[component]
pub fn DropdownMenuItem(props: DropdownMenuItemProps) -> Element {
    let context = use_context::<DropdownContext>();

    let class_name = tw_merge!(
        "relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none transition-colors",
        "focus:bg-gray-100 focus:text-gray-900 hover:bg-gray-100 hover:text-gray-900",
        "data-[disabled]:pointer-events-none data-[disabled]:opacity-50",
        if props.inset { "pl-8" } else { "" },
        props.class
    );

    let handle_click = move |e: MouseEvent| {
        if !props.disabled.unwrap_or(false) {
            if let Some(handler) = &props.onclick {
                handler.call(e);
            }
            context.close.call(());
        }
    };

    rsx! {
        div {
            class: "{class_name}",
            role: "menuitem",
            tabindex: if props.disabled.unwrap_or(false) { -1 } else { 0 },
            onclick: handle_click,
            {props.children}
        }
    }
}

// Group component
#[derive(Props, PartialEq, Clone)]
pub struct DropdownMenuGroupProps {
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub children: Element,
}

#[component]
pub fn DropdownMenuGroup(props: DropdownMenuGroupProps) -> Element {
    rsx! {
        div { class: props.class, role: "group", {props.children} }
    }
}

// Label component
#[derive(Props, PartialEq, Clone)]
pub struct DropdownMenuLabelProps {
    #[props(default)]
    pub class: Option<String>,
    #[props(default = false)]
    pub inset: bool,
    #[props(default)]
    pub children: Element,
}

#[component]
pub fn DropdownMenuLabel(props: DropdownMenuLabelProps) -> Element {
    let class_name = tw_merge!(
        "px-2 py-1.5 text-sm font-semibold text-gray-900",
        if props.inset { "pl-8" } else { "" },
        props.class
    );

    rsx! {
        div { class: "{class_name}", role: "presentation", {props.children} }
    }
}

// Separator component
#[derive(Props, PartialEq, Clone)]
pub struct DropdownMenuSeparatorProps {
    #[props(default)]
    pub class: Option<String>,
}

#[component]
pub fn DropdownMenuSeparator(props: DropdownMenuSeparatorProps) -> Element {
    let class_name = tw_merge!(
        "-mx-1 my-1 h-px bg-gray-300",
        props.class
    );

    rsx! {
        div { class: "{class_name}", role: "separator" }
    }
}

// Checkbox item component
#[derive(Props, PartialEq, Clone)]
pub struct DropdownMenuCheckboxItemProps {
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub checked: Option<bool>,
    #[props(default)]
    pub disabled: Option<bool>,
    #[props(default)]
    pub on_checked_change: Option<EventHandler<bool>>,
    #[props(default)]
    pub children: Element,
}

#[component]
pub fn DropdownMenuCheckboxItem(props: DropdownMenuCheckboxItemProps) -> Element {
    let context = use_context::<DropdownContext>();

    let class_name = tw_merge!(
        "relative flex cursor-default select-none items-center rounded-sm py-1.5 pl-8 pr-2 text-sm outline-none transition-colors",
        "focus:bg-gray-100 focus:text-gray-900 hover:bg-gray-100 hover:text-gray-900",
        "data-[disabled]:pointer-events-none data-[disabled]:opacity-50",
        props.class
    );

    let handle_click = move |_: MouseEvent| {
        if !props.disabled.unwrap_or(false) {
            let new_checked = !props.checked.unwrap_or(false);
            if let Some(handler) = &props.on_checked_change {
                handler.call(new_checked);
            }
        }
    };

    rsx! {
        div {
            class: "{class_name}",
            role: "menuitemcheckbox",
            aria_checked: props.checked.unwrap_or(false),
            tabindex: if props.disabled.unwrap_or(false) { -1 } else { 0 },
            onclick: handle_click,
            span { class: "absolute left-2 flex h-3.5 w-3.5 items-center justify-center",
                if props.checked.unwrap_or(false) {
                    svg {
                        class: "h-4 w-4",
                        xmlns: "http://www.w3.org/2000/svg",
                        width: "24",
                        height: "24",
                    }
                }
            }
        }
    }
}