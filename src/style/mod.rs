// use csscolorparser::{Color, ParseColorError};

// use crate::Theme;

mod app_style;
mod authentication_style;
mod dashboard_style;

pub use app_style::get_app_style;
pub use authentication_style::get_authentication_failed_style;
pub use dashboard_style::get_dashboard_style;

// /// struct for holding the colors in a given theme
// pub struct Colors {
//     main_background: Color,
// }

// impl Colors {
//     pub fn main_background(&self) -> String {
//         self.main_background.to_hex_string()
//     }
// }

// /// function for getting the colors in a given theme
// pub fn get_theme(theme: Theme) -> Result<Colors, ParseColorError> {
//     match theme {
//         Theme::DEFAULT => Ok(Colors {
//             main_background: Color::from_html("#474747")?,
//         }),
//     }
// }
