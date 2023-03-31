use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIter, EnumProperty, EnumString, IntoStaticStr};

use super::{guide::Guide, prelude::DescriptionType};

#[derive(
    Debug,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
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
pub enum DescriptionLayout {
    #[default]
    UA,
    G,
    U,

    GA,
    A,
}

impl DescriptionLayout {
    fn allows(&self, d_type: &DescriptionType, guide: &Guide) -> bool {
        match d_type {
            DescriptionType::Representation => true,
            DescriptionType::Guidance => matches!(self, Self::G | Self::GA),
            DescriptionType::UserRepresentation => matches!(self, Self::UA | Self::U),
            DescriptionType::AgentRepresentation => matches!(self, Self::UA | Self::GA | Self::A),
            DescriptionType::GuideInterpretation(g) => g == guide,
        }
    }

    pub fn get_layout_types(&self, guide: Guide) -> impl Iterator<Item = DescriptionType> + '_ {
        use DescriptionType::*;

        const ALL: [DescriptionType; 7] = [
            Representation,
            Guidance,
            UserRepresentation,
            AgentRepresentation,
            GuideInterpretation(Guide::Evelyn),
            GuideInterpretation(Guide::Madame),
            GuideInterpretation(Guide::Maledictus),
        ];

        ALL.into_iter().filter(move |x| self.allows(x, &guide))
    }
}
