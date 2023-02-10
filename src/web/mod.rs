pub mod advanced_view;
mod app;
pub mod card_view;
pub mod cheat;
pub mod custom_view;
pub mod guide_carousel;
pub mod js;
mod landing_view;
pub mod logo;
pub mod mailchimp;
pub mod particles;
pub mod question_view;
pub mod share_button;
pub mod share_card_view;
mod social;
mod spread_view;
pub mod capacitor;

pub mod prelude {

    pub use crate::web::app::*;
    pub use crate::web::cheat::*;
    pub use crate::web::custom_view::*;
    pub use crate::web::guide_carousel::*;
    pub use crate::web::js::*;
    pub use crate::web::logo::*;
    pub use crate::web::share_button::*;
    pub use crate::web::share_card_view::*;
    pub use crate::web::social::*;
    pub use crate::web::capacitor::*;
}
