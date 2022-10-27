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
        image_id = "1j89ck-bSrCQgj_3PAnhjpWJa0rn3QI04",
        image_filter = "paul nash",
    ))]
    Madame,
    // #[strum(props(
    //     name = "Jean-Baptiste",
    //     description = "...",
    //     image_id = "1Ie2-TscFSr4QlEYlD3SQJsWFhQCPIzoH",
    //     image_filter = "marc chagall"
    // ))]
    // Jazzman,
    #[strum(props(
        name = "Astralia Plontë",
        description = "...",
        image_id = "1G1Tpwc9HE1Zi2sUZLfAR1wIO0b7ZiOGL",
        image_filter = "rebecca guay",
    ))]
    Astralia,
    #[strum(props(
        name = "Maledictus Andronichus",
        description = "...",
        image_id = "1b6tCkQta6RH4TxvGiCwd0skY6VVGTtgH",
        image_filter = "hieronymus",
    ))]
    Maledictus,
}

impl Soothsayer {
    pub fn filter_image(&self, name: &str) -> bool {
        name.to_ascii_lowercase()
            .contains(self.get_str("image_filter").unwrap())
    }

    pub fn image_id(&self) -> &'static str {
        self.get_str("image_id").unwrap()
    }

    pub fn name(&self) -> &'static str {
        self.get_str("name").unwrap()
    }

    pub fn repr(&self) -> &'static str {
        self.into()
    }

    pub fn previous(&self) -> Option<Self> {
        use Soothsayer::*;
        match self {
            Madame => None,
            // Jazzman => Some(Madame),
            Astralia => Some(Madame),
            Maledictus => Some(Astralia),
        }
    }

    pub fn next(&self) -> Option<Self> {
        use Soothsayer::*;
        match self {
            Madame => Some(Astralia),
            //Jazzman => Some(Astralia),
            Astralia => Some(Maledictus),
            Maledictus => None,
        }
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
    #[strum(props(image_filter = "ocean",))]
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
pub enum Card {
    #[strum(props(image_filter = "empress", name = "The Empress"))]
    Empress,
    #[strum(props(image_filter = "priestess", name = "The High Priestess"))]
    Priestess,
    #[strum(props(image_filter = "beggar", name = "The Fool"))]
    Fool,
    #[strum(props(image_filter = "sorceror", name = "The Magician"))]
    Magician,
    #[strum(props(image_filter = "moon", name = "The Moon"))]
    Moon,
    #[strum(props(image_filter = "wheel", name = "The Wheel of Fortune"))]
    Wheel,
    #[strum(props(image_filter = "aerialist", name = "The Hanged Man"))]
    Hanged,
    #[strum(props(image_filter = "handcart", name = "The Chariot"))]
    Chariot,
    #[strum(props(image_filter = "kissing", name = "The Lovers"))]
    Lovers,
    #[strum(props(image_filter = "king", name = "The Emperor"))]
    Emperor,
    #[strum(props(image_filter = "pope", name = "The Hierophant"))]
    Hierophant,
    #[strum(props(image_filter = "tower", name = "The Tower"))]
    Tower,
    #[strum(props(image_filter = "globe", name = "The World"))]
    World,
    #[strum(props(image_filter = "sunrise", name = "The Sun"))]
    Sun,
    #[strum(props(image_filter = "archangel", name = "Temperance"))]
    Temperance,
    #[strum(props(image_filter = "hermit", name = "The Hermit"))]
    Hermit,
    #[strum(props(image_filter = "warrior", name = "Strength"))]
    Strength,
    #[strum(props(image_filter = "courtroom", name = "Judgement"))]
    Judgement,
    #[strum(props(image_filter = "shooting star", name = "The Star"))]
    Star,
    #[strum(props(image_filter = "grim reaper", name = "Death"))]
    Death,
    #[strum(props(image_filter = "lady justice", name = "Justice"))]
    Justice,
    #[strum(props(image_filter = "satan", name = "The Devil"))]
    Devil,
}

impl Card {
    pub fn filter_image(&self, name: &str) -> bool {
        name.to_ascii_lowercase()
            .contains(self.get_str("image_filter").unwrap())
    }

    pub fn name(&self) -> &'static str {
        self.get_str("name").unwrap()
    }

    pub fn repr(&self) -> &'static str {
        self.into()
    }
}
