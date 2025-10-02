use crate::components::ui::*;
use crate::Language;
use dioxus::prelude::*;
use lucide_dioxus::*;

struct Service {
    icon: Element,
    title: &'static str,
    description: &'static str,
    features: Vec<&'static str>,
    price: &'static str,
    badge: Option<String>,
}

struct Benefit {
    icon: Element,
    title: &'static str,
    description: &'static str,
}

#[component]
pub fn Services() -> Element {
    let current_lang = use_context::<Signal<Language>>();
    let services_en = vec![
        Service {
            icon: Bug(BugProps::builder().class("h-6 w-6 text-primary group-hover:text-primary-foreground").build()),
            title: "Manual Testing",
            description: "Comprehensive manual testing to identify bugs and usability issues before your users do.",
            features: vec!["Functional Testing", "Usability Testing", "Exploratory Testing", "Regression Testing"],
            price: "From $30/hour",
            badge: None,
        },
        Service {
            icon: Zap(ZapProps::builder().class("h-6 w-6 text-primary group-hover:text-primary-foreground").build()),
            title: "Test Automation",
            description: "Automated testing solutions to speed up your development cycle and ensure consistent quality.",
            features: vec!["API Testing", "UI Automation", "Performance Testing", "CI/CD Integration"],
            price: "From $50/hour",
            badge: None,
        },
        Service {
            icon: Code(CodeProps::builder().class("h-6 w-6 text-primary group-hover:text-primary-foreground").build()),
            title: "Rust Development",
            description: "High-performance, memory-safe applications built with Rust for system-level programming.",
            features: vec!["System Programming", "Web Backends", "CLI Tools", "Performance Optimization"],
            price: "From $60/hour",
            badge: Some("Coming Soon".to_string())
        },
        Service {
            icon: Shield(ShieldProps::builder().class("h-6 w-6 text-primary group-hover:text-primary-foreground").build()),
            title: "QA Consulting",
            description: "Strategic quality assurance consulting to improve your development processes.",
            features: vec!["QA Strategy", "Process Improvement", "Team Training", "Tool Selection"],
            price: "From $75/hour",
            badge: None
        }
    ];
    let services_pl = vec![
        Service {
            icon: Bug(BugProps::builder().class("h-6 w-6 text-primary group-hover:text-primary-foreground").build()),
            title: "Testowanie manualne",
            description: "Kompleksowe testy ręczne w celu zidentyfikowania błędów i problemów z użytecznością, zanim zrobią to użytkownicy.",
            features: vec!["Testy funkcjonalne", "Testy użyteczności", "Testowanie eksploracyjne", "Testy regresyjne"],
            price: "Od 90 zł/godzinę",
            badge: None,
        },
        Service {
            icon: Zap(ZapProps::builder().class("h-6 w-6 text-primary group-hover:text-primary-foreground").build()),
            title: "Automatyzacja testów",
            description: "Rozwiązania do automatycznego testowania, które przyspieszają cykl rozwoju i zapewniają stałą jakość.",
            features: vec!["Testowanie API", "Testowanie interfejsu użytkownika", "Testy wydajności", "Integracji CI/CD"],
            price: "Od 120 zł/godzinę",
            badge: None,
        },
        Service {
            icon: Code(CodeProps::builder().class("h-6 w-6 text-primary group-hover:text-primary-foreground").build()),
            title: "Programowanie w Rust",
            description: "Wydajne, bezpieczne pod względem pamięci aplikacje stworzone w Rust działające na poziomie systemowym",
            features: vec!["Programowanie systemowe", "Backend", "Frontend", "Narzędzia CLI", "Optymalizacja wydajności"],
            price: "Od 180 zł/godzinę",
            badge: Some("Już wkrótce".to_string())
        },
        Service {
            icon: Shield(ShieldProps::builder().class("h-6 w-6 text-primary group-hover:text-primary-foreground").build()),
            title: "Doradztwo w zakresie zapewnienia jakości",
            description: "Strategiczne doradztwo w zakresie zapewnienia jakości w celu usprawnienia procesów rozwoju oprogramowania",
            features: vec!["Strategia Testów", "Usprawnianie procesów", "Trenowanie Zespołów", "Wybór narzędzi"],
            price: "Od 200 zł/godzinę",
            badge: None
        }
    ];

    let benefits_en = vec![
        Benefit {
            icon: Target(TargetProps::builder().class("h-8 w-8 text-primary").build()),
            title: "High Bug Detection Rate",
            description: "Thorough testing processes ensure maximum bug detection",
        },
        Benefit {
            icon: Clock(ClockProps::builder().class("h-8 w-8 text-primary").build()),
            title: "Fast Turnaround",
            description: "Quick testing cycles without compromising quality",
        },
        Benefit {
            icon: Gauge(GaugeProps::builder().class("h-8 w-8 text-primary").build()),
            title: "Performance Focus",
            description: "Optimized for speed and reliability in all testing scenarios",
        },
    ];

    let benefits_pl = vec![
        Benefit{
            icon: Target(TargetProps::builder().build()),
            title: "Wysoki wskaźnik wykrywalności błędów",
            description: "Dokładne procesy testowania zapewniają maksymalną wykrywalność błędów"
        },
        Benefit{
            icon: Clock(ClockProps::builder().build()),
            title: "Szybki czas realizacji",
            description: "Szybkie cykle testowania bez utraty jakości"
        },
        Benefit{
            icon: Gauge(GaugeProps::builder().build()),
            title: "Skupienie na wydajności",
            description: "Zoptymalizowany pod kątem szybkości i niezawodności we wszystkich scenariuszach testowych"
        }
    ];

    match current_lang() {
        Language::EN => rsx! {
            section { id: "services", class: "py-20 bg-white",
                div { class: "container mx-auto px-4 sm:px-6 lg:px-8",
                    div { class: "text-center mb-16",
                        h2 { class: "text-3xl sm:text-4xl font-bold text-gray-900 mb-4",
                            "Professional Testing Services"
                        }
                        p { class: "text-xl text-gray-600 max-w-2xl mx-auto",
                            "From manual testing to automation and beyond - I provide comprehensive quality assurance solutions tailored to your project needs."
                        }
                    }

                    div { class: "grid md:grid-cols-2 lg:grid-cols-3 gap-8 mb-16",
                        for (service) in services_en.iter() {
                            DxsCard { class: "flex flex-col justify-between h-full p-6 border rounded-lg hover:shadow-lg transition-shadow",
                                if service.badge.is_some() {
                                    DxsBadge {
                                        variant: BadgeVariant::Default,
                                        class: "absolute -top-2 -right-2 bg-accent text-accent-foreground",
                                        {service.badge.clone()}
                                    }
                                }
                                DxsCardHeader {
                                    div { class: "flex items-center space-x-3 mb-3",
                                        div { class: "p-2 bg-primary-light rounded-lg hover:bg-primary hover:text-primary-foreground transition-colors duration-300",
                                            {service.icon.clone()}
                                        }
                                        div {
                                            DxsCardTitle { class: "text-xl", "{service.title}" }
                                            div { class: "text-sm font-semibold text-primary",
                                                "{service.price}"
                                            }
                                        }
                                    }
                                    DxsCardDescription { class: "text-base", "{service.description}" }
                                }
                                DxsCardContent {
                                    div { class: "space-y-3",
                                        for (feature) in service.features.iter() {
                                            div { class: "flex items-center space-x-2",
                                                CircleCheck { class: "h-4 w-4 text-primary flex-shrink-0" }
                                                span { class: "text-sm text-muted-foreground",
                                                    "{ feature }"
                                                }
                                            }
                                        }
                                    }
                                    Button {
                                        style: "margin-top: 2em",
                                        variant: ButtonVariant::Secondary,
                                        "Learn More"
                                    }
                                }
                            }
                        }
                    }
                    div { class: "bg-card rounded-2xl p-8 shadow-card",
                        h3 { class: "text-2xl font-bold text-foreground text-center mb-8",
                            "Why choose TEST-PRO"
                        }
                        div { class: "grid md:grid-cols-3 gap-8",
                            for (benefit) in benefits_en.iter() {
                                div { class: "text-center space-y-4",
                                    div { class: "inline-flex p-3 bg-primary-light rounded-full",
                                        {benefit.icon.clone()}
                                    }
                                }
                                h4 { class: "text-lg font-semibold text-foreground",
                                    "{benefit.title}"
                                }
                                p { class: "text-muted-foreground", "{benefit.description}" }
                            }
                        }
                    }
                }
            }
        },
        Language::PL => rsx! {
            section { id: "services", class: "py-20 bg-white",
                div { class: "container mx-auto px-4 sm:px-6 lg:px-8",
                    div { class: "text-center mb-16",
                        h2 { class: "text-3xl sm:text-4xl font-bold text-gray-900 mb-4",
                            "Usługi testowania"
                        }
                        p { class: "text-xl text-gray-600 max-w-2xl mx-auto",
                            "Kompleksowe rozwiązania w zakresie testowania oprogramowania dostosowane do Twoich potrzeb"
                        }
                    }

                    div { class: "grid md:grid-cols-2 lg:grid-cols-3 gap-8 mb-16",
                        for (service) in services_pl.iter() {
                            DxsCard { class: "flex flex-col justify-between h-full p-6 border rounded-lg hover:shadow-lg transition-shadow",
                                if service.badge.is_some() {
                                    DxsBadge {
                                        variant: BadgeVariant::Default,
                                        class: "absolute -top-2 -right-2 bg-accent text-accent-foreground",
                                        {service.badge.clone()}
                                    }
                                }
                                DxsCardHeader {
                                    div { class: "flex items-center space-x-3 mb-3",
                                        div { class: "p-2 bg-primary-light rounded-lg hover:bg-primary hover:text-primary-foreground transition-colors duration-300",
                                            {service.icon.clone()}
                                        }
                                        div {
                                            DxsCardTitle { class: "font-semibold tracking-tight text-xl",
                                                "{service.title}"
                                            }
                                            div { class: "text-sm font-semibold text-primary",
                                                "{service.price}"
                                            }
                                        }
                                    }
                                    DxsCardDescription { class: "text-black text-base", "{service.description}" }
                                }
                                DxsCardContent {
                                    div { class: "space-y-3 mt-4",
                                        for feature in service.features.iter() {
                                            div { class: "flex items-center space-x-2",
                                                CircleCheck { class: "h-4 w-4 text-primary flex-shrink-0" }
                                                span { class: "text-sm text-muted-foreground",
                                                    "{ feature }"
                                                }
                                            }
                                        }
                                    }
                                    DxsCardFooter { class: "space-y-3",
                                        Button {
                                            style: "margin-top: 2em",
                                            variant: ButtonVariant::Secondary,
                                            "Dowiedz się więcej"
                                        }
                                    }
                                }
                            }
                        }
                    }
                    div { class: "bg-card rounded-2xl p-8 shadow-card",
                        h3 { class: "text-2xl font-bold text-foreground text-center mb-8",
                            "Dlaczego warto wybrać TEST-PRO"
                        }
                        div { class: "grid md:grid-cols-3 gap-8",
                            for (benefit) in benefits_pl.iter() {
                                div { class: "text-center space-y-4",
                                    div { class: "inline-flex p-3 bg-primary-light rounded-full",
                                        {benefit.icon.clone()}
                                    }
                                }
                                h4 { class: "text-lg font-semibold text-foreground",
                                    "{benefit.title}"
                                }
                                p { class: "text-muted-foreground", "{benefit.description}" }
                            }
                        }
                    }
                }
            }
        },
    }
}
