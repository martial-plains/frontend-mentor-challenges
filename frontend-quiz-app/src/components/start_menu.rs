use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{components::quiz_topic::QuizTopic, providers::use_theme, types::Quiz};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct JsonQuizzes {
    quizzes: Vec<Quiz>,
}

#[component]
pub fn StartMenu() -> Element {
    let theme = use_theme();
    let data =
        use_signal(|| serde_json::from_str::<JsonQuizzes>(include_str!("../data.json")).unwrap());

    rsx! {
        section { class: "flex flex-col xl:flex-row justify-between",
            div {
                h1 { class: format!("{}", if theme().is_light() { "text-darkNavy" } else { "text-white" }),
                    "Welcome to the{' '}"
                    span { class: "block font-medium", "Frontend Quiz!" }
                }
                p { class: format!("{}", if theme().is_light() { "text-greyNavy" } else { "text-lightBluish" }),
                    "Pick a subject to get started."
                }
            }

            div { class: "mt-10 flex flex-col gap-3 md:mt-16 md:gap-6 xl:mt-0",

                for quiz in data().quizzes {
                    QuizTopic { quiz: quiz.clone() }
                }

            }
        }
    }
}
