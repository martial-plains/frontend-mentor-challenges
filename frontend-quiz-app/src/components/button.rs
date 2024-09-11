use dioxus::prelude::*;

use crate::providers::use_theme;

#[component]
pub fn Button(
    #[props(into)] onclick: EventHandler<MouseEvent>,
    #[props(into)] title: String,
) -> Element {
    let theme = use_theme();

    rsx! {
        button {
            class: format!(
                "{} bg-purpleAccent mb-3 h-14 w-full rounded-xl text-heading-s-mobile font-medium text-white  md:h-[92px] md:text-heading-s md:rounded-3xl md:mb-8",
                match theme() {
                    crate::types::Theme::Light => "drop-shadow-light",
                    crate::types::Theme::Dark => "",
                },
            ),
            onclick,
            {title}
        }
    }
}
