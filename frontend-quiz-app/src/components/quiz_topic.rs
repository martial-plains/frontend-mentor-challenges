#![allow(clippy::derive_partial_eq_without_eq)]

use dioxus::prelude::*;

use crate::{components::icon::Icon, providers::use_theme, routes::Route, types::Quiz};

#[component]
pub fn QuizTopic(quiz: Quiz) -> Element {
    let theme = use_theme();

    let mut state_quiz = use_context::<Signal<Quiz>>();
    let quiz = Signal::new(quiz);

    rsx! {
        Link {
            to: Route::Quiz { title: quiz().title },
            class: format!(
                "{} w-full h-16  rounded-xl flex items-center gap-4 pl-3 md:gap-8 md:h-20 md:rounded-3xl xl:px-5 xl:h-24 xl:min-w-[520px]",
                match theme() {
                    crate::types::Theme::Light => "bg-white drop-shadow-light",
                    crate::types::Theme::Dark => "bg-navy drop-shadow-dark",
                },
            ),
            onclick: move |_| state_quiz.set(quiz()),
            Icon { quiz: quiz() }
            p {
                class: format!(
                    "{} font-medium text-body-m-mobile md:text-heading-s",
                    match theme() {
                        crate::types::Theme::Light => "text-darkNavy",
                        crate::types::Theme::Dark => "text-white",
                    },
                ),

                {quiz().title}
            }
        }
    }
}
