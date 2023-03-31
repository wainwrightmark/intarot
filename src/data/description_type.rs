use std::{fmt::Display, str::FromStr};

use crate::data::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DescriptionType {
    Representation,
    Guidance,
    UserRepresentation,
    AgentRepresentation,
    GuideInterpretation(Guide),
}

impl DescriptionType {
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Self::Representation,
            Self::Guidance,
            Self::UserRepresentation,
            Self::AgentRepresentation,
            Self::GuideInterpretation(Guide::Evelyn),
            Self::GuideInterpretation(Guide::Madame),
            Self::GuideInterpretation(Guide::Maledictus),
        ]
        .into_iter()
    }
}

impl FromStr for DescriptionType {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "Representation" => Ok(Self::Representation),
            "Guidance" => Ok(Self::Guidance),
            "UserRepresentation" => Ok(Self::UserRepresentation),
            "AgentRepresentation" => Ok(Self::AgentRepresentation),
            "E Interpretation" => Ok(Self::GuideInterpretation(Guide::Evelyn)),
            "S Interpretation" => Ok(Self::GuideInterpretation(Guide::Madame)),
            "M Interpretation" => Ok(Self::GuideInterpretation(Guide::Maledictus)),
            _ => Err(format!("Could not parse type: '{s}'",)),
        }
    }
}

impl Display for DescriptionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DescriptionType::Representation => write!(f, "Representation"),
            DescriptionType::Guidance => write!(f, "Guidance"),
            DescriptionType::UserRepresentation => write!(f, "UserRepresentation"),
            DescriptionType::AgentRepresentation => write!(f, "AgentRepresentation"),
            DescriptionType::GuideInterpretation(g) => {
                write!(f, "{} Interpretation", g.short_name())
            }
        }
    }
}
