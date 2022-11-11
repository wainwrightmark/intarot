mod card_page;
pub mod choose_soothsayer_message;
pub mod choose_star_sign_message;
mod description_state;
pub mod messages;
mod meta_state;
mod opening_page;
mod page_state;
mod soothsayer_page;

pub mod prelude {
    pub use crate::state::description_state::*;
    pub use crate::state::meta_state::*;
    pub use crate::state::page_state::*;

    pub use crate::state::card_page::*;
    pub use crate::state::messages::*;
    pub use crate::state::opening_page::*;
    pub use crate::state::soothsayer_page::*;

    pub use crate::state::choose_soothsayer_message;
    pub use crate::state::choose_star_sign_message;
}
