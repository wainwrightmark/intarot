use base64::Engine;
use yew::prelude::*;

use yewdux::prelude::*;

use crate::data::prelude::ImageMeta;
use crate::state::prelude::*;
use crate::web::card_view::*;

#[derive(Properties, PartialEq)]
pub struct ShareCardViewProps {
    pub image_meta: ImageMeta,
}

#[function_component(ShareCardView)]
pub fn share_card_view(props: &ShareCardViewProps) -> Html {
    let descriptions_state = use_store_value::<ImageDescriptionState>();
    let description = descriptions_state
        .descriptions
        .get(&(props.image_meta.guide, props.image_meta.card))
        .map(|x| *x);

    html!(<CardView top_card={true} src_data={props.image_meta.src_data()} show_continue={true} {description} style={CardStyle::default()} />)
}
