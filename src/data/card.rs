use std::rc::Rc;

use rand::{rngs::ThreadRng, seq::SliceRandom};
use serde::{Deserialize, Serialize};
use strum::{EnumCount, EnumIter, EnumProperty, EnumString, IntoEnumIterator, IntoStaticStr};

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
)]
pub enum Card {
    #[strum(props(image_filter = "beggar", name = "The Fool"))]
    Fool,
    #[strum(props(image_filter = "sorceror", name = "The Magician"))]
    Magician,
    #[strum(props(image_filter = "priestess", name = "The High Priestess"))]
    Priestess,

    #[strum(props(image_filter = "empress", name = "The Empress"))]
    Empress,
    #[strum(props(image_filter = "king", name = "The Emperor"))]
    Emperor,
    #[strum(props(image_filter = "pope", name = "The Hierophant"))]
    Hierophant,
    #[strum(props(image_filter = "kissing", name = "The Lovers"))]
    Lovers,
    #[strum(props(image_filter = "handcart", name = "The Chariot"))]
    Chariot,
    #[strum(props(image_filter = "lady justice", name = "Justice"))]
    Justice,
    #[strum(props(image_filter = "hermit", name = "The Hermit"))]
    Hermit,
    #[strum(props(image_filter = "wheel", name = "Wheel of Fortune"))]
    Wheel,
    #[strum(props(image_filter = "warrior", name = "Strength"))]
    Strength,

    #[strum(props(image_filter = "aerialist", name = "The Hanged Man"))]
    Hanged,

    #[strum(props(image_filter = "grim reaper", name = "Death"))]
    Death,
    #[strum(props(image_filter = "archangel", name = "Temperance"))]
    Temperance,
    #[strum(props(image_filter = "satan", name = "The Devil"))]
    Devil,

    #[strum(props(image_filter = "tower", name = "The Tower"))]
    Tower,
    #[strum(props(image_filter = "shooting star", name = "The Star"))]
    Star,
    #[strum(props(image_filter = "moon", name = "The Moon"))]
    Moon,

    #[strum(props(image_filter = "sunrise", name = "The Sun"))]
    Sun,

    #[strum(props(image_filter = "courtroom", name = "Judgement"))]
    Judgement,

    #[strum(props(image_filter = "globe", name = "The World"))]
    World,
}

impl Card {
    pub fn filter_image(&self, name: &str) -> bool {
        name.to_ascii_lowercase()
            .contains(self.get_str("name").unwrap().to_ascii_lowercase().as_str())
    }

    pub fn name(&self) -> &'static str {
        self.get_str("name").unwrap()
    }

    pub fn repr(&self) -> &'static str {
        self.into()
    }

    pub fn get_random_ordering() -> Rc<[Card; Card::COUNT]> {
        let mut cards = [Card::Fool; Card::COUNT];
        for (i, c) in Card::iter().enumerate() {
            cards[i] = c
        }
        let mut rng = ThreadRng::default();
        cards.shuffle(&mut rng);

        cards.into()
    }
}
