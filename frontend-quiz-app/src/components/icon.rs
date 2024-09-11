use dioxus::prelude::*;

use crate::types::Quiz;

#[component]
pub fn Icon(quiz: ReadOnlySignal<Quiz>) -> Element {
    rsx! {
        div {
            class: format!(
                "{} p-1 rounded-md md:rounded-xl md:w-14 md:h-14 md:p-2 xl:rounded-lg",
                match quiz().title.as_str() {
                    "HTML" => "bg-[#FFF1E9]",
                    "CSS" => "bg-[#E0FDEF]",
                    "JavaScript" => "bg-[#EBF0FF]",
                    "Accessibility" => "bg-[#F6E7FF]",
                    _ => "",
                },
            ),
            img { src: quiz().icon, alt: quiz().icon }
        }
    }
}
