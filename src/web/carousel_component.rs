use itertools::Itertools;
use yew::prelude::*;
use yew_hooks::{use_swipe, UseSwipeDirection};
use yewdux::prelude::*;

use crate::state::prelude::*;

#[function_component(CarouselComponent)]
pub fn carousel_component<S: SelectCarouselMessage<TState> + 'static, TState: Store>(//props: &ClassAndStyle,
) -> Html {
    let node =  use_node_ref();
    let swipe_state = use_swipe(node.clone());

   


    let values = S::get_values();
    let current_value_rc = use_selector(|state: &TState| S::get_current_value(state));
    let current_value = current_value_rc.as_ref();
    let current_index = values
        .iter()
        .position(|x| x == current_value)
        .expect("Selected value was not one of the possible values.");
    let previous = if current_index == 0 {
        *current_value
    } else {
        values[current_index - 1]
    };
    let next = if current_index + 1 >= values.len() {
        *current_value
    } else {
        values[current_index + 1]
    };

    let select_previous = Dispatch::<TState>::new().apply_callback(move |_| previous);

    let select_next = Dispatch::<TState>::new().apply_callback(move |_| next);

    let can_select_previous = current_index != 0;
    let can_select_next = current_index + 1 < values.len();

    let items = values
        .iter()
        .map(|value| {
            let selected = current_value == value;
            let classes = if selected {
                classes!("carousel-item", "carousel-item-visible")
            } else {
                classes!("carousel-item", "carousel-item-hidden")
            };

            
            value.get_html(classes)
        })
        .collect_vec();

     // You can depend on direction/swiping etc.
     {
        let swipe_state = swipe_state.clone();
        use_effect_with_deps(move |direction| {
            // Do something based on direction.
            match **direction {
                UseSwipeDirection::Left => Dispatch::<TState>::new().apply(previous),
                UseSwipeDirection::Right => Dispatch::<TState>::new().apply(next),
                _ => (),
            }
            || ()
        }, swipe_state.direction);
    }

    html! {
        <>
        <div class="carousel" ref={node}>
            {items}

            <div class="carousel-actions">
            <button id="carousel-button-prev" aria-label="Previous" disabled={!can_select_previous} onclick={select_previous}></button>
            <button id="carousel-button-next" aria-label="Next" disabled={!can_select_next} onclick={select_next}></button>


            </div>
        </div>
        </>
    }
}