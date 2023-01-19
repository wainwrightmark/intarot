mod app;
pub mod card_view;
pub mod guide_carousel;
pub mod logo;
mod opening_view;
pub mod question_view;
pub mod restart_view;
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
