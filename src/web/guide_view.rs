use itertools::Itertools;
use strum::{EnumCount, IntoEnumIterator};
use yew::prelude::*;
use yew_hooks::{use_swipe, UseSwipeDirection};
use yew_router::prelude::use_navigator;

use super::app::Route;
use crate::data::prelude::{Guide, StarSign};

#[derive(Properties, PartialEq)]
pub struct GuideProps {
    pub sign: Option<StarSign>,
    pub guide: Option<Guide>,
    pub go_to_question: bool,
}

#[function_component(GuideView)]
pub fn soothsayer_view(props: &GuideProps) -> Html {
    let navigator = use_navigator().unwrap();
    let node = use_node_ref();
    let swipe_state = use_swipe(node.clone());
    let state = use_state(|| props.guide.unwrap_or_default());

    let current_value = *state;
    let current_index = Guide::iter()
        .position(|x| x == current_value)
        .expect("Selected value was not one of the possible values.");
    let previous = current_value.previous().unwrap_or(current_value);
    let next = current_value.next().unwrap_or(current_value);

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
                let guide = guide.clone();
                let sign = props.sign.clone();
                let navigator = navigator.clone();
                let go_to_question = props.go_to_question;
                Callback::from(move |_e: MouseEvent| {

                    if go_to_question{
                        navigator.push(&Route::Question { sign: sign.into(), guide });
                    }
                    else{
                        navigator.push(&Route::Restart { sign: sign.into(), guide });
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

    // You can depend on direction/swiping etc.
    {
        let swipe_state = swipe_state.clone();
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
