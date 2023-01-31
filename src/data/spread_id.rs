use std::{fmt::Debug, str::FromStr};

use super::prelude::*;

use base64::{DecodeError, Engine};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Default)]
pub struct SpreadId([u8; 11]);

impl SpreadId {
    pub fn new(data: &QuestionData, perm: &Perm) -> Self {
        let mut arr = [0; 11];

        let guide_byte = data.guide.short_name().chars().next().unwrap() as u8;
        arr[0] = guide_byte;

        let spread_byte = data.spread_type.short_name().chars().next().unwrap() as u8;
        arr[1] = spread_byte;

        let perm_bytes: [u8; 9] = perm.to_le_byte_array();
        arr[2..].copy_from_slice(&perm_bytes);

        Self(arr)
    }

    pub fn question_data(&self) -> Result<QuestionData, anyhow::Error> {
        let guide = Guide::from_str((self.0[0] as char).to_string().as_str())?;
        let spread_type = SpreadType::from_str((self.0[1] as char).to_string().as_str())?;

        Ok(QuestionData { guide, spread_type })
    }

    pub fn perm(&self) -> Perm {
        let mut arr = [0u8; 9];
        arr.copy_from_slice(&self.0[2..]);
        Perm::from_le_byte_array(&arr)
    }

    pub fn try_deconstruct(&self)-> Result<(QuestionData, Perm),anyhow::Error>{
        let qd = self.question_data()?;

        Ok((qd, self.perm()))
    }

    pub fn encode(&self) -> String {
        Engine::encode(&base64::prelude::BASE64_URL_SAFE_NO_PAD, self.0)
    }

    pub fn try_decode(s: String) -> Result<Self, DecodeError> {
        let vec = Engine::decode(&base64::prelude::BASE64_URL_SAFE_NO_PAD, s)?;

        let mut arr = [0u8; 11];
        arr.copy_from_slice(vec.as_slice());


        Ok(Self(arr))
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use crate::data::prelude::*;

    use super::SpreadId;

    #[test]
    pub fn test_spread_ids() {
        let perm = Perm::get_max();
        for guide in Guide::iter() {
            for spread_type in SpreadType::iter() {
                let qd = QuestionData { guide, spread_type };
                let id = SpreadId::new(&qd, &perm);
                let qd2 = id.question_data().unwrap();

                assert_eq!(qd, qd2);

                let perm2 = id.perm();

                assert_eq!(perm, perm2);

                let encoded = id.encode();
                println!("{encoded}");
                let decoded = SpreadId::try_decode(encoded).unwrap();

                assert_eq!(id, decoded);
            }
        }
    }
}
