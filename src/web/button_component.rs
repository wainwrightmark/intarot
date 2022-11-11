use crate::state::prelude::*;

use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(ButtonComponent)]
pub fn button<T: ButtonMessage<TState>, TState: Store>() -> Html {
    let onclick = Dispatch::<TState>::new().apply_callback(|_| T::default());
    let disabled = !*use_selector(|s: &TState| T::can_apply(s));

    let label = T::get_name();

    html! {
        <button aria-label={label} disabled={disabled} onclick={onclick}>{{label}}</button>
    }
}
