use dioxus::prelude::*;

use crate::{
    components::{button::Button, icon::Icon},
    providers::use_theme,
    types::Quiz,
};

#[component]
pub fn Results(title: String) -> Element {
    let mut score = use_context::<Signal<i32>>();
    let mut quiz = use_context::<Signal<Quiz>>();
    let theme = use_theme();

    let mut handle_play_again = move || {
        score.set(0);
        quiz.set(Quiz::default());
        navigator().replace("/");
    };

    rsx! {
        section { class: "flex flex-col xl:flex-row xl:gap-8 xl:justify-between",
            div {
                p {
                    class: format!(
                        "{} text-heading-l-mobile font-light md:text-heading-l-regular",
                        match theme() {
                            crate::types::Theme::Light => "text-darkNavy",
                            crate::types::Theme::Dark => "text-white",
                        },
                    ),
                    "Quiz completed "
                    span { class: "black font-medium", "You scored..." }
                }
            }

            div { class: "xl:min-w-[520px]",
                div {
                    class: format!(
                        "{} mt-10 mb-3 p-8 h-fit w-full rounded-xl flex flex-col items-center justify-center gap-4 md:mt-16 md:mb-8 md:p-12 md:rounded-3xl md:gap-10 xl:mt-0",
                        match theme() {
                            crate::types::Theme::Light => "bg-white drop-shadow-light",
                            crate::types::Theme::Dark => "bg-navy drop-shadow-dark",
                        },
                    ),

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

                    p {
                        class: format!(
                            "{} text-display-mobile font-medium md:text-display",
                            match theme() {
                                crate::types::Theme::Light => "text-darkNavy",
                                crate::types::Theme::Dark => "text-white",
                            },
                        ),

                        {score().to_string()}
                    }

                    p {
                        class: format!(
                            "{} text-body-m-mobile font-regular md:text-body-m",
                            match theme() {
                                crate::types::Theme::Light => "text-greyNavy",
                                crate::types::Theme::Dark => "text-lightBluish",
                            },
                        ),

                        {format!("out of {}", quiz().questions.len())}
                    }
                }

                Button { title: "Play Again", onclick: move |_| handle_play_again() }
            }
        }
    }
}
