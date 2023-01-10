use yew::prelude::*;
use yew_hooks::{use_swipe, UseSwipeDirection};

use yewdux::prelude::*;

use crate::state::prelude::*;

use crate::web::card_view::*;

#[derive(Properties, PartialEq, Clone)]
pub struct SpreadViewProps {}

#[function_component(SpreadView)]
pub fn spread_view(props: &SpreadViewProps) -> Html {
    // log::info!("Spread View");

    let node = use_node_ref();
    let swipe_state = use_swipe(node.clone());

    let cp = use_store_value::<CardPageState>();
    // log::info!("{:?}", cp);
    let _props = props.clone();

    {
        let swipe_state = swipe_state;
        use_effect_with_deps(
            move |direction| {
                // Do something based on direction.
                match **direction {
                    UseSwipeDirection::Left => {
                        Dispatch::<CardPageState>::new().apply(DrawMessage {})
                    }
                    UseSwipeDirection::Right => {
                        Dispatch::<CardPageState>::new().apply(ReplaceMessage {})
                    }
                    _ => (),
                }
                || ()
            },
            swipe_state.direction,
        );
    }

    let select_previous =
        Dispatch::<CardPageState>::new().apply_callback(move |_| ReplaceMessage {});
    let select_next = Dispatch::<CardPageState>::new().apply_callback(move |_| DrawMessage {});

    let _total_cards = (cp.last_hidden_card_index + 1).min(cp.finish_card_index()); //display an extra card to preload the image
    let _s_d: bool = cp.show_description;

    let can_previous = cp.can_previous();
    let can_next= cp.can_draw();

    let cards = (0..=_total_cards)
        .map(|index| {
            html!(<CardView index={index} key={index} />)
        })
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
