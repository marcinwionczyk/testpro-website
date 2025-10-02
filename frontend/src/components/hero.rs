use crate::Language;
use dioxus::html::code::language;
use dioxus::html::g::format;
use crate::components::ui::*;
use dioxus::prelude::*;
use lucide_dioxus::*;

const HERO_IMAGE: Asset = asset!("/assets/hero-testing.jpg");

#[component]
pub fn Hero() -> Element {
    let current_lang = use_context::<Signal<Language>>();
    let projects_tested = "6";
    let bugs_detected = "100";
    match current_lang() {
        Language::EN => rsx! {
            section {
                id: "home",
                class: "relative min-h-screen flex items-center bg-gradient-to-br from-blue-50 to-indigo-100",
                div { class: "container mx-auto px-4 sm:px-6 lg:px-8 py-20",
                    div { class: "grid lg:grid-cols-2 gap-12 items-center",
                        div { class: "space-y-8",
                            div { class: "space-y-4",
                                h1 { class: "text-4xl sm:text-4xl lg:text-5xl font-bold text-gray-900 leading-tight",
                                    "Professional "
                                    span { class: "text-blue-600 block  text-4xl", "Software Testing" }
                                    "Services"
                                }
                                p { class: "text-xl text-gray-600 max-w-xl",
                                    "Ensure your software quality with comprehensive testing solutions. From manual testing to automation - I deliver reliable results for your business."
                                }
                            }

                            div { class: "space-y-3",
                                for item in [
                                    "Manual & Automated Testing",
                                    "Quality Assurance Consulting",
                                    "Bug Detection & Reporting",
                                    "Performance Testing",
                                ]
                                    .iter()
                                {
                                    div { class: "flex items-center space-x-3",
                                        CircleCheck { class: "text-green-500 text-lg" }
                                        span { class: "text-gray-700 font-medium", "{item}" }
                                    }
                                }
                            }

                            div { class: "flex flex-col sm:flex-row gap-4",
                                Button { variant: ButtonVariant::Destructive, "Get Free Consultation" }
                                Button { variant: ButtonVariant::Primary, "View Portfolio" }
                            }
                        }

                        div { class: "relative",
                            div { class: "relative rounded-2xl overflow-hidden shadow-2xl",
                                img {
                                    src: {HERO_IMAGE},
                                    alt: "Professional software testing setup",
                                    class: "w-full h-auto object-cover",
                                }
                                div { class: "absolute inset-0 bg-gradient-to-tr from-blue-600/20 to-transparent" }
                            }

                            // Floating stats card
                            div { class: "absolute -bottom-6 -left-6 bg-white p-6 rounded-xl shadow-lg border",
                                div { class: "flex items-center space-x-4",
                                    div { class: "text-center",
                                        div { class: "text-2xl font-bold text-blue-600",
                                            "{projects_tested}+"
                                        }
                                        div { class: "text-sm text-gray-600",
                                            "Participation in projects"
                                        }
                                    }
                                    div { class: "w-px h-12 bg-gray-300" }
                                    div { class: "text-center",
                                        div { class: "text-2xl font-bold text-blue-600",
                                            "{bugs_detected}+"
                                        }
                                        div { class: "text-sm text-gray-600", "Bugs detected" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        },
        Language::PL => rsx! {
            section {
                id: "home",
                class: "relative min-h-screen flex items-center bg-gradient-to-br from-blue-50 to-indigo-100",
                div { class: "container mx-auto px-4 sm:px-6 lg:px-8 py-20",
                    div { class: "grid lg:grid-cols-2 gap-12 items-center",
                        div { class: "space-y-8",
                            div { class: "space-y-4",
                                h1 { class: "text-4xl sm:text-4xl lg:text-5xl font-bold text-gray-900 leading-tight",
                                    "Profesjonalne Usługi"
                                    span { class: "text-blue-600 block text-4xl",
                                        "Testowania Oprogramowania"
                                    }
                                }
                                p { class: "text-xl text-gray-600 max-w-xl",
                                    "Zapewnij jakość swojego oprogramowania dzięki kompleksowym rozwiązaniom testowym. Od testów manualnych po automatyzację – dostarczam niezawodne rezultaty dla Twojej firmy."
                                }
                            }

                            div { class: "space-y-3",
                                for item in [
                                    "Testy manualne i automatyczne",
                                    "Konsultacje odnośnie jakości oprogramowania",
                                    "Wykrywanie błędów i raportowanie",
                                    "Testowanie wydajności",
                                ]
                                    .iter()
                                {
                                    div { class: "flex items-center space-x-3",
                                        CircleCheck { class: "text-green-500 text-lg" }
                                        span { class: "text-gray-700 font-medium", "{item}" }
                                    }
                                }
                            }

                            div { class: "flex flex-col sm:flex-row gap-4",
                                Button { variant: ButtonVariant::Destructive,
                                    "Uzyskaj darmowe konsultacje"
                                }
                                Button { variant: ButtonVariant::Primary, "Zobacz Portfolio" }
                            }
                        }

                        div { class: "relative",
                            div { class: "relative rounded-2xl overflow-hidden shadow-2xl",
                                img {
                                    src: {HERO_IMAGE},
                                    alt: "Professional software testing setup",
                                    class: "w-full h-auto object-cover",
                                }
                                div { class: "absolute inset-0 bg-gradient-to-tr from-blue-600/20 to-transparent" }
                            }

                            // Floating stats card
                            div { class: "absolute -bottom-6 -left-6 bg-white p-6 rounded-xl shadow-lg border",
                                div { class: "flex items-center space-x-4",
                                    div { class: "text-center",
                                        div { class: "text-2xl font-bold text-blue-600",
                                            "{projects_tested}+"
                                        }
                                        div { class: "text-sm text-gray-600",
                                            "Uczestnictwo w projektach"
                                        }
                                    }
                                    div { class: "w-px h-12 bg-gray-300" }
                                    div { class: "text-center",
                                        div { class: "text-2xl font-bold text-blue-600",
                                            "{bugs_detected}+"
                                        }
                                        div { class: "text-sm text-gray-600", "Wykryte błędy" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        },
    }
}
