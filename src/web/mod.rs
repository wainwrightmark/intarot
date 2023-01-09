mod app;
mod button_component;
mod cards_view;
mod opening_view;
pub mod share_component;
mod guide_view;
pub mod question_view;
pub mod restart_view;

pub mod prelude {

    pub use crate::web::app::*;
    pub use crate::web::button_component::*;
    pub use crate::web::share_component::*;
}
