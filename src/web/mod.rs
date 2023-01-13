mod app;
pub mod card_view;
mod guide_view;
mod opening_view;
pub mod question_view;
pub mod restart_view;
pub mod share_card_view;
pub mod share_component;
mod spread_view;
pub mod logo;

pub mod prelude {

    pub use crate::web::app::*;
    pub use crate::web::share_card_view::*;
    pub use crate::web::share_component::*;
    pub use crate::web::logo::*;
}
