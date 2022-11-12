use itertools::Itertools;
use strum::{EnumProperty, IntoEnumIterator};
use yew::html;
use yewdux::{store::Reducer, prelude::Dispatch};

use crate::data::prelude::Soothsayer;

use super::{messages::{SelectCarouselMessage, ProceedMessage}, prelude::PageState, soothsayer_page::SoothsayerPage};

#[derive(Copy, Clone, PartialEq, Default)]
pub struct ChooseSoothsayerMessage(Soothsayer);

impl Reducer<PageState> for ChooseSoothsayerMessage {
    fn apply(self, state: std::rc::Rc<PageState>) -> std::rc::Rc<PageState> {
        match state.as_ref() {
            PageState::OpeningPage(_) => state,
            PageState::SoothsayerPage(sp) => PageState::SoothsayerPage(SoothsayerPage {
                soothsayer: self.0,
                star_sign: sp.star_sign,
            })
            .into(),
            PageState::CardPage(_) => state,
        }
    }
}

impl SelectCarouselMessage<PageState> for ChooseSoothsayerMessage {
    fn get_html(&self, classes: yew::Classes) -> yew::Html {

        let onclick = Dispatch::<PageState>::new().apply_callback(|_| ProceedMessage::default());
        html!(
            <div class={classes}>
                <h5 class="soothsayer-name" style="text-align: center;">{self.0.name()}</h5>
                <img class="soothsayer-image" onclick={onclick}
                
                
                src={format!("https://drive.google.com/uc?export=view&id={}", self.0.image_id()) }
                     alt={self.0.name()} />
                    <p class="soothsayer-text" >
                    {self.0.get_str("description")}
                    </p>
            </div>
        )
    }

    fn get_values() -> Vec<Self> {
        Soothsayer::iter().map(Self).collect_vec()
    }

    fn get_current_value(state: &PageState) -> Self {
        match state {
            PageState::OpeningPage(_) => Default::default(),
            PageState::SoothsayerPage(sp) => Self(sp.soothsayer),
            PageState::CardPage(cp) => Self(cp.soothsayer),
        }
    }
}
