use serde::{Deserialize, Serialize};
use strum::{EnumCount, EnumIter, EnumProperty, EnumString, IntoStaticStr};

#[derive(
    Copy,
    Clone,
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
)]

pub enum Soothsayer {
    #[strum(props(
        name = "Madame Sosoteris",
        description = "...",
        image_url = "1j89ck-bSrCQgj_3PAnhjpWJa0rn3QI04",
        image_filter = "paul nash",
    ))]
    Madame,
    #[strum(props(
        name = "Jean-Baptiste",
        description = "...",
        image_url = "1Ie2-TscFSr4QlEYlD3SQJsWFhQCPIzoH",
        image_filter = "marc chagall"
    ))]
    Jazzman,
    #[strum(props(
        name = "Astralia Plontë",
        description = "...",
        image_url = "1G1Tpwc9HE1Zi2sUZLfAR1wIO0b7ZiOGL",
        image_filter = "rebecca guay",
    ))]
    Astralia,
    #[strum(props(
        name = "Maledictus Andronichus",
        description = "...",
        image_url = "1b6tCkQta6RH4TxvGiCwd0skY6VVGTtgH",
        image_filter = "hieronymus",
    ))]
    Maledictus,
}

impl Soothsayer {
    pub fn filter_image(&self, name: &str) -> bool {
        name.to_ascii_lowercase()
            .contains(self.get_str("image_filter").unwrap())
    }

    pub fn file_id(&self) -> &'static str {
        self.get_str("image_url").unwrap()
    }

    pub fn name(&self) -> &'static str {
        self.get_str("name").unwrap()
    }

    pub fn repr(&self) -> &'static str {
        self.into()
    }
}

#[derive(
    Copy,
    Clone,
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
)]
pub enum StarSign {
    #[strum(props(image_filter = "aries",))]
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
    #[strum(props(image_filter = "waves",))]
    Aquarius,
    #[strum(props(image_filter = "pisces",))]
    Pisces,
}

impl StarSign {
    pub fn filter_image(&self, name: &str) -> bool {
        name.to_ascii_lowercase()
            .contains(self.get_str("image_filter").unwrap())
    }

    pub fn name(&self) -> &'static str {
        self.into()
    }

    pub fn repr(&self) -> &'static str {
        self.into()
    }
}
