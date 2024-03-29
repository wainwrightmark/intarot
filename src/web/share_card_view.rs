use std::str::FromStr;

use yew::prelude::*;

use yew_hooks::use_search_param;

use yew_router::prelude::use_navigator;
use yewdux::prelude::*;

use crate::data::description_layout::DescriptionLayout;
use crate::data::prelude::{ImageMeta, SrcData};
use crate::data::spread_id::SpreadId;
use crate::state::prelude::*;
use crate::web::card_view::*;
use crate::web::logo::Logo;
use crate::web::tarot_card::TarotCard;

#[derive(Properties, PartialEq)]
pub struct ShareCardViewProps {}

#[function_component(ShareCardView)]
pub fn share_card_view(_props: &ShareCardViewProps) -> Html {
    let navigator = use_navigator();
    let description_layout: DescriptionLayout = Default::default();
    let id = use_search_param("id".to_string());

    let spread = use_search_param("spread".to_string());
    let spread_data = spread
        .clone()
        .and_then(|x| SpreadId::try_decode(x).ok())
        .and_then(|x| x.try_deconstruct().ok());

    let referrer = use_search_param("ref".to_string());

    let event = LoggableEvent::new_share(referrer, spread, id.clone());
    LoggableEvent::try_log(event);

    if let Some((qd, perm)) = spread_data {
        Dispatch::<DataState>::new().apply(LoadSpreadMessage(qd, perm));
        navigator.unwrap().push(&crate::web::route::Route::Spread);
    } else if id.is_none() {
        navigator.unwrap().push(&crate::web::route::Route::Landing);
    }

    let image_meta = ImageMeta::from_str(id.unwrap_or_default().as_str()).ok();

    let toggle = Dispatch::<DataState>::new().apply_callback(|_| ToggleDescriptionMessage {});

    if let Some(image_meta) = image_meta {
        let guide = image_meta.guide;
        let card = image_meta.card;

        let src_data = SrcData {
            image: image_meta.image_data,
            spread_option: None,
        };

        html!(
            <>
        <div class="site" >
            <div class="container" style="overflow-x: hidden;" >

        <div class="contained col spread-area" style="margin: auto; margin-top: 0; padding-top: 0;"> //For some reason this margin: auto is needed on mobile
        <Logo clickable={true} invertible={true}/>
        <div class="cards-grid" key="cards-grid" onclick={toggle}>
        <TarotCard top_card={true} {src_data}  style={CardStyle::default()} {description_layout} face_up={true} {card} {guide} />
        </div>
        </div>
        </div>
        </div>
        </>
        )
    } else {
        html!(<> <Logo clickable={true} invertible={true}/> </> )
    }
}
