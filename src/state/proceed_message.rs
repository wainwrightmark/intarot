use std::rc::Rc;

use yew::Hook;
use yew_hooks::use_location;
use yewdux::store::Reducer;

use super::{prelude::PageState, messages::ButtonMessage, soothsayer_page::SoothsayerPage};


#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct ProceedMessage {}

impl ButtonMessage<PageState> for ProceedMessage {
    fn can_apply(state: &PageState) -> bool {
        match state {
            PageState::OpeningPage(o) => match o.star_sign {
                Some(_star_sign) => true,
                None => false,
            },
            PageState::SoothsayerPage(_s) => true,
            PageState::CardPage(_) => false,
        }
    }

    fn get_name() -> &'static str {
        "Proceed"
    }
}

impl Reducer<PageState> for ProceedMessage {
    fn apply(self, state: std::rc::Rc<PageState>) -> std::rc::Rc<PageState> {
        match state.as_ref() {
            PageState::OpeningPage(o) => match o.star_sign {
                Some(star_sign) => Rc::new(PageState::SoothsayerPage(SoothsayerPage {
                    star_sign,
                    ..Default::default()
                })),
                None => state,
            },
            PageState::SoothsayerPage(s) => {
                Rc::new(PageState::CardPage(s.into()))                

            },
            PageState::CardPage(_) => state,
        }
    }
}