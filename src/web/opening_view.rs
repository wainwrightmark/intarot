use yew::prelude::*;

use crate::state::choose_star_sign_message::ChooseStarSignMessage;
use crate::state::prelude::*;
use crate::state::proceed_message::ProceedMessage;
use crate::web::prelude::*;

#[function_component(OpeningView)]
pub fn opening_view() -> Html {
    html! {
        <div>
        <div class="sm-4 col" style="margin: auto;">

        <h3 style="color: gold; text-align: center;">
        {"The Eighth Arcana"}
        </h3>
        <p>
        {"The Eighth Arcana brings tarot into the space that lies beyond the limits of the world. Our lives are lost in wandering, the deterministic patterns of our destiny masked in the grey fog of randomness. Our soothsayers can help reveal the path your feet should tread."}
        </p>
        <p>
        {"Each of our soothsayers will offer you an interpretation of the card they draw for you, but their portentous drawings may contain the seed of further truth - only you will recognise the signs that fate has chosen for you."}
        </p>
            </div>

            <SelectComponent<ChooseStarSignMessage, PageState> style="margin:auto;"/>

            // <ButtonComponent<ProceedMessage, PageState> />

        </div>
    }
}
