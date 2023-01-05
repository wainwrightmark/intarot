use itertools::Itertools;
use strum::{EnumCount, IntoEnumIterator};
use yew::prelude::*;
use yew_hooks::{use_swipe, UseSwipeDirection};
use yew_router::prelude::use_navigator;
use yewdux::prelude::use_store_value;

use super::app::Route;
use crate::{
    data::prelude::{Soothsayer, StarSign},
    state::prelude::CardPageState,
};

#[derive(Properties, PartialEq)]
pub struct QuestionProps {
    pub sign: StarSign,
    pub soothsayer: Soothsayer,
}

#[function_component(QuestionView)]
pub fn question_view(props: &QuestionProps) -> Html {
    let card_page_state = use_store_value::<CardPageState>();
    let navigator = use_navigator().unwrap();

    let onclick = {
        let soothsayer = props.soothsayer.clone();
        let sign = props.sign.clone();
        let navigator = navigator.clone();
        let card_page_state = card_page_state.as_ref().clone();
        Callback::from(move |_e: MouseEvent| {
            let ordering = card_page_state.get_new_ordering_if_changed(sign, soothsayer);
            navigator.replace(&Route::Card {
                sign,
                soothsayer,
                ordering,
            });
        })
    };

    let skipped_state = use_state(|| false);


    let background_click = {
        let skipped_state = skipped_state.clone();
        Callback::from(move |_e: MouseEvent| {
            skipped_state.set(true);
        })
    };



    html! {

            <>
            <div class="site" style="overflow: hidden ">
                <div class="container" style="height: 100vh;" onclick={background_click}  >
                <h3 style="color: gold; text-align: center; pointer-events:none;">
            {"intarot"}
            </h3>
                <div class="col">
                <div class="row">
                <p style="margin: auto; animation-delay: -6s; pointer-events:none;" class={if *skipped_state{classes!("")} else {classes!{"fade-in"}}}>{"Take a moment to clear your mind"}</p>
                </div>
                <div class="row">
                <p style="margin: auto; animation-delay: -4.5s; pointer-events:none;" class={if *skipped_state {classes!("")} else {classes!{"fade-in"}}}>{"Think of your question"}</p>
                </div>

                <div class="row">
                <p style="margin: auto; animation-delay: -3s; pointer-events:none;" class={if *skipped_state {classes!("")} else {classes!{"fade-in"}}}>{"Remember that truth comes from within"}</p>
                </div>
                <div class="row align-middle">
                <button onclick={onclick} style="margin: auto; animation-delay: -1.5s;" class={if *skipped_state {classes!("")} else {classes!{"fade-in"}}}>{"Begin your reading"}</button>
                </div>
                </div>
                <div class="row">
    </div>
                </div>
            </div>
            </>
        }
}
