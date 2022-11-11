use yew::prelude::*;
use yewdux::prelude::*;

use crate::state::choose_soothsayer_message::ChooseSoothsayerMessage;
use crate::state::prelude::*;
use crate::web::button_component::ButtonComponent;
use crate::web::select_component::CarouselComponent;

#[function_component(SoothsayerView)]
pub fn soothsayer_view() -> Html {
    html! {
        <>
        <div class="sm-4 col" style="margin: auto; width: 100em; padding:unset;">
        <CarouselComponent<ChooseSoothsayerMessage, PageState> />
        </div>
        <div class="sm-4 col" style="margin: auto; width: 10em;">
        <ButtonComponent<ProceedMessage, PageState>/>
        </div>
        </>
    }
}
