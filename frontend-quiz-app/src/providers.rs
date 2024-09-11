use dioxus::prelude::*;

use crate::types::Theme;

#[component]
pub fn ThemeProvider(children: Element) -> Element {
    use_context_provider(|| Signal::new(Theme::Light));

    rsx! {
        {children}
    }
}

pub fn use_theme() -> Signal<Theme> {
    use_context::<Signal<Theme>>()
}
