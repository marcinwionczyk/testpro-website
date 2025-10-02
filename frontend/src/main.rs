mod components;

use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const DIOXUS_COMPONENTS_THEME: Asset = asset!("/assets/dx-components-theme.css");
// Language context
#[derive(Clone, Copy, strum::Display, strum::EnumIter, PartialEq)]
pub enum Language {
    #[strum(to_string = "EN")]
    EN,
    #[strum(to_string = "PL")]
    PL,
}

impl Default for Language {
    fn default() -> Self {
        Language::PL
    }
}


fn main() {
    console_error_panic_hook::set_once();
    launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(Language::PL));
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: DIOXUS_COMPONENTS_THEME }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div { class: "min-h-screen",
            components::Header {}
            main {
                components::Hero {}
                components::Services {}
                components::About {}
                components::Blog {}
                components::Contact {}
            }
            components::Footer {}
        }
    }
}


