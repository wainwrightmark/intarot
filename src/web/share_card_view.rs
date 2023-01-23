use std::str::FromStr;

use yew::prelude::*;

use yew_hooks::use_search_param;
use yewdux::prelude::*;

use crate::data::prelude::ImageMeta;
use crate::state::prelude::*;
use crate::web::card_view::*;
use crate::web::logo::Logo;

#[derive(Properties, PartialEq)]
pub struct ShareCardViewProps {}

#[function_component(ShareCardView)]
pub fn share_card_view(_props: &ShareCardViewProps) -> Html {
    let descriptions_state = use_store_value::<ImageDescriptionState>();
    let id = use_search_param("id".to_string()).unwrap_or_default();

    let image_meta = base64::Engine::decode(&base64::engine::general_purpose::URL_SAFE, id)
        .ok()
        .and_then(|x| String::from_utf8(x).ok())
        .and_then(|x| ImageMeta::from_str(x.as_str()).ok());

    if let Some(image_meta) = image_meta {
        let description = descriptions_state
            .descriptions
            .get(&(image_meta.guide, image_meta.card))
            .copied();

        html!(
            <>
        <div class="site" style="overflow: hidden ">
            <div class="container" >

        <div class="sm-4 col" style="margin: auto; width: 90vw; height: 100vh; ">
        <Logo clickable={true}/>
        <div class="cards-grid" key="cards-grid">
        <CardView top_card={true} src_data={image_meta.src_data()} show_continue={false} {description} style={CardStyle::default()} />
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
