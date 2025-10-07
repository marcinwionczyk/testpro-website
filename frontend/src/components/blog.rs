use std::fmt::Display;
use dioxus::prelude::*;
use lucide_dioxus::{ArrowRight, Calendar, Clock, User};
use wasm_bindgen::JsValue;
use web_sys::js_sys::{Date, JsString};
use crate::components::ui::{BadgeVariant, Button, ButtonVariant, DxsBadge, DxsCard, DxsCardContent, DxsCardDescription, DxsCardHeader, DxsCardTitle, Separator};
use crate::Language;

#[derive(Copy, Clone, PartialEq, Eq)]
enum BlogCategory {
    TestAutomation,
    RustDevelopment,
    QAProcess,
    Performance,
    APITesting
}

impl Display for BlogCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            BlogCategory::TestAutomation => "Test Automation",
            BlogCategory::RustDevelopment => "Rust Development",
            BlogCategory::QAProcess => "QA Process",
            BlogCategory::Performance => "Performance",
            BlogCategory::APITesting => "API Testing",
        }.to_string();
        write!(f, "{}", str)
    }
}
struct BlogPost {
    title: &'static str,
    excerpt: &'static str,
    category: BlogCategory,
    date: &'static str,
    read_time: &'static str,
    image: &'static str,
    featured: bool,
}

#[component]
pub fn Blog() -> Element {
    let current_lang = use_context::<Signal<Language>>();
    let blog_posts = [
        BlogPost {
            title: "Essential Testing Strategies for Modern Web Applications",
            excerpt: "Discover the most effective testing approaches for contemporary web development, including API testing, cross-browser compatibility, and performance optimization techniques.",
            category: BlogCategory::QAProcess,
            date:"2024-01-15",
            read_time: "8",
            image: "/api/placeholder/400/250",
            featured: true,
        },
        BlogPost {
            title: "Automation vs Manual Testing: When to Use Each Approach",
            excerpt: "A comprehensive guide to choosing between automated and manual testing methods based on project requirements, timeline, and budget constraints.",
            category: BlogCategory::TestAutomation,
            date: "2024-01-10",
            read_time: "6",
            image: "/api/placeholder/400/250",
            featured: false,
        }
    ];
    let categories = [
        BlogCategory::TestAutomation,
        BlogCategory::RustDevelopment,
        BlogCategory::QAProcess,
        BlogCategory::Performance,
        BlogCategory::APITesting
    ];
    match current_lang() {
        Language::EN => rsx! {
            section { id: "blog", class: "py-20 bg-white",
                div { class: "container mx-auto px-4 sm:px-6 lg:px-8",
                    div { class: "text-center mb-16",
                        h2 { class: "text-3xl sm:text-4xl font-bold text-foreground mb-4",
                            "Testing Insights & Knowledge"
                        }
                        p { class: "text-xl text-muted-foreground max-w-3xl mx-auto",
                            "Stay updated with the latest trends, best practices, and insights in software testing
                             and quality assurance. Learn from real-world experiences and practical advice."
                        }
                    }
                    // Category filter
                    div {
                        for (index , category) in categories.iter().enumerate() {
                            Button {
                                variant: if index == 0 { ButtonVariant::Primary } else { ButtonVariant::Outline },
                                size: "sm",
                                class: "text-sm",
                                "{ category.to_string() }"
                            }
                        }
                    }
                    // Featured Post
                    for (index , post) in blog_posts.iter().enumerate().filter(|(_, post)| post.featured) {
                        DxsCard {
                            key: index,
                            class: "mb-12 shadow-hover overflow-hidden",
                            div { class: "grid md:grid-cols-2 gap-0",
                                div { class: "relative",
                                    div { class: "h-64 md:h-full bg-gradient-to-br from-primary-light to-primary/20 flex items-center justify-center",
                                        div { class: "text-center p-8",
                                            DxsBadge { class: "mb-4 bg-accent text-accent-foreground",
                                                "Featured Post"
                                            }
                                            div { class: "text-6xl text-primary mb-4",
                                                "📝"
                                            }
                                            p { class: "text-sm text-muted-foreground",
                                                "Featured Article"
                                            }
                                        }
                                    }
                                }
                                div { class: "p-8 flex flex-col justify-center",
                                    div { class: "flex items-center space-x-4 mb-4",
                                        DxsBadge { variant: BadgeVariant::Secondary,
                                            "{ post.category.to_string() }"
                                        }
                                        div { class: "flex items-center text-sm text-muted-foreground space-x-4",
                                            div { class: "flex items-center space-x-1",
                                                Calendar { class: "h-4 w -4" }
                                                span { "{ post.date }" }
                                            }
                                            div { class: "flex items-center space-x-1",
                                                Clock { class: "h-4 w-4" }
                                                span { "{ post.read_time } min read" }
                                            }
                                        }
                                    }
                                    h3 { class: "text-2xl font-bold text-foreground mb-4",
                                        "{post.title}"
                                    }
                                    p { class: "text-muted-foreground mb-6 leading-relaxed",
                                        "{post.excerpt}"
                                    }
                                    Button { class: "self-start group",
                                        "Read Full Article"
                                        ArrowRight { class: "h-4 w-4 ml-2 group-hover:translate-x-1 transition-transform duration-200" }
                                    }
                                }
                            }
                        }
                    }
                    // Blog posts grid
                    div { class: "grid md:grid-cols-2 lg:grid-cols-3 gap-8",
                        for (index , post) in blog_posts.iter().enumerate().filter(|(_, post)| !post.featured) {
                            DxsCard {
                                key: index,
                                class: "shadow-card hover:shadow-hover transition-all duration-300 group overflow-hidden",
                                div { class: "h-48 bg-gradient-to-br from-primary-light to-primary/20 flex items-center justify-center",
                                    div { class: "text-center",
                                        div { class: "text-4xl text-primary mb-2",
                                            match post.category {
                                                BlogCategory::TestAutomation => {
                                                    "🤖";
                                                }
                                                BlogCategory::RustDevelopment => {
                                                    "🦀";
                                                }
                                                BlogCategory::QAProcess => {
                                                    "📊";
                                                }
                                                BlogCategory::Performance => {
                                                    "⚡";
                                                }
                                                BlogCategory::APITesting => {
                                                    "🔌";
                                                }
                                            }
                                        }
                                        DxsBadge {
                                            variant: BadgeVariant::Secondary,
                                            class: "text-xs",
                                            "{ post.category.to_string() }"
                                        }
                                    }
                                }
                                DxsCardHeader { class: "pb-3",
                                    div { class: "flex items-center justify-between text-sm text-muted-foreground mb-2",
                                        div { class: "flex items-center space-x-1",
                                            User { class: "flex items-center space-x-1" }
                                            span { "Marcin Wionczyk" }
                                        }
                                        div { class: "flex items-center space-x-1",
                                            Clock { class: "h-3 w-3" }
                                            span { "{post.read_time} min" }
                                        }
                                    }
                                    DxsCardTitle { class: "text-lg group-hover:text-primary transition-colors duration-200 line-clamp-2",
                                        "{post.title}"
                                    }
                                }
                                DxsCardContent { class: "pt-0",
                                    DxsCardDescription { class: "mb-4 line-clamp-3", "{post.excerpt" }
                                    div { class: "flex items-center justify-between",
                                        div { class: "flex items-center space-x-1 text-xs text-muted-foreground",
                                            Calendar { class: "h-3 w-3" }
                                            span { "{post.date}" }
                                        }
                                        Button {
                                            variant: ButtonVariant::Ghost,
                                            class: "text-xs",
                                            "Read More"
                                            ArrowRight { class: "h-3 w-3 ml-1" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    div { class: "text-center mt-16",
                        DxsCard { class: "bg-gradient-hero text-primary-foreground shadow-hover inline-block",
                            DxsCardContent { class: "p-8",
                                h3 { class: "text-2xl font-bold mb-4", "Want to Stay Updated?" }
                                p { class: "mb-6 opacity-90",
                                    "Subscribe to get the latest testing insights, tutorials, and industry news delivered to your inbox."
                                }
                                Button { variant: ButtonVariant::Secondary, "Subscribe to Newsletter" }
                            }
                        }
                    }
                }
            }
        },
        Language::PL => rsx! {
            section { id: "blog", class: "py-20 bg-white",
                div { class: "container mx-auto px-4 sm:px-6 lg:px-8",
                    div { class: "text-center mb-16",
                        h2 { class: "text-3xl sm:text-4xl font-bold text-foreground mb-4",
                            "Spostrzeżenia i wiedza o testowaniu"
                        }
                        p { class: "text-xl text-muted-foreground max-w-3xl mx-auto",
                            "Bądź na bieżąco z najnowszymi trendami, najlepszymi praktykami i spostrzeżeniami z zakresu testowania oprogramowania
                             i zapewniania jakości. Ucz się na podstawie rzeczywistych doświadczeń i praktycznych porad."
                        }
                    }
                    // Category filter
                    div {
                        for (index , category) in categories.iter().enumerate() {
                            Button {
                                variant: if index == 0 { ButtonVariant::Primary } else { ButtonVariant::Outline },
                                "{ category.to_string() }"
                            }
                        }
                    }
                    // Featured Post
                    for (index , post) in blog_posts.iter().enumerate().filter(|(_, post)| post.featured) {
                        DxsCard {
                            key: index,
                            class: "mb-12 shadow-hover overflow-hidden",
                            div { class: "grid md:grid-cols-2 gap-0",
                                div { class: "relative",
                                    div { class: "h-64 md:h-full bg-gradient-to-br from-primary-light to-primary/20 flex items-center justify-center",
                                        div { class: "text-center p-8",
                                            DxsBadge { class: "mb-4 bg-accent text-accent-foreground",
                                                "Polecany Post"
                                            }
                                            div { class: "text-6xl text-primary mb-4",
                                                "📝"
                                            }
                                            p { class: "text-sm text-muted-foreground",
                                                "Artykuł polecany"
                                            }
                                        }
                                    }
                                }
                                div { class: "p-8 flex flex-col justify-center",
                                    div { class: "flex items-center space-x-4 mb-4",
                                        DxsBadge { variant: BadgeVariant::Secondary,
                                            "{ post.category.to_string() }"
                                        }
                                        div { class: "flex items-center text-sm text-muted-foreground space-x-4",
                                            div { class: "flex items-center space-x-1",
                                                Calendar { class: "h-4 w -4" }
                                                span { "{ post.date }" }
                                            }
                                            div { class: "flex items-center space-x-1",
                                                Clock { class: "h-4 w-4" }
                                                span { "{ post.read_time } min czytania" }
                                            }
                                        }
                                    }
                                    h3 { class: "text-2xl font-bold text-foreground mb-4",
                                        "{post.title}"
                                    }
                                    p { class: "text-muted-foreground mb-6 leading-relaxed",
                                        "{post.excerpt}"
                                    }
                                    Button { class: "self-start group",
                                        "Read Full Article"
                                        ArrowRight { class: "h-4 w-4 ml-2 group-hover:translate-x-1 transition-transform duration-200" }
                                    }
                                }
                            }
                        }
                    }
                    // Blog posts grid
                    div { class: "grid md:grid-cols-2 lg:grid-cols-3 gap-8",
                        for (index , post) in blog_posts.iter().enumerate().filter(|(_, post)| !post.featured) {
                            DxsCard {
                                key: index,
                                class: "shadow-card hover:shadow-hover transition-all duration-300 group overflow-hidden",
                                div { class: "h-48 bg-gradient-to-br from-primary-light to-primary/20 flex items-center justify-center",
                                    div { class: "text-center",
                                        div { class: "text-4xl text-primary mb-2",
                                            match post.category {
                                                BlogCategory::TestAutomation => {
                                                    "🤖";
                                                }
                                                BlogCategory::RustDevelopment => {
                                                    "🦀";
                                                }
                                                BlogCategory::QAProcess => {
                                                    "📊";
                                                }
                                                BlogCategory::Performance => {
                                                    "⚡";
                                                }
                                                BlogCategory::APITesting => {
                                                    "🔌";
                                                }
                                            }
                                        }
                                        DxsBadge {
                                            variant: BadgeVariant::Secondary,
                                            class: "text-xs",
                                            "{ post.category.to_string() }"
                                        }
                                    }
                                }
                                DxsCardHeader { class: "pb-3",
                                    div { class: "flex items-center justify-between text-sm text-muted-foreground mb-2",
                                        div { class: "flex items-center space-x-1",
                                            User { class: "flex items-center space-x-1" }
                                            span { "Marcin Wionczyk" }
                                        }
                                        div { class: "flex items-center space-x-1",
                                            Clock { class: "h-3 w-3" }
                                            span { "{post.read_time} min" }
                                        }
                                    }
                                    DxsCardTitle { class: "text-lg group-hover:text-primary transition-colors duration-200 line-clamp-2",
                                        "{post.title}"
                                    }
                                }
                                DxsCardContent { class: "pt-0",
                                    DxsCardDescription { class: "mb-4 line-clamp-3", "{post.excerpt" }
                                    div { class: "flex items-center justify-between",
                                        div { class: "flex items-center space-x-1 text-xs text-muted-foreground",
                                            Calendar { class: "h-3 w-3" }
                                            span { "{post.date}" }
                                        }
                                        Button {
                                            variant: ButtonVariant::Ghost,
                                            class: "text-xs",
                                            "Read More"
                                            ArrowRight { class: "h-3 w-3 ml-1" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    div { class: "text-center mt-16",
                        DxsCard { class: "bg-gradient-hero text-primary-foreground shadow-hover inline-block",
                            DxsCardContent { class: "p-8",
                                h3 { class: "text-2xl font-bold mb-4", "Chcesz być na bieżąco?" }
                                p { class: "mb-6 opacity-90",
                                    "Subskrybuj, aby otrzymywać najnowsze informacje na temat testowania, samouczki i wiadomości z branży prosto do swojej skrzynki odbiorczej."
                                }
                                Button { variant: ButtonVariant::Secondary, "Zapisz się do newslettera" }
                            }
                        }
                    }
                }
            }
        },
    }

}