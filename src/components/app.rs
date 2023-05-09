use leptos::*;
use leptos_router::*;

use crate::GlobalState;

use crate::style::get_app_style;

use crate::components::{AuthenticationPage, AuthenticationPageProps, Dashboard, DashboardProps};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let state = create_rw_signal(cx, GlobalState::default()); // define global state
    provide_context(cx, state);

    // getters and setters for theme for global style use
    let (global_state_theme, _set_global_state_theme) = create_slice(
        cx,
        state,
        |state| state.theme,
        |state, theme| state.theme = theme,
    );

    let (class_name, style_val) = get_app_style(global_state_theme());

    view! {
        cx,
        class = class_name,
        <style> {style_val} </style>
        <div class="main">
            <div class="main-border">
                <Router>
                    <main>
                        <Routes>
                            <Route path="/dashboard" view=|cx| view! { cx, <Dashboard /> } />
                            <Route path="/callback" view=|cx| view! {cx, <AuthenticationPage /> }/>
                            <Route path="/*any" view=|cx| view! { cx, <p> "Not found" </p> } />
                        </Routes>
                    </main>
                </Router>
            </div>
        </div>
    }
}
