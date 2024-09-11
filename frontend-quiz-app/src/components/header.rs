use dioxus::prelude::*;

use crate::{
    components::{dark_mode::DarkMode, icon::Icon},
    providers::use_theme,
    types::Quiz,
};

#[component]
pub fn Header() -> Element {
    let theme = use_theme();
    let quiz = use_context::<Signal<Quiz>>();

    rsx! {
        header { class: "h-[72px] w-full flex justify-between items-center px-6 md:px-16 md:h-[113px] xl:px-[140px] xl:h-[192px]",
            div { class: "flex items-center gap-4 md:gap-6",
                Icon { quiz: quiz() }
                p {
                    class: format!(
                        "{} text-heading-s-mobile font-medium md:text-heading-s",
                        match theme() {
                            crate::types::Theme::Light => "text-darkNavy",
                            crate::types::Theme::Dark => "text-white",
                        },
                    ),
                    {quiz().title}
                }
            }
            DarkMode {}
        }
    }
}
