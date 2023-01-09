use yew::prelude::*;
use yew_router::prelude::use_navigator;
use crate::web::prelude::*;

#[function_component(OpeningView)]
pub fn opening_view() -> Html {
    let navigator = use_navigator().unwrap();

    let paragraph1 = include_str!(r#"../text/opening_p1.txt"#);
    let paragraph2 = include_str!(r#"../text/opening_p2.txt"#);
    let paragraph3 = include_str!(r#"../text/opening_p3.txt"#);



    let onclick = Callback::from(
        move |_: MouseEvent|{
            navigator.push(&Route::Choose {  });
        }

    );



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
                <button {onclick} style="margin: auto; display: block;" class="nice-button">{"Choose your Guide"}</button>
            </div>
        </div>
    }
}
