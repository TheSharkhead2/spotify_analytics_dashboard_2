use stylers::style_str;

use crate::Theme;

// /// struct for colors in main div (App)
// struct AppMainColors {
//     background_color: String,
// }

// /// getter function for the colors
// fn get_colors(theme: Theme) -> AppMainColors {
//     let theme_colors: Colors = get_theme(theme).unwrap(); // get colors for current theme

//     AppMainColors {
//         background_color: theme_colors.main_background(),
//     }
// }

/// getter function for the App component style
pub fn get_app_style(theme: Theme) -> (&'static str, &'static str) {
    match theme {
        Theme::DEFAULT => {
            style_str! {
                div.main {
                    background-color: var(--DEFAULT-background-color);
                    width: 100%;
                    height: 100%;
                    margin: 0px;
                    padding: 0px;
                    display: flex;
                }

                div.main-border {
                    margin: 1rem;
                    border-style: solid;
                    border-width: 3px;
                    border-radius: 1rem;
                    border-color: var(--DEFAULT-primary-color);
                    flex-grow: 1;
                }

                main {
                    width: 100%;
                    height: 100%;
                    display: flex;
                    flex-direction: horizontal;
                }
            }
        }
    }
}
