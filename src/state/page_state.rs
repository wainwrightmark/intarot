use std::rc::Rc;
use strum::EnumCount;

use crate::data::prelude::Card;

use yewdux::prelude::*;

use super::{card_page_state::CardPageState, messages::*};

impl ButtonMessage<CardPageState> for DrawMessage {
    fn can_apply(state: &CardPageState) -> bool {
        state.cards_drawn < Card::COUNT
    }

    fn get_name() -> &'static str {
        "Draw"
    }
}

impl ButtonMessage<CardPageState> for ReplaceMessage {
    fn can_apply(state: &CardPageState) -> bool {
        state.cards_drawn > 1
    }

    fn get_name() -> &'static str {
        "Replace"
    }
}

impl Reducer<CardPageState> for DrawMessage {
    fn apply(self, state: Rc<CardPageState>) -> Rc<CardPageState> {
        Rc::new(state.to_owned().draw_card())
    }
}

impl Reducer<CardPageState> for ReplaceMessage {
    fn apply(self, state: Rc<CardPageState>) -> Rc<CardPageState> {
        Rc::new(state.to_owned().replace_card())
    }
}
impl Reducer<CardPageState> for ToggleDescriptionMessage {
    fn apply(self, state: Rc<CardPageState>) -> Rc<CardPageState> {
        Rc::new(state.to_owned().toggle_description())
    }
}

