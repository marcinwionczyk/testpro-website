use crate::components::ui::{Button, ButtonVariant};
use crate::components::ui::{DropdownMenu, DropdownMenuTrigger, DropdownMenuContent, DropdownMenuItem};
use crate::Language;
use dioxus::prelude::*;
use lucide_dioxus::{Languages, Menu, X};

#[component]
pub fn Header() -> Element {
    let mut is_menu_open = use_signal(|| false);
    let mut current_lang = use_context::<Signal<Language>>();

    let nav_items = vec![
        ("home", "Home", "Strona główna"),
        ("services", "Services", "Usługi"),
        ("about", "About", "O mnie"),
        ("blog", "Blog", "Blog"),
        ("contact", "Contact", "Kontakt"),
    ];

    rsx! {
        header {
            class: "fixed top-0 w-full bg-background/95 backdrop-blur-sm border-b border-border z-50",
            id: "top",
            div { class: "container mx-auto px-4 sm:px-6 lg:px-8",
                div { class: "flex justify-between items-center h-16",
                    div { class: "flex-shrink-0",
                        h1 { class: "text-xl font-bold text-primary", "TEST-PRO" }
                        p { class: "text-xs text-muted-foreground", "Marcin Wionczyk" }
                    }

                    // Desktop Navigation
                    nav { class: "hidden md:flex space-x-8",
                        for (href , en_label , pl_label) in nav_items.iter() {
                            a {
                                href: "#{href}",
                                class: "text-foreground hover:text-primary transition-colors duration-200 text-sm font-medium",
                                {if current_lang() == Language::EN { en_label } else { pl_label }}
                            }
                        }
                    }

                    // Language Switcher & Mobile Menu
                    div { class: "flex items-center space-x-4",
                        DropdownMenu {
                            DropdownMenuTrigger {
                                Button {
                                    variant: ButtonVariant::Ghost,
                                    class: "hidden sm:flex",
                                    Languages { class: "h-4 w-4 mr-2" }
                                    "{ current_lang() }"
                                }
                            }
                            DropdownMenuContent {
                                DropdownMenuItem { onclick: move |_| current_lang.set(Language::EN),
                                    "English"
                                }
                                DropdownMenuItem { onclick: move |_| current_lang.set(Language::PL),
                                    "Polski"
                                }
                            }
                        }

                        // Mobile menu button
                        Button {
                            variant: ButtonVariant::Ghost,
                            class: "md:hidden",
                            onclick: move |_| is_menu_open.set(!is_menu_open()),
                            if is_menu_open() {
                                X { class: "h-5 w-5" }
                            } else {
                                Menu { class: "h-5 w-5" }
                            }
                        }
                    }
                }

                // Mobile Navigation
                if is_menu_open() {
                    div { class: "md:hidden",
                        div { class: "px-2 pt-2 pb-3 space-y-1 sm:px-3 bg-background border-t border-border",
                            for (href , en_label , pl_label) in nav_items.iter() {
                                a {
                                    href: "#{href}",
                                    class: "block px-3 py-2 text-base font-medium text-foreground hover:text-primary transition-colors duration-200",
                                    onclick: move |_| is_menu_open.set(false),
                                    {if current_lang() == Language::EN { en_label } else { pl_label }}
                                }
                            }
                            div {
                                DropdownMenu {
                                    DropdownMenuTrigger {
                                        Button { variant: ButtonVariant::Ghost,
                                            Languages { class: "h-4 w-4 mr-2" }
                                            "{ current_lang() }"
                                        }
                                    }
                                    DropdownMenuContent {
                                        DropdownMenuItem { onclick: move |_| current_lang.set(Language::EN),
                                            "English"
                                        }
                                        DropdownMenuItem { onclick: move |_| current_lang.set(Language::PL),
                                            "Polski"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
