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
)]

pub enum Guide {
    #[strum(props(
        name = "Evelyn Musgrave",
        image_id = "EvelynMusgrave",
        ad_image_id = "AdEvelyn",
    ))]
    #[default]
    EvelynMusgrave,

    #[strum(props(
        name = "Madame Sosoteris",
        image_id = "MadameSostertis",
        ad_image_id = "AdSosoteris",
    ))]
    Madame,

    #[strum(props(
        name = "Maledictus Andronichus",
        image_id = "MaledictusAndronichus",
        ad_image_id = "AdMaledictus",
    ))]
    Maledictus,
}

impl Guide {
    pub fn filter_image(&self, name: &str) -> bool {
        name.to_ascii_lowercase()
            .contains(self.get_str("name").unwrap().to_ascii_lowercase().as_str())
    }

    pub fn image_src(&self)-> String{
        let id = self.get_str("image_id").unwrap();
        format!("https://intarot-images.s3.eu-west-2.amazonaws.com/Soothsayers/{id}.jpg")
    }

    pub fn ad_image_src(&self) -> String {
        let id = self.get_str("ad_image_id").unwrap();
        format!("https://intarot-images.s3.eu-west-2.amazonaws.com/AdCards/{id}.jpg")
    }

    pub fn description(&self) -> &'static str {
        match self {
            Guide::EvelynMusgrave => include_str!("../text/evelyn.txt"),
            Guide::Madame => include_str!("../text/madame.txt"),
            Guide::Maledictus => include_str!("../text/maledictus.txt"),
        }
    }

    pub fn name(&self) -> &'static str {
        self.get_str("name").unwrap()
    }

    pub fn repr(&self) -> &'static str {
        self.into()
    }

    pub fn first() -> Self {
        Guide::EvelynMusgrave
    }

    pub fn previous(&self) -> Option<Self> {
        use Guide::*;
        match self {
            EvelynMusgrave => None,
            Madame => Some(EvelynMusgrave),
            Maledictus => Some(Madame),
        }
    }

    pub fn next(&self) -> Option<Self> {
        use Guide::*;
        match self {
            EvelynMusgrave => Some(Madame),
            Madame => Some(Maledictus),
            Maledictus => None,
        }
    }
}
