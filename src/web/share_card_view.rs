use std::str::FromStr;

use yew::prelude::*;

use yew_hooks::use_search_param;

use yew_router::prelude::use_navigator;
use yewdux::prelude::*;

use crate::data::description_layout::DescriptionLayout;
use crate::data::prelude::ImageMeta;
use crate::data::spread_id::SpreadId;
use crate::state::prelude::*;
use crate::web::card_view::*;
use crate::web::logo::Logo;

#[derive(Properties, PartialEq)]
pub struct ShareCardViewProps {}

#[function_component(ShareCardView)]
pub fn share_card_view(_props: &ShareCardViewProps) -> Html {
    let navigator = use_navigator();
    let descriptions_state = use_store_value::<ImageDescriptionState>();
    let description_layout: DescriptionLayout = Default::default();
    let id = use_search_param("id".to_string());

    let spread = use_search_param("spread".to_string());
    let spread_data = spread
        .clone()
        .and_then(|x| SpreadId::try_decode(x).ok())
        .and_then(|x| x.try_deconstruct().ok());

    let referrer = use_search_param("ref".to_string());

    let user = Dispatch::<UserState>::new().get();
    let event = LoggableEvent::new_share(referrer, spread, id.clone());
    if let Some(user_id) = user.user_id {
        let log = EventLog::new(user_id, event);
        log.send_log();
    } else {
        log::error!("User Id not set");
        Dispatch::<FailedLogsState>::new().apply(LogFailedMessage(event));
    }

    if let Some((qd, perm)) = spread_data {
        Dispatch::<DataState>::new().apply(LoadSpreadMessage(qd, perm));
        navigator.unwrap().push(&crate::web::app::Route::Spread);
    } else if id.is_none() {
        navigator.unwrap().push(&crate::web::app::Route::Landing);
    }

    let image_meta = ImageMeta::from_str(id.unwrap_or_default().as_str()).ok();

    if let Some(image_meta) = image_meta {
        let description = descriptions_state
            .descriptions
            .get(&(image_meta.guide, image_meta.card))
            .copied();

        html!(
            <>
        <div class="site" >
            <div class="container" style="overflow: auto;" >

        <div class="sm-4 col" style="margin: auto; width: 90vw; height: 100vh; ">
        <Logo clickable={true}/>
        <div class="cards-grid" key="cards-grid">
        <CardView top_card={true} src_data={image_meta.src_data()} show_extra_buttons={false} {description} style={CardStyle::default()} {description_layout} />
        </div>
        </div>
        </div>
        </div>
        </>
        )
    } else {
        html!(<> <Logo clickable={true}/> </> )
    }
}
