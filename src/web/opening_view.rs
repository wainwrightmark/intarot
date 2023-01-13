use std::rc::Rc;

use crate::{state::prelude::UserState, web::prelude::*};
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use yewdux::prelude::use_store_value;

#[function_component(OpeningView)]
pub fn opening_view() -> Html {
    let navigator = use_navigator().unwrap();

    let user_data: Rc<UserState> = use_store_value();
    // if user_data.has_used_before {
    //     navigator.replace(&Route::Restart {})
    // }

    let paragraph1 = include_str!(r#"../text/opening_p1.txt"#);
    let paragraph2 = include_str!(r#"../text/opening_p2.txt"#);
    let paragraph3 = include_str!(r#"../text/opening_p3.txt"#);

    let onclick = Callback::from(move |_: MouseEvent| {
        navigator.push(&Route::Choose {});
    });

    html! {
        <div class="site"  style="overflow: auto;">
            <div class="container" style="overflow: auto;" >
            <div class="sm-4 col" style="margin: auto; text-align: justify;">

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
                <button {onclick} style="margin: auto; display: block;" class="nice-button">{"Choose your Guide"}</button>
            </div>
        </div>
    }
}
