use yew::prelude::*;
use yewdux::prelude::*;

use crate::data::prelude::*;
use crate::state::prelude::*;
use crate::web::final_card::FinalCard;
use crate::web::tarot_card::TarotCard;


#[derive(Debug, Properties, Clone, PartialEq)]
pub struct SlotProperties {
    pub slot: &'static str,
    pub guide: Guide,
    pub hide: bool,
}

#[function_component(SlotView)]
pub fn slot_view(props: &SlotProperties) -> Html {
    let visibility = if props.hide { "hidden" } else { "initial" };
    let primary = props.guide.primary_color();
    let secondary = props.guide.secondary_color();
    html! {
        <div class="slot fade-in" style={format!("visibility: {visibility}; background-image: linear-gradient({primary}, {secondary});")}>
            {props.slot}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct IndexedCardViewProps {
    pub index: u8,
}

#[function_component(IndexedCardView)]
pub fn indexed_card_view(props: &IndexedCardViewProps) -> Html {
    let image_descriptions_state = use_store_value::<ImageDescriptionState>();
    let metas_state = use_store_value::<ImageMetaState>();
    let data_state = use_store_value::<DataState>();
    let spread_descriptions_state = use_store_value::<SpreadDescriptionState>();

    let slot = spread_descriptions_state.try_get_slot(&data_state.question_data, props.index);
    let description_layout = spread_descriptions_state
        .try_get_layout(&data_state.question_data, props.index)
        .unwrap_or_default();
    let descriptions = &image_descriptions_state.descriptions;

    let metas = &metas_state.metas;

    let top_card = data_state.is_top_card(props.index);
    let faceup = data_state.is_card_facing_up(props.index);
    let style = get_style(props.index, faceup, data_state.as_ref());

    //log::info!("Index: {} Style: {:?}", props.index, style);

    if let Some(meta) = data_state.get_image_meta(props.index, metas){
        let description = descriptions.get(&(meta.guide, meta.card)).unwrap().clone();
        let src_data = SrcData {
            image: meta.image_data,
            spread_option: None,
        };

        html! {
            <TarotCard {top_card} {src_data} {style} {description} {slot} {description_layout}face_up={faceup}  />
        }
    }
    else{
        let src_data = data_state.spread_src(metas);
        html! {
            <FinalCard {top_card} {src_data} {style}  />
        }
    }


}

fn get_style(index: u8, faceup: bool, state: &DataState) -> CardStyle {
    let face_down = !faceup;
    if index == state.top_card_index + 1 {
        CardStyle {
            transform: Some(TransformStyle {
                translate_x: 15,
                translate_y: 5,
                rotate_z: -30,
                face_down
            }),
            hidden: true,
            no_pointer_events: true,
        }
    } else if index > state.top_card_index + 1 {
        let rotate_z = match index % 4 {
            0 => 15 + ((index as i32) * -10),
            1 => -20 + ((index as i32) * 10),
            2 => 20 + ((index as i32) * -10),
            _ => -15 + ((index as i32) * 10),
        };

        let translate_x = match index % 4 {
            0 => 10,
            1 => -10,
            2 => 20,
            _ => -20,
        };

        let translate_y = match index % 4 {
            0 => 10,
            1 => -20,
            2 => -10,
            _ => 20,
        };

        CardStyle {
            transform: Some(TransformStyle {
                translate_x,
                translate_y,
                rotate_z,
                face_down
            }),
            hidden: true,
            no_pointer_events: true,
        }
    } else if index == state.top_card_index {
        CardStyle {
            hidden: false,
            no_pointer_events: false,
            transform: Some(TransformStyle { translate_x: 0, translate_y: 0, rotate_z: 0, face_down }),
        }
    } else {
        let rotate_z = match index % 4 {
            0 => 15 + -(index as i32),
            1 => -20 + (index as i32),
            2 => 20 + -(index as i32),
            _ => -15 + (index as i32),
        };

        let translate_x = match index % 4 {
            0 => 1,
            1 => -1,
            2 => 2,
            _ => -2,
        };

        let translate_y = match index % 4 {
            0 => 1,
            1 => 0,
            2 => -2,
            _ => -1,
        };
        CardStyle {
            hidden: false,
            no_pointer_events: true,
            transform: Some(TransformStyle {
                translate_x,
                translate_y,
                rotate_z,
                face_down
            }),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct TransformStyle {
    pub translate_x: i32,
    pub translate_y: i32,
    pub rotate_z: i32,
    pub face_down: bool
}

impl TransformStyle {
    pub fn get_style(&self) -> String {
        let translate_x = -self.translate_x;
        let translate_y = self.translate_y;
        let rotate_z = self.rotate_z;
        let rotate_y = if self.face_down{180} else{0};
        let transform = format!("transform:  translateX({translate_x}em) translateY({translate_y}em) rotateZ({rotate_z}deg) rotateY({rotate_y}deg); ");

        transform
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct CardStyle {
    pub hidden: bool,
    pub no_pointer_events: bool,
    pub transform: Option<TransformStyle>,
}

impl CardStyle {
    pub fn get_style(&self) -> String {
        let mut a = "".to_string();
        if let Some(transform) = self.transform {
            a.push_str(transform.get_style().as_str());
        }
        if self.hidden {
            a.push_str("visibility: hidden;")
        }
        if self.no_pointer_events {
            a.push_str("pointer-events: none;")
        }
        a
    }
}
