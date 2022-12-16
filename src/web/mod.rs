mod app;
mod button_component;
mod cards_view;
mod opening_view;
mod select_component;
pub mod share_component;
mod soothsayer_view;

pub mod prelude {

    pub use crate::web::app::*;
    pub use crate::web::button_component::*;
    pub use crate::web::select_component::*;
    pub use crate::web::share_component::*;
}
