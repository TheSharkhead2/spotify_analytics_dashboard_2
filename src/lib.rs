// use leptos::*;

mod components;
mod style;

pub use crate::components::{App, AppProps};

/// global theme variant
#[derive(Default, Clone, Debug, PartialEq, Copy)]
pub enum Theme {
    #[default] // set default enum value
    DEFAULT,
}

/// global state
#[derive(Default, Clone, Debug)]
pub struct GlobalState {
    pub theme: Theme,
}

// #[component]
// pub fn App(cx: Scope) -> impl IntoView {
//     view! {
//         cx,
//         <h1> "Hello!" </h1>
//     }
// }
