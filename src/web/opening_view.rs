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

    let paragraph1 = include_str!(r#"../text/opening_p1.txt"#);
    let paragraph2 = include_str!(r#"../text/opening_p2.txt"#);
    let paragraph3 = include_str!(r#"../text/opening_p3.txt"#);

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
        <div class="site"  style="overflow: auto;">
            <div class="container" style="overflow: auto;" >
            <div class="sm-4 col" style="margin: auto;">

            <h3 style="color: gold; text-align: center;">
            {"intarot"}
            </h3>
            <p>
            {paragraph1}
            </p>
            <p>
            {paragraph2}
            </p>
            <p>
            {paragraph3}
            </p>
                </div>
                <select {onchange} class="" style="margin:auto;">
                        <option selected={true} disabled={true}> {"Choose Star Sign"}  </option>
                        {options}
                    </select>
            </div>
        </div>
    }
}
