use std::{str::FromStr, fmt::{Display, Debug, Pointer}};

use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIter, EnumProperty, EnumString, IntoStaticStr, ParseError};

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
    Display,
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
    Default
)]
pub struct StarSignOption(pub Option<StarSign>);

impl Display for StarSignOption{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        if let Some(sign) = self.0{
            std::fmt::Display::fmt(&sign, f)
        }
        else{
            write!(f, "None")
        }
    }
}

impl From<Option<StarSign>> for StarSignOption{
    fn from(value: Option<StarSign>) -> Self {
        Self(value)
    }
}

impl From<StarSign> for StarSignOption{
    fn from(value: StarSign) -> Self {
        Self(Some(value))
    }
}

impl FromStr for StarSignOption{
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty(){
            Ok(Self::default())
        }
        else if s.eq_ignore_ascii_case("none"){
            Ok(Self::default())
        }
        else{
            let r = StarSign::from_str(s).map(|x|Self(Some(x)));
            r
        }
    }
}
