mod page_state;
mod description_state;
mod meta_state;
mod card_page;
mod opening_page;
mod soothsayer_page;

pub mod prelude{
    pub use crate::state::page_state::*;
    pub use crate::state::description_state::*;
    pub use crate::state::meta_state::*;

    pub use crate::state::card_page::*;
    pub use crate::state::soothsayer_page::*;
    pub use crate::state::opening_page::*;
}