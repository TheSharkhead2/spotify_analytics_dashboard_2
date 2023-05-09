use leptos::*;
use std::time::Duration;

use crate::GlobalState;

use crate::style::get_dashboard_style;

use crate::components::{TypewriterText, TypewriterTextProps};

#[component]
pub fn Dashboard(cx: Scope) -> impl IntoView {
    let state = use_context::<RwSignal<GlobalState>>(cx).expect("state to have been provided"); // grab state from context

    // getters and setters for theme for global style use
    let (global_state_theme, _set_global_state_theme) = create_slice(
        cx,
        state,
        |state| state.theme,
        |state, theme| state.theme = theme,
    );

    let (class_name, style_str) = get_dashboard_style(global_state_theme());

    view! {
        cx,
        class = class_name,
        <style> {style_str} </style>
        <div class="main">
            <TypewriterText
                text = "Dashboard"
                delay = {Duration::from_millis(100)}
                class_name = {class_name}
            />
        </div>
    }
}
