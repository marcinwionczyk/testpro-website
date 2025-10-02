use dioxus::prelude::*;
use lucide_dioxus::{ArrowUp, Github, Linkedin, Mail, Phone};
use web_sys::js_sys::Date;
use crate::Language;
use crate::components::ui::{Button, ButtonVariant, Separator};

#[component]
pub fn Footer() -> Element {
    let current_lang = use_context::<Signal<Language>>();
    let current_year = Date::new_0().get_full_year();

    let quick_links = [
        ("Home", "#home"),
        ("Services", "#services"),
        ("About", "#about"),
        ("Blog", "#blog"),
        ("Contact", "#contact")
    ];

    let services = vec![
        ("Manual Testing", "#services"),
        ("Test Automation", "#services"),
        ("QA Consulting", "#services"),
        ("Rust Development", "#services"),
    ];

    match current_lang() {
        Language::EN => rsx! {
            footer { class: "bg-secondary text-white",
                div { class: "container mx-auto px-4 sm:px-6 lg:px-8 py-16",
                    div { class: "grid lg:grid-cols-4 md:grid-cols-2 gap-8",
                        // Company info
                        div { class: "space-y-4",
                            div {
                                h3 { class: "text-2xl font-bold text-primary mb-2",
                                    "TEST-PRO"
                                }
                                p { class: "text-sm opacity-90", "Marcin Wionczyk" }
                            }
                            p { class: "text-sm opacity-80 leading-relaxed",
                                "Professional software testing services helping businesses deliver high-quality software products with confidence and reliability."
                            }
                            div { class: "space-y-2",
                                div { class: "flex items-center space-x-2 text-sm",
                                    Mail { class: "h-4 w-4 text-primary" }
                                    a {
                                        href: "mailto:testpro@marcinwionczyk.ovh",
                                        class: "hover:text-primary transition-colors",
                                        "testpro@marcinwionczyk.ovh"
                                    }
                                }
                                div { class: "flex items-center space-x-2 text-sm",
                                    Phone { class: "h-4 w-4 text-primary" }
                                    a {
                                        href: "tel:+48508169085",
                                        class: "hover:text-primary transition-colors",
                                        "+48 508 169 085"
                                    }
                                }
                            }
                        }
                        // Quick links
                        div {
                            h4 { class: "text-lg font-semibold mb-4", "Quick Links" }
                            ul { class: "space-ry-2",
                                for (name , href) in quick_links.iter() {
                                    li {
                                        a {
                                            href: href.to_string(),
                                            class: "text-sm opacity-80 hover:opacity-100 hover:text-primary transition-all duration-200",
                                            "{name}"
                                        }
                                    }
                                }
                            }
                        }
                        // Services
                        div {
                            h4 { class: "text-lg font-semibold mb-4", "Services" }
                            ul { class: "space-ry-2",
                                for service in services.iter() {
                                    li {
                                        a {
                                            href: service.1.to_string(),
                                            class: "text-sm opacity-80 hover:opacity-100 hover:text-primary transition-all duration-200",
                                            "{service.0}"
                                        }
                                    }
                                }
                            }
                        }
                        // Contact & Social
                        div {
                            h4 { class: "text-lg font-semibold mb-4", "Let's connect" }
                            p { class: "text-sm opacity-80",
                                "Ready to discuss your testing needs? Get in touch for a free consultation."
                            }
                            Button {
                                style: "margin: 1em 0 1em 0",
                                variant: ButtonVariant::Secondary,
                                a { href: "#contact", "Start a project" }
                            }
                            // Social Links
                            div {
                                p { class: "text-sm font-medium mb-3", "Follow Me" }
                                div { class: "flex space-x-3",
                                    a {
                                        href: "https://linkedin.com/in/marcin-wionczyk",
                                        target: "_blank",
                                        rel: "noopener noreferrer",
                                        class: "p-2 bg-primary/10 rounded-lg hover:bg-primary hover:text-primary-foreground transition-all duration-200",
                                        Linkedin { class: "h-4 w-4" }
                                    }
                                    a {
                                        href: "https://github.com/marcin-wionczyk",
                                        target: "_blank",
                                        rel: "noopener noreferrer",
                                        p { class: "p-2 bg-primary/10 rounded-lg hover:bg-primary hover:text-primary-foreground transition-all duration-200",
                                            Github { class: "h-4 w-4" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Separator { horizontal: true, decorative: true }
                    // Bottom footer
                    div { class: "flex flex-col md:flex-row justify-between items-center space-y-4 md:space-y-0",
                        div { class: "justify-between items-center space-y-4 md:space-y-0",
                            div { class: "text-sm opacity-80",
                                "© {current_year} TEST-PRO - Marcin Wionczyk. All rights reserved."
                            }
                        }
                        div { class: "flex items-center space-x-6",
                            a {
                                href: "#",
                                class: "text-sm opacity-80 hover:opacity-100 transition-opacity",
                                "Privacy Policy"
                            }
                            a {
                                href: "#",
                                class: "text-sm opacity-80 hover:opacity-100 transition-opacity",
                                "Terms of Service"
                            }
                        }
                    }
                    // Business info
                    div { class: "mt-6 pt-6 border-t border-primary/20",
                        div { class: "text-center",
                            p { class: "text-xs opacity-70",
                                "TEST-PRO is a sole proprietorship registered in Poland | Tax ID: PL8992376763"
                            }
                        }
                    }
                }
            }
        },
        Language::PL => rsx! {
            footer { class: "bg-secondary text-white",
                div { class: "container mx-auto px-4 sm:px-6 lg:px-8 py-16",
                    div { class: "grid lg:grid-cols-4 md:grid-cols-2 gap-8",
                        // Company info
                        div { class: "space-y-4",
                            div {
                                h3 { class: "text-2xl font-bold text-primary mb-2",
                                    "TEST-PRO"
                                }
                                p { class: "text-sm opacity-90", "Marcin Wionczyk" }
                            }
                            p { class: "text-sm opacity-80 leading-relaxed",
                                "Profesjonalne usługi testowania oprogramowania pomagające firmom dostarczać wysokiej jakości produkty programistyczne z pewnością i niezawodnością."
                            }
                            div { class: "space-y-2",
                                div { class: "flex items-center space-x-2 text-sm",
                                    Mail { class: "h-4 w-4 text-primary" }
                                    a {
                                        href: "mailto:testpro@marcinwionczyk.ovh",
                                        class: "hover:text-primary transition-colors",
                                        "testpro@marcinwionczyk.ovh"
                                    }
                                }
                                div { class: "flex items-center space-x-2 text-sm",
                                    Phone { class: "h-4 w-4 text-primary" }
                                    a {
                                        href: "tel:+48508169085",
                                        class: "hover:text-primary transition-colors",
                                        "+48 508 169 085"
                                    }
                                }
                            }
                        }
                        // Quick links
                        div {
                            h4 { class: "text-lg font-semibold mb-4", "Szybkie łącza" }
                            ul { class: "space-ry-2",
                                for (name , href) in quick_links.iter() {
                                    li {
                                        a {
                                            href: href.to_string(),
                                            class: "text-sm opacity-80 hover:opacity-100 hover:text-primary transition-all duration-200",
                                            "{name}"
                                        }
                                    }
                                }
                            }
                        }
                        // Services
                        div {
                            h4 { class: "text-lg font-semibold mb-4", "Usługi" }
                            ul { class: "space-ry-2",
                                for service in services.iter() {
                                    li {
                                        a {
                                            href: service.1.to_string(),
                                            class: "text-sm opacity-80 hover:opacity-100 hover:text-primary transition-all duration-200",
                                            "{service.0}"
                                        }
                                    }
                                }
                            }
                        }
                        // Contact & Social
                        div {
                            h4 { class: "text-lg font-semibold mb-4", "Let's connect" }
                            p { class: "text-sm opacity-80",
                                "Chcesz omówić swoje potrzeby testowe? Skontaktuj się z nami, aby umówić się na bezpłatną konsultację."
                            }
                            Button {
                                style: "margin: 1em 0 1em 0",
                                variant: ButtonVariant::Secondary,
                                a { href: "#contact", "Rozpocznij projekt" }
                            }
                            // Social Links
                            div {
                                p { class: "text-sm font-medium mb-3", "Follow Me" }
                                div { class: "flex space-x-3",
                                    a {
                                        href: "https://linkedin.com/in/marcin-wionczyk",
                                        target: "_blank",
                                        rel: "noopener noreferrer",
                                        class: "p-2 bg-primary/10 rounded-lg hover:bg-primary hover:text-primary-foreground transition-all duration-200",
                                        Linkedin { class: "h-4 w-4" }
                                    }
                                    a {
                                        href: "https://github.com/marcin-wionczyk",
                                        target: "_blank",
                                        rel: "noopener noreferrer",
                                        p { class: "p-2 bg-primary/10 rounded-lg hover:bg-primary hover:text-primary-foreground transition-all duration-200",
                                            Github { class: "h-4 w-4" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Separator { horizontal: true, decorative: true }
                    // Bottom footer
                    div { class: "flex flex-col md:flex-row justify-between items-center space-y-4 md:space-y-0",
                        div { class: "justify-between items-center space-y-4 md:space-y-0",
                            div { class: "text-sm opacity-80",
                                "© {current_year} TEST-PRO - Marcin Wionczyk. Wszystkie prawa zastrzeżone."
                            }
                        }
                        div { class: "flex items-center space-x-6",
                            a {
                                href: "#",
                                class: "text-sm opacity-80 hover:opacity-100 transition-opacity",
                                "Polityka prywatności"
                            }
                            a {
                                href: "#",
                                class: "text-sm opacity-80 hover:opacity-100 transition-opacity",
                                "Warunki korzystania z usług"
                            }
                        }
                    }
                    // Business info
                    div { class: "mt-6 pt-6 border-t border-primary/20",
                        div { class: "text-center",
                            p { class: "text-xs opacity-70",
                                "TEST-PRO jest jednoosobową działalnością gospodarczą zarejestrowaną w Polsce | NIP: PL8992376763"
                            }
                        }
                    }
                }
            }
        }
    }
}