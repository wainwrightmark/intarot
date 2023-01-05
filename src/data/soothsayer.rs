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

pub enum Soothsayer {
    #[strum(props(
        name = "Evelyn Musgrave",
        description = "A daughter of the wild English countryside, she grew up roaming the forests that ramble up to the ruined walls of her family estate. Inspired by the mythic tales of old England she draws the world through a prism of courtly love and mystical devotion. Her tarot speaks of ancient knowledge, and the possibility of beauty found in madness. ",
        image_id = "1G1Tpwc9HE1Zi2sUZLfAR1wIO0b7ZiOGL",
        ad_image_id = "1ejWDUHdurG0IqPEpDQ2iOOiO0F9646HV",
    ))]
    #[default]
    EvelynMusgrave,

    #[strum(props(
        name = "Madame Sosoteris",
        description = "The famous clairvoyante, last in a long line of European royalty. She lives alone on a wind-swept island, sketching the visions that the sea breeze brings her. Her tarot is infused with a fey wisdom; drawn in pale colours that resemble the light reflecting off shallow water and the strange salt-blanched objects that wash up daily on her shores. ",
        image_id = "11ZUYdGy4iznMGozvmc0dev15NhSxYfUl",
        ad_image_id = "1w5MWJ358-8uoBX2O64zKGQW4WGwX_CIM",
    ))]
    Madame,

    #[strum(props(
        name = "Maledictus Andronichus",
        description = "There is a dark fleshy space between the skin of sanity and the bone of madness and Maledictus is its prisoner. Beset by tortured visions of a world he cannot control, his tarot takes shape from these half-images of wild despair; a desperate warning to all who seek his guidance. Though even in his darkest depictions the faintest glimmer of hopeful prophecy survives.",
        image_id = "1b6tCkQta6RH4TxvGiCwd0skY6VVGTtgH",
        ad_image_id = "1wbCbeRF07B-93l87knjvTCubJCkJOqoY",
    ))]
    Maledictus,
    // #[strum(props(
    //     name = "Jean-Baptiste",
    //     description = "...",
    //     image_id = "1Ie2-TscFSr4QlEYlD3SQJsWFhQCPIzoH",
    //     image_filter = "marc chagall"
    // ))]
    // Jazzman,
}

impl Soothsayer {
    pub fn filter_image(&self, name: &str) -> bool {
        name.to_ascii_lowercase()
            .contains(self.get_str("name").unwrap().to_ascii_lowercase().as_str())
    }

    pub fn image_id(&self) -> &'static str {
        self.get_str("image_id").unwrap()
    }

    pub fn ad_image_id(&self)-> &'static str {
        self.get_str("ad_image_id").unwrap()
    }

    pub fn description(&self) -> &'static str {
        self.get_str("description").unwrap()
    }

    pub fn name(&self) -> &'static str {
        self.get_str("name").unwrap()
    }

    pub fn repr(&self) -> &'static str {
        self.into()
    }

    pub fn first() -> Self {
        Soothsayer::EvelynMusgrave
    }

    pub fn previous(&self) -> Option<Self> {
        use Soothsayer::*;
        match self {
            EvelynMusgrave => None,
            Madame => Some(EvelynMusgrave),
            Maledictus => Some(Madame),
        }
    }

    pub fn next(&self) -> Option<Self> {
        use Soothsayer::*;
        match self {
            EvelynMusgrave => Some(Madame),
            Madame => Some(Maledictus),
            Maledictus => None,
        }
    }
}
