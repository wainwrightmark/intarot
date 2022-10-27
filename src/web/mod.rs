mod app;
mod image_meta;
mod soothsayer;
mod state;

pub mod prelude {

    pub use crate::web::app::*;
    pub use crate::web::image_meta::*;
    pub use crate::web::soothsayer::*;
    pub use crate::web::state::*;
}