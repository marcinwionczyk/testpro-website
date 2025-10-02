use dioxus::prelude::*;
use crate::Language;

#[component]
pub fn Contact() -> Element {
    let current_lang = use_context::<Signal<Language>>();
    rsx! {
        section { id: "contact", class: "py-20 bg-gray-50",
            div { class: "container mx-auto px-4 sm:px-6 lg:px-8",
                div { class: "text-center",
                    h2 { class: "text-3xl sm:text-4xl font-bold text-gray-900 mb-8",
                        match current_lang() {
                            Language::EN => "Get In Touch",
                            Language::PL => "Skontaktuj się z nami",
                        }
                    }
                    p { class: "text-xl text-gray-600 mb-8",
                        match current_lang() {
                            Language::EN => {
                                "Ready to improve your software quality? Let's discuss your testing needs."
                            }
                            Language::PL => {
                                "Chcesz poprawić jakość swojego oprogramowania? Porozmawiajmy o Twoich potrzebach testowych."
                            }
                        }
                    }
                    button { class: "px-8 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors font-semibold",
                        match current_lang() {
                            Language::EN => "Contact me",
                            Language::PL => "Skontaktuj się ze mną",
                        }
                    }
                }
            }
        }
    }
}