use crate::{data::prelude::*, state::prelude::*};
use itertools::Itertools;
use strum::{EnumCount, IntoEnumIterator};
use yew::prelude::*;
use yew_hooks::{use_swipe, UseSwipeDirection};
use yewdux::prelude::*;

#[function_component(GuideCarousel)]
pub fn guide_carousel() -> Html {
    let node = use_node_ref();
    let swipe_state = use_swipe(node.clone());
    let (data_state, dispatch) = use_store::<DataState>();
    let question_data = data_state.question_data;

    let current_value = data_state.question_data.guide;
    let current_index = Guide::iter()
        .position(|x| x == current_value)
        .expect("Selected value was not one of the possible values.");
    let previous = current_value.previous().unwrap_or(current_value);
    let next = current_value.next().unwrap_or(current_value);

    // You can depend on direction/swiping etc.
    {
        let swipe_state = swipe_state;
        let dispatch = dispatch.clone();
        use_effect_with_deps(
            move |direction| {
                // Do something based on direction.
                match **direction {
                    UseSwipeDirection::Left => {
                        let mut question_data = question_data;
                        question_data.guide = next;

                        dispatch.apply(MaybeChangeDataMessage(question_data));
                    }
                    UseSwipeDirection::Right => {
                        let mut question_data = question_data;
                        question_data.guide = previous;

                        dispatch.apply(MaybeChangeDataMessage(question_data));
                    }
                    _ => (),
                }
                || ()
            },
            swipe_state.direction,
        );
    }

    let select_previous = {
        let dispatch = dispatch.clone();
        Callback::from(move |_e: MouseEvent| {
            let mut question_data = question_data.clone();
            question_data.guide = previous;

            dispatch.apply(MaybeChangeDataMessage(question_data));
        })
    };

    let select_next = {
        let dispatch = dispatch.clone();
        Callback::from(move |_e: MouseEvent| {
            let mut question_data = question_data.clone();
            question_data.guide = next;

            dispatch.apply(MaybeChangeDataMessage(question_data));
        })
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

            let image_classes= classes!("guide-image");

            let select_previous =  select_previous.clone();
            let select_next = select_next.clone();
            html!(
                <div class={classes}  >
            <div style="position: relative;">
                    <img class={image_classes}
                    src={guide.image_src().src() }
                         alt={guide.name()} />
                         <div class="carousel-actions">
                            <button id="carousel-button-prev" aria-label="Previous" disabled={!can_select_previous} onclick={select_previous} style="pointer-events: auto;">{"❰"}</button>
                            <button id="carousel-button-next" aria-label="Next" disabled={!can_select_next} onclick={select_next} style="pointer-events: auto;">{"❱"}</button>
                        </div>
            </div>
                    <h5 class="guide-name" style="text-align: center; white-space: nowrap;">{guide.name()}</h5>
                    <p >
                                {guide.description()}
                                </p>
                </div>

            )
        })
        .collect_vec();

    html! {
        <>
                <div class="carousel" ref={node}>
            {items}


        </div>

        </>
    }
}
