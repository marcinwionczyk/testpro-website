use crate::components::ui::{BadgeVariant, DxsBadge, DxsCard, DxsCardContent};
use crate::Language;
use dioxus::prelude::*;
use lucide_dioxus::{
    Award, AwardProps, BookOpen, BookOpenProps, Calendar, CircleCheck, FileCode2, FileCode2Props,
    Users, UsersProps,
};

struct Experience {
    icon: Element,
    title_en: &'static str,
    title_pl: &'static str,
    description_en: &'static str,
    description_pl: &'static str,
}

#[component]
pub fn About() -> Element {
    let current_lang = use_context::<Signal<Language>>();
    let skills_en = [
        "Manual Testing",
        "Test Automation",
        "Selenium WebDriver",
        "Cypress",
        "API Testing",
        "Performance Testing",
        "Security Testing",
        "Agile/Scrum",
        "JIRA",
        "TestRail",
        "Git",
        "CI/CD",
        "Rust Programming",
        "Quality Assurance",
    ];
    let skills_pl = [
        "testowanie ręczne",
        "Automatyzacja testów",
        "Selenium",
        "Cypress",
        "Testowanie APi",
        "Testowanie wydajności",
        "Testoewanie bezpieczeństwa",
        "Agile/Scrum",
        "JIRA",
        "TestRail",
        "Git",
        "CI/CD",
        "Programowanie w Rust",
        "Zapewnienie jakości",
    ];

    let experience = [
        Experience {
            icon: Award(AwardProps::builder().class("h-6 w-6 text-primary").build()),
            title_en: "5+ Years Experience",
            title_pl: "5+ lat doświadczenia",
            description_en: "Extensive experience in software testing across various industries and project types.",
            description_pl: "Bogate doświadczenie w testowaniu oprogramowania w różnych branżach i typach projektów",
        },
        Experience {
            icon: Users(UsersProps::builder().class("h-6 w-6 text-primary").build()),
            title_en: "5+ happy clients",
            title_pl: "5+ zadowolonych klientów",
            description_en: "Delivered quality testing services to startups, SMEs, and enterprise clients.",
            description_pl: "Dostarczenie wysokiej jakości usług testowych dla startupów, małych i średnich przedsiębiorstw oraz klientów korporacyjnych.",
        },
        Experience {
            icon: FileCode2(FileCode2Props::builder().class("h-6 w-6 text-primary").build()),
            title_en: "5+ Projects tested",
            title_pl: "5+ przetestowanych projektów",
            description_en: "Successfully tested web applications, mobile apps, desktop software and embedded applications.",
            description_pl: "Pomyślnie testowałem aplikacje internetowe, aplikacje mobilne, Oprogramowanie desktopowe i aplikacje wbudowane.",
        },
        Experience {
            icon: BookOpen(BookOpenProps::builder().class("h-6 w-6 text-primary").build()),
            title_en: "Continuous Learning",
            title_pl: "Ciągła nauka",
            description_en: "Staying updated with latest testing tools, methodologies, and industry best practices.",
            description_pl: "Bycie na bieżąco z najnowszymi narzędziami testowymi, metodologiami i najlepszymi praktykami branżowymi.",
        }
    ];

    match current_lang() {
        Language::EN => rsx! {
            section { id: "about", class: "py-20 bg-gray-50",
                div { class: "container mx-auto px-4 sm:px-6 lg:px-8",
                    div { class: "grid lg:grid-cols-2 gap-12 items-start",
                        // About content
                        div { class: "space-y-8",
                            div {
                                h2 { class: "text-3xl sm:text-4xl font-bold text-foreground mb-6",
                                    "About Marcin Wionczyk"
                                }
                                div { class: "space-y-4 text-muted-foreground",
                                    p { class: "text-lg",
                                        "I'm a passionate software tester with over 5 years of experience in ensuring software quality across diverse projects. My expertise spans from manual testing to advanced test automation, helping businesses deliver reliable software products."
                                    }
                                    p {
                                        "As the founder of TEST-PRO, I specialize in comprehensive quality assurance solutions that reduce bugs, improve user experience, and accelerate development cycles. I work closely with development teams to implement testing strategies that align with business objectives."
                                    }
                                    p {
                                        "Currently expanding my skill set into Rust development, I'm excited to offer high-performance, memory-safe software solutions alongside my testing expertise."
                                    }
                                }
                            }
                            // Skills
                            div {
                                h3 { class: "text-xl font-semibold text-foreground mb-4",
                                    "Technical Skills"
                                }
                                div { class: "flex flex-wrap gap-2",
                                    for skill in skills_en.iter() {
                                        DxsBadge {
                                            variant: BadgeVariant::Secondary,
                                            class: "text-sm",
                                            "{skill}"
                                        }
                                    }
                                }
                            }
                            //Certifications
                            div { class: "bg-primary-light p-6 rounded-xl",
                                h3 { class: "text-lg font-semibold text-foreground mb-3 flex items-center",
                                    Award { class: "h-5 w-5 text-primary mr-2" }
                                    "Certifications & Education"
                                }
                                div { class: "space-y-2 text-muted-foreground",
                                    div { class: "flex items-center space-x-2",
                                        CircleCheck { class: "h-4 w-4 text-primary flex-shrink-0" }
                                        span { "ISTQB Certified Tester - Foundation Level" }
                                    }
                                    div { class: "flex items-center space-x-2",
                                        CircleCheck { class: "h-4 w-4 text-primary flex-shrink-0" }
                                        span { "Let's Get Rusty's Rust Developer Bootcamp" }
                                    }
                                    div { class: "flex items-center space-x-2" }
                                }
                            }
                        }
                        // Experience stats
                        div { class: "space-y-6",
                            DxsCard { class: "bg-gradient-hero text-primary-foreground shadow-hover",
                                DxsCardContent { class: "p-6 text-center",
                                    div { class: "text-4xl font-bold", "5+" }
                                    div { class: "text-lg opacity-90", "Years of Experience" }
                                }
                            }
                            div { class: "grid grid-cols-2 gap-6",
                                for experience in experience.iter() {
                                    DxsCard { class: "shadow-card hover:shadow-hover transition-all duration-300",
                                        DxsCardContent { class: "p-6 text-center space-y-3",
                                            div { class: "inline-flex p-3 bg-primary-light rounded-full",
                                                {experience.icon.clone()}
                                            }
                                            h4 { class: "font-semibold text-foreground text-sm",
                                                "{experience.title_en}"
                                            }
                                            p { class: "text-xs text-muted-foreground leading-relaxed",
                                                "{experience.description_en}"
                                            }
                                        }
                                    }
                                }
                            }
                            // Timeline
                            DxsCard { class: "shadow-card",
                                DxsCardContent { class: "p-6",
                                    h3 { class: "text-lg font-semibold text-foreground mb-4 flex items-center",
                                        Calendar { class: "h-5 w-5 text-primary mr-2" }
                                        "Career Timeline"
                                    }
                                    div { class: "space-y-4",
                                        div { class: "flex items-start space-x-3",
                                            div { class: "w-2 h-2 bg-primary rounded-full mt-2 flex-shrink-0" }
                                            div {
                                                div { class: "font-medium text-foreground",
                                                    "Present - Future"
                                                }
                                                div { class: "text-sm text-muted-foreground",
                                                    "I want to be (part time) freelance software tester, Rust developer"
                                                }
                                            }
                                        }
                                        div { class: "flex items-start space-x-3",
                                            div { class: "w-2 h-2 bg-primary rounded-full mt-2 flex-shrink-0" }
                                            div {
                                                div { class: "font-medium text-foreground",
                                                    "2022 - Present"
                                                }
                                                div { class: "text-sm text-muted-foreground",
                                                    "API testing at Nordea Bank"
                                                }
                                            }
                                        }
                                        div { class: "flex items-start space-x-3",
                                            div { class: "w-2 h-2 bg-primary rounded-full mt-2 flex-shrink-0" }
                                            div {
                                                div { class: "font-medium text-foreground",
                                                    "2016 - 2022"
                                                }
                                                div { class: "text-sm text-muted-foreground",
                                                    "Embedded test automation engineer"
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
        },
        Language::PL => rsx! {
            section { id: "about", class: "py-20 bg-gray-50",
                div { class: "container mx-auto px-4 sm:px-6 lg:px-8",
                    div { class: "grid lg:grid-cols-2 gap-12 items-start",
                        // About content
                        div { class: "space-y-8",
                            div {
                                h2 { class: "text-3xl sm:text-4xl font-bold text-foreground mb-6",
                                    "O mnie"
                                }
                                div { class: "space-y-4 text-muted-foreground",
                                    p { class: "text-lg",
                                        "Jestem pasjonatem testowania oprogramowania z ponad 5-letnim doświadczeniem w zapewnianiu jakości oprogramowania w różnorodnych projektach. Moje doświadczenie obejmuje zarówno testy manualne, jak i zaawansowaną automatyzację testów, pomagając firmom dostarczać niezawodne produkty programistyczne."
                                    }
                                    p {
                                        "Jako założyciel TEST-PRO, specjalizuję się w kompleksowych rozwiązaniach z zakresu zapewnienia jakości, które redukują liczbę błędów, poprawiają doświadczenia użytkowników i przyspieszają cykle rozwoju oprogramowania. Ściśle współpracuję z zespołami programistycznymi, wdrażając strategie testowania zgodne z celami biznesowymi."
                                    }
                                    p {
                                        "Obecnie poszerzam swoje umiejętności o programowanie w języku Rust i chętnie zaoferuję wydajne, bezpieczne pod względem pamięci rozwiązania programistyczne, oprócz mojej wiedzy z zakresu testowania."
                                    }
                                }
                            }
                            // Skills
                            div {
                                h3 { class: "text-xl font-semibold text-foreground mb-4",
                                    "Umiejętności techniczne"
                                }
                                div { class: "flex flex-wrap gap-2",
                                    for skill in skills_pl.iter() {
                                        DxsBadge {
                                            variant: BadgeVariant::Secondary,
                                            class: "text-sm",
                                            "{skill}"
                                        }
                                    }
                                }
                            }
                            //Certifications
                            div { class: "bg-primary-light p-6 rounded-xl",
                                h3 { class: "text-lg font-semibold text-foreground mb-3 flex items-center",
                                    Award { class: "h-5 w-5 text-primary mr-2" }
                                    "Certyfikaty i edukacja"
                                }
                                div { class: "space-y-2 text-muted-foreground",
                                    div { class: "flex items-center space-x-2",
                                        CircleCheck { class: "h-4 w-4 text-primary flex-shrink-0" }
                                        span { "Certyfikowany tester ISTQB - Poziom podstawowy" }
                                    }
                                    div { class: "flex items-center space-x-2",
                                        CircleCheck { class: "h-4 w-4 text-primary flex-shrink-0" }
                                        span { "Let's Get Rusty's Rust Developer Bootcamp" }
                                    }
                                    div { class: "flex items-center space-x-2" }
                                }
                            }
                        }
                        // Experience stats
                        div { class: "space-y-6",
                            DxsCard { class: "bg-gradient-hero text-primary-foreground shadow-hover",
                                DxsCardContent { class: "p-6 text-center",
                                    div { class: "text-4xl font-bold", "5+" }
                                    div { class: "text-lg opacity-90", "lat doświadczeń" }
                                }
                            }
                            div { class: "grid grid-cols-2 gap-6",
                                for experience in experience.iter() {
                                    DxsCard { class: "shadow-card hover:shadow-hover transition-all duration-300",
                                        DxsCardContent { class: "p-6 text-center space-y-3",
                                            div { class: "inline-flex p-3 bg-primary-light rounded-full",
                                                {experience.icon.clone()}
                                            }
                                            h4 { class: "font-semibold text-foreground text-sm",
                                                "{experience.title_pl}"
                                            }
                                            p { class: "text-xs text-muted-foreground leading-relaxed",
                                                "{experience.description_pl}"
                                            }
                                        }
                                    }
                                }
                            }
                            // Timeline
                            DxsCard { class: "shadow-card",
                                DxsCardContent { class: "p-6",
                                    h3 { class: "text-lg font-semibold text-foreground mb-4 flex items-center",
                                        Calendar { class: "h-5 w-5 text-primary mr-2" }
                                        "Ścieżka kariery"
                                    }
                                    div { class: "space-y-4",
                                        div { class: "flex items-start space-x-3",
                                            div { class: "w-2 h-2 bg-primary rounded-full mt-2 flex-shrink-0" }
                                            div {
                                                div { class: "font-medium text-foreground",
                                                    "W niedalekiej przyszłości"
                                                }
                                                div { class: "text-sm text-muted-foreground",
                                                    "chce pracować jako niezależny tester oprogramowania i programista Rust"
                                                }
                                            }
                                        }
                                        div { class: "flex items-start space-x-3",
                                            div { class: "w-2 h-2 bg-primary rounded-full mt-2 flex-shrink-0" }
                                            div {
                                                div { class: "font-medium text-foreground",
                                                    "2022 - Obecnie"
                                                }
                                                div { class: "text-sm text-muted-foreground",
                                                    "Tester API w banku Nordea"
                                                }
                                            }
                                        }
                                        div { class: "flex items-start space-x-3",
                                            div { class: "w-2 h-2 bg-primary rounded-full mt-2 flex-shrink-0" }
                                            div {
                                                div { class: "font-medium text-foreground",
                                                    "2016 - 2022"
                                                }
                                                div { class: "text-sm text-muted-foreground",
                                                    "Inżynier automatyzacji testów wbudowanych"
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
        },
    }
}
