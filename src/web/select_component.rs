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
