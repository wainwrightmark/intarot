use std::rc::Rc;

use serde::{Deserialize, Serialize};
use strum::EnumCount;

use crate::{SoothsayerPage, data::prelude::Card};

use yewdux::prelude::*;

use super::{card_page::CardPage, messages::*, opening_page::OpeningPage};

#[derive(PartialEq, Eq, Store, Clone, Serialize, Deserialize)]
#[store(storage = "local")]
pub enum PageState {
    OpeningPage(OpeningPage),
    SoothsayerPage(SoothsayerPage),
    CardPage(CardPage),
}

impl Default for PageState {
    fn default() -> Self {
        Self::OpeningPage(Default::default())
    }
}

impl ButtonMessage<PageState> for ResetMessage {
    fn can_apply(_state: &PageState) -> bool {
        true
    }

    fn get_name() -> &'static str {
        "Reset"
    }
}

impl ButtonMessage<PageState> for DrawMessage {
    fn can_apply(state: &PageState) -> bool {
        let PageState::CardPage(p) = state else{return false};
        p.cards_drawn < Card::COUNT
    }

    fn get_name() -> &'static str {
        "Draw"
    }
}

impl ButtonMessage<PageState> for ReplaceMessage {
    fn can_apply(state: &PageState) -> bool {
        let PageState::CardPage(p) = state else{return false};
        p.cards_drawn > 1
    }

    fn get_name() -> &'static str {
        "Replace"
    }
}
impl ButtonMessage<PageState> for ShuffleMessage {
    fn can_apply(state: &PageState) -> bool {
        matches!(state, PageState::CardPage(_))
    }

    fn get_name() -> &'static str {
        "Shuffle"
    }
}

impl Reducer<PageState> for ResetMessage {
    fn apply(self, _: std::rc::Rc<PageState>) -> std::rc::Rc<PageState> {
        PageState::default().into()
    }
}

impl Reducer<PageState> for DrawMessage {
    fn apply(self, state: Rc<PageState>) -> Rc<PageState> {
        match state.as_ref() {
            PageState::CardPage(cp) => Rc::new(PageState::CardPage(cp.clone().draw_card())),
            _ => state,
        }
    }
}

impl Reducer<PageState> for ReplaceMessage {
    fn apply(self, state: Rc<PageState>) -> Rc<PageState> {
        match state.as_ref() {
            PageState::CardPage(cp) => Rc::new(PageState::CardPage(cp.clone().replace_card())),
            _ => state,
        }
    }
}
impl Reducer<PageState> for ShuffleMessage {
    fn apply(self, state: Rc<PageState>) -> Rc<PageState> {
        match state.as_ref() {
            PageState::CardPage(cp) => Rc::new(PageState::CardPage(cp.clone().shuffle())),
            _ => state,
        }
    }
}
impl Reducer<PageState> for ToggleDescriptionMessage {
    fn apply(self, state: Rc<PageState>) -> Rc<PageState> {
        match state.as_ref() {
            PageState::CardPage(cp) => {
                Rc::new(PageState::CardPage(cp.clone().toggle_description()))
            }
            _ => state,
        }
    }
}

/*
impl PageState {
    pub fn reset(&mut self) {
        *self = Self::default()
    }

    pub fn reroll(&mut self) {
        match self {
            PageState::OpeningPage(_) => todo!(),
            PageState::SoothsayerPage(_, _) => todo!(),
            PageState::CardPage(_, _, s, d) => {
                *s += 1;
                *d = false;
            }
        }
    }

    pub fn get_soothsayer(&self) -> Soothsayer {
        match self {
            PageState::OpeningPage(_) => Soothsayer::first(),
            PageState::SoothsayerPage(_, sayer) => *sayer,
            PageState::CardPage(_, sayer, _, _) => *sayer,
        }
    }

    pub fn get_sign(&self) -> Option<StarSign> {
        match self {
            PageState::OpeningPage(ss) => *ss,
            PageState::SoothsayerPage(ss, _) => Some(*ss),
            PageState::CardPage(ss, _, _, _) => Some(*ss),
        }
    }

    pub fn can_next_soothsayer(&self) -> bool {
        match self {
            PageState::OpeningPage(_) => false,
            PageState::SoothsayerPage(_, soothsayer) => soothsayer.next().is_some(),
            PageState::CardPage(_, _, _, _) => false,
        }
    }

    pub fn next_soothsayer(&mut self) {
        match self {
            PageState::OpeningPage(_) => {}
            PageState::SoothsayerPage(_, soothsayer) => {
                if let Some(next) = soothsayer.next() {
                    *soothsayer = next;
                }
            }
            PageState::CardPage(_, _, _, _) => {}
        }
    }

    pub fn can_previous_soothsayer(&self) -> bool {
        match self {
            PageState::OpeningPage(_) => false,
            PageState::SoothsayerPage(_, soothsayer) => soothsayer.previous().is_some(),
            PageState::CardPage(_, _, _, _) => false,
        }
    }

    pub fn previous_soothsayer(&mut self) {
        match self {
            PageState::OpeningPage(_) => {}
            PageState::SoothsayerPage(_, soothsayer) => {
                if let Some(prev) = soothsayer.previous() {
                    *soothsayer = prev;
                }
            }
            PageState::CardPage(_, _, _, _) => {}
        }
    }

    pub fn set_star_sign(&mut self, new_star_sign: Option<StarSign>) {
        match self {
            PageState::OpeningPage(ss) => *ss = new_star_sign,
            PageState::SoothsayerPage(_, _) => {}
            PageState::CardPage(_, _, _, _) => {}
        }
    }

    pub fn can_proceed(&self) -> bool {
        match self {
            PageState::OpeningPage(ss) => ss.is_some(),
            PageState::SoothsayerPage(_ss, _soothsayer) => true,
            PageState::CardPage(_, _, _, _) => false,
        }
    }

    pub fn proceed(&mut self) {
        match self {
            PageState::OpeningPage(ss) => {
                if let Some(star) = ss {
                    *self = Self::SoothsayerPage(*star, Soothsayer::first())
                }
            }
            PageState::SoothsayerPage(ss, soothsayer) => {
                *self = Self::CardPage(*ss, *soothsayer, 1, false)
            }
            PageState::CardPage(_, _, _, _) => {}
        }
    }

    pub fn show_description(&self) -> bool {
        match self {
            PageState::OpeningPage(_) => false,
            PageState::SoothsayerPage(_, _) => false,
            PageState::CardPage(_, _, _, s) => *s,
        }
    }

    pub fn toggle_show_description(&mut self) {
        match self {
            PageState::OpeningPage(_) => (),
            PageState::SoothsayerPage(_, _) => (),
            PageState::CardPage(_, _, _, s) => *s = !(*s),
        }
    }


}
 */
