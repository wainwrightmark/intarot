pub mod advanced_view;
mod app;
pub mod card_view;
pub mod guide_carousel;
mod landing_view;
pub mod logo;
pub mod question_view;
pub mod share_button;
pub mod share_card_view;
pub mod share_component;
mod spread_view;

pub mod prelude {

    pub use crate::web::app::*;
    pub use crate::web::guide_carousel::*;
    pub use crate::web::logo::*;
    pub use crate::web::share_card_view::*;
    pub use crate::web::share_component::*;
}
