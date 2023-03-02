use yew::prelude::*;
use yew_hooks::{use_swipe, UseSwipeDirection};

use yewdux::prelude::*;

use crate::data::achievement::Achievement;
use crate::state::prelude::*;

use crate::web::capacitor;
use crate::web::card_view::*;
use crate::web::prelude::*;

use capacitor_bindings::haptics::*;

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
                    UseSwipeDirection::Right => {
                        if data_state.can_draw() {
                            Dispatch::<DataState>::new().apply(DrawMessage {})
                        } else {
                            Dispatch::<AchievementsState>::new()
                                .apply(AchievementEarnedMessage(Achievement::SwipeWrongWay));
                            capacitor::do_or_report_error(|| async {
                                Haptics::notification(NotificationType::Warning).await
                            });
                            if data_state.cards_facing_up == 0 {
                                angry_animate_top_card_right_facedown();
                            } else {
                                angry_animate_top_card_right();
                            }
                        }
                    }
                    UseSwipeDirection::Left => {
                        if data_state.can_replace() {
                            Dispatch::<DataState>::new().apply(ReplaceMessage {})
                        } else {
                            Dispatch::<AchievementsState>::new()
                                .apply(AchievementEarnedMessage(Achievement::SwipeWrongWay));
                            capacitor::do_or_report_error(|| async {
                                Haptics::notification(NotificationType::Warning).await
                            });
                            if data_state.cards_facing_up == 0 {
                                angry_animate_top_card_left_facedown();
                            } else {
                                angry_animate_top_card_left();
                            }
                        }
                    }
                    _ => (),
                }
                || ()
            },
            swipe_state.direction,
        );
    }

    let select_next = Dispatch::<DataState>::new().apply_callback(move |_| ReplaceMessage {});
    let select_previous = Dispatch::<DataState>::new().apply_callback(move |_| DrawMessage {});

    let _total_cards = (data_state.last_hidden_card_index + 1)
        .min(data_state.question_data.spread_type.total_cards()); //display an extra card to preload the image
    let _s_d: bool = data_state.show_description;

    let can_previous = data_state.can_draw();
    let can_next = data_state.can_replace();

    let cards = (0..=_total_cards)
        .map(|index| html!(<IndexedCardView index={index} key={index} />))
        .collect::<Html>();

    let toggle = Dispatch::<DataState>::new().apply_callback(|_| ToggleDescriptionMessage {});

    html!(
        <>
        <div class="site" style="overflow: hidden ">
            <div class="container" >

        <div class="contained col spread-area" style="margin: auto" ref={node}> //For some reason this margin: auto is needed on mobile
        <div class="cards-grid" key="cards-grid" onclick={toggle}>
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
