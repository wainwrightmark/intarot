use std::str::FromStr;

use itertools::Itertools;
use strum::IntoEnumIterator;
use web_sys::HtmlSelectElement;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::data::prelude::StarSign;
use crate::web::prelude::*;

#[function_component(OpeningView)]
pub fn opening_view() -> Html {
    let navigator = use_navigator().unwrap();

    let onchange = Callback::from(move |e: Event| {
        let input: HtmlSelectElement = e.target_unchecked_into();
        let s = input.value();

        if let Ok(sign) = StarSign::from_str(s.as_str()) {
            navigator.push(&Route::Soothsayer { sign });
        }
    });

    let options = StarSign::iter()
    .map(|value| {        
        html!(  <option value={value.repr()} selected={false} disabled={false}> {value.name()}  </option>
        )
    })
    .collect_vec();

    html! {
        <div>
        <div class="sm-4 col" style="margin: auto;">

        <h3 style="color: gold; text-align: center;">
        {"intarot"}
        </h3>
        <p>
        {"intarot brings tarot into the space that lies beyond the limits of the world. Our lives are lost in wandering, the deterministic patterns of our destiny masked in the grey fog of randomness. Our soothsayers can help reveal the path your feet should tread."}
        </p>
        <p>
        {"Each of our soothsayers will offer you an interpretation of the card they draw for you, but their portentous drawings may contain the seed of further truth - only you will recognise the signs that fate has chosen for you."}
        </p>
            </div>
            <select {onchange} class="" style="margin:auto;">
                    <option selected={true} disabled={true}> {"Choose Star Sign"}  </option>
                    {options}
                </select>
        </div>
    }
}
