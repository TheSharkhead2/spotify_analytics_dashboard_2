use leptos::*;

use spotify_analytics_dashboard_2::{App, AppProps};

fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}
