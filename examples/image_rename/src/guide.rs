use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIter, EnumProperty, EnumString, IntoStaticStr};


#[derive(
    Copy,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
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
    Hash,
)]

pub enum Guide {
    #[strum(props(
        name = "Evelyn Musgrave",
        image_id = "EvelynMusgrave",
        ad_image_id = "AdEvelyn",
        primary_color = "#ffd5d5",
        secondary_color = "#ffe5d5",
    ))]
    #[default]
    #[strum(serialize = "Evelyn", serialize = "E")]

    Evelyn,

    #[strum(props(
        name = "Madame Sosoteris",
        image_id = "MadameSostertis",
        ad_image_id = "AdSosoteris",
        primary_color = "#d7e3f4",
        secondary_color = "#dbdee3",
    ))]
    #[strum(serialize = "Madame", serialize = "S")]
    Madame,

    #[strum(props(
        name = "Maledictus Andronichus",
        image_id = "MaledictusAndronichus",
        ad_image_id = "AdMaledictus",
        primary_color = "#f6d5ff",
        secondary_color = "#ffffff",
    ))]
    #[strum(serialize = "Maledictus", serialize = "M")]
    Maledictus,
}

impl Guide {

    pub fn short_name(&self)-> &'static str{
        match self {
            Guide::Evelyn => "E",
            Guide::Madame => "S",
            Guide::Maledictus => "M",
        }
    }

    // pub fn try_from_short_name(s: &str)-> Self{

    // }

    pub fn filter_image(&self, name: &str) -> bool {
        name.to_ascii_lowercase()
            .contains(self.get_str("name").unwrap().to_ascii_lowercase().as_str())
    }


    pub fn name(&self) -> &'static str {
        self.get_str("name").unwrap()
    }

    pub fn primary_color(&self) -> &'static str {
        self.get_str("primary_color").unwrap()
    }

    pub fn secondary_color(&self) -> &'static str {
        self.get_str("secondary_color").unwrap()
    }

    pub fn repr(&self) -> &'static str {
        self.into()
    }

    pub fn first() -> Self {
        Guide::Evelyn
    }

    pub fn previous(&self) -> Option<Self> {
        use Guide::*;
        match self {
            Evelyn => None,
            Madame => Some(Evelyn),
            Maledictus => Some(Madame),
        }
    }

    pub fn next(&self) -> Option<Self> {
        use Guide::*;
        match self {
            Evelyn => Some(Madame),
            Madame => Some(Maledictus),
            Maledictus => None,
        }
    }
}
