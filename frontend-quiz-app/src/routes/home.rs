use dioxus::prelude::*;

use crate::{components::header::Header, providers::use_theme, routes::Route};

#[component]
pub fn Home() -> Element {
    let theme = use_theme();

    rsx! {
        div {
            class: format!(
                "{} min-h-screen bg-no-repeat xl:bg-cover",
                match theme() {
                    crate::types::Theme::Light => {
                        "bg-mobile-light bg-lightGrey md:bg-tablet-light xl:bg-desktop-light"
                    }
                    crate::types::Theme::Dark => {
                        "bg-mobile-dark bg-darkNavy md:bg-tablet-dark xl:bg-desktop-dark"
                    }
                },
            ),

            Header {}

            main { class: "mt-8 mx-6 md:mx-16 xl:mx-[140px]", Outlet::<Route> {} }
        }
    }
}
