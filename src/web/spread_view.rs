use yew::prelude::*;
use yew_hooks::{use_swipe, UseSwipeDirection};

use yewdux::prelude::*;

use crate::state::prelude::*;

use crate::web::card_view::*;

#[wasm_bindgen::prelude::wasm_bindgen(
    inline_js = r##"export function angry_animate_top_card_right() {

    document.querySelector(".top_card").animate(
    [
        { transform: 'translateX(0) rotate(0deg)' },
        { transform: 'translateX(100px) rotate(-10deg)' },
        { transform: 'translateX(50px) rotate(10deg)' },
        { transform: 'translateX(70px) rotate(-15deg)' },
        { transform: 'translateX(50px) rotate(5deg)' },
        { transform: 'translateX(0) rotate(0deg)' },
    ], {
      duration: 500,
      iterations: 1
    }
  ) }"##
)]
extern "C" {
    fn angry_animate_top_card_right();
}

#[wasm_bindgen::prelude::wasm_bindgen(
    inline_js = r##"export function angry_animate_top_card_left() {

    document.querySelector(".top_card").animate(
    [
      { transform: 'translateX(0) rotate(0deg)' },
      { transform: 'translateX(-100px) rotate(10deg)' },
      { transform: 'translateX(-50px) rotate(-10deg)' },
      { transform: 'translateX(-70px) rotate(15deg)' },
      { transform: 'translateX(-50px) rotate(-5deg)' },
      { transform: 'translateX(0) rotate(0deg)' },
    ], {
      duration: 500,
      iterations: 1
    }
  ) }"##
)]
extern "C" {
    fn angry_animate_top_card_left();
}

#[derive(Properties, PartialEq, Clone)]
pub struct SpreadViewProps {}

#[function_component(SpreadView)]
pub fn spread_view(props: &SpreadViewProps) -> Html {
    // log::info!("Spread View");

    let node = use_node_ref();
    let swipe_state = use_swipe(node.clone());

    let data_state = use_store_value::<DataState>();
    // log::info!("{:?}", cp);
    let _props = props.clone();

    {
        let swipe_state = swipe_state;
        let data_state = data_state.clone();
        use_effect_with_deps(
            move |direction| {
                // Do something based on direction.
                match **direction {
                    UseSwipeDirection::Left => {
                        if data_state.can_draw() {
                            Dispatch::<DataState>::new().apply(DrawMessage {})
                        } else {
                            angry_animate_top_card_left();
                        }
                    }
                    UseSwipeDirection::Right => {
                        if data_state.can_previous() {
                            Dispatch::<DataState>::new().apply(ReplaceMessage {})
                        } else {
                            angry_animate_top_card_right();
                        }
                    }
                    _ => (),
                }
                || ()
            },
            swipe_state.direction,
        );
    }

    let select_previous = Dispatch::<DataState>::new().apply_callback(move |_| ReplaceMessage {});
    let select_next = Dispatch::<DataState>::new().apply_callback(move |_| DrawMessage {});

    let _total_cards = (data_state.last_hidden_card_index + 1)
        .min(data_state.question_data.spread_type.total_cards()); //display an extra card to preload the image
    let _s_d: bool = data_state.show_description;

    let can_previous = data_state.can_previous();
    let can_next = data_state.can_draw();

    let cards = (0..=_total_cards)
        .map(|index| html!(<IndexedCardView index={index} key={index} />))
        .collect::<Html>();

    html!(
        <>
        <div class="site" style="overflow: hidden ">
            <div class="container" >

        <div class="sm-4 col" style="margin: auto; width: 90vw; height: 100vh;" ref={node}>
        <div class="cards-grid" key="cards-grid">
        { cards }
        </div>
        <div class="card-actions" style="pointer-events: none;">
            <button id="card-button-prev" aria-label="Previous" disabled={!can_previous}  onclick={select_previous} style="pointer-events: auto;" class="card-arrow">{"❰"}</button>
            <button id="card-button-next" aria-label="Next"  disabled={!can_next} onclick={select_next}  style="pointer-events: auto;" class="card-arrow">{"❱"}</button>
        </div>
        </div>
        </div>
        </div>
        </>
    )
}
