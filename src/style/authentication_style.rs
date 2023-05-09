use stylers::style_str;

use crate::Theme;

/// getter function for the authentication failed style
pub fn get_authentication_failed_style(theme: Theme) -> (&'static str, &'static str) {
    match theme {
        Theme::DEFAULT => {
            style_str! {
                div.main {
                    display: flex;
                    height: 100%;
                    width: 100%;
                    justify-content: center;
                    align-items: center;
                }

                div.stack {
                    display: flex;
                    flex-direction: column;
                    gap: 1rem;
                    justify-content: center;
                    align-items: center;
                }

                button.back {
                    background-color: var(--DEFAULT-background-color);
                    color: var(--DEFAULT-primary-color);
                    border-style: solid;
                    border-width: 2px;
                    border-radius: 1rem;
                    border-color: var(--DEFAULT-primary-color);
                    font-size: 1.5rem;
                    display: inline-block;
                    padding: 0.5rem 1rem;
                    transition-duration: 0.3s;
                }
                button.back:hover {
                    color: var(--DEFAULT-secondary-color);
                    border-color: var(--DEFAULT-secondary-color);
                }

                span.typewriter-text {
                    font-size: 2.2rem;
                    margin: 1rem 0 1rem 1rem;
                    color: var(--DEFAULT-primary-color);
                    font-weight: 600;
                }

                div.typing-container {
                    display: inline-block;
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
