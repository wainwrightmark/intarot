pub mod advanced_view;
mod app;
pub mod app_redirect;
pub mod back_button;
pub mod capacitor;
pub mod card_view;
pub mod final_card;
pub mod guide_carousel;
pub mod js;
mod landing_view;
pub mod logo;
pub mod modal_dialog;
pub mod notifications;
pub mod particles;
pub mod preferences_view;
pub mod question_view;
pub mod rate;
pub mod route;
pub mod share_button;
pub mod share_card_view;
mod social;
mod spread_view;
pub mod startup;
pub mod tarot_card;

pub mod prelude {

    pub use crate::web::app::*;
    pub use crate::web::app_redirect::*;
    pub use crate::web::back_button::*;
    pub use crate::web::guide_carousel::*;
    pub use crate::web::js::*;
    pub use crate::web::logo::*;
    pub use crate::web::notifications::*;
    pub use crate::web::route::*;
    pub use crate::web::share_button::*;
    pub use crate::web::share_card_view::*;
    pub use crate::web::social::*;
}
