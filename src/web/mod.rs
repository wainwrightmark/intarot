pub mod advanced_view;
mod app;
pub mod card_view;
pub mod guide_carousel;
pub mod js;
mod landing_view;
pub mod logo;
pub mod question_view;
pub mod share_button;
pub mod share_card_view;
pub mod share_component;
mod spread_view;
mod social;

pub mod prelude {

    pub use crate::web::app::*;
    pub use crate::web::guide_carousel::*;
    pub use crate::web::js::*;
    pub use crate::web::logo::*;
    pub use crate::web::share_button::*;
    pub use crate::web::share_card_view::*;
    pub use crate::web::share_component::*;
    pub use crate::web::social::*;
}
