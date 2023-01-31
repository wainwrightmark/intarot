pub mod achievement;
mod card;
pub mod description_layout;
mod guide;
pub mod image_description;
mod image_meta;
pub mod question_data;
pub mod spread_type;
pub mod src_data;
pub mod spread_id;

pub mod prelude {
    pub use crate::data::achievement::*;
    pub use crate::data::card::*;
    pub use crate::data::description_layout::*;
    pub use crate::data::guide::*;
    pub use crate::data::image_description::*;
    pub use crate::data::image_meta::*;
    pub use crate::data::question_data::*;
    pub use crate::data::spread_type::*;
    pub use crate::data::src_data::*;

    use importunate::Permutation;
    pub type Perm= Permutation::<u128, 22>;
}
