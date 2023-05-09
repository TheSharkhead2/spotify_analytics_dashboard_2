use leptos::*;
use leptos_router::*;
use std::time::Duration;

use crate::GlobalState;

use crate::style::get_authentication_failed_style;

use crate::components::{TypewriterText, TypewriterTextProps};

/// definte struct for deconstructing spotify callback auth code response
#[derive(Params, PartialEq, Debug)]
struct ApiCallback {
    state: String,
    code: String,
}

/// Authentication page for when the user finishing authenticating
#[component]
pub fn AuthenticationPage(cx: Scope) -> impl IntoView {
    let query = use_query::<ApiCallback>(cx); // get query

    // map query to tuple of Option<String>s
    let query_result = move || {
        query.with(|query| {
            query
                .as_ref()
                .map(|query| (Some(query.state.clone()), Some(query.code.clone())))
                .unwrap_or((None, None))
        })
    };

    view! {
        cx,
        <Show
            when=move || query_result().0.is_some()
            fallback=|cx| view! { cx, <AuthenticationFailed /> }
        >
            <p> {query_result().0} </p>
            <p> {query_result().1} </p>
        </Show>
    }
}

/// Authentication failed page
#[component]
fn AuthenticationFailed(cx: Scope) -> impl IntoView {
    let state = use_context::<RwSignal<GlobalState>>(cx).expect("state to have been provided"); // grab state from context

    // getters and setters for theme for global style use
    let (global_state_theme, _set_global_state_theme) = create_slice(
        cx,
        state,
        |state| state.theme,
        |state, theme| state.theme = theme,
    );

    let (class_name, style_str) = get_authentication_failed_style(global_state_theme());

    let navigate = use_navigate(cx); // function to navigate through routes

    view! {
        cx,
        class=class_name,
        <style>{style_str}</style>
        <div class="main">
            <div class="stack">
                <TypewriterText
                    text="Authentication Failed"
                    delay={Duration::from_millis(100)}
                    class_name={class_name}
                />
                <button class="back" on:click=move |_| {_ = navigate("/", Default::default()); }>
                    "Back"
                </button>
            </div>
        </div>
    }
}
