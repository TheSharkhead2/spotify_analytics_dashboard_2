use stylers::style_str;

use crate::Theme;

/// getter function for the dashboard style
pub fn get_dashboard_style(theme: Theme) -> (&'static str, &'static str) {
    match theme {
        Theme::DEFAULT => {
            style_str! {
                div.main {
                    display: flex;
                    height: 100%;
                    width: 100%;
                    justify-content: flex-start;
                    align-items: flex-start;
                }

                span.typewriter-text {
                    font-size: 2.2rem;
                    margin: 1rem 0 1rem 1rem;
                    color: var(--DEFAULT-primary-color);
                    font-weight: 600;
                    flex-grow: 1;
                }

                div.typing-container {
                    display: flex;
                    justify-content: flex-start;
                    align-items: center;
                }

                span.input-cursor {
                    display: inline-block;
                    width: 2px;
                    height: 2.2rem;
                    background-color: var(--DEFAULT-primary-color);
                    margin-left: 2px;
                    animation: blink 0.6s linear infinite alternate;
                }

                @keyframes blink {
                    0% { opacity: 1; }
                    40% { opacity: 1; }
                    60% { opacity: 0; }
                    100% { opacity: 0; }
                }
            }
        }
    }
}
