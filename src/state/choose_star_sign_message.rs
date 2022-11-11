use std::{rc::Rc, str::FromStr};

use itertools::Itertools;
use strum::IntoEnumIterator;
use yewdux::store::Reducer;

use crate::data::prelude::StarSign;

use super::prelude::{PageState, SelectMessage};

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct ChooseStarSignMessage(Option<StarSign>);

impl Reducer<PageState> for ChooseStarSignMessage {
    fn apply(self, _state: Rc<PageState>) -> Rc<PageState> {
        Rc::new(PageState::OpeningPage(super::prelude::OpeningPage {
            star_sign: self.0,
        }))
    }
}

impl SelectMessage<PageState> for ChooseStarSignMessage {
    fn get_values() -> Vec<Self> {
        std::iter::once(None)
            .chain(StarSign::iter().map(Some))
            .map(Self)
            .collect_vec()
    }

    fn parse_repr(s: &str) -> Self {
        Self(StarSign::from_str(s).ok())
    }

    fn get_current_value(state: &PageState) -> Self {
        Self(match state {
            PageState::OpeningPage(o) => o.star_sign,
            PageState::SoothsayerPage(s) => Some(s.star_sign),
            PageState::CardPage(c) => Some(c.star_sign),
        })
    }

    fn repr(&self) -> &'static str {
        match self.0 {
            Some(ss) => ss.repr(),
            None => "",
        }
    }

    fn name(&self) -> &'static str {
        match self.0 {
            Some(ss) => ss.name(),
            None => "Choose Star Sign",
        }
    }

    fn disabled(&self) -> bool {
        self.0.is_none()
    }
}
