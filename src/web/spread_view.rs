use rand::rngs::ThreadRng;
use rand::RngCore;
use yew::prelude::*;
use yew_hooks::{use_swipe, UseSwipeDirection};
use yew_router::prelude::use_navigator;
use yewdux::prelude::*;

use crate::data::prelude::*;
use crate::state::prelude::*;
use crate::web::prelude::{Route, ShareComponent};

use crate::web::card_view::{self, CardView};
use crate::web::final_card_view::{self, FinalCardView};

#[derive(Properties, PartialEq, Clone)]
pub struct SpreadViewProps {
    pub sign: Option<StarSign>,
    pub guide: Guide,
    pub seed: u32,
}

#[function_component(SpreadView)]
pub fn spread_view(props: &SpreadViewProps) -> Html {
    let node = use_node_ref();
    let swipe_state = use_swipe(node.clone());

    let (cp, dispatch) = use_store::<CardPageState>();
    let props = props.clone();
    use_effect(move || dispatch.reduce(|x| x.on_load(props.sign, props.guide, props.seed)));

    {
        let swipe_state = swipe_state.clone();
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

    let can_previous = cp.cards_drawn > 1;
    let can_next = cp.cards_drawn < 23;

    let select_previous =
        Dispatch::<CardPageState>::new().apply_callback(move |_| ReplaceMessage {});
    let select_next = Dispatch::<CardPageState>::new().apply_callback(move |_| DrawMessage {});

    let (metas_state, _) = use_store::<ImageMetaState>();
    let (descriptions_state, _) = use_store::<ImageDescriptionState>();

    let Some(ds) = descriptions_state.descriptions.as_ref() else{
        return html!();
    };

    let metas = cp.get_possible_image_metas(metas_state.as_ref());
    let total_cards = cp.cards_drawn + 1; //display an extra card to preload the image
    let s_d: bool = cp.show_description;

    let final_card = if true {
        Some(html!(<FinalCardView total_cards={total_cards} key={"final card"} />))
    } else {
        None
    };

    let items = metas
        .into_iter()
        //.take(cp.cards_drawn)
        .take(cp.max_drawn)
        .enumerate()
        .map(|(index, image_meta)|
        {
            let description = ds[&(image_meta.guide, image_meta.card)].clone();
            let key = image_meta.card.name().clone();
            let should_shake = !cp.has_shown_description && index + 1 == cp.cards_drawn;
            html!(<CardView index={index} meta={image_meta} show_description={s_d} description={description} total_cards={total_cards} key={key} should_shake={should_shake } sign={props.sign}  />)
        })
        .chain(final_card)
        .collect::<Html>();

    html!(
        <>
        <div class="site" style="overflow: hidden ">
            <div class="container" >

        <div class="sm-4 col" style="margin: auto; width: 90vw; height: 100vh;" ref={node}>
        <div class="cards-grid" key="cards-grid">
        { items }
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
