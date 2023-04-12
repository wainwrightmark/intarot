use yew::prelude::*;




#[function_component(BackButton)]
pub fn back_button() -> Html {
    let back_button: Html;
    #[cfg(any(feature = "ios"))]
    {
        use yew_router::prelude::*;
        use crate::web::prelude::Route;
        let navigator = use_navigator().unwrap();

        let on_back_click = {
            Callback::from(move |_: MouseEvent| {
                navigator.push(&Route::Landing {});
            })
        };

        back_button = html! {
            <>

            <button onclick={on_back_click} style="margin: auto; display: block;" class="nice-button advanced-view-button">{"Back"}</button>

            </>

        }
    }
    #[cfg(not(any(feature = "ios")))]
    {
        back_button = html! {
            <>
            </>
        }
    }

    back_button
}
