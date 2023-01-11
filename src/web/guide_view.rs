use itertools::Itertools;
use strum::{EnumCount, IntoEnumIterator};
use yew::prelude::*;
use yew_hooks::{use_swipe, UseSwipeDirection};
use yew_router::prelude::use_navigator;
use yewdux::prelude::{use_store_value, Dispatch};

use super::app::Route;
use crate::{data::prelude::*, state::prelude::*};

#[derive(Properties, PartialEq)]
pub struct GuideProps {
    pub go_to_question: bool,
}

#[function_component(GuideView)]
pub fn guide_view(props: &GuideProps) -> Html {
    let node = use_node_ref();
    let swipe_state = use_swipe(node.clone());
    let card_page_state = use_store_value::<DataState>();
    let state = use_state(|| card_page_state.user_data.guide);
    let navigator = use_navigator().unwrap();

    let current_value = *state;
    let current_index = Guide::iter()
        .position(|x| x == current_value)
        .expect("Selected value was not one of the possible values.");
    let previous = current_value.previous().unwrap_or(current_value);
    let next = current_value.next().unwrap_or(current_value);

    // You can depend on direction/swiping etc.
    {
        let state = state.clone();
        let swipe_state = swipe_state;
        use_effect_with_deps(
            move |direction| {
                // Do something based on direction.
                match **direction {
                    UseSwipeDirection::Left => state.set(next),
                    UseSwipeDirection::Right => state.set(previous),
                    _ => (),
                }
                || ()
            },
            swipe_state.direction,
        );
    }



    let select_previous = {
        let state = state.clone();
        Callback::from(move |_| state.set(previous))
    };

    let select_next = {
        let state = state.clone();
        Callback::from(move |_| state.set(next))
    };

    let can_select_previous = current_index != 0;
    let can_select_next = current_index + 1 < Guide::COUNT;


    let items = Guide::iter()
        .map(|guide| {
            let selected = current_value == guide;
            let classes = if selected {
                classes!("carousel-item", "carousel-item-visible")
            } else {
                classes!("carousel-item", "carousel-item-hidden")
            };

            let onclick = {

                let go_to_question = props.go_to_question;
                let navigator = navigator.clone();
                let user_data = card_page_state.user_data;
                let state = state.clone();
                Callback::from(move |_e: MouseEvent| {
                    let mut user_data = user_data;
                    user_data.guide = *state;

                    Dispatch::<DataState>::new().apply(MaybeChangeDataMessage(user_data));
                    if go_to_question{
                        navigator.push(&Route::Question { });
                    }
                    else{
                        navigator.push(&Route::Restart {  });
                    }

                })
            };

            let select_previous = select_previous.clone();
            let select_next = select_next.clone();
            html!(
                <div class={classes}  >
                    <h5 class="guide-name" style="text-align: center;">{"Choose Guide"}</h5>


                    <div>
                    <img class="guide-image"
                    onclick={onclick.clone()}
                    src={format!("https://drive.google.com/uc?export=view&id={}", guide.image_id()) }
                         alt={guide.name()} />
                         <div class="carousel-actions" style="pointer-events: none;">
            <button id="carousel-button-prev" aria-label="Previous" disabled={!can_select_previous} onclick={select_previous} style="pointer-events: auto;">{"❰"}</button>
            <button id="carousel-button-next" aria-label="Next" disabled={!can_select_next} onclick={select_next} style="pointer-events: auto;">{"❱"}</button>

            </div>
                    </div>
                    <h4 class="guide-name" style="text-align: center; margin-top: 3vh; margin-bottom: 1vh;">{guide.name()}</h4>
                        <p class="guide-text" style="margin-top: 0vh;" >
                        {guide.description()}
                        </p>
                        <button onclick={onclick} class="nice-button" style="margin: auto; display: block;">{"Choose"}</button>
                </div>
            )
        })
        .collect_vec();



    html! {
        <>
        <div class="site" style="overflow: hidden ">
            <div class="container" >
        <div class="sm-4 col" style="margin: auto; width: 90em; padding:unset;">
        <div class="carousel" ref={node}>
            {items}


        </div>
        </div>
        </div>
        </div>
        </>
    }
}
