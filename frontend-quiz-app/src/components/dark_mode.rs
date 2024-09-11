use dioxus::prelude::*;

use crate::{providers::use_theme, types::Theme};

#[component]
pub fn DarkMode() -> Element {
    let mut theme = use_theme();

    let mut handle_theme = move || {
        theme.set({
            if theme().is_light() {
                Theme::Dark
            } else {
                Theme::Light
            }
        });
    };

    rsx! {
        div { class: "flex items-center gap-2 md:gap-4",
            img {
                src: match theme() {
                    crate::types::Theme::Light => asset!("assets/images/icon-sun-dark.svg"),
                    crate::types::Theme::Dark => asset!("assets/images/icon-sun-light.svg"),
                },
                alt: "sun"
            }
            input {
                r#type: "checkbox",
                name: "theme",
                class: "peer hidden",
                checked: theme().is_dark()
            }
            label {
                r#for: "theme",
                class: "bg-purpleAccent w-8 h-5 rounded-full cursor-pointer relative before:content-[''] before:absolute before:bg-white  before:w-3 before:h-3 before:rounded-full before:m-1 before:transition-all before:duration-300 peer-checked:before:translate-x-full md:w-12 md:h-7 md:before:w-5 md:before:h-5",
                onclick: move |_| handle_theme()
            }
            img {
                src: match theme() {
                    crate::types::Theme::Light => asset!("assets/images/icon-moon-dark.svg"),
                    crate::types::Theme::Dark => asset!("assets/images/icon-moon-light.svg"),
                },
                alt: "moon"
            }
        }
    }
}
