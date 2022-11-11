use serde::{Deserialize, Serialize};
use strum::{EnumCount, EnumIter, EnumProperty, EnumString, IntoStaticStr};

#[derive(
    Copy,
    Clone,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    Debug,
    Serialize,
    Deserialize,
    EnumProperty,
    EnumString,
    EnumIter,
    EnumCount,
    IntoStaticStr,
    Default,
)]
pub enum StarSign {
    #[strum(props(image_filter = "aries",))]
    #[default]
    Aries,
    #[strum(props(image_filter = "taurus",))]
    Taurus,
    #[strum(props(image_filter = "gemini",))]
    Gemini,
    #[strum(props(image_filter = "crab",))]
    Cancer,
    #[strum(props(image_filter = "lion",))]
    Leo,
    #[strum(props(image_filter = "demeter",))]
    Virgo,
    #[strum(props(image_filter = "libra",))]
    Libra,
    #[strum(props(image_filter = "scorpio",))]
    Scorpio,
    #[strum(props(image_filter = "sagittarius",))]
    Sagittarius,
    #[strum(props(image_filter = "capricorn",))]
    Capricorn,
    #[strum(props(image_filter = "ocean",))]
    Aquarius,
    #[strum(props(image_filter = "pisces",))]
    Pisces,
}

impl StarSign {
    pub fn filter_image(&self, name: &str) -> bool {
        name.to_ascii_lowercase()
            .contains(self.name().to_ascii_lowercase().as_str())
    }

    pub fn name(&self) -> &'static str {
        self.into()
    }

    pub fn repr(&self) -> &'static str {
        self.into()
    }
}
