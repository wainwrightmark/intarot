use itertools::Itertools;
use web_sys::HtmlSelectElement;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::state::prelude::*;

#[derive(Properties, PartialEq)]

pub struct ClassAndStyle {
    #[prop_or_default()]
    pub classes: Classes,
    #[prop_or_default()]
    pub style: AttrValue,
}

#[function_component(SelectComponent)]
pub fn select_component<S: SelectMessage<TState> + 'static, TState: Store>(
    props: &ClassAndStyle,
) -> Html {
    let onchange = Dispatch::<TState>::new().apply_callback(|e: Event| {
        let input: HtmlSelectElement = e.target_unchecked_into();
        let s = input.value();

        let v = S::parse_repr(s.as_str());
        v
    });
    let current_value = use_selector(|state: &TState| S::get_current_value(state));

    let options = S::get_values()
        .into_iter()
        .map(|value| {
            let selected = value == *current_value;
            html!(  <option value={value.repr()} {selected} disabled={value.disabled()}> {value.name()}  </option>
            )
        })
        .collect_vec();

    html!(
        <select {onchange} class={props.classes.clone()} style={props.style.clone()}>
            {options}
        </select>
    )
}

#[function_component(CarouselComponent)]
pub fn carousel_component<S: SelectCarouselMessage<TState> + 'static, TState: Store>(//props: &ClassAndStyle,
) -> Html {
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

    html! {
        <>
        <div class="carousel">
            {items}

            <div class="carousel-actions">
            <button id="carousel-button-prev" aria-label="Previous" disabled={!can_select_previous} onclick={select_previous}></button>
            <button id="carousel-button-next" aria-label="Next" disabled={!can_select_next} onclick={select_next}></button>


            </div>
        </div>
        </>
    }
}
